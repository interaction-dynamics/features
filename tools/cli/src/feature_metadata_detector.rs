//! Module for detecting feature metadata in source code comments
//!
//! This module scans source files for special comments that start with "--feature-"
//! and contain feature metadata. It supports multiple programming languages
//! and their respective comment styles.

use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Represents a single metadata entry's properties (key-value pairs)
type MetadataProperties = HashMap<String, String>;

/// Represents a list of metadata entries for a specific metadata key
type MetadataEntries = Vec<MetadataProperties>;

/// Maps metadata keys (e.g., "feature-flag") to their entries
type MetadataByKey = HashMap<String, MetadataEntries>;

/// Maps feature names to their metadata, organized by metadata key
pub type FeatureMetadataMap = HashMap<String, MetadataByKey>;

#[derive(Debug, Clone)]
pub struct FeatureMetadataComment {
    #[allow(dead_code)]
    pub file_path: String,
    #[allow(dead_code)]
    pub line_number: usize,
    pub metadata_key: String,
    pub properties: MetadataProperties,
}

/// Detects comment start patterns for various languages based on file extension
fn get_comment_patterns(extension: &str) -> Vec<CommentPattern> {
    match extension {
        // C-style comments (C, C++, Java, JavaScript, TypeScript, Rust, etc.)
        "rs" | "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "java" | "js" | "jsx" | "ts" | "tsx"
        | "go" | "cs" | "swift" | "kt" | "scala" => vec![
            CommentPattern::LineComment("//"),
            CommentPattern::BlockComment("/*", "*/"),
        ],
        // Python, Bash, Shell, Ruby, Perl, YAML, etc.
        "py" | "sh" | "bash" | "rb" | "pl" | "yml" | "yaml" | "toml" => {
            vec![CommentPattern::LineComment("#")]
        }
        // HTML, XML
        "html" | "xml" | "svg" => vec![CommentPattern::BlockComment("<!--", "-->")],
        // CSS, SCSS, Less
        "css" | "scss" | "less" => vec![
            CommentPattern::LineComment("//"),
            CommentPattern::BlockComment("/*", "*/"),
        ],
        // Lua
        "lua" => vec![
            CommentPattern::LineComment("--"),
            CommentPattern::BlockComment("--[[", "]]"),
        ],
        // SQL
        "sql" => vec![
            CommentPattern::LineComment("--"),
            CommentPattern::BlockComment("/*", "*/"),
        ],
        // Default: try common patterns
        _ => vec![
            CommentPattern::LineComment("//"),
            CommentPattern::LineComment("#"),
            CommentPattern::BlockComment("/*", "*/"),
        ],
    }
}

#[derive(Debug, Clone)]
enum CommentPattern {
    LineComment(&'static str),
    BlockComment(&'static str, &'static str),
}

/// Extracts the content from a comment, handling different comment styles
fn extract_comment_content(line: &str, patterns: &[CommentPattern]) -> Option<String> {
    let trimmed = line.trim();

    for pattern in patterns {
        match pattern {
            CommentPattern::LineComment(prefix) => {
                if let Some(content) = trimmed.strip_prefix(prefix) {
                    return Some(content.trim().to_string());
                }
            }
            CommentPattern::BlockComment(start, end) => {
                if let Some(mut content) = trimmed.strip_prefix(start) {
                    // Remove ending marker if present on same line
                    if let Some(stripped) = content.strip_suffix(end) {
                        content = stripped;
                    }
                    return Some(content.trim().to_string());
                }
            }
        }
    }

    None
}

/// Parses properties from a feature flag comment
/// Format: "key: value, key2: value2, ..."
fn parse_properties(content: &str) -> MetadataProperties {
    let mut properties = MetadataProperties::new();

    // Split by comma and parse key:value pairs
    for part in content.split(',') {
        let part = part.trim();
        if let Some(colon_pos) = part.find(':') {
            let key = part[..colon_pos].trim();
            let value = part[colon_pos + 1..].trim();

            if !key.is_empty() {
                properties.insert(key.to_string(), value.to_string());
            }
        }
    }

    properties
}

/// Checks if a line contains a feature metadata comment
/// Returns the metadata key (e.g., "feature-flag") and the properties
fn check_line_for_feature_metadata(
    line: &str,
    patterns: &[CommentPattern],
) -> Option<(String, MetadataProperties)> {
    if let Some(comment_content) = extract_comment_content(line, patterns) {
        // Check if the comment contains "--feature-" pattern
        if let Some(feature_start) = comment_content.find("--feature-") {
            // Extract the metadata key (e.g., "--feature-flag" -> "feature-flag")
            let after_dashes = &comment_content[feature_start + 2..]; // Skip "--"

            // Find where the metadata key ends (at space, comma, or end of string)
            let metadata_key_end = after_dashes
                .find(|c: char| c.is_whitespace() || c == ',')
                .unwrap_or(after_dashes.len());

            let metadata_key = after_dashes[..metadata_key_end].to_string();

            // Look for the "feature:" part which contains the actual metadata
            if let Some(feature_pos) = comment_content.find("feature:") {
                // Extract everything from "feature:" onwards
                let metadata_part = &comment_content[feature_pos..];
                let properties = parse_properties(metadata_part);

                // Only return if we actually found some properties
                if !properties.is_empty() {
                    return Some((metadata_key, properties));
                }
            }
        }
    }

    None
}

/// Scans a single file for feature metadata comments
fn scan_file(file_path: &Path) -> Result<Vec<FeatureMetadataComment>> {
    let mut results = Vec::new();

    // Get file extension
    let extension = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");

    let patterns = get_comment_patterns(extension);

    // Read file content
    let content = fs::read_to_string(file_path)?;

    // Check each line
    for (line_number, line) in content.lines().enumerate() {
        if let Some((metadata_key, properties)) = check_line_for_feature_metadata(line, &patterns) {
            results.push(FeatureMetadataComment {
                file_path: file_path.to_string_lossy().to_string(),
                line_number: line_number + 1, // Line numbers are 1-based
                metadata_key,
                properties,
            });
        }
    }

    Ok(results)
}

/// Scans a directory recursively for feature metadata comments
///
/// Returns a nested map:
/// - Outer key: feature name (from "feature:feature-1")
/// - Inner key: metadata key (from "--feature-flag", "--feature-experiment", etc.)
/// - Value: vector of property maps
pub fn scan_directory_for_feature_metadata(dir_path: &Path) -> Result<FeatureMetadataMap> {
    let mut feature_metadata = FeatureMetadataMap::new();

    // Skip common directories that shouldn't be scanned
    let skip_dirs = [
        "node_modules",
        "target",
        "dist",
        "build",
        ".git",
        ".svn",
        ".hg",
        "vendor",
        "__pycache__",
        ".next",
        ".nuxt",
        "coverage",
    ];

    for entry in WalkDir::new(dir_path)
        .into_iter()
        .filter_entry(|e| {
            if e.file_type().is_dir() {
                let dir_name = e.file_name().to_string_lossy();
                !skip_dirs.contains(&dir_name.as_ref())
            } else {
                true
            }
        })
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file()
            && let Ok(comments) = scan_file(entry.path())
        {
            for comment in comments {
                // Get the feature name from the properties
                if let Some(feature_name) = comment.properties.get("feature") {
                    feature_metadata
                        .entry(feature_name.clone())
                        .or_default()
                        .entry(comment.metadata_key)
                        .or_default()
                        .push(comment.properties);
                }
            }
        }
    }

    Ok(feature_metadata)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_comment_content_single_line() {
        let patterns = vec![CommentPattern::LineComment("//")];
        let line = "// This is a comment";
        assert_eq!(
            extract_comment_content(line, &patterns),
            Some("This is a comment".to_string())
        );
    }

    #[test]
    fn test_extract_comment_content_block() {
        let patterns = vec![CommentPattern::BlockComment("/*", "*/")];
        let line = "/* This is a block comment */";
        assert_eq!(
            extract_comment_content(line, &patterns),
            Some("This is a block comment".to_string())
        );
    }

    #[test]
    fn test_extract_comment_content_hash() {
        let patterns = vec![CommentPattern::LineComment("#")];
        let line = "# This is a Python comment";
        assert_eq!(
            extract_comment_content(line, &patterns),
            Some("This is a Python comment".to_string())
        );
    }

    #[test]
    fn test_parse_properties() {
        let content =
            "feature:feature-1, type: experiment, owner: #owner, introduced_on: 2025-12-31";
        let props = parse_properties(content);

        assert_eq!(props.get("feature"), Some(&"feature-1".to_string()));
        assert_eq!(props.get("type"), Some(&"experiment".to_string()));
        assert_eq!(props.get("owner"), Some(&"#owner".to_string()));
        assert_eq!(props.get("introduced_on"), Some(&"2025-12-31".to_string()));
    }

    #[test]
    fn test_check_line_for_feature_metadata_js_style() {
        let patterns = vec![CommentPattern::BlockComment("/*", "*/")];
        let line = "/** --feature-flag feature:feature-1, type: experiment, owner: #owner */";

        let result = check_line_for_feature_metadata(line, &patterns);
        assert!(result.is_some());

        let (metadata_key, props) = result.unwrap();
        assert_eq!(metadata_key, "feature-flag");
        assert_eq!(props.get("feature"), Some(&"feature-1".to_string()));
        assert_eq!(props.get("type"), Some(&"experiment".to_string()));
    }
    #[test]
    fn test_check_line_for_feature_metadata_rust_style() {
        let patterns = vec![CommentPattern::LineComment("//")];
        let line = "// --feature-flag feature:my-feature, enabled: true";

        let result = check_line_for_feature_metadata(line, &patterns);
        assert!(result.is_some());

        let (metadata_key, props) = result.unwrap();
        assert_eq!(metadata_key, "feature-flag");
        assert_eq!(props.get("feature"), Some(&"my-feature".to_string()));
        assert_eq!(props.get("enabled"), Some(&"true".to_string()));
    }

    #[test]
    fn test_check_line_for_feature_metadata_python_style() {
        let patterns = vec![CommentPattern::LineComment("#")];
        let line = "# --feature-flag feature:analytics, team: data-team";

        let result = check_line_for_feature_metadata(line, &patterns);
        assert!(result.is_some());

        let (metadata_key, props) = result.unwrap();
        assert_eq!(metadata_key, "feature-flag");
        assert_eq!(props.get("feature"), Some(&"analytics".to_string()));
        assert_eq!(props.get("team"), Some(&"data-team".to_string()));
    }

    #[test]
    fn test_no_feature_metadata_in_regular_comment() {
        let patterns = vec![CommentPattern::LineComment("//")];
        let line = "// This is just a regular comment";

        assert!(check_line_for_feature_metadata(line, &patterns).is_none());
    }

    #[test]
    fn test_different_metadata_keys() {
        let patterns = vec![CommentPattern::LineComment("//")];

        let line1 = "// --feature-experiment feature:test-feature, status: active";
        let result1 = check_line_for_feature_metadata(line1, &patterns);
        assert!(result1.is_some());
        let (metadata_key1, _) = result1.unwrap();
        assert_eq!(metadata_key1, "feature-experiment");

        let line2 = "// --feature-toggle feature:another-feature, enabled: true";
        let result2 = check_line_for_feature_metadata(line2, &patterns);
        assert!(result2.is_some());
        let (metadata_key2, _) = result2.unwrap();
        assert_eq!(metadata_key2, "feature-toggle");
    }
}
