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
