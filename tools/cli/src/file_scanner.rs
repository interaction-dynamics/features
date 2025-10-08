use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::models::Feature;
use crate::readme_parser::read_readme_info;

pub fn list_files_recursive(dir: &Path) -> Result<Vec<Feature>> {
    let entries = fs::read_dir(dir)
        .with_context(|| format!("could not read directory `{}`", dir.display()))?;

    let mut features: Vec<Feature> = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();

        if path.is_dir() {
            if dir.ends_with("features") {
                let readme_path = path.join("README.md");
                let (owner, description, meta) = read_readme_info(&readme_path)?;

                let new_features = list_files_recursive(&path);

                features.push(Feature {
                    name: name.to_string(),
                    description,
                    owner,
                    path: path.to_string_lossy().to_string(),
                    features: new_features?,
                    meta,
                });
            } else {
                let new_features = list_files_recursive(&path);
                features.extend(new_features?);
            }
        }
    }

    return Ok(features);
}
