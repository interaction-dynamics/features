use anyhow::{Context, Result};
use git2::Repository;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::git_helper::get_all_commits_by_path;
use crate::models::{Change, Feature};
use crate::readme_parser::read_readme_info;

fn is_documentation_directory(dir_path: &Path) -> bool {
    let dir_name = dir_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");

    // Common documentation directory names
    let doc_dirs = ["docs", "__docs__", ".docs"];

    doc_dirs.contains(&dir_name.to_lowercase().as_str())
}

fn is_inside_documentation_directory(dir_path: &Path) -> bool {
    // Check if any parent directory is a documentation directory
    for ancestor in dir_path.ancestors().skip(1) {
        if is_documentation_directory(ancestor) {
            return true;
        }
    }
    false
}

fn is_direct_subfolder_of_features(dir_path: &Path) -> bool {
    if let Some(parent) = dir_path.parent()
        && let Some(parent_name) = parent.file_name().and_then(|name| name.to_str())
    {
        return parent_name == "features";
    }
    false
}

fn find_readme_file(dir_path: &Path) -> Option<std::path::PathBuf> {
    let readme_candidates = ["README.md", "README.mdx"];

    for candidate in &readme_candidates {
        let readme_path = dir_path.join(candidate);
        if readme_path.exists() {
            return Some(readme_path);
        }
    }

    None
}

/// Check if a directory has a README with `feature: true` in front matter
fn has_feature_flag_in_readme(dir_path: &Path) -> bool {
    if let Some(readme_path) = find_readme_file(dir_path)
        && let Ok(content) = fs::read_to_string(&readme_path)
    {
        // Check if content starts with YAML front matter (---)
        if let Some(stripped) = content.strip_prefix("---\n")
            && let Some(end_pos) = stripped.find("\n---\n")
        {
            let yaml_content = &stripped[..end_pos];

            // Parse YAML front matter
            if let Ok(yaml_value) = serde_yaml::from_str::<serde_yaml::Value>(yaml_content)
                && let Some(mapping) = yaml_value.as_mapping()
            {
                // Check for feature: true
                if let Some(feature_value) =
                    mapping.get(serde_yaml::Value::String("feature".to_string()))
                {
                    return feature_value.as_bool() == Some(true);
                }
            }
        }
    }
    false
}

/// Check if a directory should be treated as a feature
fn is_feature_directory(dir_path: &Path) -> bool {
    // Skip documentation directories
    if is_documentation_directory(dir_path) || is_inside_documentation_directory(dir_path) {
        return false;
    }

    // Check if it's a direct subfolder of "features" (existing behavior)
    if is_direct_subfolder_of_features(dir_path) {
        return true;
    }

    // Check if the directory has a README with feature: true
    has_feature_flag_in_readme(dir_path)
}

pub fn list_files_recursive(dir: &Path) -> Result<Vec<Feature>> {
    list_files_recursive_impl(dir, None)
}

pub fn list_files_recursive_with_changes(dir: &Path) -> Result<Vec<Feature>> {
    // Get all commits once at the beginning for efficiency
    let all_commits = get_all_commits_by_path(dir).unwrap_or_default();
    list_files_recursive_impl(dir, Some(&all_commits))
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

fn process_feature_directory(
    path: &Path,
    name: &str,
    changes_map: Option<&HashMap<String, Vec<Change>>>,
) -> Result<Feature> {
    // Try to find and read README file, use defaults if not found
    let (owner, description, mut meta) = if let Some(readme_path) = find_readme_file(path) {
        read_readme_info(&readme_path)?
    } else {
        (
            "Unknown".to_string(),
            "".to_string(),
            std::collections::HashMap::new(),
        )
    };

    // Remove the 'feature' key from meta if it exists (it's redundant since we know it's a feature)
    meta.remove("feature");

    let changes = if let Some(map) = changes_map {
        // Convert the absolute path to a repo-relative path and look up changes
        get_changes_for_path(path, map).unwrap_or_default()
    } else {
        Vec::new()
    };

    // Always include decisions regardless of include_changes flag
    let decisions = read_decision_files(path).unwrap_or_default();

    // Check if this feature has nested features in a 'features' subdirectory
    let nested_features_path = path.join("features");
    let mut nested_features = if nested_features_path.exists() && nested_features_path.is_dir() {
        list_files_recursive_impl(&nested_features_path, changes_map).unwrap_or_default()
    } else {
        Vec::new()
    };

    // Also check for nested features marked with feature: true in subdirectories
    let entries = fs::read_dir(path)
        .with_context(|| format!("could not read directory `{}`", path.display()))?;

    let mut entries: Vec<_> = entries.collect::<Result<_, _>>()?;
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let entry_path = entry.path();
        let entry_name = entry_path.file_name().unwrap().to_string_lossy();

        if entry_path.is_dir()
            && entry_name != "features" // Don't process 'features' folder twice
            && !is_documentation_directory(&entry_path)
            && has_feature_flag_in_readme(&entry_path)
        {
            let nested_feature = process_feature_directory(&entry_path, &entry_name, changes_map)?;
            nested_features.push(nested_feature);
        }
    }

    Ok(Feature {
        name: name.to_string(),
        description,
        owner,
        path: path.to_string_lossy().to_string(),
        features: nested_features,
        meta,
        changes,
        decisions,
    })
}

fn list_files_recursive_impl(
    dir: &Path,
    changes_map: Option<&HashMap<String, Vec<Change>>>,
) -> Result<Vec<Feature>> {
    let entries = fs::read_dir(dir)
        .with_context(|| format!("could not read directory `{}`", dir.display()))?;

    let mut entries: Vec<_> = entries.collect::<Result<_, _>>()?;
    entries.sort_by_key(|entry| entry.path());

    let mut features: Vec<Feature> = Vec::new();

    for entry in entries {
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();

        if path.is_dir() {
            if is_feature_directory(&path) {
                let feature = process_feature_directory(&path, &name, changes_map)?;
                features.push(feature);
            } else if !is_documentation_directory(&path)
                && !is_inside_documentation_directory(&path)
            {
                // Recursively search for features in non-documentation subdirectories
                let new_features = list_files_recursive_impl(&path, changes_map)?;
                features.extend(new_features);
            }
        }
    }

    Ok(features)
}

/// Get changes for a specific path from the pre-computed changes map
fn get_changes_for_path(
    path: &Path,
    changes_map: &HashMap<String, Vec<Change>>,
) -> Result<Vec<Change>> {
    // Canonicalize the path
    let canonical_path = std::fs::canonicalize(path)?;

    // Find the repository and get the working directory
    let repo = Repository::discover(path)?;
    let repo_workdir = repo
        .workdir()
        .context("repository has no working directory")?;

    // Convert to relative path from repo root
    let relative_path = canonical_path
        .strip_prefix(repo_workdir)
        .context("path is not within repository")?;

    let relative_path_str = relative_path.to_string_lossy().to_string();

    // Look up the changes in the map
    Ok(changes_map
        .get(&relative_path_str)
        .cloned()
        .unwrap_or_default())
}
