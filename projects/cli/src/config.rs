//! The project configuration file.
//!
//! A YAML file, `.features.yml` (or `.features.yaml`), holding default values
//! for the command-line options. Each key is the name of an option, in
//! snake_case: options shared by several commands are at the top level, the
//! options of a single command are in a section named after it.
//!
//! ```yaml
//! path: src
//! skip_changes: true
//! serve:
//!   port: 4000
//! check:
//!   rules:
//!     missing-owner: error
//! ```
//!
//! Options given on the command line win over the file. Relative paths in the
//! file are relative to the folder of the file.
//!
//! It is found in the scanned folder, then the project folder, then the
//! current folder, unless `--config <path>` points to another file.
//!
//! Not to be confused with the per-feature `FEATURES.toml` files, which
//! describe a single feature.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

use crate::checker::OutputFormat;

/// Names of the configuration file, in order of preference.
pub const CONFIG_FILE_NAMES: [&str; 2] = [".features.yml", ".features.yaml"];

/// The content of the configuration file.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ProjectConfig {
    /// The folder to scan for features
    pub path: Option<PathBuf>,
    /// `--skip-changes`
    pub skip_changes: Option<bool>,
    /// `--ignore-path`, added to the ones given on the command line
    pub ignore_path: Vec<PathBuf>,
    /// `--include-skipped-dirs`
    pub include_skipped_dirs: Option<bool>,
    /// `--coverage-dir`
    pub coverage_dir: Option<PathBuf>,
    /// `--project-dir`
    pub project_dir: Option<PathBuf>,
    /// `--strip-emails`
    pub strip_emails: Option<bool>,
    /// `--codeowners-path`
    pub codeowners_path: Option<PathBuf>,
    /// `--codeowners-prefix`
    pub codeowners_prefix: Option<String>,

    /// Options of `features scan`
    pub scan: ScanSection,
    /// Options of `features serve`
    pub serve: ServeSection,
    /// Options of `features build`
    pub build: BuildSection,
    /// Options of `features find-owner`
    pub find_owner: JsonSection,
    /// Options of `features owners`
    pub owners: JsonSection,
    /// Options of `features diff`
    pub diff: DiffSection,
    /// Options of `features check`
    pub check: CheckSection,

    /// The file the configuration was read from
    #[serde(skip)]
    pub source: Option<PathBuf>,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ScanSection {
    pub json: Option<bool>,
    pub flat: Option<bool>,
    pub description: Option<bool>,
    pub coverage: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ServeSection {
    pub port: Option<u16>,
    /// `--allow-origin`, added to the ones given on the command line
    pub allow_origin: Vec<String>,
    pub generate_codeowners: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct BuildSection {
    pub build_dir: Option<PathBuf>,
    pub generate_codeowners: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct JsonSection {
    pub json: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DiffSection {
    pub since: Option<String>,
    pub committed_only: Option<bool>,
    pub json: Option<bool>,
    pub markdown: Option<bool>,
    pub owners_only: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CheckSection {
    pub json: Option<bool>,
    pub format: Option<OutputFormat>,
    pub generate_codeowners: Option<bool>,
    /// Settings of the check rules, validated by the checker
    pub rules: Option<serde_yaml::Value>,
}

/// Finds the configuration file to use.
///
/// An explicit path wins. Otherwise the first configuration file found in
/// `search_dirs` is used.
pub fn find_config_file(explicit: Option<&Path>, search_dirs: &[&Path]) -> Result<Option<PathBuf>> {
    if let Some(path) = explicit {
        if !path.is_file() {
            anyhow::bail!("config file `{}` does not exist", path.display());
        }
        return Ok(Some(path.to_path_buf()));
    }

    Ok(search_dirs
        .iter()
        .flat_map(|dir| CONFIG_FILE_NAMES.iter().map(move |name| dir.join(name)))
        .find(|path| path.is_file()))
}

fn resolve(base: &Path, path: &mut PathBuf) {
    if path.is_relative() {
        *path = base.join(&*path);
    }
}

impl ProjectConfig {
    /// Parses the content of a configuration file. Relative paths are kept as they are.
    pub fn parse(content: &str) -> Result<Self> {
        if content.trim().is_empty() {
            return Ok(Self::default());
        }
        let value: serde_yaml::Value = serde_yaml::from_str(content)?;
        if value.is_null() {
            return Ok(Self::default());
        }
        Ok(serde_yaml::from_value(value)?)
    }

    /// Makes the relative paths of the configuration relative to `base` instead
    fn resolve_paths(&mut self, base: &Path) {
        for path in [
            &mut self.path,
            &mut self.coverage_dir,
            &mut self.project_dir,
            &mut self.codeowners_path,
            &mut self.build.build_dir,
        ]
        .into_iter()
        .flatten()
        {
            resolve(base, path);
        }
        for path in &mut self.ignore_path {
            resolve(base, path);
        }
    }

    /// Reads a configuration file, making its relative paths relative to its folder.
    ///
    /// When that folder is inside `current_dir`, the paths stay relative (to `current_dir`).
    pub fn read(path: &Path, current_dir: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("could not read `{}`", path.display()))?;
        let mut config = Self::parse(&content)
            .with_context(|| format!("invalid config `{}`", path.display()))?;
        let base = path.parent().unwrap_or(Path::new(""));
        let base = base.strip_prefix(current_dir).unwrap_or(base);
        config.resolve_paths(base);
        config.source = Some(path.to_path_buf());
        Ok(config)
    }

    /// Finds and reads the configuration file, or returns an empty configuration if there is none
    pub fn load(
        explicit: Option<&Path>,
        search_dirs: &[&Path],
        current_dir: &Path,
    ) -> Result<Self> {
        match find_config_file(explicit, search_dirs)? {
            Some(path) => Self::read(&path, current_dir),
            None => Ok(Self::default()),
        }
    }

    /// Finds and reads the configuration of a scan of `scanned_dir`.
    ///
    /// Searches the scanned folder, then `project_dir`, then `current_dir`.
    pub fn load_for_scan(
        explicit: Option<&Path>,
        scanned_dir: &Path,
        project_dir: Option<&Path>,
        current_dir: &Path,
    ) -> Result<Self> {
        let mut search_dirs = vec![scanned_dir];
        search_dirs.extend(project_dir);
        search_dirs.push(current_dir);
        Self::load(explicit, &search_dirs, current_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_every_option() {
        let config = ProjectConfig::parse(
            r#"
path: src
skip_changes: true
ignore_path: [generated, vendor]
include_skipped_dirs: true
coverage_dir: reports/coverage
project_dir: .
strip_emails: true
codeowners_path: .github/CODEOWNERS
codeowners_prefix: "@org/"
scan:
  json: true
  flat: true
  description: true
  coverage: true
serve:
  port: 4000
  allow_origin: [http://localhost:5173]
  generate_codeowners: true
build:
  build_dir: dist
  generate_codeowners: true
find_owner:
  json: true
owners:
  json: true
diff:
  since: origin/main
  committed_only: true
  json: false
  markdown: true
  owners_only: false
check:
  json: false
  format: github
  generate_codeowners: true
  rules:
    missing-owner: error
"#,
        )
        .unwrap();

        assert_eq!(config.path, Some(PathBuf::from("src")));
        assert_eq!(config.skip_changes, Some(true));
        assert_eq!(
            config.ignore_path,
            vec![PathBuf::from("generated"), PathBuf::from("vendor")]
        );
        assert_eq!(config.codeowners_prefix.as_deref(), Some("@org/"));
        assert_eq!(config.scan.flat, Some(true));
        assert_eq!(config.serve.port, Some(4000));
        assert_eq!(config.serve.allow_origin, vec!["http://localhost:5173"]);
        assert_eq!(config.build.build_dir, Some(PathBuf::from("dist")));
        assert_eq!(config.find_owner.json, Some(true));
        assert_eq!(config.diff.since.as_deref(), Some("origin/main"));
        assert_eq!(config.diff.markdown, Some(true));
        assert_eq!(config.check.format, Some(OutputFormat::Github));
        assert!(config.check.rules.is_some());
    }

    #[test]
    fn test_empty_config() {
        assert_eq!(ProjectConfig::parse("").unwrap(), ProjectConfig::default());
        assert_eq!(
            ProjectConfig::parse("# only a comment\n").unwrap(),
            ProjectConfig::default()
        );
    }

    #[test]
    fn test_unknown_keys_are_errors() {
        // A typo at the top level
        let err = ProjectConfig::parse("skip_change: true\n").unwrap_err();
        assert!(err.to_string().contains("skip_change"), "{err}");
        // A typo in a section
        assert!(ProjectConfig::parse("serve:\n  prot: 4000\n").is_err());
        // An unknown section
        assert!(ProjectConfig::parse("deploy:\n  target: prod\n").is_err());
    }

    #[test]
    fn test_wrong_types_are_errors() {
        assert!(ProjectConfig::parse("serve:\n  port: high\n").is_err());
        assert!(ProjectConfig::parse("skip_changes: maybe\n").is_err());
        assert!(ProjectConfig::parse("check:\n  format: xml\n").is_err());
    }

    #[test]
    fn test_invalid_yaml_is_an_error() {
        assert!(ProjectConfig::parse("check: [unclosed\n").is_err());
        assert!(ProjectConfig::parse("- a list\n").is_err());
    }

    #[test]
    fn test_read_resolves_paths_relative_to_the_file() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join(".features.yml");
        std::fs::write(
            &file,
            "path: src\nignore_path: [gen, /abs]\ncoverage_dir: cov\nbuild:\n  build_dir: out\n",
        )
        .unwrap();

        let config = ProjectConfig::read(&file, Path::new("/elsewhere")).unwrap();
        assert_eq!(config.path, Some(dir.path().join("src")));
        assert_eq!(
            config.ignore_path,
            vec![dir.path().join("gen"), PathBuf::from("/abs")]
        );
        assert_eq!(config.coverage_dir, Some(dir.path().join("cov")));
        assert_eq!(config.build.build_dir, Some(dir.path().join("out")));
        assert_eq!(config.source, Some(file.clone()));

        // Inside the current folder, paths stay relative to it
        let config = ProjectConfig::read(&file, dir.path()).unwrap();
        assert_eq!(config.path, Some(PathBuf::from("src")));
        assert_eq!(config.ignore_path[1], PathBuf::from("/abs"));
    }

    #[test]
    fn test_find_config_file_order() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        std::fs::write(second.path().join(".features.yaml"), "").unwrap();

        let found = find_config_file(None, &[first.path(), second.path()]).unwrap();
        assert_eq!(found, Some(second.path().join(".features.yaml")));

        std::fs::write(second.path().join(".features.yml"), "").unwrap();
        let found = find_config_file(None, &[first.path(), second.path()]).unwrap();
        assert_eq!(found, Some(second.path().join(".features.yml")));

        std::fs::write(first.path().join(".features.yml"), "").unwrap();
        let found = find_config_file(None, &[first.path(), second.path()]).unwrap();
        assert_eq!(found, Some(first.path().join(".features.yml")));

        assert_eq!(find_config_file(None, &[]).unwrap(), None);
    }

    #[test]
    fn test_explicit_config_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("custom.yml");
        assert!(find_config_file(Some(&path), &[dir.path()]).is_err());

        std::fs::write(&path, "check:\n  rules: {}\n").unwrap();
        std::fs::write(dir.path().join(".features.yml"), "skip_changes: true\n").unwrap();
        let config = ProjectConfig::load(Some(&path), &[dir.path()], dir.path()).unwrap();
        assert_eq!(config.source, Some(path));
        assert_eq!(config.skip_changes, None);
        assert!(config.check.rules.is_some());
    }
}
