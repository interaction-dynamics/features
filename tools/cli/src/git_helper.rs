use anyhow::{Context, Result};
use git2::Repository;
use std::path::Path;

use crate::models::Change;

pub fn get_commits_for_path(repo_path: &Path, feature_path: &str) -> Result<Vec<Change>> {
    let repo = Repository::discover(repo_path).with_context(|| {
        format!(
            "failed to discover git repository at `{}`",
            repo_path.display()
        )
    })?;

    // Convert the feature path to be relative to the repository root
    let repo_workdir = repo
        .workdir()
        .context("repository has no working directory")?;
    let feature_abs_path = std::fs::canonicalize(feature_path)
        .with_context(|| format!("failed to canonicalize path `{}`", feature_path))?;

    let relative_path = feature_abs_path
        .strip_prefix(repo_workdir)
        .with_context(|| format!("path `{}` is not within repository", feature_path))?;

    let relative_path_str = relative_path.to_string_lossy();

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    let mut changes = Vec::new();

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;

        // Check if this commit affects the feature path
        if commit_affects_path(&repo, &commit, &relative_path_str)? {
            let author = commit.author();
            let message = commit.message().unwrap_or("").to_string();

            // Split message into title and description
            let lines: Vec<&str> = message.lines().collect();
            let title = lines.first().unwrap_or(&"").to_string();
            let description = if lines.len() > 1 {
                lines[1..].join("\n").trim().to_string()
            } else {
                String::new()
            };

            changes.push(Change {
                title,
                author_name: author.name().unwrap_or("Unknown").to_string(),
                author_email: author.email().unwrap_or("").to_string(),
                description,
                date: format_timestamp(commit.time()),
                hash: format!("{}", oid),
            });
        }
    }

    Ok(changes)
}

fn commit_affects_path(repo: &Repository, commit: &git2::Commit, path: &str) -> Result<bool> {
    // For the first commit (no parents), check if any files in the path exist in the tree
    if commit.parent_count() == 0 {
        let tree = commit.tree()?;
        return Ok(tree_contains_path(&tree, path));
    }

    // For commits with parents, check the diff
    let tree = commit.tree()?;
    let parent = commit.parent(0)?;
    let parent_tree = parent.tree()?;

    let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;

    let mut affects_path = false;
    diff.foreach(
        &mut |delta, _| {
            if let Some(path_str) = delta.new_file().path() {
                if path_str.starts_with(path) {
                    affects_path = true;
                }
            }
            if let Some(path_str) = delta.old_file().path() {
                if path_str.starts_with(path) {
                    affects_path = true;
                }
            }
            true
        },
        None,
        None,
        None,
    )?;

    Ok(affects_path)
}

fn tree_contains_path(tree: &git2::Tree, path: &str) -> bool {
    tree.get_path(Path::new(path)).is_ok()
}

fn format_timestamp(time: git2::Time) -> String {
    let timestamp = time.seconds();
    let datetime = chrono::DateTime::from_timestamp(timestamp, 0)
        .unwrap_or_else(|| chrono::DateTime::from_timestamp(0, 0).unwrap());
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
