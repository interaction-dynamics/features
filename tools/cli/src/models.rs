use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Change {
    pub title: String,
    pub author_name: String,
    pub author_email: String,
    pub description: String,
    pub date: String,
    pub hash: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Feature {
    pub name: String,
    pub description: String,
    pub owner: String,
    pub path: String,
    pub features: Vec<Feature>,
    pub meta: HashMap<String, serde_json::Value>,
    pub changes: Vec<Change>,
    pub decisions: Vec<String>,
}
