use std::collections::HashMap;

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
    pub commits: HashMap<String, serde_json::Value>,
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
    pub meta: HashMap<String, serde_json::Value>,
    pub changes: Vec<Change>,
    pub decisions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<Stats>,
}
