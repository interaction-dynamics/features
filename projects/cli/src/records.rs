//! Specs and decision records.
//!
//! Records live next to the code they describe:
//!
//! * per feature, in `__docs__/specs`, `__docs__/decisions`, `.docs/specs` and
//!   `.docs/decisions` (every folder is read);
//! * at the root of the scanned folder, in `docs/specs`, `docs/decisions` and
//!   `docs/adr`. A root record with a `feature: <name or path>` frontmatter field
//!   belongs to that feature, every other one is a project-level record;
//! * at the root too, in the layouts of spec-driven tools: GitHub Spec Kit
//!   (`specs/NNN-name/`) and Kiro (`.kiro/specs/<name>/`), see [`layouts`].
//!
//! A record is a markdown file (`specs/001-login.md`) or a folder
//! (`specs/003-sso/`) whose main file is `spec.md`, `decision.md`, `README.md`
//! or `index.md`, the other markdown files being its sub-files.
//!
//! Metadata comes from the YAML frontmatter (`status`, `date`, `title`,
//! `decisions`, `spec`/`specs`, `supersedes`, `feature`), with fallbacks for
//! classic MADR files (`**Status**: Accepted` line or `## Status` section).

use anyhow::{Context, Result, bail};
use serde_yaml::Value;
use std::fs;
use std::path::{Path, PathBuf};

use crate::models::{
    Feature, Record, RecordFile, RecordKind, RecordLinks, RecordSource, TaskProgress,
};

mod layouts;

/// Feature-level documentation folders, read in this order
const FEATURE_DOCS_DIRS: &[&str] = &["__docs__", ".docs"];

/// Main file of a folder record, by order of preference
const FOLDER_MAIN_FILES: &[&str] = &["spec.md", "decision.md", "README.md", "index.md"];

/// Statuses recognized as the first word of a longer status line
/// (`Superseded by ADR-0005` is `superseded`)
const KNOWN_STATUSES: &[&str] = &[
    "draft",
    "proposed",
    "accepted",
    "rejected",
    "deprecated",
    "superseded",
    "implemented",
];

/// Statuses meaning a record no longer applies
pub const INACTIVE_STATUSES: &[&str] = &["superseded", "rejected", "deprecated"];

/// Read the records of the feature whose folder is `feature_dir`.
/// Paths are made relative to `base_path`.
pub fn read_feature_records(feature_dir: &Path, base_path: &Path) -> Vec<Record> {
    let mut records = Vec::new();
    for docs in FEATURE_DOCS_DIRS {
        let docs_dir = feature_dir.join(docs);
        records.extend(read_records_dir(
            &docs_dir.join("specs"),
            RecordKind::Spec,
            base_path,
        ));
        records.extend(read_records_dir(
            &docs_dir.join("decisions"),
            RecordKind::Decision,
            base_path,
        ));
    }
    records
}

/// Read the records at the root of `base_path`: the `docs/` folder, Spec Kit's
/// `specs/` folder and Kiro's `.kiro/specs/` folder
pub fn read_root_records(base_path: &Path) -> Vec<Record> {
    let docs_dir = base_path.join("docs");
    let mut records = read_records_dir(&docs_dir.join("specs"), RecordKind::Spec, base_path);
    for folder in ["decisions", "adr"] {
        records.extend(read_records_dir(
            &docs_dir.join(folder),
            RecordKind::Decision,
            base_path,
        ));
    }
    records.extend(layouts::read_speckit_dir(
        &base_path.join("specs"),
        base_path,
    ));
    records.extend(layouts::read_kiro_dir(
        &base_path.join(".kiro").join("specs"),
        base_path,
    ));
    records
}

/// Read every record of one `specs`/`decisions` folder, sorted by name
fn read_records_dir(dir: &Path, kind: RecordKind, base_path: &Path) -> Vec<Record> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut paths: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
    paths.sort();

    paths
        .iter()
        .filter_map(|path| {
            if path.is_dir() {
                read_folder_record(path, kind, base_path)
            } else if is_markdown(path) && !is_readme(path) {
                let raw = fs::read_to_string(path).ok()?;
                let id = path.file_stem()?.to_string_lossy().to_string();
                Some(parse_record(
                    kind,
                    &id,
                    &relative_path(path, base_path),
                    &raw,
                ))
            } else {
                None
            }
        })
        .collect()
}

/// Read a folder record: its main file and its other markdown files
fn read_folder_record(dir: &Path, kind: RecordKind, base_path: &Path) -> Option<Record> {
    let id = dir.file_name()?.to_string_lossy().to_string();
    let main = FOLDER_MAIN_FILES
        .iter()
        .map(|name| dir.join(name))
        .find(|path| path.is_file());

    let mut sub_paths: Vec<PathBuf> = fs::read_dir(dir)
        .ok()?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|path| path.is_file() && is_markdown(path) && Some(path) != main.as_ref())
        .collect();
    sub_paths.sort();

    if main.is_none() && sub_paths.is_empty() {
        return None;
    }

    let mut record = match &main {
        Some(path) => {
            let raw = fs::read_to_string(path).ok()?;
            parse_record(kind, &id, &relative_path(path, base_path), &raw)
        }
        None => parse_record(kind, &id, &relative_path(dir, base_path), ""),
    };

    record.files = sub_paths
        .iter()
        .filter_map(|path| {
            Some(RecordFile {
                name: path.file_name()?.to_string_lossy().to_string(),
                path: relative_path(path, base_path),
                content: fs::read_to_string(path).ok()?,
            })
        })
        .collect();

    let mut tasks = count_tasks(&record.content);
    for file in &record.files {
        let file_tasks = count_tasks(&file.content);
        tasks.done += file_tasks.done;
        tasks.total += file_tasks.total;
    }
    record.tasks = (tasks.total > 0).then_some(tasks);

    Some(record)
}

fn is_markdown(path: &Path) -> bool {
    path.extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("mdx"))
}

fn is_readme(path: &Path) -> bool {
    path.file_stem()
        .is_some_and(|stem| stem.eq_ignore_ascii_case("readme"))
}

fn relative_path(path: &Path, base_path: &Path) -> String {
    path.strip_prefix(base_path)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// Build a record from the raw content of its main file
pub fn parse_record(kind: RecordKind, id: &str, path: &str, raw: &str) -> Record {
    let (frontmatter, body) = split_frontmatter(raw);
    let field = |keys: &[&str]| -> Option<&Value> {
        let map = frontmatter.as_ref()?;
        keys.iter().find_map(|key| map.get(*key))
    };
    let list = |keys: &[&str]| -> Vec<String> {
        let mut values: Vec<String> = Vec::new();
        if let Some(map) = frontmatter.as_ref() {
            for key in keys {
                if let Some(value) = map.get(*key) {
                    for item in yaml_list(value) {
                        if !values.contains(&item) {
                            values.push(item);
                        }
                    }
                }
            }
        }
        values
    };

    let status = field(&["status"])
        .and_then(yaml_string)
        .or_else(|| section_line(body, "status"))
        .or_else(|| field_line(body, "status"))
        .map(|status| normalize_status(&status))
        .filter(|status| !status.is_empty());

    let date = field(&["date"])
        .and_then(yaml_string)
        .or_else(|| field_line(body, "date"));

    let title = field(&["title"])
        .and_then(yaml_string)
        .or_else(|| first_heading(body))
        .unwrap_or_else(|| id.to_string());

    let tasks = count_tasks(body);

    Record {
        kind,
        id: id.to_string(),
        title,
        status,
        date,
        path: path.to_string(),
        content: body.trim_start_matches(['\n', '\r']).to_string(),
        files: Vec::new(),
        source: RecordSource::Native,
        links: RecordLinks {
            decisions: list(&["decisions", "decision"]),
            specs: list(&["specs", "spec"]),
            supersedes: list(&["supersedes"]),
            superseded_by: list(&["superseded_by", "superseded-by"]),
        },
        tasks: (tasks.total > 0).then_some(tasks),
        feature: field(&["feature"]).and_then(yaml_string),
    }
}

/// Split the YAML frontmatter (between `---` lines at the top) from the body
fn split_frontmatter(raw: &str) -> (Option<serde_yaml::Mapping>, &str) {
    let raw = raw.strip_prefix('\u{feff}').unwrap_or(raw);
    let Some(rest) = raw
        .strip_prefix("---\n")
        .or_else(|| raw.strip_prefix("---\r\n"))
    else {
        return (None, raw);
    };

    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        if line.trim_end() == "---" {
            let yaml = &rest[..offset];
            let body = &rest[offset + line.len()..];
            let mapping = match serde_yaml::from_str::<Value>(yaml) {
                Ok(Value::Mapping(mapping)) => Some(mapping),
                _ => None,
            };
            return (mapping, body);
        }
        offset += line.len();
    }
    (None, raw)
}

fn yaml_string(value: &Value) -> Option<String> {
    let text = match value {
        Value::String(s) => s.trim().to_string(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        _ => return None,
    };
    (!text.is_empty()).then_some(text)
}

/// A single value or a list of values; strings are also split on commas
fn yaml_list(value: &Value) -> Vec<String> {
    let items: Vec<String> = match value {
        Value::Sequence(items) => items.iter().filter_map(yaml_string).collect(),
        other => yaml_string(other).into_iter().collect(),
    };
    items
        .iter()
        .flat_map(|item| item.split(','))
        .map(|item| item.trim().trim_end_matches(".md").to_string())
        .filter(|item| !item.is_empty())
        .collect()
}

/// Lines of `body` outside code fences
fn prose_lines(body: &str) -> impl Iterator<Item = &str> {
    let mut in_fence = false;
    body.lines().filter(move |line| {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            return false;
        }
        !in_fence
    })
}

/// Remove list markers and bold markers around a line
fn strip_markers(line: &str) -> String {
    line.trim()
        .trim_start_matches(['*', '-', '+'])
        .replace("**", "")
        .replace("__", "")
        .trim()
        .to_string()
}

/// Value of a `Key: value` line (`**Status**: Accepted`, `* Status: proposed`)
fn field_line(body: &str, key: &str) -> Option<String> {
    prose_lines(body).find_map(|line| {
        let line = strip_markers(line);
        let (name, value) = line.split_once(':')?;
        let value = value.trim();
        (name.trim().eq_ignore_ascii_case(key) && !value.is_empty()).then(|| value.to_string())
    })
}

/// First line of the `## <name>` section
fn section_line(body: &str, name: &str) -> Option<String> {
    let mut in_section = false;
    for line in prose_lines(body) {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            if in_section {
                return None;
            }
            in_section = trimmed
                .trim_start_matches('#')
                .trim()
                .eq_ignore_ascii_case(name);
        } else if in_section && !trimmed.is_empty() {
            return Some(strip_markers(trimmed));
        }
    }
    None
}

fn normalize_status(status: &str) -> String {
    let status = status.trim().to_lowercase();
    let first_word: String = status.chars().take_while(|c| c.is_alphabetic()).collect();
    if KNOWN_STATUSES.contains(&first_word.as_str()) {
        first_word
    } else {
        status
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_string()
    }
}

/// Text of the first `#` heading, else of the first heading of any level
fn first_heading(body: &str) -> Option<String> {
    let headings: Vec<&str> = prose_lines(body)
        .map(str::trim)
        .filter(|line| line.starts_with('#'))
        .collect();
    headings
        .iter()
        .find(|line| line.starts_with("# "))
        .or_else(|| headings.first())
        .map(|line| line.trim_start_matches('#').trim().replace('`', ""))
        .filter(|title| !title.is_empty())
}

/// Count `- [ ]` and `- [x]` lines outside code fences
pub fn count_tasks(body: &str) -> TaskProgress {
    let mut progress = TaskProgress::default();
    for line in prose_lines(body) {
        let line = line.trim_start();
        let Some(rest) = line
            .strip_prefix("- ")
            .or_else(|| line.strip_prefix("* "))
            .or_else(|| line.strip_prefix("+ "))
        else {
            continue;
        };
        let rest = rest.trim_start();
        if rest.starts_with("[ ]") {
            progress.total += 1;
        } else if rest.starts_with("[x]") || rest.starts_with("[X]") {
            progress.total += 1;
            progress.done += 1;
        }
    }
    progress
}

/// Index of the record `target` points to: same id, or same number prefix
/// (`002` matches `002-use-jwt`). Records of `kind` are preferred.
fn find_record(records: &[Record], target: &str, kind: Option<RecordKind>) -> Option<usize> {
    let matches_kind = |r: &Record| kind.is_none_or(|k| r.kind == k);
    let exact =
        |pred: &dyn Fn(&Record) -> bool| records.iter().position(|r| r.id == target && pred(r));
    if let Some(i) = exact(&matches_kind).or_else(|| exact(&|_| true)) {
        return Some(i);
    }

    let by_number = |pred: &dyn Fn(&Record) -> bool| -> Option<usize> {
        let found: Vec<usize> = records
            .iter()
            .enumerate()
            .filter(|(_, r)| pred(r) && record_number(&r.id) == Some(target))
            .map(|(i, _)| i)
            .collect();
        (found.len() == 1).then(|| found[0])
    };
    by_number(&matches_kind).or_else(|| by_number(&|_| true))
}

/// Leading number of an id: `002` for `002-use-jwt`
fn record_number(id: &str) -> Option<&str> {
    let end = id.find(|c: char| !c.is_ascii_digit()).unwrap_or(id.len());
    (end > 0).then(|| &id[..end])
}

fn push_unique(list: &mut Vec<String>, value: &str) {
    if !list.iter().any(|v| v == value) {
        list.push(value.to_string());
    }
}

/// Resolve the links of `records` against each other: links pointing at a
/// record are rewritten to its full id, and every link is mirrored on the
/// record it points to. Links to unknown records are kept as written.
pub fn resolve_links(records: &mut [Record]) {
    // Rewrite links to the full id of the record they point to
    let snapshot: Vec<Record> = records.to_vec();
    for record in records.iter_mut() {
        let own_kind = record.kind;
        let resolve = |links: &mut Vec<String>, kind: Option<RecordKind>| {
            let resolved: Vec<String> = links
                .iter()
                .map(|link| {
                    find_record(&snapshot, link, kind)
                        .map(|i| snapshot[i].id.clone())
                        .unwrap_or_else(|| link.clone())
                })
                .collect();
            links.clear();
            for link in resolved {
                push_unique(links, &link);
            }
        };
        resolve(&mut record.links.decisions, Some(RecordKind::Decision));
        resolve(&mut record.links.specs, Some(RecordKind::Spec));
        resolve(&mut record.links.supersedes, Some(own_kind));
        resolve(&mut record.links.superseded_by, Some(own_kind));
    }

    // Mirror every link on its target
    let snapshot: Vec<Record> = records.to_vec();
    for source in &snapshot {
        let mirror = |records: &mut [Record],
                      target: &str,
                      kind: Option<RecordKind>,
                      field: fn(&mut RecordLinks) -> &mut Vec<String>| {
            if let Some(i) = find_record(&snapshot, target, kind)
                && (snapshot[i].id != source.id || snapshot[i].kind != source.kind)
            {
                push_unique(field(&mut records[i].links), &source.id);
            }
        };
        let back_field: fn(&mut RecordLinks) -> &mut Vec<String> = match source.kind {
            RecordKind::Spec => |links| &mut links.specs,
            RecordKind::Decision => |links| &mut links.decisions,
        };
        for target in &source.links.decisions {
            mirror(records, target, Some(RecordKind::Decision), back_field);
        }
        for target in &source.links.specs {
            mirror(records, target, Some(RecordKind::Spec), back_field);
        }
        for target in &source.links.supersedes {
            mirror(records, target, Some(source.kind), |links| {
                &mut links.superseded_by
            });
        }
        for target in &source.links.superseded_by {
            mirror(records, target, Some(source.kind), |links| {
                &mut links.supersedes
            });
        }
    }
}

/// Resolve the links of `local` records against themselves and `shared` ones
/// (the project-level records). Only `local` is changed.
pub fn resolve_links_with(local: &mut Vec<Record>, shared: &[Record]) {
    if local.is_empty() {
        return;
    }
    let count = local.len();
    local.extend(shared.iter().cloned());
    resolve_links(local);
    local.truncate(count);
}

/// Sort records by date, then kind and id. Records without a date come first.
pub fn sort_records(records: &mut [Record]) {
    records.sort_by(|a, b| {
        a.date
            .cmp(&b.date)
            .then_with(|| a.id.cmp(&b.id))
            .then_with(|| a.path.cmp(&b.path))
    });
}

/// Path of the feature a `feature:` frontmatter value names: a feature path,
/// a feature name (case insensitive), or the last segment of a feature path
pub fn find_feature_path(features: &[Feature], target: &str) -> Option<String> {
    let target = target.trim().trim_end_matches('/');
    let mut all: Vec<&Feature> = Vec::new();
    fn collect<'a>(features: &'a [Feature], all: &mut Vec<&'a Feature>) {
        for feature in features {
            all.push(feature);
            collect(&feature.features, all);
        }
    }
    collect(features, &mut all);

    all.iter()
        .find(|f| f.path == target)
        .or_else(|| all.iter().find(|f| f.name.eq_ignore_ascii_case(target)))
        .or_else(|| {
            all.iter()
                .find(|f| f.path.rsplit('/').next() == Some(target))
        })
        .map(|f| f.path.clone())
}

/// Path of the feature a Spec Kit or Kiro folder name names: the name
/// without its `NNN-` prefix, slugified, is a feature's slugified name or the
/// last segment of its path (first match in tree order)
fn feature_path_from_folder(features: &[Feature], folder: &str) -> Option<String> {
    let target = slugify(layouts::strip_number_prefix(folder));
    if target.is_empty() {
        return None;
    }
    features.iter().find_map(|feature| {
        let segment = feature.path.rsplit('/').next().unwrap_or_default();
        if slugify(&feature.name) == target || slugify(segment) == target {
            Some(feature.path.clone())
        } else {
            feature_path_from_folder(&feature.features, folder)
        }
    })
}

/// Root records split into the ones attached to a feature (with its path)
/// and the project-level ones. A record is attached by its `feature:`
/// frontmatter field, or for Spec Kit and Kiro records by its folder name.
pub fn split_root_records(
    root_records: Vec<Record>,
    features: &[Feature],
) -> (Vec<(String, Record)>, Vec<Record>) {
    let mut attached = Vec::new();
    let mut project = Vec::new();
    for record in root_records {
        let by_frontmatter = record
            .feature
            .as_deref()
            .and_then(|target| find_feature_path(features, target));
        let by_folder = || {
            (record.feature.is_none() && !record.source.is_native())
                .then(|| feature_path_from_folder(features, &record.id))
                .flatten()
        };
        match by_frontmatter.or_else(by_folder) {
            Some(path) => attached.push((path, record)),
            None => project.push(record),
        }
    }
    (attached, project)
}

/// The project-level records of `base_path`: root records not attached to a
/// feature, with their links resolved
pub fn project_records(features: &[Feature], base_path: &Path) -> Vec<Record> {
    let (_, mut project) = split_root_records(read_root_records(base_path), features);
    resolve_links(&mut project);
    sort_records(&mut project);
    project
}

/// Problems found in the records of a feature (or of the project): broken
/// links, missing status, superseded records without replacement and
/// implemented specs with open tasks. `scope` is every record the links can
/// point to.
pub fn record_warnings(records: &[Record], scope: &[Record], owner: &str) -> Vec<String> {
    let mut warnings = Vec::new();
    let exists = |id: &str| scope.iter().chain(records.iter()).any(|r| r.id == id);

    for record in records {
        let name = format!("{} '{}' ({})", record.kind.label(), record.id, owner);
        let links = &record.links;
        for (field, targets) in [
            ("decisions", &links.decisions),
            ("specs", &links.specs),
            ("supersedes", &links.supersedes),
            ("superseded_by", &links.superseded_by),
        ] {
            for target in targets {
                if !exists(target) {
                    warnings.push(format!(
                        "{name} links to unknown record '{target}' in `{field}` ({})",
                        record.path
                    ));
                }
            }
        }

        match record.status.as_deref() {
            // Kiro has no status, Spec Kit's is optional
            None if record.source.is_native() => {
                warnings.push(format!("{name} has no status ({})", record.path))
            }
            Some("superseded") if links.superseded_by.is_empty() => warnings.push(format!(
                "{name} is superseded but no record supersedes it ({})",
                record.path
            )),
            Some("implemented") if record.kind == RecordKind::Spec => {
                if let Some(tasks) = record.tasks.filter(|t| t.done < t.total) {
                    warnings.push(format!(
                        "{name} is implemented but has {} open task(s) ({})",
                        tasks.total - tasks.done,
                        record.path
                    ));
                }
            }
            _ => {}
        }
    }
    warnings
}

/// Lowercase, dash-separated version of `title` for a file name
pub fn slugify(title: &str) -> String {
    let mut slug = String::new();
    for c in title.trim().chars().flat_map(char::to_lowercase) {
        if c.is_alphanumeric() {
            slug.push(c);
        } else if !slug.ends_with('-') && !slug.is_empty() {
            slug.push('-');
        }
    }
    slug.trim_end_matches('-').to_string()
}

/// Next free `NNN` number in a records folder
pub fn next_record_number(dir: &Path) -> u32 {
    let max = fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter_map(|e| {
                    let name = e.file_name().to_string_lossy().to_string();
                    record_number(&name).and_then(|n| n.parse::<u32>().ok())
                })
                .max()
                .unwrap_or(0)
        })
        .unwrap_or(0);
    max + 1
}

/// Markdown of a new record
pub fn record_template(kind: RecordKind, title: &str, date: &str) -> String {
    match kind {
        RecordKind::Spec => format!(
            "---\nstatus: draft\ndate: {date}\ndecisions: []\n---\n\n# {title}\n\n\
             ## Context\n\nWhy this change is needed.\n\n\
             ## Requirements\n\n- What must be true when it is done.\n\n\
             ## Out of scope\n\n- What is explicitly not covered.\n\n\
             ## Plan\n\n- [ ] First step\n"
        ),
        RecordKind::Decision => format!(
            "---\nstatus: proposed\ndate: {date}\n---\n\n# {title}\n\n\
             ## Context and Problem Statement\n\nWhat is the problem, and why does it need a decision?\n\n\
             ## Considered Options\n\n- Option 1\n- Option 2\n\n\
             ## Decision Outcome\n\nChosen option: \"Option 1\", because...\n\n\
             ### Consequences\n\n- Good, because...\n- Bad, because...\n"
        ),
    }
}

/// Write a new numbered record in the records folder of `feature`
/// (a feature name or path, `.` for the project-level `docs/` folder).
/// Returns the path of the created file.
pub fn create_record(
    base_path: &Path,
    features: &[Feature],
    kind: RecordKind,
    feature: &str,
    title: &str,
    date: &str,
) -> Result<PathBuf> {
    let slug = slugify(title);
    if slug.is_empty() {
        bail!("the title must contain at least one letter or digit");
    }

    let dir = if feature == "." {
        base_path.join("docs").join(kind.folder_name())
    } else {
        let feature_path = find_feature_path(features, feature)
            .with_context(|| format!("no feature named or at path `{feature}`"))?;
        base_path
            .join(feature_path)
            .join("__docs__")
            .join(kind.folder_name())
    };

    fs::create_dir_all(&dir)
        .with_context(|| format!("could not create directory `{}`", dir.display()))?;
    let path = dir.join(format!("{:03}-{slug}.md", next_record_number(&dir)));
    if path.exists() {
        bail!("`{}` already exists", path.display());
    }
    fs::write(&path, record_template(kind, title.trim(), date))
        .with_context(|| format!("could not write `{}`", path.display()))?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn feature(name: &str, path: &str, features: Vec<Feature>) -> Feature {
        Feature {
            name: name.to_string(),
            description: String::new(),
            owner: String::new(),
            is_owner_inherited: false,
            path: path.to_string(),
            features,
            meta: BTreeMap::new(),
            changes: Vec::new(),
            decisions: Vec::new(),
            records: Vec::new(),
            stats: None,
            dependencies: Vec::new(),
        }
    }

    fn record(kind: RecordKind, id: &str, raw: &str) -> Record {
        parse_record(kind, id, &format!("{id}.md"), raw)
    }

    #[test]
    fn test_parse_frontmatter() {
        let r = record(
            RecordKind::Spec,
            "001-login-flow",
            "---\nstatus: Accepted\ndate: 2026-01-02\ndecisions: [002-use-jwt, \"003\"]\nsupersedes: 000-old\n---\n\n# Login flow\n\n- [x] one\n- [ ] two\n",
        );
        assert_eq!(r.title, "Login flow");
        assert_eq!(r.status.as_deref(), Some("accepted"));
        assert_eq!(r.date.as_deref(), Some("2026-01-02"));
        assert_eq!(r.links.decisions, vec!["002-use-jwt", "003"]);
        assert_eq!(r.links.supersedes, vec!["000-old"]);
        assert_eq!(r.tasks, Some(TaskProgress { done: 1, total: 2 }));
        assert!(r.content.starts_with("# Login flow"));
    }

    #[test]
    fn test_parse_classic_madr_status_line_and_section() {
        let bold = record(
            RecordKind::Decision,
            "0001-x",
            "# 1. Use Postgres\n\n**Status**: Superseded by [ADR-0005](0005-x.md)\n\nDate: 2020-05-01\n",
        );
        assert_eq!(bold.title, "1. Use Postgres");
        assert_eq!(bold.status.as_deref(), Some("superseded"));
        assert_eq!(bold.date.as_deref(), Some("2020-05-01"));

        let list = record(RecordKind::Decision, "a", "# A\n\n* Status: proposed\n");
        assert_eq!(list.status.as_deref(), Some("proposed"));

        let section = record(
            RecordKind::Decision,
            "b",
            "# B\n\n## Status\n\nAccepted\n\n## Context\n\nStatus: nope\n",
        );
        assert_eq!(section.status.as_deref(), Some("accepted"));

        let none = record(RecordKind::Decision, "c", "Just text\n");
        assert_eq!(none.status, None);
        assert_eq!(none.title, "c");
    }

    #[test]
    fn test_tasks_ignore_code_fences() {
        let tasks = count_tasks("- [x] a\n```\n- [ ] not a task\n```\n  * [X] b\n- [ ] c\n");
        assert_eq!(tasks, TaskProgress { done: 2, total: 3 });
    }

    #[test]
    fn test_resolve_links_both_directions() {
        let mut records = vec![
            record(
                RecordKind::Spec,
                "001-login-flow",
                "---\ndecisions: [\"002\"]\n---\n# Login\n",
            ),
            record(
                RecordKind::Decision,
                "002-use-jwt",
                "---\nsupersedes: 001-use-sessions\n---\n# JWT\n",
            ),
            record(RecordKind::Decision, "001-use-sessions", "# Sessions\n"),
            record(
                RecordKind::Decision,
                "003-refresh",
                "---\nspec: 001-login-flow\n---\n# Refresh\n",
            ),
        ];
        resolve_links(&mut records);

        assert_eq!(
            records[0].links.decisions,
            vec!["002-use-jwt", "003-refresh"]
        );
        assert_eq!(records[1].links.specs, vec!["001-login-flow"]);
        assert_eq!(records[2].links.superseded_by, vec!["002-use-jwt"]);
        assert_eq!(records[3].links.specs, vec!["001-login-flow"]);
    }

    #[test]
    fn test_warnings() {
        let mut records = vec![
            record(
                RecordKind::Spec,
                "001-a",
                "---\nstatus: implemented\ndecisions: [099-missing]\n---\n- [ ] open\n",
            ),
            record(
                RecordKind::Decision,
                "002-b",
                "---\nstatus: superseded\n---\n",
            ),
            record(RecordKind::Decision, "003-c", "# no status\n"),
        ];
        resolve_links(&mut records);
        let warnings = record_warnings(&records, &[], "feature");
        assert_eq!(warnings.len(), 4, "{warnings:#?}");
        assert!(warnings[0].contains("unknown record '099-missing'"));
        assert!(warnings[1].contains("implemented but has 1 open task"));
        assert!(warnings[2].contains("superseded but no record supersedes it"));
        assert!(warnings[3].contains("has no status"));
    }

    #[test]
    fn test_read_files_folders_and_root_records() {
        let temp = tempfile::TempDir::new().unwrap();
        let base = temp.path();
        let feature_dir = base.join("features").join("auth");
        let specs = feature_dir.join("__docs__").join("specs");
        let decisions = feature_dir.join(".docs").join("decisions");
        fs::create_dir_all(specs.join("003-sso")).unwrap();
        fs::create_dir_all(&decisions).unwrap();
        fs::write(specs.join("README.md"), "# Index\n").unwrap();
        fs::write(
            specs.join("001-login.md"),
            "---\nstatus: draft\n---\n# Login\n",
        )
        .unwrap();
        fs::write(specs.join("003-sso").join("spec.md"), "# SSO\n").unwrap();
        fs::write(specs.join("003-sso").join("tasks.md"), "- [x] a\n- [ ] b\n").unwrap();
        fs::write(decisions.join("002-jwt.md"), "# JWT\n").unwrap();

        let records = read_feature_records(&feature_dir, base);
        let ids: Vec<&str> = records.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, vec!["001-login", "003-sso", "002-jwt"]);
        let sso = &records[1];
        assert_eq!(sso.path, "features/auth/__docs__/specs/003-sso/spec.md");
        assert_eq!(sso.files.len(), 1);
        assert_eq!(sso.files[0].name, "tasks.md");
        assert_eq!(sso.tasks, Some(TaskProgress { done: 1, total: 2 }));
        assert_eq!(records[2].kind, RecordKind::Decision);

        let docs = base.join("docs");
        fs::create_dir_all(docs.join("adr")).unwrap();
        fs::create_dir_all(docs.join("specs")).unwrap();
        fs::write(docs.join("adr").join("001-monorepo.md"), "# Monorepo\n").unwrap();
        fs::write(
            docs.join("specs").join("010-auth.md"),
            "---\nfeature: auth\n---\n# Auth spec\n",
        )
        .unwrap();
        let features = vec![feature("Auth", "features/auth", vec![])];
        let (attached, project) = split_root_records(read_root_records(base), &features);
        assert_eq!(attached.len(), 1);
        assert_eq!(attached[0].0, "features/auth");
        assert_eq!(project.len(), 1);
        assert_eq!(project[0].id, "001-monorepo");
    }

    #[test]
    fn test_create_record() {
        let temp = tempfile::TempDir::new().unwrap();
        let base = temp.path();
        let features = vec![feature("Auth", "features/auth", vec![])];
        let dir = base.join("features/auth/__docs__/decisions");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("004-old.md"), "").unwrap();

        let path = create_record(
            base,
            &features,
            RecordKind::Decision,
            "Auth",
            "Use JWT tokens!",
            "2026-01-01",
        )
        .unwrap();
        assert_eq!(path, dir.join("005-use-jwt-tokens.md"));
        let r = parse_record(
            RecordKind::Decision,
            "x",
            "x",
            &fs::read_to_string(&path).unwrap(),
        );
        assert_eq!(r.status.as_deref(), Some("proposed"));
        assert_eq!(r.title, "Use JWT tokens!");

        let root = create_record(
            base,
            &features,
            RecordKind::Spec,
            ".",
            "Big plan",
            "2026-01-01",
        )
        .unwrap();
        assert_eq!(root, base.join("docs/specs/001-big-plan.md"));

        assert!(create_record(base, &features, RecordKind::Spec, "nope", "x", "d").is_err());
    }
}
