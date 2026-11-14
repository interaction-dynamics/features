//! `missing-readme`: a large feature without documentation.

use serde::Deserialize;

use crate::checker::{CheckContext, Rule, Severity, Violation, walk_features};
use crate::models::Feature;

pub struct MissingReadme;

/// Files that document a feature
const DOC_FILES: [&str; 3] = ["README.md", "README.mdx", "FEATURES.toml"];

#[derive(Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Options {
    /// Features with more files than this need a README
    pub max_files_without_readme: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            max_files_without_readme: 10,
        }
    }
}

impl Rule for MissingReadme {
    type Options = Options;

    fn id(&self) -> &'static str {
        "missing-readme"
    }

    fn description(&self) -> &'static str {
        "A large feature has no README.md, README.mdx or FEATURES.toml"
    }

    fn default_severity(&self) -> Severity {
        Severity::Off
    }

    fn check(&self, features: &[Feature], ctx: &CheckContext, options: &Options) -> Vec<Violation> {
        let mut violations = Vec::new();
        walk_features(features, &mut |feature| {
            let Some(files_count) = feature.stats.as_ref().and_then(|s| s.files_count) else {
                return;
            };
            if files_count <= options.max_files_without_readme {
                return;
            }

            let dir = ctx.base_path.join(&feature.path);
            if DOC_FILES.iter().any(|name| dir.join(name).is_file()) {
                return;
            }

            violations.push(Violation::for_feature(
                feature,
                format!(
                    "Feature has {} files (more than {}) but no README.md, README.mdx or FEATURES.toml",
                    files_count, options.max_files_without_readme
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
    use crate::models::Stats;
    use std::collections::BTreeMap;

    fn with_files(mut feature: Feature, files: usize) -> Feature {
        feature.stats = Some(Stats {
            files_count: Some(files),
            lines_count: None,
            todos_count: None,
            commits: BTreeMap::new(),
            coverage: None,
        });
        feature
    }

    #[test]
    fn test_reports_large_features_without_readme() {
        let dir = tempfile::tempdir().unwrap();
        for name in ["big", "documented", "small", "toml"] {
            std::fs::create_dir_all(dir.path().join(name)).unwrap();
        }
        std::fs::write(dir.path().join("documented/README.mdx"), "").unwrap();
        std::fs::write(dir.path().join("toml/FEATURES.toml"), "").unwrap();

        let features = vec![
            with_files(feature("big", "big"), 11),
            with_files(feature("documented", "documented"), 50),
            with_files(feature("small", "small"), 10),
            with_files(feature("toml", "toml"), 50),
            feature("no-stats", "no-stats"),
        ];
        let mut context = ctx();
        context.base_path = dir.path().to_path_buf();

        let violations = MissingReadme.check(&features, &context, &Options::default());
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].path, "big");
        assert_eq!(
            violations[0].message,
            "Feature has 11 files (more than 10) but no README.md, README.mdx or FEATURES.toml"
        );
    }

    #[test]
    fn test_threshold_is_configurable() {
        let dir = tempfile::tempdir().unwrap();
        let features = vec![with_files(feature("small", "small"), 2)];
        let mut context = ctx();
        context.base_path = dir.path().to_path_buf();

        let options = Options {
            max_files_without_readme: 1,
        };
        assert_eq!(MissingReadme.check(&features, &context, &options).len(), 1);
    }
}
