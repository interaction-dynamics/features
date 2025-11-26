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
    list_files_recursive_impl(dir, dir, None, None)
}

pub fn list_files_recursive_with_changes(dir: &Path) -> Result<Vec<Feature>> {
    // Get all commits once at the beginning for efficiency
    let all_commits = get_all_commits_by_path(dir).unwrap_or_default();
    list_files_recursive_impl(dir, dir, Some(&all_commits), None)
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

/// Count the number of files in a feature directory (excluding documentation)
fn count_files(feature_path: &Path, nested_feature_paths: &[String]) -> usize {
    let mut file_count = 0;

    if let Ok(entries) = fs::read_dir(feature_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let path_str = path.to_string_lossy().to_string();

            // Skip documentation directories
            if is_documentation_directory(&path) {
                continue;
            }

            // Skip nested feature directories
            if nested_feature_paths
                .iter()
                .any(|nfp| path_str.starts_with(nfp))
            {
                continue;
            }

            if path.is_file() {
                file_count += 1;
            } else if path.is_dir() {
                // Recursively count files in subdirectories
                file_count += count_files(&path, nested_feature_paths);
            }
        }
    }

    file_count
}

/// Count the total number of lines in all files in a feature directory (excluding documentation)
fn count_lines(feature_path: &Path, nested_feature_paths: &[String]) -> usize {
    let mut line_count = 0;

    if let Ok(entries) = fs::read_dir(feature_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let path_str = path.to_string_lossy().to_string();

            // Skip documentation directories
            if is_documentation_directory(&path) {
                continue;
            }

            // Skip nested feature directories
            if nested_feature_paths
                .iter()
                .any(|nfp| path_str.starts_with(nfp))
            {
                continue;
            }

            if path.is_file() {
                // Try to read the file and count lines
                if let Ok(content) = fs::read_to_string(&path) {
                    line_count += content.lines().count();
                }
            } else if path.is_dir() {
                // Recursively count lines in subdirectories
                line_count += count_lines(&path, nested_feature_paths);
            }
        }
    }

    line_count
}

/// Count the total number of TODO comments in all files in a feature directory (excluding documentation)
fn count_todos(feature_path: &Path, nested_feature_paths: &[String]) -> usize {
    let mut todo_count = 0;

    if let Ok(entries) = fs::read_dir(feature_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let path_str = path.to_string_lossy().to_string();

            // Skip documentation directories
            if is_documentation_directory(&path) {
                continue;
            }

            // Skip nested feature directories
            if nested_feature_paths
                .iter()
                .any(|nfp| path_str.starts_with(nfp))
            {
                continue;
            }

            if path.is_file() {
                // Try to read the file and count TODO comments
                if let Ok(content) = fs::read_to_string(&path) {
                    for line in content.lines() {
                        // Look for TODO in comments (case-insensitive)
                        let line_upper = line.to_uppercase();
                        if line_upper.contains("TODO") {
                            todo_count += 1;
                        }
                    }
                }
            } else if path.is_dir() {
                // Recursively count TODOs in subdirectories
                todo_count += count_todos(&path, nested_feature_paths);
            }
        }
    }

    todo_count
}

/// Get the paths affected by a specific commit
fn get_commit_affected_paths(repo: &Repository, commit_hash: &str) -> Vec<String> {
    let Ok(oid) = git2::Oid::from_str(commit_hash) else {
        return Vec::new();
    };

    let Ok(commit) = repo.find_commit(oid) else {
        return Vec::new();
    };

    let mut paths = Vec::new();

    // For the first commit (no parents), get all files in the tree
    if commit.parent_count() == 0 {
        if let Ok(tree) = commit.tree() {
            collect_all_tree_paths(repo, &tree, "", &mut paths);
        }
        return paths;
    }

    // For commits with parents, check the diff
    let Ok(tree) = commit.tree() else {
        return Vec::new();
    };

    let Ok(parent) = commit.parent(0) else {
        return Vec::new();
    };

    let Ok(parent_tree) = parent.tree() else {
        return Vec::new();
    };

    if let Ok(diff) = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None) {
        let _ = diff.foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path()
                    && let Some(path_str) = path.to_str()
                {
                    paths.push(path_str.to_string());
                }
                if let Some(path) = delta.old_file().path()
                    && let Some(path_str) = path.to_str()
                    && !paths.contains(&path_str.to_string())
                {
                    paths.push(path_str.to_string());
                }
                true
            },
            None,
            None,
            None,
        );
    }

    paths
}

/// Collect all file paths in a tree (helper for get_commit_affected_paths)
fn collect_all_tree_paths(
    repo: &Repository,
    tree: &git2::Tree,
    prefix: &str,
    paths: &mut Vec<String>,
) {
    for entry in tree.iter() {
        if let Some(name) = entry.name() {
            let path = if prefix.is_empty() {
                name.to_string()
            } else {
                format!("{}/{}", prefix, name)
            };

            paths.push(path.clone());

            if entry.kind() == Some(git2::ObjectType::Tree)
                && let Ok(obj) = entry.to_object(repo)
                && let Ok(subtree) = obj.peel_to_tree()
            {
                collect_all_tree_paths(repo, &subtree, &path, paths);
            }
        }
    }
}

/// Compute statistics from changes for a feature
fn compute_stats_from_changes(
    changes: &[Change],
    feature_path: &Path,
    nested_features: &[Feature],
) -> Option<Stats> {
    if changes.is_empty() {
        return None;
    }

    // Collect paths of nested features to exclude from commit counts
    let nested_feature_paths: Vec<String> =
        nested_features.iter().map(|f| f.path.clone()).collect();

    // Get repository to check commit details
    let repo = Repository::discover(feature_path).ok();

    // Get the feature's relative path from repo root
    let feature_relative_path = if let Some(ref r) = repo {
        if let Ok(canonical_path) = std::fs::canonicalize(feature_path) {
            if let Some(workdir) = r.workdir() {
                canonical_path
                    .strip_prefix(workdir)
                    .ok()
                    .map(|p| p.to_string_lossy().to_string())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    // Filter changes to only include those that affect files in this feature
    // (not exclusively in nested features)
    let filtered_changes: Vec<&Change> = changes
        .iter()
        .filter(|change| {
            // If we don't have repo access, include all changes
            let Some(ref r) = repo else {
                return true;
            };

            let Some(ref feature_rel_path) = feature_relative_path else {
                return true;
            };

            // Get the files affected by this commit
            let affected_files = get_commit_affected_paths(r, &change.hash);

            // Check if any affected file is in this feature but not in a nested feature
            affected_files.iter().any(|file_path| {
                // File must be in this feature
                let in_feature = file_path.starts_with(feature_rel_path);

                // File must not be exclusively in a nested feature
                let in_nested = nested_feature_paths.iter().any(|nested_path| {
                    // Convert nested absolute path to relative path
                    if let Ok(nested_canonical) = std::fs::canonicalize(nested_path)
                        && let Some(workdir) = r.workdir()
                        && let Ok(nested_rel) = nested_canonical.strip_prefix(workdir)
                    {
                        let nested_rel_str = nested_rel.to_string_lossy();
                        return file_path.starts_with(nested_rel_str.as_ref());
                    }
                    false
                });

                in_feature && !in_nested
            })
        })
        .collect();

    let mut commits = HashMap::new();

    // Add total commit count
    commits.insert(
        "total_commits".to_string(),
        serde_json::json!(filtered_changes.len()),
    );

    // Count commits by author
    let mut authors_count: HashMap<String, usize> = HashMap::new();
    for change in &filtered_changes {
        *authors_count.entry(change.author_name.clone()).or_insert(0) += 1;
    }
    commits.insert(
        "authors_count".to_string(),
        serde_json::json!(authors_count),
    );

    // Count commits by conventional commit type
    let mut count_by_type: HashMap<String, usize> = HashMap::new();
    for change in &filtered_changes {
        let commit_type = extract_commit_type(&change.title);
        *count_by_type.entry(commit_type).or_insert(0) += 1;
    }
    commits.insert(
        "count_by_type".to_string(),
        serde_json::json!(count_by_type),
    );

    // Get first and last commit dates
    if let Some(first) = filtered_changes.first() {
        commits.insert(
            "first_commit_date".to_string(),
            serde_json::json!(first.date.clone()),
        );
    }
    if let Some(last) = filtered_changes.last() {
        commits.insert(
            "last_commit_date".to_string(),
            serde_json::json!(last.date.clone()),
        );
    }

    // Count files and lines in the feature directory (excluding nested features)
    let files_count = count_files(feature_path, &nested_feature_paths);
    let lines_count = count_lines(feature_path, &nested_feature_paths);
    let todos_count = count_todos(feature_path, &nested_feature_paths);

    Some(Stats {
        files_count: Some(files_count),
        lines_count: Some(lines_count),
        todos_count: Some(todos_count),
        commits,
        coverage: None,
    })
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
    base_path: &Path,
    name: &str,
    changes_map: Option<&HashMap<String, Vec<Change>>>,
    parent_owner: Option<&str>,
) -> Result<Feature> {
    // Try to find and read README file, use defaults if not found
    let mut readme_info = if let Some(readme_path) = find_readme_file(path) {
        read_readme_info(&readme_path)?
    } else {
        use crate::readme_parser::ReadmeInfo;
        ReadmeInfo {
            title: None,
            owner: "".to_string(),
            description: "".to_string(),
            meta: std::collections::HashMap::new(),
        }
    };

    // Remove the 'feature' key from meta if it exists (it's redundant since we know it's a feature)
    readme_info.meta.remove("feature");

    let changes = if let Some(map) = changes_map {
        // Convert the absolute path to a repo-relative path and look up changes
        get_changes_for_path(path, map).unwrap_or_default()
    } else {
        Vec::new()
    };

    // Always include decisions regardless of include_changes flag
    let decisions = read_decision_files(path).unwrap_or_default();

    // Determine the actual owner and whether it's inherited
    let (actual_owner, is_owner_inherited) = if readme_info.owner.is_empty() {
        if let Some(parent) = parent_owner {
            (parent.to_string(), true)
        } else {
            ("".to_string(), false)
        }
    } else {
        (readme_info.owner.clone(), false)
    };

    // Check if this feature has nested features in a 'features' subdirectory
    let nested_features_path = path.join("features");
    let mut nested_features = if nested_features_path.exists() && nested_features_path.is_dir() {
        list_files_recursive_impl(
            &nested_features_path,
            base_path,
            changes_map,
            Some(&actual_owner),
        )
        .unwrap_or_default()
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
                let nested_feature = process_feature_directory(
                    &entry_path,
                    base_path,
                    &entry_name,
                    changes_map,
                    Some(&actual_owner),
                )?;
                nested_features.push(nested_feature);
            } else {
                // This directory is not a feature, but might contain features
                // Recursively search for features inside it
                let deeper_features = list_files_recursive_impl(
                    &entry_path,
                    base_path,
                    changes_map,
                    Some(&actual_owner),
                )?;
                nested_features.extend(deeper_features);
            }
        }
    }

    // Compute stats from changes if available
    let stats = compute_stats_from_changes(&changes, path, &nested_features);

    // Make path relative to base_path
    let relative_path = path
        .strip_prefix(base_path)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();

    Ok(Feature {
        name: readme_info.title.unwrap_or_else(|| name.to_string()),
        description: readme_info.description,
        owner: actual_owner,
        is_owner_inherited,
        path: relative_path,
        features: nested_features,
        meta: readme_info.meta,
        changes,
        decisions,
        stats,
    })
}

fn list_files_recursive_impl(
    dir: &Path,
    base_path: &Path,
    changes_map: Option<&HashMap<String, Vec<Change>>>,
    parent_owner: Option<&str>,
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
                let feature =
                    process_feature_directory(&path, base_path, &name, changes_map, parent_owner)?;
                features.push(feature);
            } else if !is_documentation_directory(&path)
                && !is_inside_documentation_directory(&path)
            {
                // Recursively search for features in non-documentation subdirectories
                let new_features =
                    list_files_recursive_impl(&path, base_path, changes_map, parent_owner)?;
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
