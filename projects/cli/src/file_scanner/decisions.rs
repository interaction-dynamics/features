//! Architecture decision records stored next to each feature.

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use super::for_each_feature_mut;
use crate::models::Feature;

/// Fill in each feature's `decisions` from its `.docs/decisions` or
/// `__docs__/decisions` folder
pub(super) fn populate_decisions(features: &mut [Feature], base_path: &Path) {
    for_each_feature_mut(features, &mut |feature| {
        feature.decisions = read_decision_files(&base_path.join(&feature.path)).unwrap_or_default();
    });
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

            // Collect all decision file paths first
            let mut decision_paths_vec = Vec::new();

            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                // Skip README.md files and only process .md files
                if path.is_file()
                    && let Some(file_name) = path.file_name()
                {
                    let file_name_str = file_name.to_string_lossy();
                    if file_name_str.ends_with(".md") && file_name_str != "README.md" {
                        decision_paths_vec.push(path);
                    }
                }
            }

            // Sort decision files alphabetically by filename
            decision_paths_vec.sort_by(|a, b| {
                let name_a = a.file_name().unwrap_or_default().to_string_lossy();
                let name_b = b.file_name().unwrap_or_default().to_string_lossy();
                name_a.cmp(&name_b)
            });

            // Read the sorted files
            for path in decision_paths_vec {
                let content = fs::read_to_string(&path).with_context(|| {
                    format!("could not read decision file `{}`", path.display())
                })?;
                decisions.push(content);
            }

            break; // If we found one of the directories, don't check the other
        }
    }

    Ok(decisions)
}
