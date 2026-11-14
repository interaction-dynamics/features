//! Integration tests for diff mode (`--since`).
//!
//! Each test builds a temporary git repository whose features live in a
//! subfolder (`app/src`), commits a base state, then modifies, renames and
//! deletes files and checks which features the CLI reports as touched.

use std::fs;
use std::path::Path;
use std::process::Command;

use git2::{Repository, Signature};
use serde_json::Value;
use tempfile::TempDir;

const SCAN_ROOT: &str = "app/src";

fn write(root: &Path, path: &str, content: &str) {
    let full = root.join(path);
    fs::create_dir_all(full.parent().unwrap()).unwrap();
    fs::write(full, content).unwrap();
}

/// Stage every change in the working tree (including deletions) and commit it.
fn commit_all(repo: &Repository, message: &str) -> git2::Oid {
    let mut index = repo.index().unwrap();
    index
        .add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)
        .unwrap();
    index.update_all(["*"].iter(), None).unwrap();
    index.write().unwrap();
    let tree = repo.find_tree(index.write_tree().unwrap()).unwrap();
    let signature = Signature::now("Test", "test@example.com").unwrap();
    let parents: Vec<_> = repo
        .head()
        .ok()
        .and_then(|head| head.peel_to_commit().ok())
        .into_iter()
        .collect();
    let parents: Vec<_> = parents.iter().collect();
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &parents,
    )
    .unwrap()
}

/// Create the repository, commit the base state as branch `base`, then commit
/// a change on top and leave some uncommitted edits in the working tree.
fn setup_repo() -> TempDir {
    let dir = TempDir::new().unwrap();
    let root = dir.path();
    let repo = Repository::init(root).unwrap();

    write(root, "README.md", "# Repo\n");
    write(root, "app/src/utils/u.ts", "export const u = 1\n");
    write(
        root,
        "app/src/features/auth/README.md",
        "---\nowner: team-auth\n---\n# Auth\n",
    );
    write(root, "app/src/features/auth/a.ts", "export const a = 1\n");
    write(
        root,
        "app/src/features/auth/old.ts",
        "export function moved() {\n  return 'this file moves to billing'\n}\n",
    );
    write(
        root,
        "app/src/features/billing/README.md",
        "---\nowner: team-billing\n---\n# Billing\n",
    );
    write(
        root,
        "app/src/features/billing/b.ts",
        "export const b = 1\n",
    );
    write(
        root,
        "app/src/features/billing/gone.ts",
        "export const gone = 1\n",
    );
    write(
        root,
        "app/src/features/billing/features/invoices/README.md",
        "# Invoices\n",
    );
    write(
        root,
        "app/src/features/billing/features/invoices/i.ts",
        "export const i = 1\n",
    );

    let base = commit_all(&repo, "base");
    repo.branch("base", &repo.find_commit(base).unwrap(), false)
        .unwrap();

    // Committed change: modify, rename across features, delete, touch a file
    // outside any feature
    write(root, "app/src/features/auth/a.ts", "export const a = 2\n");
    fs::rename(
        root.join("app/src/features/auth/old.ts"),
        root.join("app/src/features/billing/moved.ts"),
    )
    .unwrap();
    fs::remove_file(root.join("app/src/features/billing/gone.ts")).unwrap();
    write(root, "app/src/utils/u.ts", "export const u = 2\n");
    write(root, "README.md", "# Repo\n\nUpdated\n");
    commit_all(&repo, "change");

    // Uncommitted: an unstaged edit and an untracked file
    write(
        root,
        "app/src/features/billing/features/invoices/i.ts",
        "export const i = 2\n",
    );
    write(root, "app/src/features/auth/new.ts", "export const n = 1\n");

    dir
}

fn run_cli(root: &Path, args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_features"))
        .current_dir(root)
        .args(args)
        .output()
        .expect("failed to run features binary");
    assert!(
        output.status.success(),
        "CLI failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn run_json(root: &Path, extra: &[&str]) -> Value {
    let mut args = vec![SCAN_ROOT, "--since", "base", "--json"];
    args.extend_from_slice(extra);
    serde_json::from_str(&run_cli(root, &args)).expect("output should be JSON")
}

fn feature<'a>(report: &'a Value, path: &str) -> &'a Value {
    report["features"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["path"] == path)
        .unwrap_or_else(|| panic!("feature {} not in report: {:#}", path, report))
}

fn file_entries(feature: &Value) -> Vec<(String, String, Option<String>)> {
    feature["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| {
            (
                f["status"].as_str().unwrap().to_string(),
                f["path"].as_str().unwrap().to_string(),
                f["old_path"].as_str().map(str::to_string),
            )
        })
        .collect()
}

fn feature_paths(report: &Value) -> Vec<&str> {
    report["features"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["path"].as_str().unwrap())
        .collect()
}

#[test]
fn reports_committed_and_uncommitted_changes_by_feature() {
    let repo = setup_repo();
    let report = run_json(repo.path(), &[]);

    assert_eq!(report["since"], "base");
    assert_eq!(report["scan_root"], SCAN_ROOT);
    assert_eq!(report["include_uncommitted"], true);
    assert_eq!(
        feature_paths(&report),
        [
            "features/auth",
            "features/billing",
            "features/billing/features/invoices"
        ]
    );

    let rename = (
        "renamed".to_string(),
        "app/src/features/billing/moved.ts".to_string(),
        Some("app/src/features/auth/old.ts".to_string()),
    );

    let auth = feature(&report, "features/auth");
    assert_eq!(auth["owner"], "team-auth");
    assert_eq!(auth["is_owner_inherited"], false);
    let auth_files = file_entries(auth);
    assert!(auth_files.contains(&("modified".into(), "app/src/features/auth/a.ts".into(), None)));
    assert!(auth_files.contains(&("added".into(), "app/src/features/auth/new.ts".into(), None)));
    assert!(
        auth_files.contains(&rename),
        "rename should touch its old feature"
    );

    let billing = feature(&report, "features/billing");
    assert_eq!(billing["owner"], "team-billing");
    let billing_files = file_entries(billing);
    assert!(
        billing_files.contains(&rename),
        "rename should touch its new feature"
    );
    assert!(billing_files.contains(&(
        "deleted".into(),
        "app/src/features/billing/gone.ts".into(),
        None
    )));

    let invoices = feature(&report, "features/billing/features/invoices");
    assert_eq!(invoices["owner"], "team-billing");
    assert_eq!(invoices["is_owner_inherited"], true);

    let outside: Vec<_> = report["files_outside_features"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["path"].as_str().unwrap())
        .collect();
    assert_eq!(outside, ["README.md", "app/src/utils/u.ts"]);
}

#[test]
fn committed_only_ignores_the_working_tree() {
    let repo = setup_repo();
    let report = run_json(repo.path(), &["--committed-only"]);

    assert_eq!(report["include_uncommitted"], false);
    assert_eq!(
        feature_paths(&report),
        ["features/auth", "features/billing"]
    );
    let auth_files = file_entries(feature(&report, "features/auth"));
    assert!(
        !auth_files
            .iter()
            .any(|(_, path, _)| path.ends_with("new.ts"))
    );
}

#[test]
fn owners_only_prints_unique_prefixed_owners() {
    let repo = setup_repo();
    let output = run_cli(
        repo.path(),
        &[SCAN_ROOT, "--since", "base", "--owners-only"],
    );
    assert_eq!(output, "@team-auth\n@team-billing\n");

    let output = run_cli(
        repo.path(),
        &[
            SCAN_ROOT,
            "--since",
            "base",
            "--owners-only",
            "--codeowners-prefix",
            "@my-org/",
        ],
    );
    assert_eq!(output, "@my-org/team-auth\n@my-org/team-billing\n");
}

#[test]
fn markdown_and_text_formats() {
    let repo = setup_repo();
    let markdown = run_cli(
        repo.path(),
        &[SCAN_ROOT, "--since", "base", "--format", "markdown"],
    );
    assert!(markdown.contains("| Feature | Path | Owner | Files |"));
    assert!(markdown.contains("| `features/auth` | @team-auth |"));
    assert!(markdown.contains("@team-billing (inherited)"));

    let text = run_cli(repo.path(), &[SCAN_ROOT, "--since", "base"]);
    assert!(text.contains("owner: team-auth"));
    assert!(text.contains("R app/src/features/auth/old.ts -> app/src/features/billing/moved.ts"));
    assert!(text.contains("Files outside any feature:"));
}

#[test]
fn no_changes_exits_successfully_with_empty_report() {
    let repo = setup_repo();
    let output = run_cli(
        repo.path(),
        &[SCAN_ROOT, "--since", "HEAD", "--committed-only", "--json"],
    );
    let report: Value = serde_json::from_str(&output).unwrap();
    assert!(report["features"].as_array().unwrap().is_empty());
    assert!(
        report["files_outside_features"]
            .as_array()
            .unwrap()
            .is_empty()
    );

    let output = run_cli(
        repo.path(),
        &[
            SCAN_ROOT,
            "--since",
            "HEAD",
            "--committed-only",
            "--owners-only",
        ],
    );
    assert_eq!(output, "");
}

#[test]
fn path_defaults_to_current_directory() {
    let repo = setup_repo();
    let output = run_cli(
        &repo.path().join(SCAN_ROOT),
        &["--since", "base", "--owners-only"],
    );
    assert_eq!(output, "@team-auth\n@team-billing\n");
}

#[test]
fn rejects_unknown_revision() {
    let repo = setup_repo();
    let output = Command::new(env!("CARGO_BIN_EXE_features"))
        .current_dir(repo.path())
        .args([SCAN_ROOT, "--since", "does-not-exist"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown git revision"));
}
