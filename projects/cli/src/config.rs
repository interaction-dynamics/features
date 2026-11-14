//! The project configuration file.
//!
//! A YAML file, `.features.yml` (or `.features.yaml`), with one section per
//! part of the tool:
//!
//! ```yaml
//! check:
//!   rules:
//!     missing-owner: error
//! ```
//!
//! It is found in the scanned folder, then the project folder, then the
//! current folder, unless `--config <path>` points to another file.
//!
//! Not to be confused with the per-feature `FEATURES.toml` files, which
//! describe a single feature.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Names of the configuration file, in order of preference.
pub const CONFIG_FILE_NAMES: [&str; 2] = [".features.yml", ".features.yaml"];

/// The content of the configuration file.
///
/// Each section is kept as raw YAML and validated by the part of the tool
/// that uses it. Unknown sections are allowed.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ProjectConfig {
    /// Settings of `features check`
    #[serde(default)]
    pub check: Option<serde_yaml::Value>,

    /// The file the configuration was read from
    #[serde(skip)]
    pub path: Option<PathBuf>,
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

impl ProjectConfig {
    /// Parses the content of a configuration file
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

    /// Reads a configuration file
    pub fn read(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("could not read `{}`", path.display()))?;
        let mut config = Self::parse(&content)
            .with_context(|| format!("invalid config `{}`", path.display()))?;
        config.path = Some(path.to_path_buf());
        Ok(config)
    }

    /// Finds and reads the configuration file, or returns an empty configuration if there is none
    pub fn load(explicit: Option<&Path>, search_dirs: &[&Path]) -> Result<Self> {
        match find_config_file(explicit, search_dirs)? {
            Some(path) => Self::read(&path),
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
        Self::load(explicit, &search_dirs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sections() {
        let config = ProjectConfig::parse("check:\n  rules:\n    missing-owner: error\n").unwrap();
        assert!(config.check.is_some());
    }

    #[test]
    fn test_empty_and_unknown_sections_are_allowed() {
        assert!(ProjectConfig::parse("").unwrap().check.is_none());
        assert!(
            ProjectConfig::parse("# only a comment\n")
                .unwrap()
                .check
                .is_none()
        );
        assert!(
            ProjectConfig::parse("serve:\n  port: 4000\n")
                .unwrap()
                .check
                .is_none()
        );
    }

    #[test]
    fn test_invalid_yaml_is_an_error() {
        assert!(ProjectConfig::parse("check: [unclosed\n").is_err());
        assert!(ProjectConfig::parse("- a list\n").is_err());
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
        std::fs::write(dir.path().join(".features.yml"), "").unwrap();
        let config = ProjectConfig::load(Some(&path), &[dir.path()]).unwrap();
        assert_eq!(config.path, Some(path));
        assert!(config.check.is_some());
    }
}
