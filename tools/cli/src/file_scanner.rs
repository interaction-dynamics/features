use anyhow::{Context, Result};
use git2::Repository;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::git_helper::get_all_commits_by_path;
use crate::models::{Change, Feature, Stats};
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

/// Compute statistics from changes for a feature
fn compute_stats_from_changes(changes: &[Change]) -> Option<Stats> {
    if changes.is_empty() {
        return None;
    }

    let mut commits = HashMap::new();

    // Add total commit count
    commits.insert(
        "total_commits".to_string(),
        serde_json::json!(changes.len()),
    );

    // Count commits by author
    let mut authors_count: HashMap<String, usize> = HashMap::new();
    for change in changes {
        *authors_count.entry(change.author_name.clone()).or_insert(0) += 1;
    }
    commits.insert(
        "authors_count".to_string(),
        serde_json::json!(authors_count),
    );

    // Count commits by conventional commit type
    let mut count_by_type: HashMap<String, usize> = HashMap::new();
    for change in changes {
        let commit_type = extract_commit_type(&change.title);
        *count_by_type.entry(commit_type).or_insert(0) += 1;
    }
    commits.insert(
        "count_by_type".to_string(),
        serde_json::json!(count_by_type),
    );

    // Get first and last commit dates
    if let Some(first) = changes.first() {
        commits.insert(
            "first_commit_date".to_string(),
            serde_json::json!(first.date.clone()),
        );
    }
    if let Some(last) = changes.last() {
        commits.insert(
            "last_commit_date".to_string(),
            serde_json::json!(last.date.clone()),
        );
    }

    Some(Stats { commits })
}

/// Extract the commit type from a conventional commit title
fn extract_commit_type(title: &str) -> String {
    // Common conventional commit types
    let known_types = [
        "feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore",
        "revert",
    ];

    // Check if the title follows conventional commit format (type: description or type(scope): description)
    if let Some(colon_pos) = title.find(':') {
        let prefix = &title[..colon_pos];

        // Remove scope if present (e.g., "feat(auth)" -> "feat")
        let type_part = if let Some(paren_pos) = prefix.find('(') {
            &prefix[..paren_pos]
        } else {
            prefix
        };

        let type_part = type_part.trim().to_lowercase();

        // Check if it's a known conventional commit type
        if known_types.contains(&type_part.as_str()) {
            return type_part;
        }
    }

    // If not a conventional commit, return "other"
    "other".to_string()
}

fn process_feature_directory(
    path: &Path,
    name: &str,
    changes_map: Option<&HashMap<String, Vec<Change>>>,
) -> Result<Feature> {
    // Try to find and read README file, use defaults if not found
    let (title, owner, description, mut meta) = if let Some(readme_path) = find_readme_file(path) {
        read_readme_info(&readme_path)?
    } else {
        (
            None,
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
        {
            if has_feature_flag_in_readme(&entry_path) {
                // This directory is a feature itself
                let nested_feature =
                    process_feature_directory(&entry_path, &entry_name, changes_map)?;
                nested_features.push(nested_feature);
            } else {
                // This directory is not a feature, but might contain features
                // Recursively search for features inside it
                let deeper_features = list_files_recursive_impl(&entry_path, changes_map)?;
                nested_features.extend(deeper_features);
            }
        }
    }

    // Compute stats from changes if available
    let stats = compute_stats_from_changes(&changes);

    Ok(Feature {
        name: title.unwrap_or_else(|| name.to_string()),
        description,
        owner,
        path: path.to_string_lossy().to_string(),
        features: nested_features,
        meta,
        changes,
        decisions,
        stats,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_commit_type() {
        // Test standard conventional commit types
        assert_eq!(extract_commit_type("feat: add new feature"), "feat");
        assert_eq!(extract_commit_type("fix: resolve bug"), "fix");
        assert_eq!(extract_commit_type("docs: update README"), "docs");
        assert_eq!(extract_commit_type("style: format code"), "style");
        assert_eq!(
            extract_commit_type("refactor: improve structure"),
            "refactor"
        );
        assert_eq!(extract_commit_type("perf: optimize performance"), "perf");
        assert_eq!(extract_commit_type("test: add unit tests"), "test");
        assert_eq!(extract_commit_type("build: update dependencies"), "build");
        assert_eq!(extract_commit_type("ci: fix CI pipeline"), "ci");
        assert_eq!(extract_commit_type("chore: update gitignore"), "chore");
        assert_eq!(
            extract_commit_type("revert: undo previous commit"),
            "revert"
        );

        // Test with scope
        assert_eq!(extract_commit_type("feat(auth): add login"), "feat");
        assert_eq!(
            extract_commit_type("fix(api): resolve endpoint issue"),
            "fix"
        );
        assert_eq!(
            extract_commit_type("docs(readme): update instructions"),
            "docs"
        );

        // Test case insensitivity
        assert_eq!(extract_commit_type("FEAT: uppercase type"), "feat");
        assert_eq!(extract_commit_type("Fix: mixed case"), "fix");
        assert_eq!(extract_commit_type("DOCS: all caps"), "docs");

        // Test non-conventional commits
        assert_eq!(extract_commit_type("random commit message"), "other");
        assert_eq!(extract_commit_type("update: not conventional"), "other");
        assert_eq!(
            extract_commit_type("feature: close but not standard"),
            "other"
        );
        assert_eq!(extract_commit_type("no colon here"), "other");
        assert_eq!(extract_commit_type(""), "other");

        // Test edge cases
        assert_eq!(extract_commit_type("feat:no space after colon"), "feat");
        assert_eq!(extract_commit_type("feat  : extra spaces"), "feat");
        assert_eq!(
            extract_commit_type("feat(scope)(weird): nested parens"),
            "feat"
        );
    }
}
