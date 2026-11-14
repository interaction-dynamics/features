//! Printing of check reports.

use anyhow::Result;
use colored::Colorize;
use std::path::{Path, PathBuf};

use super::{CheckContext, CheckReport, Finding, Severity};

/// How the check report is printed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, clap::ValueEnum, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    /// Human-readable findings, grouped by rule
    #[default]
    Text,
    /// A JSON document with the findings and a summary
    Json,
    /// GitHub Actions workflow commands (`::error file=...::message`)
    Github,
}

pub fn print(report: &CheckReport, ctx: &CheckContext, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Text => print!("{}", format_text(report)),
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(report)?),
        OutputFormat::Github => {
            let prefix = display_prefix(&ctx.base_path);
            for finding in &report.findings {
                println!("{}", format_github(finding, &ctx.base_path, &prefix));
            }
            print!("{}", format_summary(report));
        }
    }
    Ok(())
}

fn location(finding: &Finding) -> String {
    match (&finding.file, finding.line) {
        (Some(file), Some(line)) => format!("{}:{}", file, line),
        (Some(file), None) => file.clone(),
        _ => finding.path.clone(),
    }
}

fn format_text(report: &CheckReport) -> String {
    let mut out = String::new();
    let mut current_rule = None;

    for finding in &report.findings {
        if current_rule != Some(finding.rule) {
            current_rule = Some(finding.rule);
            let count = report
                .findings
                .iter()
                .filter(|f| f.rule == finding.rule)
                .count();
            let label = match finding.severity {
                Severity::Error => format!("error[{}]", finding.rule).red().bold(),
                _ => format!("warn[{}]", finding.rule).yellow().bold(),
            };
            out.push_str(&format!(
                "{} {} finding{}\n",
                label,
                count,
                if count == 1 { "" } else { "s" }
            ));
        }
        out.push_str(&format!("  {}: {}\n", location(finding), finding.message));
    }

    if !report.findings.is_empty() {
        out.push('\n');
    }
    out.push_str(&format_summary(report));
    out
}

fn format_summary(report: &CheckReport) -> String {
    let summary = &report.summary;
    let rules = format!(
        "{} rule{} run",
        summary.rules.len(),
        if summary.rules.len() == 1 { "" } else { "s" }
    );
    let disabled = if summary.disabled_rules.is_empty() {
        String::new()
    } else {
        format!(", disabled: {}", summary.disabled_rules.join(", "))
    };

    if summary.errors == 0 && summary.warnings == 0 {
        format!("All checks passed ({}{}).\n", rules, disabled)
    } else {
        format!(
            "{} error(s), {} warning(s) ({}{}).\n",
            summary.errors, summary.warnings, rules, disabled
        )
    }
}

/// The scanned directory as a path relative to the current directory, so
/// that annotations point to files GitHub can find in the repository.
fn display_prefix(base_path: &Path) -> PathBuf {
    let absolute = base_path
        .canonicalize()
        .unwrap_or_else(|_| base_path.to_path_buf());
    std::env::current_dir()
        .ok()
        .and_then(|cwd| cwd.canonicalize().ok())
        .and_then(|cwd| absolute.strip_prefix(cwd).ok().map(Path::to_path_buf))
        .unwrap_or_else(|| base_path.to_path_buf())
}

/// The file a finding points to: its own file, else the feature's README, else the feature folder
fn annotation_file(finding: &Finding, base_path: &Path) -> String {
    if let Some(file) = &finding.file {
        return file.clone();
    }
    for readme in ["README.md", "README.mdx", "FEATURES.toml"] {
        let candidate = Path::new(&finding.path).join(readme);
        if base_path.join(&candidate).is_file() {
            return candidate.to_string_lossy().to_string();
        }
    }
    finding.path.clone()
}

fn escape_data(value: &str) -> String {
    value
        .replace('%', "%25")
        .replace('\r', "%0D")
        .replace('\n', "%0A")
}

fn escape_property(value: &str) -> String {
    escape_data(value).replace(':', "%3A").replace(',', "%2C")
}

fn format_github(finding: &Finding, base_path: &Path, prefix: &Path) -> String {
    let command = match finding.severity {
        Severity::Error => "error",
        _ => "warning",
    };
    let file = prefix.join(annotation_file(finding, base_path));
    let mut properties = vec![format!(
        "file={}",
        escape_property(&file.to_string_lossy().replace('\\', "/"))
    )];
    if let Some(line) = finding.line {
        properties.push(format!("line={}", line));
    }
    properties.push(format!(
        "title={}",
        escape_property(&format!("features check: {}", finding.rule))
    ));

    format!(
        "::{} {}::{}",
        command,
        properties.join(","),
        escape_data(&finding.message)
    )
}

#[cfg(test)]
mod tests {
    use super::super::Summary;
    use super::*;

    fn finding(rule: &'static str, severity: Severity, file: Option<&str>) -> Finding {
        Finding {
            rule,
            severity,
            feature: "billing".to_string(),
            path: "features/billing".to_string(),
            file: file.map(String::from),
            line: file.map(|_| 3),
            message: "Something, wrong: 100%".to_string(),
        }
    }

    fn report(findings: Vec<Finding>) -> CheckReport {
        let errors = findings
            .iter()
            .filter(|f| f.severity == Severity::Error)
            .count();
        let warnings = findings.len() - errors;
        CheckReport {
            findings,
            summary: Summary {
                errors,
                warnings,
                rules: vec!["missing-owner", "cross-feature-import"],
                disabled_rules: vec!["min-coverage"],
            },
        }
    }

    #[test]
    fn test_format_text_groups_by_rule() {
        colored::control::set_override(false);
        let text = format_text(&report(vec![
            finding("missing-owner", Severity::Warn, None),
            finding("missing-owner", Severity::Warn, None),
            finding(
                "cross-feature-import",
                Severity::Error,
                Some("features/billing/a.ts"),
            ),
        ]));

        assert!(text.contains("warn[missing-owner] 2 findings\n"), "{text}");
        assert!(text.contains("error[cross-feature-import] 1 finding\n"));
        assert!(text.contains("  features/billing/a.ts:3: Something"));
        assert!(text.contains("  features/billing: Something"));
        assert!(text.contains("1 error(s), 2 warning(s) (2 rules run, disabled: min-coverage)."));
    }

    #[test]
    fn test_format_text_when_everything_passes() {
        let text = format_text(&report(vec![]));
        assert_eq!(
            text,
            "All checks passed (2 rules run, disabled: min-coverage).\n"
        );
    }

    #[test]
    fn test_format_github_escapes_and_points_to_files() {
        let line = format_github(
            &finding(
                "cross-feature-import",
                Severity::Error,
                Some("features/billing/a.ts"),
            ),
            Path::new("/nonexistent"),
            Path::new("src"),
        );
        assert_eq!(
            line,
            "::error file=src/features/billing/a.ts,line=3,title=features check%3A cross-feature-import::Something, wrong: 100%25"
        );

        let line = format_github(
            &finding("missing-owner", Severity::Warn, None),
            Path::new("/nonexistent"),
            Path::new(""),
        );
        assert!(
            line.starts_with("::warning file=features/billing,title="),
            "{line}"
        );
    }

    #[test]
    fn test_annotation_file_prefers_readme() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("features/billing")).unwrap();
        std::fs::write(dir.path().join("features/billing/README.md"), "").unwrap();
        assert_eq!(
            annotation_file(&finding("missing-owner", Severity::Warn, None), dir.path()),
            "features/billing/README.md"
        );
    }
}
