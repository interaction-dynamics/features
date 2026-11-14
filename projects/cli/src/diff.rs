//! Diff mode: list the features touched by a change, with their owners.
//!
//! The change is everything between the merge base of a git ref and `HEAD`,
//! plus (optionally) uncommitted and untracked files in the working tree.
//! Every changed file is mapped to its closest feature with the same logic as
//! `--find-owner`, so a pull request can ping the teams that own the code.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use git2::{Delta, DiffFile, DiffFindOptions, DiffOptions, ObjectType, Oid, Repository};

use crate::codeowners::format_owner;
use crate::models::Feature;
use crate::ownership::find_owner_for_relative_path;

/// Label used for features that have no owner, as in CODEOWNERS generation.
const UNKNOWN_OWNER: &str = "Unknown";

/// How a file changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ChangeStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
    Copied,
    Typechange,
}

impl ChangeStatus {
    fn letter(self) -> char {
        match self {
            ChangeStatus::Added => 'A',
            ChangeStatus::Modified => 'M',
            ChangeStatus::Deleted => 'D',
            ChangeStatus::Renamed => 'R',
            ChangeStatus::Copied => 'C',
            ChangeStatus::Typechange => 'T',
        }
    }
}

/// A changed file. Paths are relative to the repository root and use `/`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ChangedFile {
    /// Path after the change (the removed path for deletions)
    pub path: String,
    /// Path before the change, for renames and copies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_path: Option<String>,
    pub status: ChangeStatus,
}

impl ChangedFile {
    fn display(&self) -> String {
        match &self.old_path {
            Some(old) => format!("{} -> {}", old, self.path),
            None => self.path.clone(),
        }
    }
}

/// The files changed in a repository since a ref.
#[derive(Debug, Clone)]
pub struct ChangeSet {
    /// Merge base of the ref and HEAD
    pub merge_base: String,
    /// Commit HEAD points to
    pub head: String,
    /// The scanned folder, relative to the repository root (empty for the root)
    pub scan_root: PathBuf,
    pub files: Vec<ChangedFile>,
}

/// A feature touched by the change.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TouchedFeature {
    pub name: String,
    /// Feature folder, relative to the scanned path (as in the other outputs)
    pub path: String,
    pub owner: String,
    pub is_owner_inherited: bool,
    pub files: Vec<ChangedFile>,
}

/// The result of diff mode.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DiffReport {
    pub since: String,
    pub merge_base: String,
    pub head: String,
    pub include_uncommitted: bool,
    /// The scanned folder, relative to the repository root (empty for the root)
    pub scan_root: String,
    pub features: Vec<TouchedFeature>,
    pub files_outside_features: Vec<ChangedFile>,
}

/// Collect the files changed between the merge base of `since` and HEAD.
///
/// With `include_uncommitted`, the comparison is against the working tree
/// instead of HEAD, so staged, unstaged and untracked files are included too.
pub fn collect_changes(
    base_path: &Path,
    since: &str,
    include_uncommitted: bool,
) -> Result<ChangeSet> {
    let repo = Repository::discover(base_path).with_context(|| {
        format!(
            "failed to discover git repository at `{}`",
            base_path.display()
        )
    })?;
    let workdir = repo
        .workdir()
        .ok_or_else(|| anyhow!("diff mode needs a repository with a working tree"))?;

    let canonical_workdir = std::fs::canonicalize(workdir)?;
    let canonical_base = std::fs::canonicalize(base_path)
        .with_context(|| format!("path `{}` does not exist", base_path.display()))?;
    let scan_root = canonical_base
        .strip_prefix(&canonical_workdir)
        .map(Path::to_path_buf)
        .unwrap_or_default();

    let since_commit = repo
        .revparse_single(since)
        .and_then(|object| object.peel_to_commit())
        .with_context(|| format!("unknown git revision `{}`", since))?;
    let head_commit = repo
        .head()
        .and_then(|head| head.peel_to_commit())
        .context("failed to resolve HEAD")?;
    let merge_base = repo
        .merge_base(since_commit.id(), head_commit.id())
        .with_context(|| format!("`{}` and HEAD have no common ancestor", since))?;
    let base_tree = repo.find_commit(merge_base)?.tree()?;

    let mut options = DiffOptions::new();
    let mut diff = if include_uncommitted {
        options.include_untracked(true).recurse_untracked_dirs(true);
        repo.diff_tree_to_workdir_with_index(Some(&base_tree), Some(&mut options))?
    } else {
        let head_tree = head_commit.tree()?;
        repo.diff_tree_to_tree(Some(&base_tree), Some(&head_tree), Some(&mut options))?
    };

    let mut find_options = DiffFindOptions::new();
    find_options.renames(true).for_untracked(true);
    diff.find_similar(Some(&mut find_options))?;

    let empty_blob = Oid::hash_object(ObjectType::Blob, b"")?;
    let is_empty = |file: &DiffFile| file.size() == 0 || file.id() == empty_blob;
    let to_string = |path: &Path| path.to_string_lossy().replace('\\', "/");

    let mut files = Vec::new();
    for delta in diff.deltas() {
        let old_path = delta.old_file().path().map(to_string);
        let new_path = delta.new_file().path().map(to_string);
        let mut push = |path: Option<String>, old_path: Option<String>, status| {
            if let Some(path) = path {
                files.push(ChangedFile {
                    path,
                    old_path,
                    status,
                });
            }
        };

        match delta.status() {
            Delta::Added | Delta::Untracked => push(new_path, None, ChangeStatus::Added),
            Delta::Modified => push(new_path, None, ChangeStatus::Modified),
            Delta::Typechange => push(new_path, None, ChangeStatus::Typechange),
            Delta::Deleted => push(old_path, None, ChangeStatus::Deleted),
            // Empty files are all "similar" to each other. Like git, don't pair
            // them as renames, so an unrelated feature isn't reported as touched.
            Delta::Renamed if is_empty(&delta.old_file()) || is_empty(&delta.new_file()) => {
                push(old_path, None, ChangeStatus::Deleted);
                push(new_path, None, ChangeStatus::Added);
            }
            Delta::Copied if is_empty(&delta.old_file()) || is_empty(&delta.new_file()) => {
                push(new_path, None, ChangeStatus::Added);
            }
            Delta::Renamed => push(new_path, old_path, ChangeStatus::Renamed),
            Delta::Copied => push(new_path, old_path, ChangeStatus::Copied),
            _ => {}
        }
    }

    Ok(ChangeSet {
        merge_base: merge_base.to_string(),
        head: head_commit.id().to_string(),
        scan_root,
        files,
    })
}

/// Map each changed file to the features that contain it.
///
/// A rename touches the features of both its old and new path. Files outside
/// every feature, including files outside the scanned folder, are listed
/// separately.
pub fn build_report(
    since: &str,
    include_uncommitted: bool,
    changes: &ChangeSet,
    features: &[Feature],
) -> DiffReport {
    let mut touched: BTreeMap<String, TouchedFeature> = BTreeMap::new();
    let mut outside = Vec::new();

    for file in &changes.files {
        let mut feature_paths = BTreeSet::new();
        let candidates = std::iter::once(&file.path).chain(file.old_path.as_ref());

        for repo_path in candidates {
            let Ok(relative) = Path::new(repo_path).strip_prefix(&changes.scan_root) else {
                continue;
            };
            let Some(info) = find_owner_for_relative_path(relative, features) else {
                continue;
            };
            if !feature_paths.insert(info.feature_path.clone()) {
                continue;
            }
            touched
                .entry(info.feature_path.clone())
                .or_insert_with(|| TouchedFeature {
                    name: info.feature_name,
                    path: info.feature_path,
                    owner: info.owner,
                    is_owner_inherited: info.inherited,
                    files: Vec::new(),
                })
                .files
                .push(file.clone());
        }

        if feature_paths.is_empty() {
            outside.push(file.clone());
        }
    }

    DiffReport {
        since: since.to_string(),
        merge_base: changes.merge_base.clone(),
        head: changes.head.clone(),
        include_uncommitted,
        scan_root: changes.scan_root.to_string_lossy().replace('\\', "/"),
        features: touched.into_values().collect(),
        files_outside_features: outside,
    }
}

/// Unique owners of the touched features, sorted, with `prefix` added.
///
/// Features without an owner are left out, so the list can be passed straight
/// to `gh pr edit --add-reviewer`.
pub fn unique_owners(report: &DiffReport, prefix: &str) -> Vec<String> {
    report
        .features
        .iter()
        .filter(|feature| !feature.owner.is_empty())
        .map(|feature| format_owner(&feature.owner, prefix))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn owner_label(feature: &TouchedFeature) -> String {
    let owner = if feature.owner.is_empty() {
        UNKNOWN_OWNER
    } else {
        &feature.owner
    };
    if feature.is_owner_inherited {
        format!("{} (inherited)", owner)
    } else {
        owner.to_string()
    }
}

fn short(hash: &str) -> &str {
    &hash[..hash.len().min(7)]
}

/// Human-readable report for the terminal.
pub fn render_text(report: &DiffReport) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "Features touched since {} (merge base {}{}):",
        report.since,
        short(&report.merge_base),
        if report.include_uncommitted {
            ", including uncommitted changes"
        } else {
            ""
        }
    );

    if report.features.is_empty() {
        let _ = writeln!(out, "  No features touched.");
    }
    for feature in &report.features {
        let _ = writeln!(
            out,
            "\n  {} ({}) - owner: {}",
            feature.name,
            feature.path,
            owner_label(feature)
        );
        for file in &feature.files {
            let _ = writeln!(out, "    {} {}", file.status.letter(), file.display());
        }
    }

    if !report.files_outside_features.is_empty() {
        let _ = writeln!(out, "\nFiles outside any feature:");
        for file in &report.files_outside_features {
            let _ = writeln!(out, "    {} {}", file.status.letter(), file.display());
        }
    }
    out
}

fn escape_markdown_cell(value: &str) -> String {
    value.replace('|', "\\|")
}

/// Markdown report suited to a pull request comment.
pub fn render_markdown(report: &DiffReport, owner_prefix: &str) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "### Features touched by this change\n");

    if report.features.is_empty() {
        let _ = writeln!(out, "No features touched.");
    } else {
        let _ = writeln!(out, "| Feature | Path | Owner | Files |");
        let _ = writeln!(out, "| --- | --- | --- | --- |");
        for feature in &report.features {
            let owner = if feature.owner.is_empty() {
                UNKNOWN_OWNER.to_string()
            } else {
                format_owner(&feature.owner, owner_prefix)
            };
            let owner = if feature.is_owner_inherited {
                format!("{} (inherited)", owner)
            } else {
                owner
            };
            let _ = writeln!(
                out,
                "| {} | `{}` | {} | {} |",
                escape_markdown_cell(&feature.name),
                escape_markdown_cell(&feature.path),
                escape_markdown_cell(&owner),
                feature.files.len()
            );
        }
    }

    for feature in &report.features {
        let _ = writeln!(
            out,
            "\n<details><summary>{} ({} files)</summary>\n",
            escape_markdown_cell(&feature.name),
            feature.files.len()
        );
        for file in &feature.files {
            let _ = writeln!(out, "- `{}` {}", file.status.letter(), file.display());
        }
        let _ = writeln!(out, "\n</details>");
    }

    if !report.files_outside_features.is_empty() {
        let _ = writeln!(
            out,
            "\n<details><summary>Files outside any feature ({})</summary>\n",
            report.files_outside_features.len()
        );
        for file in &report.files_outside_features {
            let _ = writeln!(out, "- `{}` {}", file.status.letter(), file.display());
        }
        let _ = writeln!(out, "\n</details>");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn feature(name: &str, path: &str, owner: &str, children: Vec<Feature>) -> Feature {
        Feature {
            name: name.to_string(),
            key: name.to_string(),
            description: String::new(),
            owner: owner.to_string(),
            is_owner_inherited: false,
            path: path.to_string(),
            features: children,
            meta: Default::default(),
            links: Default::default(),
            changes: Vec::new(),
            decisions: Vec::new(),
            records: Vec::new(),
            stats: None,
            dependencies: Vec::new(),
            external: Default::default(),
        }
    }

    fn file(path: &str, status: ChangeStatus) -> ChangedFile {
        ChangedFile {
            path: path.to_string(),
            old_path: None,
            status,
        }
    }

    fn change_set(scan_root: &str, files: Vec<ChangedFile>) -> ChangeSet {
        ChangeSet {
            merge_base: "0123456789abcdef".to_string(),
            head: "fedcba9876543210".to_string(),
            scan_root: PathBuf::from(scan_root),
            files,
        }
    }

    fn sample_features() -> Vec<Feature> {
        vec![
            feature(
                "auth",
                "features/auth",
                "team-a",
                vec![feature(
                    "login",
                    "features/auth/features/login",
                    "team-b",
                    vec![],
                )],
            ),
            feature("billing", "features/billing", "", vec![]),
        ]
    }

    #[test]
    fn groups_files_by_closest_feature() {
        let changes = change_set(
            "",
            vec![
                file("features/auth/a.ts", ChangeStatus::Modified),
                file("features/auth/features/login/b.ts", ChangeStatus::Added),
                file("features/auth/c.ts", ChangeStatus::Deleted),
                file("README.md", ChangeStatus::Modified),
            ],
        );
        let report = build_report("main", false, &changes, &sample_features());

        let paths: Vec<_> = report.features.iter().map(|f| f.path.as_str()).collect();
        assert_eq!(paths, ["features/auth", "features/auth/features/login"]);
        assert_eq!(report.features[0].files.len(), 2);
        assert_eq!(report.features[1].owner, "team-b");
        assert_eq!(
            report.files_outside_features,
            [file("README.md", ChangeStatus::Modified)]
        );
    }

    #[test]
    fn rename_touches_both_features() {
        let mut renamed = file("features/billing/x.ts", ChangeStatus::Renamed);
        renamed.old_path = Some("features/auth/x.ts".to_string());
        let report = build_report(
            "main",
            false,
            &change_set("", vec![renamed]),
            &sample_features(),
        );

        let paths: Vec<_> = report.features.iter().map(|f| f.path.as_str()).collect();
        assert_eq!(paths, ["features/auth", "features/billing"]);
        assert!(report.files_outside_features.is_empty());
    }

    #[test]
    fn rename_within_a_feature_is_listed_once() {
        let mut renamed = file("features/auth/new.ts", ChangeStatus::Renamed);
        renamed.old_path = Some("features/auth/old.ts".to_string());
        let report = build_report(
            "main",
            false,
            &change_set("", vec![renamed]),
            &sample_features(),
        );

        assert_eq!(report.features.len(), 1);
        assert_eq!(report.features[0].files.len(), 1);
    }

    #[test]
    fn strips_scan_root_and_reports_files_outside_it() {
        let changes = change_set(
            "app/src",
            vec![
                file("app/src/features/auth/a.ts", ChangeStatus::Modified),
                file("features/auth/a.ts", ChangeStatus::Modified),
                file("docs/guide.md", ChangeStatus::Added),
            ],
        );
        let report = build_report("main", false, &changes, &sample_features());

        assert_eq!(report.scan_root, "app/src");
        assert_eq!(report.features.len(), 1);
        assert_eq!(
            report.features[0].files[0].path,
            "app/src/features/auth/a.ts"
        );
        assert_eq!(report.files_outside_features.len(), 2);
    }

    #[test]
    fn no_changes_gives_empty_report() {
        let report = build_report("main", true, &change_set("", vec![]), &sample_features());
        assert!(report.features.is_empty());
        assert!(report.files_outside_features.is_empty());
        assert!(render_text(&report).contains("No features touched."));
        assert!(render_markdown(&report, "@").contains("No features touched."));
        assert!(unique_owners(&report, "@").is_empty());
    }

    #[test]
    fn unique_owners_are_sorted_prefixed_and_skip_unknown() {
        let changes = change_set(
            "",
            vec![
                file("features/auth/features/login/a.ts", ChangeStatus::Modified),
                file("features/auth/a.ts", ChangeStatus::Modified),
                file("features/billing/a.ts", ChangeStatus::Modified),
            ],
        );
        let report = build_report("main", false, &changes, &sample_features());
        assert_eq!(unique_owners(&report, "@"), ["@team-a", "@team-b"]);
        assert_eq!(unique_owners(&report, ""), ["team-a", "team-b"]);
    }

    #[test]
    fn markdown_lists_features_and_escapes_pipes() {
        let features = vec![feature("a|b", "features/ab", "team-a", vec![])];
        let changes = change_set("", vec![file("features/ab/x.ts", ChangeStatus::Added)]);
        let markdown = render_markdown(&build_report("main", false, &changes, &features), "@");

        assert!(markdown.contains("| a\\|b | `features/ab` | @team-a | 1 |"));
        assert!(markdown.contains("- `A` features/ab/x.ts"));
    }

    #[test]
    fn text_shows_unknown_owner_and_renames() {
        let mut renamed = file("features/billing/new.ts", ChangeStatus::Renamed);
        renamed.old_path = Some("features/billing/old.ts".to_string());
        let text = render_text(&build_report(
            "main",
            false,
            &change_set("", vec![renamed]),
            &sample_features(),
        ));

        assert!(text.contains("billing (features/billing) - owner: Unknown"));
        assert!(text.contains("R features/billing/old.ts -> features/billing/new.ts"));
    }
}
