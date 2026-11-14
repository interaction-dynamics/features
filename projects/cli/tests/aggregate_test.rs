//! Integration tests for the `features aggregate` command

use serde_json::{Value, json};
use std::path::Path;
use std::process::Command;

fn feature(name: &str, path: &str, children: Vec<Value>) -> Value {
    json!({
        "name": name,
        "description": "",
        "owner": "",
        "is_owner_inherited": false,
        "path": path,
        "features": children,
        "meta": {},
        "changes": [],
        "decisions": [],
        "dependencies": []
    })
}

fn write_report(path: &Path, features: Value) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, serde_json::to_string_pretty(&features).unwrap()).unwrap();
}

/// Repository "repo-a": B > C, auth > login. Repository "repo-d" (in a folder): C > B, login.
fn create_reports() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    write_report(
        &dir.path().join("repo-a.json"),
        json!([
            feature("B", "src/b", vec![feature("C", "src/b/c", vec![])]),
            feature(
                "auth",
                "src/auth",
                vec![feature("login", "src/auth/login", vec![])]
            ),
        ]),
    );
    write_report(
        &dir.path().join("repo-d").join("features.json"),
        json!([
            feature("C", "c", vec![feature("B", "c/b", vec![])]),
            feature("login", "login", vec![feature("sso", "login/sso", vec![])]),
            feature("billing", "billing", vec![]),
        ]),
    );
    dir
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_features"))
        .args(args)
        .output()
        .expect("failed to run features")
}

#[test]
fn test_aggregate_json_merges_features_and_reports_loops() {
    let reports = create_reports();
    let output = run(&["aggregate", reports.path().to_str().unwrap(), "--json"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["repositories"], json!(["repo-a", "repo-d"]));

    let roots: Vec<(&str, &str)> = report["features"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| (f["name"].as_str().unwrap(), f["path"].as_str().unwrap()))
        .collect();
    assert_eq!(
        roots,
        vec![
            ("auth", "auth"),
            ("B", "B@repo-a"),
            ("billing", "billing"),
            ("C", "C@repo-d"),
        ]
    );

    // login is merged under auth and gets the children of both repositories
    let auth = &report["features"][0];
    assert_eq!(
        auth["repositories"],
        json!([{ "name": "repo-a", "path": "src/auth" }])
    );
    let login = &auth["features"][0];
    assert_eq!(login["path"], "auth/login");
    assert_eq!(
        login["repositories"],
        json!([
            { "name": "repo-a", "path": "src/auth/login" },
            { "name": "repo-d", "path": "login" }
        ])
    );
    assert_eq!(login["features"][0]["path"], "auth/login/sso");

    // B and C are not merged
    let errors = report["errors"].as_array().unwrap();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["type"], "cycle");
    assert_eq!(errors[0]["features"], json!(["B", "C"]));
    assert_eq!(errors[0]["occurrences"].as_array().unwrap().len(), 4);
}

#[test]
fn test_aggregate_creates_a_static_build() {
    let reports = create_reports();
    let build_dir = tempfile::tempdir().unwrap();
    let output = run(&[
        "aggregate",
        reports.path().to_str().unwrap(),
        "--build-dir",
        build_dir.path().to_str().unwrap(),
    ]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let features: Value = serde_json::from_str(
        &std::fs::read_to_string(build_dir.path().join("features.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(features["errors"].as_array().unwrap().len(), 1);
    assert!(build_dir.path().join("metadata.json").exists());
}

#[test]
fn test_aggregate_fails_without_reports() {
    let empty = tempfile::tempdir().unwrap();
    let output = run(&["aggregate", empty.path().to_str().unwrap(), "--json"]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("no features JSON report"));
}
