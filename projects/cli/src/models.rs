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
    pub description: String,
    pub owner: String,
    #[serde(rename = "is_owner_inherited")]
    pub is_owner_inherited: bool,
    pub path: String,
    pub features: Vec<Feature>,
    pub meta: BTreeMap<String, serde_json::Value>,
    pub changes: Vec<Change>,
    pub decisions: Vec<String>,
    /// Specs and decision records of the feature (see [`Record`])
    #[serde(default)]
    pub records: Vec<Record>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<Stats>,
    pub dependencies: Vec<Dependency>,
}

/// Whether a record describes what to build (spec) or why it was built that way (decision)
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecordKind {
    Spec,
    Decision,
}

/// Layout a record was read from
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecordSource {
    /// `__docs__/{specs,decisions}`, `.docs/...` or root `docs/...`
    Native,
    /// GitHub Spec Kit: root `specs/NNN-name/spec.md`
    Speckit,
    /// Kiro: `.kiro/specs/<name>/requirements.md`
    Kiro,
}

/// A secondary file of a folder record (`plan.md`, `tasks.md`, `contracts/api.yaml`...)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RecordFile {
    /// Path relative to the record folder
    pub name: String,
    /// Path relative to the scanned folder
    pub path: String,
    pub content: String,
}

/// Ids of the records linked to a record, resolved in both directions
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RecordLinks {
    pub decisions: Vec<String>,
    pub specs: Vec<String>,
    pub supersedes: Vec<String>,
    pub superseded_by: Vec<String>,
}

/// Checkbox progress of a spec (`- [ ]` / `- [x]`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TaskProgress {
    pub done: usize,
    pub total: usize,
}

/// A spec or a decision record
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Record {
    pub kind: RecordKind,
    /// File stem or folder name, e.g. `001-login-flow`
    pub id: String,
    pub title: String,
    /// Lowercased first word of the status (`accepted`, `superseded`...)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Path of the main file, relative to the scanned folder
    pub path: String,
    /// Markdown of the main file
    pub content: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<RecordFile>,
    pub source: RecordSource,
    #[serde(default)]
    pub links: RecordLinks,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasks: Option<TaskProgress>,
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
            description: String::new(),
            owner: String::new(),
            is_owner_inherited: false,
            path: String::from(name),
            features,
            meta: BTreeMap::new(),
            changes,
            decisions: Vec::new(),
            records: Vec::new(),
            stats: None,
            dependencies: Vec::new(),
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
