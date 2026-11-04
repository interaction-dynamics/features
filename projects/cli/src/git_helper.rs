use anyhow::{Context, Result};
use git2::Repository;
use std::collections::HashMap;
use std::path::Path;

use crate::models::Change;

/// Get the repository URL from git config.
/// Tries to get the remote origin URL, returns None if not found.
pub fn get_repository_url(repo_path: &Path) -> Option<String> {
    let repo = Repository::discover(repo_path).ok()?;

    // Try to get the remote origin URL
    if let Ok(remote) = repo.find_remote("origin")
        && let Some(url) = remote.url()
    {
        return Some(url.to_string());
    }

    None
}

fn format_timestamp(time: git2::Time) -> String {
    let timestamp = time.seconds();
    let datetime = chrono::DateTime::from_timestamp(timestamp, 0)
        .unwrap_or_else(|| chrono::DateTime::from_timestamp(0, 0).unwrap());
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// The commit history of a repository, indexed for lookups by path and by commit.
#[derive(Default)]
pub struct CommitHistory {
    /// Changes for each repo-relative directory, in the order they were walked
    pub changes_by_path: HashMap<String, Vec<Change>>,
    /// Paths affected by each commit, by commit hash
    pub affected_paths: HashMap<String, Vec<String>>,
}

/// Get all commits for all paths in the repository at once.
/// This is much more efficient than calling get_commits_for_path for each path individually.
pub fn get_all_commits_by_path(repo_path: &Path) -> Result<CommitHistory> {
    let repo = Repository::discover(repo_path).with_context(|| {
        format!(
            "failed to discover git repository at `{}`",
            repo_path.display()
        )
    })?;

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    let mut history = CommitHistory::default();

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
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

        let change = Change {
            title,
            author_name: author.name().unwrap_or("").to_string(),
            author_email: author.email().unwrap_or("").to_string(),
            description,
            date: format_timestamp(commit.time()),
            hash: format!("{}", oid),
        };

        // Get all paths affected by this commit
        let affected_paths = get_affected_paths(&repo, &commit)?;

        // For each affected file path, add the change to all ancestor directories
        // This matches the behavior of commit_affects_path which uses starts_with
        for file_path in &affected_paths {
            let path_obj = Path::new(file_path);

            // Add to all ancestor directories (not the file itself, only dirs)
            for ancestor in path_obj.ancestors().skip(1) {
                if ancestor == Path::new("") {
                    break;
                }
                let ancestor_str = ancestor.to_string_lossy().to_string();

                // Commits arrive one at a time, so this commit can only be the last
                // entry. If it's there, all higher ancestors already have it as well.
                let changes_list = history.changes_by_path.entry(ancestor_str).or_default();
                if changes_list.last().is_some_and(|c| c.hash == change.hash) {
                    break;
                }
                changes_list.push(change.clone());
            }
        }

        history.affected_paths.insert(change.hash, affected_paths);
    }

    Ok(history)
}

/// Get all paths affected by a commit
fn get_affected_paths(repo: &Repository, commit: &git2::Commit) -> Result<Vec<String>> {
    let mut paths = Vec::new();

    // For the first commit (no parents), get all files in the tree
    if commit.parent_count() == 0 {
        let tree = commit.tree()?;
        collect_tree_paths(repo, &tree, "", &mut paths)?;
        return Ok(paths);
    }

    // For commits with parents, check the diff
    let tree = commit.tree()?;
    let parent = commit.parent(0)?;
    let parent_tree = parent.tree()?;

    let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;

    diff.foreach(
        &mut |delta, _| {
            let new_path = delta.new_file().path().and_then(|p| p.to_str());
            if let Some(path_str) = new_path {
                paths.push(path_str.to_string());
            }
            if let Some(path_str) = delta.old_file().path().and_then(|p| p.to_str())
                && new_path != Some(path_str)
                && !paths.iter().any(|p| p == path_str)
            {
                paths.push(path_str.to_string());
            }
            true
        },
        None,
        None,
        None,
    )?;

    Ok(paths)
}

/// Recursively collect all paths in a tree
fn collect_tree_paths(
    repo: &Repository,
    tree: &git2::Tree,
    prefix: &str,
    paths: &mut Vec<String>,
) -> Result<()> {
    for entry in tree.iter() {
        if let Some(name) = entry.name() {
            let path = if prefix.is_empty() {
                name.to_string()
            } else {
                format!("{}/{}", prefix, name)
            };

            paths.push(path.clone());

            if entry.kind() == Some(git2::ObjectType::Tree)
                && let Ok(subtree) = entry.to_object(repo).and_then(|obj| obj.peel_to_tree())
            {
                collect_tree_paths(repo, &subtree, &path, paths)?;
            }
        }
    }
    Ok(())
}
