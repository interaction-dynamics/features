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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<Stats>,
    pub dependencies: Vec<Dependency>,
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
