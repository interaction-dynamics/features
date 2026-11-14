//! Read records from the native, Spec Kit and Kiro layouts.

use std::fs;
use std::path::{Path, PathBuf};

use super::parse::{
    count_tasks, date_from_body, first_heading, humanize, normalize_status, split_frontmatter,
    status_from_body,
};
use crate::models::{Record, RecordFile, RecordKind, RecordLinks, RecordSource, TaskProgress};

/// A record before it is attached to a feature
#[derive(Debug)]
pub(super) struct RawRecord {
    pub record: Record,
    /// `feature:` key of the frontmatter
    pub feature: Option<String>,
}

/// Main file of a native folder record, in order of preference
const NATIVE_MAIN_FILES: &[&str] = &["spec.md", "README.md", "index.md", "requirements.md"];
/// Main file of a Spec Kit folder
const SPECKIT_MAIN_FILES: &[&str] = &["spec.md"];
/// Main file of a Kiro folder
const KIRO_MAIN_FILES: &[&str] = &["requirements.md", "spec.md", "README.md"];

/// Order of the sub-files of a folder record, others come after, sorted by name
const FILE_ORDER: &[&str] = &[
    "spec.md",
    "requirements.md",
    "design.md",
    "plan.md",
    "research.md",
    "data-model.md",
    "quickstart.md",
    "tasks.md",
];

/// Extensions of the sub-files worth showing
const TEXT_EXTENSIONS: &[&str] = &[
    "md", "mdx", "txt", "yaml", "yml", "json", "graphql", "gql", "proto", "sql",
];

/// Headings that don't name the spec (Kiro uses them as the title of every file)
const GENERIC_TITLES: &[&str] = &[
    "requirements",
    "requirements document",
    "design",
    "design document",
    "implementation plan",
    "tasks",
    "spec",
    "specification",
];

fn is_markdown(path: &Path) -> bool {
    path.extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("mdx"))
}

/// README and templates are documentation about records, not records
fn is_ignored_file(name: &str) -> bool {
    let lower = name.to_lowercase();
    let stem = lower.trim_end_matches(".mdx").trim_end_matches(".md");
    stem == "readme" || stem == "template" || stem.ends_with("-template") || stem.starts_with('.')
}

fn sorted_entries(dir: &Path) -> Vec<PathBuf> {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .map(|entries| entries.filter_map(|e| e.ok().map(|e| e.path())).collect())
        .unwrap_or_default();
    entries.sort();
    entries
}

fn relative(path: &Path, base: &Path) -> String {
    path.strip_prefix(base)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn name_of(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default()
}

/// Read every record of a native `specs`/`decisions`/`adr` folder: one per
/// Markdown file and one per sub-folder
pub(super) fn read_native_dir(dir: &Path, kind: RecordKind, base: &Path) -> Vec<RawRecord> {
    if !dir.is_dir() {
        return Vec::new();
    }
    sorted_entries(dir)
        .into_iter()
        .filter_map(|path| {
            let name = name_of(&path);
            if path.is_dir() {
                if name.starts_with('.') {
                    return None;
                }
                read_folder_record(&path, kind, RecordSource::Native, NATIVE_MAIN_FILES, base)
            } else if is_markdown(&path) && !is_ignored_file(&name) {
                read_file_record(&path, kind, RecordSource::Native, base)
            } else {
                None
            }
        })
        .collect()
}

/// Read Spec Kit's root `specs/` folder: `NNN-name/` folders with a `spec.md`
/// are Spec Kit specs, loose Markdown files are native specs
pub(super) fn read_speckit_dir(dir: &Path, base: &Path) -> Vec<RawRecord> {
    if !dir.is_dir() {
        return Vec::new();
    }
    sorted_entries(dir)
        .into_iter()
        .filter_map(|path| {
            let name = name_of(&path);
            if path.is_dir() {
                read_folder_record(
                    &path,
                    RecordKind::Spec,
                    RecordSource::Speckit,
                    SPECKIT_MAIN_FILES,
                    base,
                )
            } else if is_markdown(&path) && !is_ignored_file(&name) {
                read_file_record(&path, RecordKind::Spec, RecordSource::Native, base)
            } else {
                None
            }
        })
        .collect()
}

/// Read Kiro's `.kiro/specs/<name>/` folders
pub(super) fn read_kiro_dir(dir: &Path, base: &Path) -> Vec<RawRecord> {
    if !dir.is_dir() {
        return Vec::new();
    }
    sorted_entries(dir)
        .into_iter()
        .filter(|path| path.is_dir())
        .filter_map(|path| {
            read_folder_record(
                &path,
                RecordKind::Spec,
                RecordSource::Kiro,
                KIRO_MAIN_FILES,
                base,
            )
        })
        .collect()
}

fn read_file_record(
    path: &Path,
    kind: RecordKind,
    source: RecordSource,
    base: &Path,
) -> Option<RawRecord> {
    let content = fs::read_to_string(path).ok()?;
    let id = path.file_stem()?.to_string_lossy().to_string();
    Some(build_record(
        kind,
        source,
        id,
        relative(path, base),
        &content,
        Vec::new(),
    ))
}

/// A folder record: the first existing main file, the other text files as sub-files
fn read_folder_record(
    dir: &Path,
    kind: RecordKind,
    source: RecordSource,
    main_files: &[&str],
    base: &Path,
) -> Option<RawRecord> {
    let mut files: Vec<(String, PathBuf)> = walkdir::WalkDir::new(dir)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|entry| entry.depth() == 0 || !name_of(entry.path()).starts_with('.'))
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .is_some_and(|ext| TEXT_EXTENSIONS.iter().any(|t| ext.eq_ignore_ascii_case(t)))
        })
        .map(|entry| (relative(entry.path(), dir), entry.path().to_path_buf()))
        .collect();

    let main_index = main_files
        .iter()
        .find_map(|main| files.iter().position(|(name, _)| name == main))
        .or_else(|| {
            // Native folders fall back to their first top-level Markdown file
            (source == RecordSource::Native).then(|| {
                files
                    .iter()
                    .position(|(name, path)| !name.contains('/') && is_markdown(path))
            })?
        })?;
    let (_, main_path) = files.remove(main_index);
    let content = fs::read_to_string(&main_path).ok()?;

    files.sort_by_key(|(name, _)| {
        let rank = FILE_ORDER
            .iter()
            .position(|f| f == name)
            .unwrap_or(FILE_ORDER.len() + usize::from(name.contains('/')));
        (rank, name.clone())
    });
    let sub_files = files
        .into_iter()
        .filter_map(|(name, path)| {
            Some(RecordFile {
                content: fs::read_to_string(&path).ok()?,
                path: relative(&path, base),
                name,
            })
        })
        .collect();

    Some(build_record(
        kind,
        source,
        name_of(dir),
        relative(&main_path, base),
        &content,
        sub_files,
    ))
}

/// Strip a link down to a record id: `../decisions/002-use-jwt.md` → `002-use-jwt`
pub(super) fn normalize_link(link: &str) -> String {
    let name = link
        .trim()
        .trim_end_matches('/')
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or_default();
    name.strip_suffix(".md")
        .or_else(|| name.strip_suffix(".mdx"))
        .unwrap_or(name)
        .to_string()
}

fn links(values: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for value in values.iter().map(|v| normalize_link(v)) {
        if !value.is_empty() && !out.contains(&value) {
            out.push(value);
        }
    }
    out
}

fn build_record(
    kind: RecordKind,
    source: RecordSource,
    id: String,
    path: String,
    content: &str,
    files: Vec<RecordFile>,
) -> RawRecord {
    let (frontmatter, body) = split_frontmatter(content);

    let heading = first_heading(body).filter(|heading| {
        source != RecordSource::Kiro || !GENERIC_TITLES.contains(&heading.to_lowercase().as_str())
    });
    let title = frontmatter
        .string("title")
        .or(heading)
        .unwrap_or_else(|| humanize(&id));

    let status = frontmatter
        .string("status")
        .and_then(|status| normalize_status(&status))
        .or_else(|| status_from_body(body));
    let date = frontmatter.string("date").or_else(|| date_from_body(body));

    let mut specs = frontmatter.list("specs");
    specs.extend(frontmatter.list("spec"));
    let mut superseded_by = frontmatter.list("superseded_by");
    superseded_by.extend(frontmatter.list("superseded-by"));
    let record_links = RecordLinks {
        decisions: links(frontmatter.list("decisions")),
        specs: links(specs),
        supersedes: links(frontmatter.list("supersedes")),
        superseded_by: links(superseded_by),
    };

    let tasks = (kind == RecordKind::Spec)
        .then(|| {
            let source_text = files
                .iter()
                .find(|file| file.name == "tasks.md")
                .map_or(body, |file| file.content.as_str());
            let (done, total) = count_tasks(source_text);
            (total > 0).then_some(TaskProgress { done, total })
        })
        .flatten();

    RawRecord {
        feature: frontmatter.string("feature"),
        record: Record {
            kind,
            id,
            title,
            status,
            date,
            path,
            content: body.trim_start_matches(['\n', '\r']).to_string(),
            files,
            source,
            links: record_links,
            tasks,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_link() {
        assert_eq!(normalize_link("002-use-jwt"), "002-use-jwt");
        assert_eq!(normalize_link("../decisions/002-use-jwt.md"), "002-use-jwt");
        assert_eq!(normalize_link("specs/003-sso/"), "003-sso");
    }

    #[test]
    fn test_ignored_files() {
        assert!(is_ignored_file("README.md"));
        assert!(is_ignored_file("adr-template.md"));
        assert!(is_ignored_file("template.md"));
        assert!(!is_ignored_file("001-use-jwt.md"));
    }

    #[test]
    fn test_folder_record_with_sub_files() {
        let temp = tempfile::TempDir::new().unwrap();
        let base = temp.path();
        let dir = base.join("specs").join("003-sso");
        fs::create_dir_all(dir.join("contracts")).unwrap();
        fs::write(dir.join("spec.md"), "---\nstatus: draft\n---\n# SSO\n").unwrap();
        fs::write(dir.join("tasks.md"), "- [x] a\n- [ ] b\n").unwrap();
        fs::write(dir.join("plan.md"), "# Plan\n").unwrap();
        fs::write(dir.join("contracts").join("api.yaml"), "openapi: 3.0.0\n").unwrap();
        fs::write(dir.join("logo.png"), [0u8, 1, 2]).unwrap();

        let records = read_native_dir(&base.join("specs"), RecordKind::Spec, base);
        assert_eq!(records.len(), 1);
        let record = &records[0].record;
        assert_eq!(record.id, "003-sso");
        assert_eq!(record.title, "SSO");
        assert_eq!(record.status.as_deref(), Some("draft"));
        assert_eq!(record.path, "specs/003-sso/spec.md");
        let names: Vec<&str> = record.files.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, vec!["plan.md", "tasks.md", "contracts/api.yaml"]);
        assert_eq!(record.tasks, Some(TaskProgress { done: 1, total: 2 }));
    }
}
