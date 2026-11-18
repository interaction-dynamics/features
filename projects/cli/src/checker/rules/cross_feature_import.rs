//! `cross-feature-import`: a feature importing code from a sibling feature.
//!
//! Importing from a parent or child feature is fine; importing from a
//! feature in another branch of the tree couples features that should be
//! independent.

use serde::Deserialize;
use std::path::Path;

use crate::checker::{CheckContext, Rule, Severity, Violation, walk_features};
use crate::models::{DependencyType, Feature};

pub struct CrossFeatureImport;

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct Options {
    /// Paths of features any feature may import from (e.g. shared features),
    /// relative to the scanned directory. Their nested features are allowed too.
    pub allow: Vec<String>,
}

impl Rule for CrossFeatureImport {
    type Options = Options;

    fn id(&self) -> &'static str {
        "cross-feature-import"
    }

    fn description(&self) -> &'static str {
        "A feature imports code from a sibling feature"
    }

    fn default_severity(&self) -> Severity {
        Severity::Off
    }

    fn check(
        &self,
        features: &[Feature],
        _ctx: &CheckContext,
        options: &Options,
    ) -> Vec<Violation> {
        let mut violations = Vec::new();
        walk_features(features, &mut |feature| {
            for dependency in &feature.dependencies {
                if !matches!(dependency.dependency_type, DependencyType::Sibling) {
                    continue;
                }
                let target = Path::new(&dependency.feature_path);
                if options
                    .allow
                    .iter()
                    .any(|allowed| target.starts_with(allowed))
                {
                    continue;
                }

                violations.push(Violation {
                    feature: feature.name.clone(),
                    path: feature.path.clone(),
                    file: Some(dependency.source_filename.clone()),
                    line: Some(dependency.line),
                    message: format!(
                        "Imports from sibling feature '{}': {}",
                        dependency.feature_path,
                        dependency.content.trim()
                    ),
                });
            }
        });
        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checker::test_support::*;
    use crate::models::Dependency;

    fn dependency(target: &str, dependency_type: DependencyType, line: usize) -> Dependency {
        Dependency {
            source_filename: "features/billing/index.ts".to_string(),
            target_filename: format!("{}/index.ts", target),
            line,
            content: format!("  import x from '../{}'  ", target),
            feature_path: target.to_string(),
            dependency_type,
        }
    }

    fn features() -> Vec<Feature> {
        let mut billing = feature("billing", "features/billing");
        billing.dependencies = vec![
            dependency("features/checkout", DependencyType::Sibling, 3),
            dependency("features/shared/ui", DependencyType::Sibling, 4),
            dependency(
                "features/billing/features/invoices",
                DependencyType::Child,
                5,
            ),
            dependency("features", DependencyType::Parent, 6),
        ];
        vec![billing]
    }

    #[test]
    fn test_reports_sibling_imports_only() {
        let violations = CrossFeatureImport.check(&features(), &ctx(), &Options::default());
        assert_eq!(violations.len(), 2);
        assert_eq!(
            violations[0].file.as_deref(),
            Some("features/billing/index.ts")
        );
        assert_eq!(violations[0].line, Some(3));
        assert_eq!(
            violations[0].message,
            "Imports from sibling feature 'features/checkout': import x from '../features/checkout'"
        );
    }

    #[test]
    fn test_allowed_targets_and_their_children_are_skipped() {
        let options = Options {
            allow: vec!["features/shared".to_string()],
        };
        let violations = CrossFeatureImport.check(&features(), &ctx(), &options);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, Some(3));
    }

    #[test]
    fn test_allow_matches_whole_path_components() {
        let options = Options {
            allow: vec!["features/check".to_string()],
        };
        let violations = CrossFeatureImport.check(&features(), &ctx(), &options);
        assert_eq!(violations.len(), 2);
    }
}
