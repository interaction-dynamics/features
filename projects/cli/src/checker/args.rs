//! Command-line options of the checks.

use anyhow::{Context, Result};
use serde::Serialize;

use super::{CheckConfig, CliOverrides, OutputFormat, Severity, registry};
use crate::config::ProjectConfig;

/// Options of `features --check`, meant to be flattened into the CLI.
#[derive(Debug, Clone, Default, PartialEq, clap::Args)]
pub struct CheckArgs {
    /// Set the severity of a check rule (error, warn or off), e.g. --rule min-coverage=error.
    /// Can be repeated
    #[arg(long = "rule", value_name = "ID=SEVERITY", value_parser = parse_rule)]
    pub rules: Vec<(String, Severity)>,

    /// Turn on a check rule that is off (it then reports errors). Can be repeated
    #[arg(long, value_name = "ID")]
    pub enable_rule: Vec<String>,

    /// Turn off a check rule. Can be repeated
    #[arg(long, value_name = "ID")]
    pub disable_rule: Vec<String>,

    /// Output format of the checks [default: text, or json with --json]
    #[arg(long, value_enum, value_name = "FORMAT")]
    pub format: Option<OutputFormat>,

    /// List the check rules with their effective severity, then exit
    #[arg(long)]
    pub list_rules: bool,
}

fn parse_rule(arg: &str) -> Result<(String, Severity), String> {
    CliOverrides::parse_rule_arg(arg).map_err(|e| e.to_string())
}

impl CheckArgs {
    /// Whether any check option was given
    pub fn is_used(&self) -> bool {
        !self.rules.is_empty()
            || !self.enable_rule.is_empty()
            || !self.disable_rule.is_empty()
            || self.format.is_some()
            || self.list_rules
    }

    /// The CLI overrides of the rule settings
    pub fn overrides(&self) -> CliOverrides {
        CliOverrides {
            rules: self.rules.clone(),
            enable: self.enable_rule.clone(),
            disable: self.disable_rule.clone(),
        }
    }

    /// The rule settings: the `check` section of the configuration file,
    /// then the CLI overrides.
    pub fn resolve_config(&self, project: &ProjectConfig) -> Result<CheckConfig> {
        CheckConfig::resolve(project.check.rules.as_ref(), &self.overrides()).with_context(|| {
            match &project.source {
                Some(path) => format!(
                    "invalid check configuration (config file `{}`)",
                    path.display()
                ),
                None => "invalid check configuration".to_string(),
            }
        })
    }

    /// Applies a `--json` flag: JSON output unless `--format` says otherwise
    pub fn with_json(mut self, json: bool) -> Self {
        if json && self.format.is_none() {
            self.format = Some(OutputFormat::Json);
        }
        self
    }

    /// The output format, text by default
    pub fn output_format(&self) -> OutputFormat {
        self.format.unwrap_or_default()
    }
}

#[derive(Serialize)]
struct RuleListing {
    id: &'static str,
    severity: Severity,
    description: &'static str,
}

/// Prints the rules with their effective severity.
pub fn print_rules(config: &CheckConfig, format: OutputFormat) -> Result<()> {
    let rules: Vec<RuleListing> = registry()
        .iter()
        .map(|rule| RuleListing {
            id: rule.id(),
            severity: config.rule(rule.id()).severity,
            description: rule.description(),
        })
        .collect();

    if format == OutputFormat::Json {
        println!("{}", serde_json::to_string_pretty(&rules)?);
        return Ok(());
    }

    let width = rules.iter().map(|r| r.id.len()).max().unwrap_or(0);
    for rule in rules {
        println!(
            "{:<width$}  {:<5}  {}",
            rule.id,
            rule.severity.to_string(),
            rule.description,
            width = width
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[derive(Parser)]
    struct TestCli {
        #[command(flatten)]
        check: CheckArgs,
    }

    #[test]
    fn test_parses_rule_flags() {
        let cli = TestCli::try_parse_from([
            "features",
            "--rule",
            "min-coverage=error",
            "--enable-rule",
            "missing-readme",
            "--enable-rule",
            "cross-feature-import",
            "--disable-rule",
            "stale-flag",
            "--format",
            "github",
        ])
        .unwrap();

        assert_eq!(
            cli.check.rules,
            vec![("min-coverage".to_string(), Severity::Error)]
        );
        assert_eq!(
            cli.check.enable_rule,
            vec!["missing-readme", "cross-feature-import"]
        );
        assert_eq!(cli.check.disable_rule, vec!["stale-flag"]);
        assert_eq!(cli.check.format, Some(OutputFormat::Github));
        assert!(cli.check.is_used());
    }

    #[test]
    fn test_rejects_invalid_rule_flag() {
        assert!(TestCli::try_parse_from(["features", "--rule", "min-coverage"]).is_err());
        assert!(TestCli::try_parse_from(["features", "--rule", "min-coverage=loud"]).is_err());
        assert!(TestCli::try_parse_from(["features", "--format", "xml"]).is_err());
    }

    #[test]
    fn test_output_format() {
        let args = CheckArgs::default();
        assert!(!args.is_used());
        assert_eq!(args.output_format(), OutputFormat::Text);
        assert_eq!(args.with_json(true).output_format(), OutputFormat::Json);

        let args = CheckArgs {
            format: Some(OutputFormat::Github),
            ..Default::default()
        };
        assert_eq!(args.with_json(true).output_format(), OutputFormat::Github);
    }

    #[test]
    fn test_resolve_config_applies_file_then_cli() {
        let project = ProjectConfig::parse("check:\n  rules:\n    missing-owner: error\n").unwrap();

        let config = CheckArgs::default().resolve_config(&project).unwrap();
        assert_eq!(config.rule("missing-owner").severity, Severity::Error);

        // The CLI wins over the file
        let args = CheckArgs {
            disable_rule: vec!["missing-owner".to_string()],
            ..Default::default()
        };
        let config = args.resolve_config(&project).unwrap();
        assert_eq!(config.rule("missing-owner").severity, Severity::Off);

        // Without a file, the defaults apply
        let config = CheckArgs::default()
            .resolve_config(&ProjectConfig::default())
            .unwrap();
        assert_eq!(config.rule("missing-owner").severity, Severity::Warn);
    }

    #[test]
    fn test_resolve_config_errors_name_the_file() {
        let mut project = ProjectConfig::parse("check:\n  rules:\n    nope: error\n").unwrap();
        project.source = Some(std::path::PathBuf::from("custom.yml"));
        let err = CheckArgs::default().resolve_config(&project).unwrap_err();
        let message = format!("{:#}", err);
        assert!(message.contains("custom.yml"), "{message}");
        assert!(message.contains("unknown rule `nope`"), "{message}");
    }
}
