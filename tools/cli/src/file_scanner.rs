use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::git_helper::get_commits_for_path;
use crate::models::Feature;
use crate::readme_parser::read_readme_info;

/// Feature Detection Strategy:
///
/// This module implements a flexible feature detection system that identifies features
/// by the presence of README.md or README.mdx files in directories. Unlike the previous
/// approach that required directories to be under a "features" folder, this new approach
/// treats any directory containing a README file as a potential feature.
///
/// Key behaviors:
/// - Searches for README.md or README.mdx files in any directory
/// - Excludes documentation directories (docs, __docs__, decisions, etc.)
/// - Supports nested feature hierarchies
/// - Parses YAML frontmatter for metadata
/// - Extracts feature descriptions from README content

/// Determines if a directory is a documentation directory that should be excluded
/// from feature detection. This prevents README files in documentation folders
/// from being treated as feature definitions.
fn is_documentation_directory(dir_path: &Path) -> bool {
    let dir_name = dir_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");

    // Common documentation directory names
    let doc_dirs = [
        "docs",
        "__docs__",
        ".docs",
        "doc",
        "__doc__",
        ".doc",
        "decisions",
        "decision",
        "adrs",
        "adr",
        "guides",
        "guide",
        "tutorials",
        "tutorial",
    ];

    doc_dirs.contains(&dir_name.to_lowercase().as_str())
}

/// Checks if the current directory is nested inside any documentation directory.
/// This ensures we don't treat features inside documentation folders as actual features.
fn is_inside_documentation_directory(dir_path: &Path) -> bool {
    // Check if any parent directory is a documentation directory
    for ancestor in dir_path.ancestors().skip(1) {
        if is_documentation_directory(ancestor) {
            return true;
        }
    }
    false
}

/// Searches for README.md or README.mdx files in the given directory.
/// Returns the path to the first README file found, preferring .md over .mdx.
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

/// Core recursive function that scans directories for features.
///
/// A directory is considered a feature if:
/// 1. It contains a README.md or README.mdx file
/// 2. It's not a documentation directory (docs, decisions, etc.)
/// 3. It's not nested inside a documentation directory
///
/// This approach is more flexible than the previous requirement of being under
/// a "features" directory, allowing features to be organized in any structure.
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
            // Skip documentation directories and directories inside them
            if !is_documentation_directory(&path)
                && !is_inside_documentation_directory(&path)
                && find_readme_file(&path).is_some()
            {
                let readme_path = find_readme_file(&path).unwrap();
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
