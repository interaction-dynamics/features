//! `min-coverage`: a feature whose line coverage is too low.

use anyhow::Result;
use serde::Deserialize;

use crate::checker::{CheckContext, Rule, Severity, Violation, walk_features};
use crate::models::Feature;

pub struct MinCoverage;

#[derive(Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Options {
    /// Minimum line coverage, in percent
    pub threshold: f64,
}

impl Default for Options {
    fn default() -> Self {
        Self { threshold: 80.0 }
    }
}

impl Rule for MinCoverage {
    type Options = Options;

    fn id(&self) -> &'static str {
        "min-coverage"
    }

    fn description(&self) -> &'static str {
        "A feature's line coverage is below the threshold (only features with coverage data)"
    }

    fn default_severity(&self) -> Severity {
        Severity::Off
    }

    fn validate(&self, options: &Options) -> Result<()> {
        if !(0.0..=100.0).contains(&options.threshold) {
            anyhow::bail!(
                "`threshold` must be between 0 and 100, got {}",
                options.threshold
            );
        }
        Ok(())
    }

    fn check(
        &self,
        features: &[Feature],
        _ctx: &CheckContext,
        options: &Options,
    ) -> Vec<Violation> {
        let mut violations = Vec::new();
        walk_features(features, &mut |feature| {
            let Some(coverage) = feature.stats.as_ref().and_then(|s| s.coverage.as_ref()) else {
                return;
            };
            if coverage.lines_total == 0 || coverage.line_coverage_percent >= options.threshold {
                return;
            }
            violations.push(Violation::for_feature(
                feature,
                format!(
                    "Line coverage is {:.1}% ({}/{} lines), below {}%",
                    coverage.line_coverage_percent,
                    coverage.lines_covered,
                    coverage.lines_total,
                    options.threshold
                ),
            ));
        });
        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checker::test_support::*;
    use crate::coverage_parser::CoverageStats;
    use crate::models::Stats;
    use std::collections::BTreeMap;

    fn with_coverage(mut feature: Feature, covered: usize, total: usize) -> Feature {
        let percent = if total == 0 {
            0.0
        } else {
            covered as f64 * 100.0 / total as f64
        };
        feature.stats = Some(Stats {
            files_count: None,
            lines_count: None,
            todos_count: None,
            commits: BTreeMap::new(),
            coverage: Some(CoverageStats {
                lines_total: total,
                lines_covered: covered,
                lines_missed: total - covered,
                line_coverage_percent: percent,
                ..Default::default()
            }),
        });
        feature
    }

    #[test]
    fn test_reports_features_below_threshold() {
        let features = vec![
            with_coverage(feature("low", "low"), 1, 4),
            with_coverage(feature("exact", "exact"), 4, 5),
            with_coverage(feature("empty", "empty"), 0, 0),
            feature("no-coverage", "no-coverage"),
        ];

        let violations = MinCoverage.check(&features, &ctx(), &Options::default());
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].path, "low");
        assert_eq!(
            violations[0].message,
            "Line coverage is 25.0% (1/4 lines), below 80%"
        );
    }

    #[test]
    fn test_threshold_must_be_a_percentage() {
        assert!(MinCoverage.validate(&Options { threshold: 120.0 }).is_err());
        assert!(MinCoverage.validate(&Options { threshold: -1.0 }).is_err());
        assert!(MinCoverage.validate(&Options { threshold: 50.0 }).is_ok());
    }
}
