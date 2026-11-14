//! Specs and decision records.
//!
//! Records are read from:
//!
//! - each feature's `__docs__/{decisions,specs}` and `.docs/{decisions,specs}`
//! - the scanned folder's `docs/{decisions,adr,specs}` (native layout)
//! - the scanned folder's `specs/NNN-name/spec.md` ([GitHub Spec Kit](https://github.com/github/spec-kit))
//! - the scanned folder's `.kiro/specs/<name>/requirements.md` ([Kiro](https://kiro.dev/docs/specs/))
//!
//! A root record is attached to a feature by its `feature:` frontmatter key or,
//! for Spec Kit and Kiro, by its folder name (see [`find_feature`]). The others
//! are project-level records. Links between records are then resolved in both
//! directions (see [`resolve_links`]).

mod parse;
mod readers;

use std::collections::HashMap;
use std::path::Path;

use crate::models::{Feature, Record, RecordKind, RecordSource};
use parse::{slugify, strip_number_prefix};
use readers::{RawRecord, read_kiro_dir, read_native_dir, read_speckit_dir};

/// Fill in each feature's `records` and return the project-level records
pub fn populate_records(features: &mut [Feature], base_path: &Path) -> Vec<Record> {
    read_feature_records(features, base_path);

    let refs = feature_refs(features);
    let mut attached: HashMap<String, Vec<Record>> = HashMap::new();
    let mut project = Vec::new();
    for raw in read_root_records(base_path) {
        match find_feature(&refs, &raw) {
            Some(path) => attached.entry(path).or_default().push(raw.record),
            None => project.push(raw.record),
        }
    }
    attach(features, &mut attached);

    let mut flat = Vec::new();
    take_records(features, &mut flat);
    flat.extend(project.into_iter().map(|record| (None, record)));
    resolve_links(&mut flat);

    let mut by_feature: HashMap<String, Vec<Record>> = HashMap::new();
    let mut project = Vec::new();
    for (owner, record) in flat {
        match owner {
            Some(path) => by_feature.entry(path).or_default().push(record),
            None => project.push(record),
        }
    }
    attach(features, &mut by_feature);
    project
}

fn read_feature_records(features: &mut [Feature], base_path: &Path) {
    for feature in features {
        let dir = base_path.join(&feature.path);
        for docs in ["__docs__", ".docs"] {
            for (folder, kind) in [
                ("specs", RecordKind::Spec),
                ("decisions", RecordKind::Decision),
            ] {
                feature.records.extend(
                    read_native_dir(&dir.join(docs).join(folder), kind, base_path)
                        .into_iter()
                        .map(|raw| raw.record),
                );
            }
        }
        read_feature_records(&mut feature.features, base_path);
    }
}

fn read_root_records(base_path: &Path) -> Vec<RawRecord> {
    let docs = base_path.join("docs");
    let mut records = read_native_dir(&docs.join("specs"), RecordKind::Spec, base_path);
    records.extend(read_native_dir(
        &docs.join("decisions"),
        RecordKind::Decision,
        base_path,
    ));
    records.extend(read_native_dir(
        &docs.join("adr"),
        RecordKind::Decision,
        base_path,
    ));
    records.extend(read_speckit_dir(&base_path.join("specs"), base_path));
    records.extend(read_kiro_dir(
        &base_path.join(".kiro").join("specs"),
        base_path,
    ));
    records
}

/// Append the records keyed by feature path to their feature
fn attach(features: &mut [Feature], records: &mut HashMap<String, Vec<Record>>) {
    for feature in features {
        if let Some(found) = records.remove(&feature.path) {
            feature.records.extend(found);
        }
        attach(&mut feature.features, records);
    }
}

/// Move every feature record out of the tree, with the path of its feature
fn take_records(features: &mut [Feature], out: &mut Vec<(Option<String>, Record)>) {
    for feature in features {
        let path = feature.path.clone();
        out.extend(
            feature
                .records
                .drain(..)
                .map(|record| (Some(path.clone()), record)),
        );
        take_records(&mut feature.features, out);
    }
}

/// What a root record can be matched against
struct FeatureRef {
    path: String,
    name: String,
    segment: String,
}

/// Every feature, parents before their nested features
fn feature_refs(features: &[Feature]) -> Vec<FeatureRef> {
    let mut refs = Vec::new();
    fn walk(features: &[Feature], refs: &mut Vec<FeatureRef>) {
        for feature in features {
            refs.push(FeatureRef {
                path: feature.path.replace('\\', "/"),
                name: slugify(&feature.name),
                segment: slugify(feature.path.rsplit(['/', '\\']).next().unwrap_or_default()),
            });
            walk(&feature.features, refs);
        }
    }
    walk(features, &mut refs);
    refs
}

/// The feature a root record belongs to:
///
/// 1. its `feature:` key matches a feature path (exact, then suffix), name or
///    folder name;
/// 2. for Spec Kit and Kiro, its folder name without the `NNN-` prefix matches
///    a feature name or folder name (first match, parents first);
/// 3. otherwise none: the record is project-level.
fn find_feature(refs: &[FeatureRef], raw: &RawRecord) -> Option<String> {
    if let Some(key) = &raw.feature {
        let key = key.trim().trim_matches('/').replace('\\', "/");
        let slug = slugify(&key);
        return refs
            .iter()
            .find(|f| f.path == key)
            .or_else(|| refs.iter().find(|f| f.path.ends_with(&format!("/{}", key))))
            .or_else(|| refs.iter().find(|f| f.name == slug))
            .or_else(|| refs.iter().find(|f| f.segment == slug))
            .map(|f| f.path.clone());
    }

    if matches!(
        raw.record.source,
        RecordSource::Speckit | RecordSource::Kiro
    ) {
        let slug = slugify(strip_number_prefix(&raw.record.id));
        if slug.is_empty() {
            return None;
        }
        return refs
            .iter()
            .find(|f| f.name == slug || f.segment == slug)
            .map(|f| f.path.clone());
    }

    None
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LinkType {
    Decisions,
    Specs,
    Supersedes,
    SupersededBy,
}

const LINK_TYPES: [LinkType; 4] = [
    LinkType::Decisions,
    LinkType::Specs,
    LinkType::Supersedes,
    LinkType::SupersededBy,
];

impl LinkType {
    fn name(self) -> &'static str {
        match self {
            LinkType::Decisions => "decisions",
            LinkType::Specs => "specs",
            LinkType::Supersedes => "supersedes",
            LinkType::SupersededBy => "superseded_by",
        }
    }

    fn of(self, record: &Record) -> &Vec<String> {
        match self {
            LinkType::Decisions => &record.links.decisions,
            LinkType::Specs => &record.links.specs,
            LinkType::Supersedes => &record.links.supersedes,
            LinkType::SupersededBy => &record.links.superseded_by,
        }
    }

    fn of_mut(self, record: &mut Record) -> &mut Vec<String> {
        match self {
            LinkType::Decisions => &mut record.links.decisions,
            LinkType::Specs => &mut record.links.specs,
            LinkType::Supersedes => &mut record.links.supersedes,
            LinkType::SupersededBy => &mut record.links.superseded_by,
        }
    }

    /// The link type a target gets back from a record of kind `from`
    fn reverse(self, from: RecordKind) -> LinkType {
        match self {
            LinkType::Decisions | LinkType::Specs => match from {
                RecordKind::Spec => LinkType::Specs,
                RecordKind::Decision => LinkType::Decisions,
            },
            LinkType::Supersedes => LinkType::SupersededBy,
            LinkType::SupersededBy => LinkType::Supersedes,
        }
    }
}

/// Finds the record a link points to
struct RecordIndex<'a> {
    /// Feature path (none for project-level) and id of every record
    keys: Vec<(Option<&'a str>, &'a str)>,
}

impl<'a> RecordIndex<'a> {
    fn new(records: impl Iterator<Item = (Option<&'a str>, &'a Record)>) -> Self {
        Self {
            keys: records.map(|(owner, r)| (owner, r.id.as_str())).collect(),
        }
    }

    /// Resolve `link` from the record at `from`: same id (or same number for a
    /// bare number), in the same feature first, then project-level, then anywhere
    fn resolve(&self, link: &str, from: usize) -> Option<usize> {
        let number = leading_number(link).filter(|_| link.chars().all(|c| c.is_ascii_digit()));
        let candidates: Vec<usize> = self
            .keys
            .iter()
            .enumerate()
            .filter(|(i, (_, id))| {
                *i != from && (*id == link || (number.is_some() && leading_number(id) == number))
            })
            .map(|(i, _)| i)
            .collect();
        let owner = self.keys[from].0;
        candidates
            .iter()
            .find(|&&i| owner.is_some() && self.keys[i].0 == owner)
            .or_else(|| candidates.iter().find(|&&i| self.keys[i].0.is_none()))
            .or_else(|| candidates.first())
            .copied()
    }
}

fn leading_number(id: &str) -> Option<u64> {
    let digits: String = id.chars().take_while(char::is_ascii_digit).collect();
    digits.parse().ok()
}

/// Rewrite each resolved link to the target's id and add the reverse link to
/// the target: a spec listing a decision gives the decision a `specs` link back,
/// `supersedes` gives the target a `superseded_by` link
fn resolve_links(records: &mut [(Option<String>, Record)]) {
    let mut forward: Vec<(usize, LinkType, usize, String)> = Vec::new();
    let mut reverse: Vec<(usize, LinkType, String)> = Vec::new();
    {
        let index = RecordIndex::new(records.iter().map(|(o, r)| (o.as_deref(), r)));
        for (i, (_, record)) in records.iter().enumerate() {
            for link_type in LINK_TYPES {
                for (position, link) in link_type.of(record).iter().enumerate() {
                    if let Some(j) = index.resolve(link, i) {
                        let target_id = records[j].1.id.clone();
                        forward.push((i, link_type, position, target_id));
                        reverse.push((j, link_type.reverse(record.kind), record.id.clone()));
                    }
                }
            }
        }
    }

    for (i, link_type, position, target_id) in forward {
        link_type.of_mut(&mut records[i].1)[position] = target_id;
    }
    for (j, link_type, id) in reverse {
        let links = link_type.of_mut(&mut records[j].1);
        if !links.contains(&id) {
            links.push(id);
        }
    }
    for (_, record) in records.iter_mut() {
        for link_type in LINK_TYPES {
            let links = link_type.of_mut(record);
            let mut seen = Vec::with_capacity(links.len());
            links.retain(|link| {
                let keep = !seen.contains(link);
                seen.push(link.clone());
                keep
            });
        }
    }
}

/// Every record of the tree and the project, with its feature path
fn all_records<'a>(
    features: &'a [Feature],
    project: &'a [Record],
) -> Vec<(Option<&'a str>, &'a Record)> {
    fn walk<'a>(features: &'a [Feature], out: &mut Vec<(Option<&'a str>, &'a Record)>) {
        for feature in features {
            out.extend(
                feature
                    .records
                    .iter()
                    .map(|r| (Some(feature.path.as_str()), r)),
            );
            walk(&feature.features, out);
        }
    }
    let mut out = Vec::new();
    walk(features, &mut out);
    out.extend(project.iter().map(|r| (None, r)));
    out
}

/// Problems worth fixing in the records, for `features check`:
/// broken links, native records without status, superseded records with
/// nothing superseding them and implemented specs with open tasks
pub fn record_warnings(features: &[Feature], project: &[Record]) -> Vec<String> {
    let records = all_records(features, project);
    let index = RecordIndex::new(records.iter().copied());
    let mut warnings = Vec::new();

    for (i, (_, record)) in records.iter().enumerate() {
        for link_type in LINK_TYPES {
            for link in link_type.of(record) {
                if index.resolve(link, i).is_none() {
                    warnings.push(format!(
                        "{}: `{}` link `{}` does not match any spec or decision",
                        record.path,
                        link_type.name(),
                        link
                    ));
                }
            }
        }

        match record.status.as_deref() {
            None if record.source == RecordSource::Native => {
                warnings.push(format!("{}: missing status", record.path));
            }
            Some("superseded") if record.links.superseded_by.is_empty() => {
                warnings.push(format!(
                    "{}: superseded but no record supersedes it (add `supersedes: {}` to the replacement)",
                    record.path, record.id
                ));
            }
            Some("implemented") => {
                if let Some(tasks) = record.tasks.filter(|t| t.done < t.total) {
                    warnings.push(format!(
                        "{}: implemented but {} of {} tasks are still open",
                        record.path,
                        tasks.total - tasks.done,
                        tasks.total
                    ));
                }
            }
            _ => {}
        }
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::RecordLinks;
    use std::collections::BTreeMap;
    use std::fs;

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

    fn record(kind: RecordKind, id: &str, links: RecordLinks) -> Record {
        Record {
            kind,
            id: id.to_string(),
            title: id.to_string(),
            status: Some("accepted".to_string()),
            date: None,
            path: format!("{}.md", id),
            content: String::new(),
            files: Vec::new(),
            source: RecordSource::Native,
            links,
            tasks: None,
        }
    }

    fn write(path: &Path, content: &str) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    #[test]
    fn test_links_are_resolved_in_both_directions() {
        let mut records = vec![
            (
                Some("auth".to_string()),
                record(
                    RecordKind::Spec,
                    "001-login",
                    RecordLinks {
                        decisions: vec!["2".to_string()],
                        ..Default::default()
                    },
                ),
            ),
            (
                Some("auth".to_string()),
                record(RecordKind::Decision, "002-use-jwt", RecordLinks::default()),
            ),
            (
                None,
                record(
                    RecordKind::Decision,
                    "003-use-paseto",
                    RecordLinks {
                        supersedes: vec!["002-use-jwt".to_string()],
                        ..Default::default()
                    },
                ),
            ),
        ];

        resolve_links(&mut records);

        assert_eq!(records[0].1.links.decisions, vec!["002-use-jwt"]);
        assert_eq!(records[1].1.links.specs, vec!["001-login"]);
        assert_eq!(records[1].1.links.superseded_by, vec!["003-use-paseto"]);
        assert!(records[2].1.links.specs.is_empty());
    }

    #[test]
    fn test_same_feature_wins_over_other_features() {
        let mut records = vec![
            (
                Some("a".to_string()),
                record(
                    RecordKind::Spec,
                    "001-spec",
                    RecordLinks {
                        decisions: vec!["001-x".to_string()],
                        ..Default::default()
                    },
                ),
            ),
            (
                Some("b".to_string()),
                record(RecordKind::Decision, "001-x", RecordLinks::default()),
            ),
            (
                Some("a".to_string()),
                record(RecordKind::Decision, "001-x", RecordLinks::default()),
            ),
        ];
        resolve_links(&mut records);
        assert!(records[1].1.links.specs.is_empty());
        assert_eq!(records[2].1.links.specs, vec!["001-spec"]);
    }

    #[test]
    fn test_populate_records_reads_every_folder_and_maps_root_records() {
        let temp = tempfile::TempDir::new().unwrap();
        let base = temp.path();
        let auth = base.join("features").join("auth");
        write(
            &auth.join("__docs__/specs/001-login-flow.md"),
            "---\nstatus: accepted\ndecisions: [002-use-jwt]\n---\n# Login flow\n",
        );
        write(
            &auth.join(".docs/decisions/002-use-jwt.md"),
            "# Use JWT\n\n## Status\n\nAccepted\n",
        );
        write(
            &base.join("docs/decisions/010-billing-db.md"),
            "---\nstatus: accepted\nfeature: billing\n---\n# Billing database\n",
        );
        write(
            &base.join("docs/adr/0001-monorepo.md"),
            "# Monorepo\n\nStatus: accepted\n",
        );
        write(
            &base.join("specs/004-auth/spec.md"),
            "# Feature Specification: Auth revamp\n\n**Status**: Draft\n",
        );
        write(
            &base.join(".kiro/specs/unknown-thing/requirements.md"),
            "# Requirements Document\n",
        );

        let mut features = vec![
            feature("Auth", "features/auth", Vec::new()),
            feature("Billing", "features/billing", Vec::new()),
        ];
        let project = populate_records(&mut features, base);

        let auth_ids: Vec<&str> = features[0].records.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(auth_ids, vec!["001-login-flow", "002-use-jwt", "004-auth"]);
        assert_eq!(features[0].records[1].links.specs, vec!["001-login-flow"]);
        assert_eq!(features[0].records[2].source, RecordSource::Speckit);
        assert_eq!(features[0].records[2].title, "Auth revamp");
        assert_eq!(features[0].records[2].status.as_deref(), Some("draft"));

        assert_eq!(features[1].records.len(), 1);
        assert_eq!(features[1].records[0].id, "010-billing-db");

        let project_ids: Vec<&str> = project.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(project_ids, vec!["0001-monorepo", "unknown-thing"]);
        assert_eq!(project[1].title, "Unknown thing");
        assert_eq!(project[1].source, RecordSource::Kiro);
    }

    #[test]
    fn test_record_warnings() {
        let mut superseded = record(RecordKind::Decision, "001-old", RecordLinks::default());
        superseded.status = Some("superseded".to_string());
        let mut no_status = record(
            RecordKind::Spec,
            "002-spec",
            RecordLinks {
                decisions: vec!["999-missing".to_string()],
                ..Default::default()
            },
        );
        no_status.status = None;
        let mut implemented = record(RecordKind::Spec, "003-done", RecordLinks::default());
        implemented.status = Some("implemented".to_string());
        implemented.tasks = Some(crate::models::TaskProgress { done: 1, total: 3 });
        let mut kiro = record(RecordKind::Spec, "kiro-spec", RecordLinks::default());
        kiro.status = None;
        kiro.source = RecordSource::Kiro;

        let warnings = record_warnings(&[], &[superseded, no_status, implemented, kiro]);
        assert_eq!(warnings.len(), 4, "{:#?}", warnings);
        assert!(warnings[0].contains("001-old.md: superseded"));
        assert!(warnings[1].contains("`decisions` link `999-missing`"));
        assert!(warnings[2].contains("002-spec.md: missing status"));
        assert!(warnings[3].contains("2 of 3 tasks are still open"));
    }
}
