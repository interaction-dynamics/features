//! Validation checks run by `features check`.
//!
//! Checks are organised as a registry of rules. Each rule has a stable
//! kebab-case id, a default severity and optional typed options. The
//! effective severity and options of every rule come from the defaults,
//! then `check.rules` in `features.config.yaml`, then the CLI overrides.
//!
//! Only findings of rules with the `error` severity make the check fail.

mod args;
mod config;
mod report;
mod rules;

use anyhow::Result;
use chrono::NaiveDate;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::models::{Feature, Record};
use crate::records::record_warnings;

pub use args::{CheckArgs, print_rules};
pub use config::{CheckConfig, CliOverrides};
pub use report::OutputFormat;

/// How a rule's findings are treated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Findings make the check fail
    Error,
    /// Findings are reported but don't make the check fail
    #[serde(alias = "warning")]
    Warn,
    /// The rule doesn't run
    Off,
}

impl std::str::FromStr for Severity {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.trim().to_lowercase().as_str() {
            "error" => Ok(Severity::Error),
            "warn" | "warning" => Ok(Severity::Warn),
            "off" => Ok(Severity::Off),
            other => anyhow::bail!(
                "invalid severity `{}`, expected `error`, `warn` or `off`",
                other
            ),
        }
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Severity::Error => "error",
            Severity::Warn => "warn",
            Severity::Off => "off",
        };
        f.write_str(s)
    }
}

/// Information the rules need besides the features themselves.
#[derive(Debug, Clone)]
pub struct CheckContext {
    /// The scanned directory, which feature paths are relative to
    pub base_path: PathBuf,
    /// The date used to compute the age of feature flags
    pub today: NaiveDate,
    /// The project-level specs and decision records, see [`crate::records`]
    pub project_records: Vec<Record>,
}

impl CheckContext {
    pub fn new(base_path: &Path, project_records: Vec<Record>) -> Self {
        Self {
            base_path: base_path.to_path_buf(),
            today: chrono::Local::now().date_naive(),
            project_records,
        }
    }
}

/// A problem a rule found, before the rule's severity is applied.
#[derive(Debug, Clone, PartialEq)]
pub struct Violation {
    /// Name of the feature the problem is about
    pub feature: String,
    /// Path of the feature, relative to the scanned directory
    pub path: String,
    /// File the problem is in, relative to the scanned directory
    pub file: Option<String>,
    /// 1-based line in `file`
    pub line: Option<usize>,
    pub message: String,
}

impl Violation {
    fn for_feature(feature: &Feature, message: impl Into<String>) -> Self {
        Self {
            feature: feature.name.clone(),
            path: feature.path.clone(),
            file: None,
            line: None,
            message: message.into(),
        }
    }
}

/// A violation reported by a rule, with the rule's id and severity.
#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub rule: &'static str,
    pub severity: Severity,
    pub feature: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,
    pub message: String,
}

/// A check rule with typed options.
trait Rule {
    type Options: DeserializeOwned + Default;

    fn id(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn default_severity(&self) -> Severity;
    fn check(
        &self,
        features: &[Feature],
        ctx: &CheckContext,
        options: &Self::Options,
    ) -> Vec<Violation>;

    /// Validates options once they are parsed
    fn validate(&self, _options: &Self::Options) -> Result<()> {
        Ok(())
    }
}

/// Object-safe view of a [`Rule`], with options given as a YAML mapping.
pub trait DynRule {
    fn id(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn default_severity(&self) -> Severity;
    fn validate_options(&self, options: &serde_yaml::Mapping) -> Result<()>;
    fn run(
        &self,
        features: &[Feature],
        ctx: &CheckContext,
        options: &serde_yaml::Mapping,
    ) -> Result<Vec<Violation>>;
}

fn parse_options<T: DeserializeOwned + Default>(
    rule_id: &str,
    options: &serde_yaml::Mapping,
) -> Result<T> {
    if options.is_empty() {
        return Ok(T::default());
    }
    serde_yaml::from_value(serde_yaml::Value::Mapping(options.clone()))
        .map_err(|e| anyhow::anyhow!("invalid options for rule `{}`: {}", rule_id, e))
}

impl<R: Rule> DynRule for R {
    fn id(&self) -> &'static str {
        Rule::id(self)
    }

    fn description(&self) -> &'static str {
        Rule::description(self)
    }

    fn default_severity(&self) -> Severity {
        Rule::default_severity(self)
    }

    fn validate_options(&self, options: &serde_yaml::Mapping) -> Result<()> {
        let parsed: R::Options = parse_options(Rule::id(self), options)?;
        self.validate(&parsed)
            .map_err(|e| anyhow::anyhow!("invalid options for rule `{}`: {}", Rule::id(self), e))
    }

    fn run(
        &self,
        features: &[Feature],
        ctx: &CheckContext,
        options: &serde_yaml::Mapping,
    ) -> Result<Vec<Violation>> {
        let parsed: R::Options = parse_options(Rule::id(self), options)?;
        Ok(self.check(features, ctx, &parsed))
    }
}

/// All the rules, in the order they run and are reported.
pub fn registry() -> Vec<Box<dyn DynRule>> {
    rules::all()
}

/// The ids of all the rules
pub fn rule_ids() -> Vec<&'static str> {
    registry().iter().map(|rule| rule.id()).collect()
}

/// The result of running the checks.
#[derive(Debug, Clone, Serialize)]
pub struct CheckReport {
    pub findings: Vec<Finding>,
    pub summary: Summary,
}

#[derive(Debug, Clone, Serialize)]
pub struct Summary {
    pub errors: usize,
    pub warnings: usize,
    /// Ids of the rules that ran
    pub rules: Vec<&'static str>,
    /// Ids of the rules that were turned off
    pub disabled_rules: Vec<&'static str>,
}

impl CheckReport {
    pub fn has_errors(&self) -> bool {
        self.summary.errors > 0
    }
}

/// Runs every enabled rule and collects their findings.
pub fn run(features: &[Feature], config: &CheckConfig, ctx: &CheckContext) -> Result<CheckReport> {
    let mut findings = Vec::new();
    let mut ran = Vec::new();
    let mut disabled = Vec::new();

    for rule in registry() {
        let rule_config = config.rule(rule.id());
        if rule_config.severity == Severity::Off {
            disabled.push(rule.id());
            continue;
        }
        ran.push(rule.id());

        let mut violations = rule.run(features, ctx, &rule_config.options)?;
        violations.sort_by(|a, b| {
            (&a.path, &a.file, a.line, &a.message).cmp(&(&b.path, &b.file, b.line, &b.message))
        });

        findings.extend(violations.into_iter().map(|v| Finding {
            rule: rule.id(),
            severity: rule_config.severity,
            feature: v.feature,
            path: v.path,
            file: v.file,
            line: v.line,
            message: v.message,
        }));
    }

    let errors = findings
        .iter()
        .filter(|f| f.severity == Severity::Error)
        .count();
    let warnings = findings
        .iter()
        .filter(|f| f.severity == Severity::Warn)
        .count();

    Ok(CheckReport {
        findings,
        summary: Summary {
            errors,
            warnings,
            rules: ran,
            disabled_rules: disabled,
        },
    })
}

/// Runs the checks, prints the report and fails if any `error` finding was found.
pub fn run_checks(
    features: &[Feature],
    config: &CheckConfig,
    ctx: &CheckContext,
    format: OutputFormat,
) -> Result<()> {
    let report = run(features, config, ctx)?;
    report::print(&report, ctx, format)?;

    if report.has_errors() {
        anyhow::bail!("Check failed: {} error(s) found", report.summary.errors);
    }

    Ok(())
}

/// Keys shared by several features, with the paths of those features
pub fn duplicate_keys(features: &[Feature]) -> BTreeMap<String, Vec<String>> {
    let mut key_to_paths: BTreeMap<String, Vec<String>> = BTreeMap::new();
    walk_features(features, &mut |feature| {
        key_to_paths
            .entry(feature.key.clone())
            .or_default()
            .push(feature.path.clone());
    });
    key_to_paths.retain(|_, paths| paths.len() > 1);
    key_to_paths
}

/// Warnings about the specs and decision records of every feature and of the project
pub fn check_records(features: &[Feature], project_records: &[Record]) -> Vec<String> {
    let mut warnings = record_warnings(project_records, &[], "project");
    walk_features(features, &mut |feature| {
        warnings.extend(record_warnings(
            &feature.records,
            project_records,
            &format!("feature '{}'", feature.name),
        ));
    });
    warnings
}

/// Calls `f` for every feature, nested features included.
fn walk_features<'a>(features: &'a [Feature], f: &mut impl FnMut(&'a Feature)) {
    for feature in features {
        f(feature);
        walk_features(&feature.features, f);
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;

    pub fn feature(name: &str, path: &str) -> Feature {
        Feature {
            name: name.to_string(),
            key: name.to_string(),
            description: String::new(),
            owner: "team".to_string(),
            is_owner_inherited: false,
            path: path.to_string(),
            features: Vec::new(),
            meta: BTreeMap::new(),
            links: BTreeMap::new(),
            changes: Vec::new(),
            decisions: Vec::new(),
            records: Vec::new(),
            stats: None,
            dependencies: Vec::new(),
            external: BTreeMap::new(),
        }
    }

    pub fn ctx() -> CheckContext {
        CheckContext {
            base_path: PathBuf::from("/nonexistent"),
            today: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            project_records: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::*;
    use super::*;

    #[test]
    fn test_severity_from_str() {
        assert_eq!("error".parse::<Severity>().unwrap(), Severity::Error);
        assert_eq!("WARN".parse::<Severity>().unwrap(), Severity::Warn);
        assert_eq!("warning".parse::<Severity>().unwrap(), Severity::Warn);
        assert_eq!("off".parse::<Severity>().unwrap(), Severity::Off);
        assert!("fatal".parse::<Severity>().is_err());
    }

    #[test]
    fn test_rule_ids_are_unique_and_kebab_case() {
        let ids = rule_ids();
        let mut sorted = ids.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len());
        for id in ids {
            assert!(
                id.chars().all(|c| c.is_ascii_lowercase() || c == '-'),
                "{id} is not kebab-case"
            );
        }
    }

    #[test]
    fn test_run_applies_severity_and_counts() {
        let features = vec![feature("a", "x/a"), feature("a", "y/a")];

        let config = CheckConfig::default();
        let report = run(&features, &config, &ctx()).unwrap();
        // duplicate-names is an error, duplicate-keys (same key `a`) a warning
        assert_eq!(report.summary.errors, 1);
        assert_eq!(report.summary.warnings, 1);
        assert!(report.has_errors());
        assert_eq!(report.findings[0].rule, "duplicate-names");

        let overrides = CliOverrides {
            rules: vec![("duplicate-names".to_string(), Severity::Warn)],
            ..Default::default()
        };
        let config = CheckConfig::resolve(None, &overrides).unwrap();
        let report = run(&features, &config, &ctx()).unwrap();
        assert_eq!(report.summary.errors, 0);
        assert_eq!(report.summary.warnings, 2);
        assert!(!report.has_errors());
    }

    #[test]
    fn test_run_skips_disabled_rules() {
        let features = vec![feature("a", "x/a"), feature("a", "y/a")];
        let overrides = CliOverrides {
            disable: vec!["duplicate-names".to_string(), "duplicate-keys".to_string()],
            ..Default::default()
        };
        let config = CheckConfig::resolve(None, &overrides).unwrap();
        let report = run(&features, &config, &ctx()).unwrap();
        assert!(report.findings.is_empty());
        assert!(report.summary.disabled_rules.contains(&"duplicate-names"));
    }
}
