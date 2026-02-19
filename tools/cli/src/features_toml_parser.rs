//! Module for parsing FEATURES.toml files
//!
//! This module provides functionality to read and parse FEATURES.toml files
//! that contain feature metadata (name, owner, description, and custom meta fields).

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct FeaturesToml {
    pub name: Option<String>,
    pub owner: Option<String>,
    pub description: Option<String>,
    #[serde(flatten)]
    pub meta: HashMap<String, serde_json::Value>,
}

/// Reads and parses a FEATURES.toml file
pub fn read_features_toml(path: &Path) -> Result<FeaturesToml> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("could not read FEATURES.toml at `{}`", path.display()))?;

    let parsed: FeaturesToml = toml::from_str(&content)
        .with_context(|| format!("could not parse FEATURES.toml at `{}`", path.display()))?;

    Ok(parsed)
}

/// Finds a FEATURES.toml file in a directory
pub fn find_features_toml(dir_path: &Path) -> Option<std::path::PathBuf> {
    let features_toml_path = dir_path.join("FEATURES.toml");
    if features_toml_path.exists() {
        Some(features_toml_path)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_parse_basic_features_toml() {
        let temp_dir = TempDir::new().unwrap();
        let toml_path = temp_dir.path().join("FEATURES.toml");
        let mut file = fs::File::create(&toml_path).unwrap();
        writeln!(
            file,
            r#"
name = "Test Feature"
owner = "test-team"
description = "A test feature"
"#
        )
        .unwrap();

        let result = read_features_toml(&toml_path).unwrap();
        assert_eq!(result.name, Some("Test Feature".to_string()));
        assert_eq!(result.owner, Some("test-team".to_string()));
        assert_eq!(result.description, Some("A test feature".to_string()));
    }

    #[test]
    fn test_parse_features_toml_with_meta() {
        let temp_dir = TempDir::new().unwrap();
        let toml_path = temp_dir.path().join("FEATURES.toml");
        let mut file = fs::File::create(&toml_path).unwrap();
        writeln!(
            file,
            r#"
name = "Feature with Meta"
owner = "team-a"
description = "Feature with extra metadata"
status = "active"
priority = "high"
"#
        )
        .unwrap();

        let result = read_features_toml(&toml_path).unwrap();
        assert_eq!(result.name, Some("Feature with Meta".to_string()));
        assert_eq!(result.owner, Some("team-a".to_string()));
        assert!(result.meta.contains_key("status"));
        assert!(result.meta.contains_key("priority"));
    }

    #[test]
    fn test_find_features_toml() {
        let temp_dir = TempDir::new().unwrap();
        let toml_path = temp_dir.path().join("FEATURES.toml");
        fs::File::create(&toml_path).unwrap();

        let found = find_features_toml(temp_dir.path());
        assert!(found.is_some());
        assert_eq!(found.unwrap(), toml_path);
    }

    #[test]
    fn test_find_features_toml_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let found = find_features_toml(temp_dir.path());
        assert!(found.is_none());
    }
}
