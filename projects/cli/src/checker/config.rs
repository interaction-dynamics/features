//! Configuration of the check rules.
//!
//! The configuration comes from, in increasing order of precedence:
//! 1. each rule's default severity and options,
//! 2. the `check` section of the project configuration file (`.features.yml`),
//! 3. the CLI overrides (`--rule`, `--enable-rule`, `--disable-rule`).
//!
//! ```yaml
//! check:
//!   rules:
//!     cross-feature-import: error   # severity only
//!     missing-readme:               # severity and options
//!       severity: warn
//!       max_files_without_readme: 5
//! ```

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::BTreeMap;

use super::{Severity, registry};

/// The effective severity and options of a rule.
#[derive(Debug, Clone, PartialEq)]
pub struct RuleConfig {
    pub severity: Severity,
    pub options: serde_yaml::Mapping,
}

/// The effective configuration of every rule.
#[derive(Debug, Clone)]
pub struct CheckConfig {
    rules: BTreeMap<&'static str, RuleConfig>,
}

impl Default for CheckConfig {
    fn default() -> Self {
        let rules = registry()
            .iter()
            .map(|rule| {
                (
                    rule.id(),
                    RuleConfig {
                        severity: rule.default_severity(),
                        options: serde_yaml::Mapping::new(),
                    },
                )
            })
            .collect();
        Self { rules }
    }
}

/// Rule settings given on the command line.
#[derive(Debug, Clone, Default)]
pub struct CliOverrides {
    /// `--rule <id>=<severity>`
    pub rules: Vec<(String, Severity)>,
    /// `--enable-rule <id>`: turns an `off` rule into `error`, keeps the severity of other rules
    pub enable: Vec<String>,
    /// `--disable-rule <id>`: turns the rule `off`
    pub disable: Vec<String>,
}

impl CliOverrides {
    /// Parses a `<id>=<severity>` argument
    pub fn parse_rule_arg(arg: &str) -> Result<(String, Severity)> {
        let (id, severity) = arg
            .split_once('=')
            .with_context(|| format!("invalid rule `{}`, expected `<id>=<severity>`", arg))?;
        Ok((id.trim().to_string(), severity.parse()?))
    }
}

/// The `check` section of the configuration file
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct CheckSection {
    #[serde(default)]
    rules: BTreeMap<String, RuleEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RuleEntry {
    Severity(Severity),
    Settings(serde_yaml::Mapping),
}

fn validate_rule_id(id: &str, source: &str) -> Result<&'static str> {
    let ids = super::rule_ids();
    ids.iter()
        .copied()
        .find(|known| *known == id)
        .with_context(|| {
            format!(
                "unknown rule `{}` in {}. Known rules: {}",
                id,
                source,
                ids.join(", ")
            )
        })
}

impl CheckConfig {
    /// Builds the configuration from the defaults, the `check` section of the
    /// configuration file (if any) and the CLI overrides.
    pub fn resolve(section: Option<&serde_yaml::Value>, overrides: &CliOverrides) -> Result<Self> {
        let mut config = Self::default();

        if let Some(section) = section {
            config
                .apply_section(section)
                .context("invalid `check` section in the config file")?;
        }

        config.apply_overrides(overrides)?;
        config.validate()?;

        Ok(config)
    }

    fn apply_section(&mut self, section: &serde_yaml::Value) -> Result<()> {
        if section.is_null() {
            return Ok(());
        }
        let section: CheckSection = serde_yaml::from_value(section.clone())?;

        for (id, entry) in section.rules {
            let id = validate_rule_id(&id, "check.rules")?;
            let rule = self.rules.get_mut(id).expect("known rule");

            match entry {
                RuleEntry::Severity(severity) => rule.severity = severity,
                RuleEntry::Settings(mut settings) => {
                    if let Some(severity) = settings.remove("severity") {
                        rule.severity = serde_yaml::from_value(severity).with_context(|| {
                            format!(
                                "invalid severity for rule `{}`, expected `error`, `warn` or `off`",
                                id
                            )
                        })?;
                    }
                    rule.options = settings;
                }
            }
        }

        Ok(())
    }

    fn apply_overrides(&mut self, overrides: &CliOverrides) -> Result<()> {
        for (id, severity) in &overrides.rules {
            let id = validate_rule_id(id, "--rule")?;
            self.rules.get_mut(id).expect("known rule").severity = *severity;
        }

        for id in &overrides.enable {
            let id = validate_rule_id(id, "--enable-rule")?;
            let rule = self.rules.get_mut(id).expect("known rule");
            if rule.severity == Severity::Off {
                rule.severity = Severity::Error;
            }
        }

        for id in &overrides.disable {
            let id = validate_rule_id(id, "--disable-rule")?;
            self.rules.get_mut(id).expect("known rule").severity = Severity::Off;
        }

        Ok(())
    }

    fn validate(&self) -> Result<()> {
        for rule in registry() {
            rule.validate_options(&self.rule(rule.id()).options)?;
        }
        Ok(())
    }

    /// The configuration of a rule
    pub fn rule(&self, id: &str) -> &RuleConfig {
        self.rules
            .get(id)
            .unwrap_or_else(|| panic!("unknown rule `{}`", id))
    }

    /// Whether the rule runs
    pub fn is_enabled(&self, id: &str) -> bool {
        self.rule(id).severity != Severity::Off
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_yaml(content: &str) -> Result<CheckConfig> {
        let section: serde_yaml::Value = serde_yaml::from_str(content)?;
        CheckConfig::resolve(Some(&section), &CliOverrides::default())
    }

    #[test]
    fn test_defaults() {
        let config = CheckConfig::default();
        assert_eq!(config.rule("duplicate-names").severity, Severity::Error);
        assert_eq!(config.rule("missing-owner").severity, Severity::Warn);
        assert_eq!(config.rule("stale-flag").severity, Severity::Warn);
        assert_eq!(config.rule("missing-readme").severity, Severity::Off);
        assert_eq!(config.rule("min-coverage").severity, Severity::Off);
        assert_eq!(config.rule("cross-feature-import").severity, Severity::Off);
    }

    #[test]
    fn test_severity_shorthand_and_settings() {
        let config = from_yaml(
            r#"
rules:
  cross-feature-import: error
  missing-owner: off
  missing-readme:
    severity: warning
    max_files_without_readme: 3
"#,
        )
        .unwrap();

        assert_eq!(
            config.rule("cross-feature-import").severity,
            Severity::Error
        );
        // `off` is a string in YAML 1.2, not a boolean
        assert_eq!(config.rule("missing-owner").severity, Severity::Off);
        let readme = config.rule("missing-readme");
        assert_eq!(readme.severity, Severity::Warn);
        assert_eq!(
            readme.options.get("max_files_without_readme"),
            Some(&serde_yaml::Value::from(3))
        );
    }

    #[test]
    fn test_settings_without_severity_keep_default() {
        let config = from_yaml("rules:\n  stale-flag:\n    max_age_days: 30\n").unwrap();
        assert_eq!(config.rule("stale-flag").severity, Severity::Warn);
    }

    #[test]
    fn test_empty_section_keeps_defaults() {
        let config = from_yaml("~").unwrap();
        assert_eq!(config.rule("duplicate-names").severity, Severity::Error);
        let config = from_yaml("rules: {}").unwrap();
        assert_eq!(config.rule("missing-owner").severity, Severity::Warn);
    }

    #[test]
    fn test_unknown_rule_is_an_error() {
        let err = from_yaml("rules:\n  missing-owners: error\n").unwrap_err();
        let message = format!("{:#}", err);
        assert!(
            message.contains("unknown rule `missing-owners`"),
            "{message}"
        );
        assert!(message.contains("missing-owner"), "{message}");
    }

    #[test]
    fn test_unknown_option_is_an_error() {
        let err = from_yaml("rules:\n  missing-readme:\n    max_files: 3\n").unwrap_err();
        assert!(format!("{:#}", err).contains("missing-readme"));
    }

    #[test]
    fn test_invalid_severity_is_an_error() {
        assert!(from_yaml("rules:\n  missing-owner: fatal\n").is_err());
        assert!(from_yaml("rules:\n  missing-owner:\n    severity: fatal\n").is_err());
    }

    #[test]
    fn test_unknown_check_key_is_an_error() {
        assert!(from_yaml("rule: {}\n").is_err());
    }

    #[test]
    fn test_overrides_precedence() {
        let section: serde_yaml::Value =
            serde_yaml::from_str("rules:\n  missing-owner: error\n  min-coverage: warn\n").unwrap();
        let config = CheckConfig::resolve(
            Some(&section),
            &CliOverrides {
                rules: vec![("missing-owner".to_string(), Severity::Warn)],
                enable: vec!["missing-readme".to_string(), "min-coverage".to_string()],
                disable: vec!["duplicate-names".to_string()],
            },
        )
        .unwrap();

        // --rule beats the file
        assert_eq!(config.rule("missing-owner").severity, Severity::Warn);
        // --enable-rule turns an off rule into error
        assert_eq!(config.rule("missing-readme").severity, Severity::Error);
        // --enable-rule keeps the severity of a rule that is already on
        assert_eq!(config.rule("min-coverage").severity, Severity::Warn);
        // --disable-rule beats the default
        assert!(!config.is_enabled("duplicate-names"));
    }

    #[test]
    fn test_disable_wins_over_enable() {
        let config = CheckConfig::resolve(
            None,
            &CliOverrides {
                enable: vec!["missing-readme".to_string()],
                disable: vec!["missing-readme".to_string()],
                ..Default::default()
            },
        )
        .unwrap();
        assert!(!config.is_enabled("missing-readme"));
    }

    #[test]
    fn test_unknown_rule_in_overrides_is_an_error() {
        let result = CheckConfig::resolve(
            None,
            &CliOverrides {
                disable: vec!["nope".to_string()],
                ..Default::default()
            },
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_rule_arg() {
        assert_eq!(
            CliOverrides::parse_rule_arg("min-coverage=error").unwrap(),
            ("min-coverage".to_string(), Severity::Error)
        );
        assert!(CliOverrides::parse_rule_arg("min-coverage").is_err());
        assert!(CliOverrides::parse_rule_arg("min-coverage=loud").is_err());
    }
}
