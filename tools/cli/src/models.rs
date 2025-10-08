use std::collections::HashMap;

#[derive(Debug, serde::Serialize)]
pub struct Feature {
    pub name: String,
    pub description: String,
    pub owner: String,
    pub path: String,
    pub features: Vec<Feature>,
    pub meta: HashMap<String, serde_json::Value>,
}
