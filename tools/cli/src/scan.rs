//! Unified scanning API for features
//!
//! This module provides a high-level interface for scanning features with configurable options.
//! It consolidates the functionality from file_scanner and coverage_parser into a single,
//! easy-to-use API.

use anyhow::Result;
use std::path::Path;

use crate::coverage_parser::{self, map_coverage_to_features, parse_coverage_reports};
use crate::file_scanner::{list_files_recursive, list_files_recursive_with_changes};
use crate::models::Feature;

/// Configuration options for scanning features
#[derive(Debug, Clone)]
pub struct ScanConfig<'a> {
    /// Whether to include git history (changes, commits, stats)
    pub skip_changes: bool,

    /// Whether to add coverage information to features
    pub should_add_coverage: bool,

    /// Optional override for coverage directory location
    /// If None, will search in multiple default locations
    pub coverage_dir_override: Option<&'a Path>,

    /// Current working directory (used for finding coverage)
    pub current_dir: &'a Path,

    /// Optional project directory (used for finding coverage)
    pub project_dir: Option<&'a Path>,
}

impl<'a> ScanConfig<'a> {
    /// Create a new ScanConfig with minimal required parameters
    pub fn new(current_dir: &'a Path) -> Self {
        Self {
            skip_changes: false,
            should_add_coverage: false,
            coverage_dir_override: None,
            current_dir,
            project_dir: None,
        }
    }

    /// Set whether to skip git history
    pub fn skip_changes(mut self, skip: bool) -> Self {
        self.skip_changes = skip;
        self
    }

    /// Set whether to add coverage information
    pub fn with_coverage(mut self, should_add: bool) -> Self {
        self.should_add_coverage = should_add;
        self
    }

    /// Set a specific coverage directory to use
    pub fn coverage_dir(mut self, dir: &'a Path) -> Self {
        self.coverage_dir_override = Some(dir);
        self
    }

    /// Set the project directory for finding coverage
    pub fn project_dir(mut self, dir: &'a Path) -> Self {
        self.project_dir = Some(dir);
        self
    }
}

/// Scan features in a directory with the given configuration
///
/// # Arguments
///
/// * `base_path` - The directory to scan for features
/// * `config` - Configuration options for the scan
///
/// # Returns
///
/// A vector of Feature objects representing the discovered features
///
/// # Example
///
/// ```no_run
/// use features_cli::scan::{scan_features, ScanConfig};
/// use std::path::Path;
///
/// let current_dir = std::env::current_dir().unwrap();
/// let config = ScanConfig::new(&current_dir)
///     .skip_changes(false)
///     .with_coverage(true);
///
/// let features = scan_features(Path::new("./src"), config).unwrap();
/// println!("Found {} features", features.len());
/// ```
pub fn scan_features(base_path: &Path, config: ScanConfig) -> Result<Vec<Feature>> {
    // Step 1: Scan features with or without git history
    let mut features = if config.skip_changes {
        list_files_recursive(base_path)?
    } else {
        list_files_recursive_with_changes(base_path)?
    };

    // Step 2: Add coverage if requested
    if config.should_add_coverage {
        add_coverage_to_features(
            &mut features,
            base_path,
            config.coverage_dir_override,
            config.current_dir,
            config.project_dir,
        );
    }

    Ok(features)
}

/// Add coverage information to features
///
/// This function searches for coverage reports in multiple locations and
/// updates the features with coverage statistics.
fn add_coverage_to_features(
    features: &mut [Feature],
    base_path: &Path,
    coverage_dir_override: Option<&Path>,
    current_dir: &Path,
    project_dir: Option<&Path>,
) {
    let coverage_dirs = if let Some(override_dir) = coverage_dir_override {
        // If override is provided, only use that directory
        vec![override_dir.to_path_buf()]
    } else {
        // Check multiple locations:
        // 1. .coverage and coverage in base_path
        // 2. .coverage and coverage in current directory (where executable is run)
        // 3. .coverage and coverage in project_dir (if provided)
        let mut dirs = vec![
            base_path.join(".coverage"),
            base_path.join("coverage"),
            current_dir.join(".coverage"),
            current_dir.join("coverage"),
        ];

        // Add project_dir coverage directories if provided
        if let Some(proj_dir) = project_dir {
            let proj_coverage = proj_dir.join(".coverage");
            let proj_coverage_plain = proj_dir.join("coverage");

            // Only add if different from already added paths
            if !dirs.contains(&proj_coverage) {
                dirs.push(proj_coverage);
            }
            if !dirs.contains(&proj_coverage_plain) {
                dirs.push(proj_coverage_plain);
            }
        }

        dirs
    };

    for coverage_dir in &coverage_dirs {
        // Parse coverage reports if the directory exists
        if let Ok(coverage_map) = parse_coverage_reports(coverage_dir, base_path)
            && !coverage_map.is_empty()
        {
            // Use coverage from the first directory found
            let feature_coverage = map_coverage_to_features(features, coverage_map, base_path);
            update_features_with_coverage(features, &feature_coverage);
            break; // Stop after finding coverage in one directory
        }
    }
}

/// Recursively update features with coverage data
fn update_features_with_coverage(
    features: &mut [Feature],
    feature_coverage: &std::collections::HashMap<String, coverage_parser::CoverageStats>,
) {
    for feature in features {
        if let Some(coverage) = feature_coverage.get(&feature.path) {
            // Update or create stats
            if let Some(stats) = &mut feature.stats {
                stats.coverage = Some(coverage.clone());
            } else {
                feature.stats = Some(crate::models::Stats {
                    files_count: None,
                    lines_count: None,
                    todos_count: None,
                    commits: std::collections::BTreeMap::new(),
                    coverage: Some(coverage.clone()),
                });
            }
        }

        // Recursively update nested features
        update_features_with_coverage(&mut feature.features, feature_coverage);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_scan_config_builder() {
        let current_dir = PathBuf::from("/tmp");
        let project_dir = PathBuf::from("/project");

        let config = ScanConfig::new(&current_dir)
            .skip_changes(true)
            .with_coverage(true)
            .project_dir(&project_dir);

        assert!(config.skip_changes);
        assert!(config.should_add_coverage);
        assert!(config.project_dir.is_some());
    }

    #[test]
    fn test_scan_config_defaults() {
        let current_dir = PathBuf::from("/tmp");
        let config = ScanConfig::new(&current_dir);

        assert!(!config.skip_changes);
        assert!(!config.should_add_coverage);
        assert!(config.coverage_dir_override.is_none());
        assert!(config.project_dir.is_none());
    }

    #[test]
    fn test_scan_features_basic() {
        let test_path = PathBuf::from("../../examples/tests_skip_changes/src");

        if !test_path.exists() {
            println!("Skipping test - test path does not exist");
            return;
        }

        let current_dir = std::env::current_dir().unwrap();
        let config = ScanConfig::new(&current_dir).skip_changes(true);

        let result = scan_features(&test_path, config);
        assert!(result.is_ok(), "Failed to scan features");

        let features = result.unwrap();
        assert!(!features.is_empty(), "Should find at least one feature");
    }

    #[test]
    fn test_scan_features_with_changes() {
        let test_path = PathBuf::from("../../examples/tests-with-changes/src");

        if !test_path.exists() {
            println!("Skipping test - test path does not exist");
            return;
        }

        let current_dir = std::env::current_dir().unwrap();
        let config = ScanConfig::new(&current_dir).skip_changes(false);

        let result = scan_features(&test_path, config);
        assert!(result.is_ok(), "Failed to scan features with changes");

        let features = result.unwrap();
        assert!(!features.is_empty(), "Should find at least one feature");

        // At least one feature should have changes
        let has_changes = features.iter().any(|f| !f.changes.is_empty());
        assert!(has_changes, "At least one feature should have git history");
    }
}
