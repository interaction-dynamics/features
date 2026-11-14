//! Git history for each feature: its changes and commit statistics.

use anyhow::{Context, Result};
use git2::Repository;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use super::for_each_feature_mut;
use crate::git_helper::CommitHistory;
use crate::models::{Change, Feature};

/// Fill in each feature's `changes` from the pre-computed commit history, and
/// its commit statistics when the file stats pass already gave it `stats`.
pub(super) fn populate_history(
    features: &mut [Feature],
    base_path: &Path,
    history: &CommitHistory,
) {
    for_each_feature_mut(features, &mut |feature| {
        let Ok(feature_history) = get_feature_history(&base_path.join(&feature.path), history)
        else {
            return;
        };

        // Commit statistics are only available if the feature has changes
        if !feature_history.changes.is_empty()
            && let Some(stats) = feature.stats.as_mut()
        {
            stats.commits = compute_commit_stats(&feature_history, history, &feature.features);
        }

        feature.changes = feature_history.changes;
    });
}

/// A feature's changes and where it sits in its repository
struct FeatureHistory {
    changes: Vec<Change>,
    repo_workdir: PathBuf,
    relative_path: String,
}

/// Get a feature's changes from the pre-computed commit history
fn get_feature_history(path: &Path, history: &CommitHistory) -> Result<FeatureHistory> {
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
        .context("path is not within repository")?
        .to_string_lossy()
        .to_string();

    // Look up the changes in the map
    let changes = history
        .changes_by_path
        .get(&relative_path)
        .cloned()
        .unwrap_or_default();

    Ok(FeatureHistory {
        changes,
        repo_workdir: repo_workdir.to_path_buf(),
        relative_path,
    })
}

/// Compute commit statistics for a feature
fn compute_commit_stats(
    feature_history: &FeatureHistory,
    history: &CommitHistory,
    nested_features: &[Feature],
) -> BTreeMap<String, serde_json::Value> {
    // Convert nested feature paths to repo-relative paths once for all commits
    let nested_relative_paths: Vec<String> = nested_features
        .iter()
        .filter_map(|feature| {
            let canonical = fs::canonicalize(&feature.path).ok()?;
            let relative = canonical.strip_prefix(&feature_history.repo_workdir).ok()?;
            Some(relative.to_string_lossy().to_string())
        })
        .collect();

    // Filter changes to only include those that affect files in this feature
    // (not exclusively in nested features)
    let filtered_changes: Vec<&Change> = feature_history
        .changes
        .iter()
        .filter(|change| {
            let Some(affected_files) = history.affected_paths.get(&change.hash) else {
                return false;
            };

            // Check if any affected file is in this feature but not in a nested feature
            affected_files.iter().any(|file_path| {
                file_path.starts_with(&feature_history.relative_path)
                    && !nested_relative_paths
                        .iter()
                        .any(|nested_path| file_path.starts_with(nested_path.as_str()))
            })
        })
        .collect();

    let mut commits = BTreeMap::new();

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

    commits
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
