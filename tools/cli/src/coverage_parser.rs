use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::models::Feature;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileCoverageStats {
    pub lines_total: usize,
    pub lines_covered: usize,
    pub lines_missed: usize,
    pub line_coverage_percent: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches_total: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches_covered: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_coverage_percent: Option<f64>,
}

impl Default for FileCoverageStats {
    fn default() -> Self {
        Self {
            lines_total: 0,
            lines_covered: 0,
            lines_missed: 0,
            line_coverage_percent: 0.0,
            branches_total: None,
            branches_covered: None,
            branch_coverage_percent: None,
        }
    }
}

impl FileCoverageStats {
    pub fn new() -> Self {
        Self {
            lines_total: 0,
            lines_covered: 0,
            lines_missed: 0,
            line_coverage_percent: 0.0,
            branches_total: None,
            branches_covered: None,
            branch_coverage_percent: None,
        }
    }

    pub fn calculate_percentages(&mut self) {
        if self.lines_total > 0 {
            self.line_coverage_percent =
                (self.lines_covered as f64 / self.lines_total as f64) * 100.0;
        }

        if let (Some(total), Some(covered)) = (self.branches_total, self.branches_covered)
            && total > 0
        {
            self.branch_coverage_percent = Some((covered as f64 / total as f64) * 100.0);
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoverageStats {
    pub lines_total: usize,
    pub lines_covered: usize,
    pub lines_missed: usize,
    pub line_coverage_percent: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches_total: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches_covered: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_coverage_percent: Option<f64>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub files: HashMap<String, FileCoverageStats>,
}

impl Default for CoverageStats {
    fn default() -> Self {
        Self {
            lines_total: 0,
            lines_covered: 0,
            lines_missed: 0,
            line_coverage_percent: 0.0,
            branches_total: None,
            branches_covered: None,
            branch_coverage_percent: None,
            files: HashMap::new(),
        }
    }
}

impl CoverageStats {
    // used by tests
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            lines_total: 0,
            lines_covered: 0,
            lines_missed: 0,
            line_coverage_percent: 0.0,
            branches_total: None,
            branches_covered: None,
            branch_coverage_percent: None,
            files: HashMap::new(),
        }
    }

    pub fn calculate_percentages(&mut self) {
        if self.lines_total > 0 {
            self.line_coverage_percent =
                (self.lines_covered as f64 / self.lines_total as f64) * 100.0;
        }

        if let (Some(total), Some(covered)) = (self.branches_total, self.branches_covered)
            && total > 0
        {
            self.branch_coverage_percent = Some((covered as f64 / total as f64) * 100.0);
        }
    }

    pub fn merge(&mut self, other: &CoverageStats) {
        self.lines_total += other.lines_total;
        self.lines_covered += other.lines_covered;
        // Recalculate lines_missed based on merged totals
        self.lines_missed = self.lines_total.saturating_sub(self.lines_covered);

        if let Some(other_branches_total) = other.branches_total {
            self.branches_total = Some(self.branches_total.unwrap_or(0) + other_branches_total);
        }

        if let Some(other_branches_covered) = other.branches_covered {
            self.branches_covered =
                Some(self.branches_covered.unwrap_or(0) + other_branches_covered);
        }

        // Merge file-level coverage
        for (file_path, file_stats) in &other.files {
            self.files.insert(file_path.clone(), file_stats.clone());
        }

        self.calculate_percentages();
    }
}

#[derive(Debug)]
struct FileCoverage {
    path: PathBuf,
    lines_total: usize,
    lines_covered: usize,
    branches_total: usize,
    branches_covered: usize,
}

/// Detects and parses coverage reports from the .coverage directory
pub fn parse_coverage_reports(coverage_dir: &Path) -> Result<HashMap<String, CoverageStats>> {
    let mut coverage_map: HashMap<String, CoverageStats> = HashMap::new();

    if !coverage_dir.exists() {
        return Ok(coverage_map);
    }

    // Find all coverage files in the directory
    let entries = fs::read_dir(coverage_dir).context("Failed to read coverage directory")?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            // Detect file type and parse accordingly
            if (file_name.ends_with(".xml") || file_name.contains("cobertura"))
                && let Ok(file_coverage) = parse_cobertura_xml(&path)
            {
                merge_file_coverage(&mut coverage_map, file_coverage);
            } else if (file_name.ends_with(".info") || file_name.contains("lcov"))
                && let Ok(file_coverage) = parse_lcov(&path)
            {
                merge_file_coverage(&mut coverage_map, file_coverage);
            }
        }
    }

    Ok(coverage_map)
}

/// Merge file coverage data into the coverage map
fn merge_file_coverage(
    coverage_map: &mut HashMap<String, CoverageStats>,
    file_coverage: Vec<FileCoverage>,
) {
    for fc in file_coverage {
        let path_str = fc.path.to_string_lossy().to_string();

        let stats = coverage_map.entry(path_str.clone()).or_default();

        // Create individual file stats
        let mut file_stats = FileCoverageStats::new();
        file_stats.lines_total = fc.lines_total;
        file_stats.lines_covered = fc.lines_covered;
        file_stats.lines_missed = fc.lines_total.saturating_sub(fc.lines_covered);

        if fc.branches_total > 0 {
            file_stats.branches_total = Some(fc.branches_total);
            file_stats.branches_covered = Some(fc.branches_covered);
        }

        file_stats.calculate_percentages();

        // Store file-level coverage
        stats.files.insert(path_str, file_stats.clone());

        // Update aggregate stats
        stats.lines_total += fc.lines_total;
        stats.lines_covered += fc.lines_covered;
        stats.lines_missed += fc.lines_total.saturating_sub(fc.lines_covered);

        if fc.branches_total > 0 {
            stats.branches_total = Some(stats.branches_total.unwrap_or(0) + fc.branches_total);
            stats.branches_covered =
                Some(stats.branches_covered.unwrap_or(0) + fc.branches_covered);
        }

        stats.calculate_percentages();
    }
}

/// Parse Cobertura XML format
fn parse_cobertura_xml(path: &Path) -> Result<Vec<FileCoverage>> {
    let content = fs::read_to_string(path).context("Failed to read Cobertura XML file")?;

    let mut file_coverage = Vec::new();

    // Simple XML parsing without external dependencies
    // This is a basic parser that looks for <class> elements with coverage attributes
    let lines: Vec<&str> = content.lines().collect();
    let mut current_file: Option<String> = None;
    let mut lines_total = 0;
    let mut lines_covered = 0;
    let mut branches_total = 0;
    let mut branches_covered = 0;

    for line in lines {
        let trimmed = line.trim();

        // Look for class or file elements with filename attribute
        if trimmed.contains("<class") || trimmed.contains("<file") {
            // Save previous file if exists
            if let Some(file_path) = current_file.take() {
                if lines_total > 0 {
                    file_coverage.push(FileCoverage {
                        path: PathBuf::from(file_path),
                        lines_total,
                        lines_covered,
                        branches_total,
                        branches_covered,
                    });
                }
                lines_total = 0;
                lines_covered = 0;
                branches_total = 0;
                branches_covered = 0;
            }

            // Extract filename
            if let Some(filename) = extract_attribute(trimmed, "filename") {
                current_file = Some(filename);
            } else if let Some(filename) = extract_attribute(trimmed, "name") {
                current_file = Some(filename);
            }

            // Extract coverage metrics if present in the same tag
            if let Some(val) = extract_attribute(trimmed, "lines-valid") {
                lines_total = val.parse().unwrap_or(0);
            }
            if let Some(val) = extract_attribute(trimmed, "lines-covered") {
                lines_covered = val.parse().unwrap_or(0);
            }
            if let Some(val) = extract_attribute(trimmed, "branches-valid") {
                branches_total = val.parse().unwrap_or(0);
            }
            if let Some(val) = extract_attribute(trimmed, "branches-covered") {
                branches_covered = val.parse().unwrap_or(0);
            }
        }

        // Alternative: extract from line elements
        if current_file.is_some() && trimmed.contains("<line") {
            if let Some(hits) = extract_attribute(trimmed, "hits") {
                lines_total += 1;
                if hits.parse::<usize>().unwrap_or(0) > 0 {
                    lines_covered += 1;
                }
            }

            // Check for branch coverage
            if let Some(branch) = extract_attribute(trimmed, "branch")
                && branch == "true"
                && let Some(condition_coverage) = extract_attribute(trimmed, "condition-coverage")
                && let Some((covered, total)) = parse_condition_coverage(&condition_coverage)
            {
                branches_total += total;
                branches_covered += covered;
            }
        }
    }

    // Save last file
    if let Some(file_path) = current_file
        && lines_total > 0
    {
        file_coverage.push(FileCoverage {
            path: PathBuf::from(file_path),
            lines_total,
            lines_covered,
            branches_total,
            branches_covered,
        });
    }

    Ok(file_coverage)
}

/// Parse Lcov format
fn parse_lcov(path: &Path) -> Result<Vec<FileCoverage>> {
    let content = fs::read_to_string(path).context("Failed to read Lcov file")?;

    let mut file_coverage = Vec::new();
    let mut current_file: Option<&str> = None;
    let mut lines_total = 0;
    let mut lines_covered = 0;
    let mut branches_total = 0;
    let mut branches_covered = 0;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("SF:") {
            // Start of a new file
            if let Some(file_path) = current_file.take() {
                file_coverage.push(FileCoverage {
                    path: PathBuf::from(file_path),
                    lines_total,
                    lines_covered,
                    branches_total,
                    branches_covered,
                });
                lines_total = 0;
                lines_covered = 0;
                branches_total = 0;
                branches_covered = 0;
            }
            current_file = trimmed.strip_prefix("SF:");
        } else if trimmed.starts_with("DA:")
            && let Some(comma_pos) = trimmed.find(',')
            && let Ok(count) = trimmed[comma_pos + 1..].parse::<usize>()
        {
            lines_total += 1;
            if count > 0 {
                lines_covered += 1;
            }
        } else if trimmed.starts_with("BRDA:") {
            // Branch coverage: BRDA:line_number,block_number,branch_number,taken
            branches_total += 1;
            let parts: Vec<&str> = trimmed
                .strip_prefix("BRDA:")
                .expect("")
                .split(',')
                .collect();
            if parts.len() >= 4 {
                let taken = parts[3];
                if taken != "-" && taken != "0" {
                    branches_covered += 1;
                }
            }
        } else if trimmed.starts_with("LF:")
            && let Ok(count) = trimmed[3..].parse::<usize>()
        {
            lines_total = count;
        } else if trimmed.starts_with("LH:")
            && let Ok(count) = trimmed[3..].parse::<usize>()
        {
            lines_covered = count;
        } else if trimmed.starts_with("BRF:")
            && let Ok(count) = trimmed[4..].parse::<usize>()
        {
            branches_total = count;
        } else if trimmed.starts_with("BRH:")
            && let Ok(count) = trimmed[4..].parse::<usize>()
        {
            branches_covered = count;
        } else if trimmed == "end_of_record" {
            // End of current file record
            if let Some(file_path) = current_file.take() {
                file_coverage.push(FileCoverage {
                    path: PathBuf::from(file_path),
                    lines_total,
                    lines_covered,
                    branches_total,
                    branches_covered,
                });
                lines_total = 0;
                lines_covered = 0;
                branches_total = 0;
                branches_covered = 0;
            }
        }
    }

    // Save last file if not already saved
    if let Some(file_path) = current_file {
        file_coverage.push(FileCoverage {
            path: PathBuf::from(file_path),
            lines_total,
            lines_covered,
            branches_total,
            branches_covered,
        });
    }

    Ok(file_coverage)
}

/// Extract an attribute value from an XML tag
fn extract_attribute(line: &str, attr_name: &str) -> Option<String> {
    let pattern = format!("{}=\"", attr_name);
    if let Some(start) = line.find(&pattern) {
        let value_start = start + pattern.len();
        if let Some(end) = line[value_start..].find('"') {
            return Some(line[value_start..value_start + end].to_string());
        }
    }
    None
}

/// Parse condition coverage string like "50% (1/2)"
fn parse_condition_coverage(coverage_str: &str) -> Option<(usize, usize)> {
    if let Some(paren_start) = coverage_str.find('(')
        && let Some(paren_end) = coverage_str.find(')')
    {
        let fraction = &coverage_str[paren_start + 1..paren_end];
        let parts: Vec<&str> = fraction.split('/').collect();
        if parts.len() == 2 {
            let covered = parts[0].parse().ok()?;
            let total = parts[1].parse().ok()?;
            return Some((covered, total));
        }
    }
    None
}

/// Map coverage data to features
pub fn map_coverage_to_features(
    features: &[Feature],
    coverage_map: HashMap<String, CoverageStats>,
    base_path: &Path,
) -> HashMap<String, CoverageStats> {
    let mut feature_coverage: HashMap<String, CoverageStats> = HashMap::new();

    // Normalize base path
    let canonical_base = std::fs::canonicalize(base_path).ok();

    for (file_path, coverage) in coverage_map {
        // Find which feature this file belongs to
        if let Some(feature_name) =
            find_feature_for_file(&file_path, features, canonical_base.as_deref())
        {
            let stats = feature_coverage.entry(feature_name.clone()).or_default();

            // Add each file's coverage to the feature
            for (individual_file_path, file_stats) in &coverage.files {
                // Check if this individual file belongs to the current feature
                if let Some(file_feature) =
                    find_feature_for_file(individual_file_path, features, canonical_base.as_deref())
                    && file_feature == feature_name
                {
                    stats
                        .files
                        .insert(individual_file_path.clone(), file_stats.clone());
                }
            }

            stats.merge(&coverage);
        }
    }

    feature_coverage
}

/// Find which feature a file belongs to
fn find_feature_for_file(
    file_path: &str,
    features: &[Feature],
    canonical_base: Option<&Path>,
) -> Option<String> {
    let file_path_buf = PathBuf::from(file_path);

    // Try to canonicalize the file path, or use as-is if it fails
    let canonical_file = std::fs::canonicalize(&file_path_buf)
        .or_else(|_| {
            // If absolute path fails, try relative to base
            if let Some(base) = canonical_base {
                std::fs::canonicalize(base.join(&file_path_buf))
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::NotFound, ""))
            }
        })
        .ok();

    // Normalize file path by removing leading ./ and converting to string
    let normalized_file = normalize_path(file_path);

    fn search_features(
        canonical_file: Option<&Path>,
        normalized_file: &str,
        features: &[Feature],
    ) -> Option<String> {
        for feature in features {
            let feature_path = PathBuf::from(&feature.path);

            // Try canonical comparison first
            if let Some(cf) = canonical_file
                && let Ok(canonical_feature) = std::fs::canonicalize(&feature_path)
                && cf.starts_with(&canonical_feature)
            {
                // Check nested features first (more specific)
                if let Some(nested) =
                    search_features(canonical_file, normalized_file, &feature.features)
                {
                    return Some(nested);
                }
                return Some(feature.name.clone());
            }

            // Fallback to normalized string comparison
            let normalized_feature = normalize_path(&feature.path);

            if normalized_file.starts_with(&normalized_feature) {
                // Check nested features first (more specific)
                if let Some(nested) =
                    search_features(canonical_file, normalized_file, &feature.features)
                {
                    return Some(nested);
                }
                return Some(feature.name.clone());
            }
        }
        None
    }

    search_features(canonical_file.as_deref(), &normalized_file, features)
}

/// Normalize a path by removing leading ./ and converting to forward slashes
fn normalize_path(path: &str) -> String {
    let path = path.trim_start_matches("./");
    let path = path.replace('\\', "/");
    path.to_string()
}
