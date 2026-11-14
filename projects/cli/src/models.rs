use std::collections::BTreeMap;

use crate::coverage_parser::CoverageStats;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Change {
    pub title: String,
    pub author_name: String,
    pub author_email: String,
    pub description: String,
    pub date: String,
    pub hash: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Stats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub todos_count: Option<usize>,
    pub commits: BTreeMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<CoverageStats>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Feature {
    pub name: String,
    /// Stable identifier used by external tools to refer to the feature
    /// (folder basename slugified, or `key` from the README front matter)
    #[serde(default)]
    pub key: String,
    pub description: String,
    pub owner: String,
    #[serde(rename = "is_owner_inherited")]
    pub is_owner_inherited: bool,
    pub path: String,
    pub features: Vec<Feature>,
    pub meta: BTreeMap<String, serde_json::Value>,
    /// Links to external tools (`links` in the README front matter), by name
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub links: BTreeMap<String, String>,
    pub changes: Vec<Change>,
    pub decisions: Vec<String>,
    /// Specs and decision records of the feature, see [`Record`]
    #[serde(default)]
    pub records: Vec<Record>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<Stats>,
    pub dependencies: Vec<Dependency>,
    /// Data imported from external tools with `--data`, by metric name
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub external: BTreeMap<String, serde_json::Value>,
}

/// What a [`Record`] documents
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum RecordKind {
    /// What to build: requirements, plan, tasks
    Spec,
    /// Why it was built that way (architecture decision record)
    #[default]
    Decision,
}

impl RecordKind {
    /// Name of the folder holding records of this kind (`specs`, `decisions`)
    pub fn folder_name(self) -> &'static str {
        match self {
            RecordKind::Spec => "specs",
            RecordKind::Decision => "decisions",
        }
    }

    /// Short label for terminal output
    pub fn label(self) -> &'static str {
        match self {
            RecordKind::Spec => "Spec",
            RecordKind::Decision => "ADR",
        }
    }
}

/// Links between records, by record id. Filled in both directions when scanning.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RecordLinks {
    /// Related decisions (`decisions:` frontmatter, mostly on specs)
    pub decisions: Vec<String>,
    /// Related specs (`spec:` / `specs:` frontmatter, mostly on decisions)
    pub specs: Vec<String>,
    /// Records this one replaces (`supersedes:` frontmatter)
    pub supersedes: Vec<String>,
    /// Records replacing this one (the other side of `supersedes`)
    pub superseded_by: Vec<String>,
}

impl RecordLinks {
    pub fn is_empty(&self) -> bool {
        self.decisions.is_empty()
            && self.specs.is_empty()
            && self.supersedes.is_empty()
            && self.superseded_by.is_empty()
    }
}

/// A secondary file of a folder record (`plan.md`, `tasks.md`...)
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RecordFile {
    pub name: String,
    pub path: String,
    pub content: String,
}

/// Checklist progress: `- [x]` lines over `- [ ]` and `- [x]` lines
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TaskProgress {
    pub done: usize,
    pub total: usize,
}

/// A spec or a decision record, read from a `specs`, `decisions` or `adr` folder
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Record {
    pub kind: RecordKind,
    /// File stem, or folder name for a folder record (e.g. `002-use-jwt`)
    pub id: String,
    pub title: String,
    /// Lowercased status (draft, proposed, accepted, implemented, superseded...)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Path of the main file, relative to the scanned folder
    pub path: String,
    /// Markdown of the main file, without its frontmatter
    pub content: String,
    /// Other markdown files of a folder record
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<RecordFile>,
    pub links: RecordLinks,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<TaskProgress>,
    /// Feature a project-level record is attached to (`feature:` frontmatter)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DependencyType {
    Parent,
    Child,
    Sibling,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Dependency {
    #[serde(rename = "sourceFilename")]
    pub source_filename: String,
    #[serde(rename = "targetFilename")]
    pub target_filename: String,
    pub line: usize,
    pub content: String,
    #[serde(rename = "featurePath")]
    pub feature_path: String,
    #[serde(rename = "type")]
    pub dependency_type: DependencyType,
}

/// Empties the author email of every change in `features` and their nested features.
pub fn strip_author_emails(features: &mut [Feature]) {
    for feature in features {
        for change in &mut feature.changes {
            change.author_email.clear();
        }
        strip_author_emails(&mut feature.features);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn change(email: &str) -> Change {
        Change {
            title: String::from("feat: something"),
            author_name: String::from("Jane"),
            author_email: String::from(email),
            description: String::new(),
            date: String::from("2026-01-01 00:00:00"),
            hash: String::from("abc123"),
        }
    }

    fn feature(name: &str, changes: Vec<Change>, features: Vec<Feature>) -> Feature {
        Feature {
            name: String::from(name),
            key: String::from(name),
            description: String::new(),
            owner: String::new(),
            is_owner_inherited: false,
            path: String::from(name),
            features,
            meta: BTreeMap::new(),
            links: BTreeMap::new(),
            changes,
            decisions: Vec::new(),
            records: Vec::new(),
            stats: None,
            dependencies: Vec::new(),
            external: BTreeMap::new(),
        }
    }

    #[test]
    fn test_strip_author_emails_recurses_into_nested_features() {
        let mut features = vec![feature(
            "parent",
            vec![change("jane@example.com")],
            vec![feature("child", vec![change("john@example.com")], vec![])],
        )];

        strip_author_emails(&mut features);

        assert_eq!(features[0].changes[0].author_email, "");
        assert_eq!(features[0].changes[0].author_name, "Jane");
        assert_eq!(features[0].features[0].changes[0].author_email, "");
    }
}
