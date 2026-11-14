//! Integration tests for `features --check` and its configurable rules.
//!
//! Runs the binary on `examples/check-rules`, whose `.features.yml` turns
//! every rule on and whose files break each of them once.

use serde_json::Value;
use std::path::PathBuf;
use std::process::{Command, Output};

fn example_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/check-rules")
}

fn run_check(extra_args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_features"))
        .current_dir(example_dir())
        .args(["src", "--check", "--skip-changes"])
        .args(extra_args)
        .output()
        .expect("failed to run the features binary")
}

fn json_report(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|e| {
        panic!(
            "invalid JSON ({e}):\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}

/// (rule, severity, path, file, line) of a finding
type FindingKey = (String, String, String, Option<String>, Option<u64>);

/// The key of every finding
fn finding_keys(report: &Value) -> Vec<FindingKey> {
    report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| {
            (
                f["rule"].as_str().unwrap().to_string(),
                f["severity"].as_str().unwrap().to_string(),
                f["path"].as_str().unwrap().to_string(),
                f["file"].as_str().map(String::from),
                f["line"].as_u64(),
            )
        })
        .collect()
}

fn key(
    rule: &str,
    severity: &str,
    path: &str,
    file: Option<&str>,
    line: Option<u64>,
) -> FindingKey {
    (
        rule.to_string(),
        severity.to_string(),
        path.to_string(),
        file.map(String::from),
        line,
    )
}

#[test]
fn test_every_rule_reports_its_finding() {
    let output = run_check(&["--format", "json"]);
    assert_eq!(output.status.code(), Some(1), "errors must fail the check");

    let report = json_report(&output);
    assert_eq!(
        finding_keys(&report),
        vec![
            key(
                "duplicate-names",
                "error",
                "features/billing/features/invoices",
                None,
                None
            ),
            key(
                "missing-owner",
                "warn",
                "features/billing/features/invoices",
                None,
                None
            ),
            key("missing-owner", "warn", "features/checkout", None, None),
            key(
                "missing-owner",
                "warn",
                "features/legacy/features/invoices",
                None,
                None
            ),
            key("missing-readme", "warn", "features/checkout", None, None),
            key("min-coverage", "error", "features/billing", None, None),
            key("stale-flag", "warn", "features/billing", None, None),
            key(
                "cross-feature-import",
                "error",
                "features/billing",
                Some("features/billing/index.ts"),
                Some(2)
            ),
        ]
    );

    let messages: Vec<&str> = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["message"].as_str().unwrap())
        .collect();
    assert!(messages[5].starts_with("Line coverage is 25.0% (1/4 lines), below 50%"));
    assert!(
        messages[6].starts_with("Feature flag 'new-invoice-layout' was introduced on 2020-01-01")
    );
    assert!(messages[7].contains("'features/checkout'"));

    assert_eq!(report["summary"]["errors"], 3);
    assert_eq!(report["summary"]["warnings"], 5);
    assert_eq!(report["summary"]["rules"].as_array().unwrap().len(), 6);
}

#[test]
fn test_json_flag_selects_json_output() {
    let output = run_check(&["--json"]);
    let report = json_report(&output);
    assert_eq!(report["summary"]["errors"], 3);
}

#[test]
fn test_cli_overrides_beat_the_config_file() {
    let output = run_check(&[
        "--format",
        "json",
        "--disable-rule",
        "duplicate-names",
        "--disable-rule",
        "min-coverage",
        "--rule",
        "cross-feature-import=warn",
    ]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "warnings alone must not fail the check: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let report = json_report(&output);
    assert_eq!(report["summary"]["errors"], 0);
    assert_eq!(report["summary"]["warnings"], 6);
    assert_eq!(
        report["summary"]["disabled_rules"],
        serde_json::json!(["duplicate-names", "min-coverage"])
    );
}

#[test]
fn test_explicit_config_file_replaces_the_discovered_one() {
    let dir = tempfile::tempdir().unwrap();
    let config = dir.path().join("strict.yml");
    std::fs::write(
        &config,
        "check:\n  rules:\n    duplicate-names: off\n    missing-owner: error\n    stale-flag: off\n",
    )
    .unwrap();

    let output = run_check(&["--format", "json", "--config", config.to_str().unwrap()]);
    let report = json_report(&output);
    let rules: Vec<String> = finding_keys(&report)
        .into_iter()
        .map(|(rule, severity, ..)| format!("{rule}:{severity}"))
        .collect();
    // Defaults apply to the rules the explicit file doesn't mention
    assert_eq!(
        rules,
        vec!["missing-owner:error"],
        "missing-readme, min-coverage and cross-feature-import are off by default"
    );
}

#[test]
fn test_text_output_groups_findings_by_rule() {
    let output = run_check(&[]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("error[duplicate-names] 1 finding"),
        "{stdout}"
    );
    assert!(
        stdout.contains("warn[missing-owner] 3 findings"),
        "{stdout}"
    );
    assert!(
        stdout.contains("features/billing/index.ts:2: Imports from sibling feature"),
        "{stdout}"
    );
    assert!(stdout.contains("3 error(s), 5 warning(s) (6 rules run)."));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Check failed: 3 error(s) found"));
}

#[test]
fn test_github_output_annotates_files_relative_to_the_current_dir() {
    let output = run_check(&["--format", "github"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(
            "::error file=src/features/billing/index.ts,line=2,title=features check%3A cross-feature-import::"
        ),
        "{stdout}"
    );
    assert!(
        stdout.contains("::warning file=src/features/checkout,title=features check%3A missing-owner::Feature has no owner"),
        "{stdout}"
    );
    assert!(
        stdout.contains(
            "::error file=src/features/billing/README.md,title=features check%3A min-coverage::"
        ),
        "{stdout}"
    );
}

#[test]
fn test_list_rules_shows_effective_severities() {
    let output = run_check(&["--list-rules", "--disable-rule", "stale-flag"]);
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 6, "{stdout}");
    assert!(lines[0].starts_with("duplicate-names"));
    assert!(stdout.contains("stale-flag            off"), "{stdout}");
    assert!(stdout.contains("min-coverage          error"), "{stdout}");
}

#[test]
fn test_unknown_rule_is_rejected() {
    let output = run_check(&["--enable-rule", "missing-tests"]);
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unknown rule `missing-tests`"), "{stderr}");
}

#[test]
fn test_check_options_require_check() {
    let output = Command::new(env!("CARGO_BIN_EXE_features"))
        .current_dir(example_dir())
        .args(["src", "--disable-rule", "stale-flag"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("can only be used with `features check`"),
        "{stderr}"
    );
}

#[test]
fn test_check_subcommand_matches_the_legacy_flag() {
    let subcommand = Command::new(env!("CARGO_BIN_EXE_features"))
        .current_dir(example_dir())
        .args(["check", "src", "--skip-changes", "--json"])
        .output()
        .unwrap();
    let legacy = run_check(&["--json"]);

    assert_eq!(subcommand.status.code(), Some(1));
    assert_eq!(json_report(&subcommand), json_report(&legacy));
}

#[test]
fn test_config_file_provides_the_other_options() {
    // `path: src` and `skip_changes: true` come from .features.yml
    let from_config = Command::new(env!("CARGO_BIN_EXE_features"))
        .current_dir(example_dir())
        .args(["check", "--json"])
        .output()
        .unwrap();
    assert_eq!(from_config.status.code(), Some(1));
    assert_eq!(
        json_report(&from_config),
        json_report(&run_check(&["--json"]))
    );

    // The output format can come from the file too
    let dir = tempfile::tempdir().unwrap();
    let config = dir.path().join("ci.yml");
    std::fs::write(
        &config,
        format!(
            "path: {}\nskip_changes: true\ncheck:\n  format: github\n",
            example_dir().join("src").display()
        ),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_features"))
        .current_dir(example_dir())
        .args(["check", "--config", config.to_str().unwrap()])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("::error file=src/features/billing/features/invoices"),
        "{stdout}"
    );
}

#[test]
fn test_unknown_config_key_is_an_error() {
    let dir = tempfile::tempdir().unwrap();
    let config = dir.path().join("typo.yml");
    std::fs::write(&config, "skip_change: true\n").unwrap();

    let output = run_check(&["--config", config.to_str().unwrap()]);
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("typo.yml"), "{stderr}");
    assert!(stderr.contains("skip_change"), "{stderr}");
}

#[test]
fn test_default_rules_on_a_project_without_config() {
    // no-features has no .features.yml; only the rules on by default run
    let output = Command::new(env!("CARGO_BIN_EXE_features"))
        .current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/no-features"))
        .args([".", "--check", "--skip-changes", "--format", "json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let report = json_report(&output);
    assert_eq!(
        report["summary"]["rules"],
        serde_json::json!(["duplicate-names", "missing-owner", "stale-flag"])
    );
}

#[test]
fn test_config_errors_name_the_file() {
    let dir = tempfile::tempdir().unwrap();
    let config = dir.path().join("broken.yml");
    std::fs::write(&config, "check:\n  rules:\n    missing-tests: error\n").unwrap();

    let output = run_check(&["--config", config.to_str().unwrap()]);
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("broken.yml"), "{stderr}");
    assert!(stderr.contains("unknown rule `missing-tests`"), "{stderr}");
}

#[test]
fn test_missing_config_file_is_an_error() {
    let output = run_check(&["--config", "does-not-exist.yml"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("does-not-exist.yml"));
}
