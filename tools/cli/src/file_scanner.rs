use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::git_helper::get_commits_for_path;
use crate::models::Feature;
use crate::readme_parser::read_readme_info;

pub fn list_files_recursive(dir: &Path) -> Result<Vec<Feature>> {
    list_files_recursive_impl(dir, false)
}

pub fn list_files_recursive_with_changes(dir: &Path) -> Result<Vec<Feature>> {
    list_files_recursive_impl(dir, true)
}

fn read_decision_files(feature_path: &Path) -> Result<Vec<String>> {
    let mut decisions = Vec::new();

    // Check both "decision" and "decisions" folder names
    let decision_paths = [
        feature_path.join(".docs").join("decisions"),
        feature_path.join("__docs__").join("decisions"),
    ];

    for decisions_dir in &decision_paths {
        if decisions_dir.exists() && decisions_dir.is_dir() {
            let entries = fs::read_dir(decisions_dir).with_context(|| {
                format!(
                    "could not read decisions directory `{}`",
                    decisions_dir.display()
                )
            })?;

            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                // Skip README.md files and only process .md files
                if path.is_file()
                    && let Some(file_name) = path.file_name()
                {
                    let file_name_str = file_name.to_string_lossy();
                    if file_name_str.ends_with(".md") && file_name_str != "README.md" {
                        let content = fs::read_to_string(&path).with_context(|| {
                            format!("could not read decision file `{}`", path.display())
                        })?;
                        decisions.push(content);
                    }
                }
            }
            break; // If we found one of the directories, don't check the other
        }
    }

    Ok(decisions)
}

fn list_files_recursive_impl(dir: &Path, include_changes: bool) -> Result<Vec<Feature>> {
    let entries = fs::read_dir(dir)
        .with_context(|| format!("could not read directory `{}`", dir.display()))?;

    let mut entries: Vec<_> = entries.collect::<Result<_, _>>()?;
    entries.sort_by_key(|entry| entry.path());

    let mut features: Vec<Feature> = Vec::new();

    for entry in entries {
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();

        if path.is_dir() {
            if dir.ends_with("features") {
                let readme_path = path.join("README.md");
                let (owner, description, meta) = read_readme_info(&readme_path)?;

                let new_features = list_files_recursive_impl(&path, include_changes);

                let changes = if include_changes {
                    get_commits_for_path(&path, &path.to_string_lossy()).unwrap_or_default()
                } else {
                    Vec::new()
                };

                let decisions = if include_changes {
                    read_decision_files(&path).unwrap_or_default()
                } else {
                    Vec::new()
                };

                features.push(Feature {
                    name: name.to_string(),
                    description,
                    owner,
                    path: path.to_string_lossy().to_string(),
                    features: new_features?,
                    meta,
                    changes,
                    decisions,
                });
            } else {
                let new_features = list_files_recursive_impl(&path, include_changes);
                features.extend(new_features?);
            }
        }
    }

    Ok(features)
}
