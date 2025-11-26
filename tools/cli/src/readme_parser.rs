use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Information extracted from a README file
pub struct ReadmeInfo {
    /// Optional title extracted from the first markdown heading
    pub title: Option<String>,
    /// Owner of the feature
    pub owner: String,
    /// Description content (everything after the first heading)
    pub description: String,
    /// Additional metadata from YAML frontmatter
    pub meta: HashMap<String, serde_json::Value>,
}

fn extract_first_title(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();

        // Check if this line is a title (starting with #)
        if trimmed.starts_with('#') {
            // Remove all leading # characters and whitespace
            let title = trimmed.trim_start_matches('#').trim();
            if !title.is_empty() {
                return Some(title.to_string());
            }
        }
    }
    None
}

fn read_readme_content(content: &str) -> String {
    let mut found_first_title = false;
    let mut lines_after_title: Vec<&str> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Check if we found the first title (starting with #)
        if !found_first_title && trimmed.starts_with('#') {
            found_first_title = true;
            continue;
        }

        // Collect all lines after the first title
        if found_first_title {
            lines_after_title.push(line);
        }
    }

    // Join all lines after the first title
    lines_after_title.join("\n").trim().to_string()
}

/// Reads README information from README.md or README.mdx files
/// Returns ReadmeInfo containing title, owner, description, and metadata
/// The title is extracted from the first markdown heading (# Title)
pub fn read_readme_info(readme_path: &Path) -> Result<ReadmeInfo> {
    if !readme_path.exists() {
        return Ok(ReadmeInfo {
            title: None,
            owner: "".to_string(),
            description: "".to_string(),
            meta: HashMap::new(),
        });
    }

    let content = fs::read_to_string(readme_path)
        .with_context(|| format!("could not read README file at `{}`", readme_path.display()))?;

    let mut title: Option<String> = None;
    let mut owner = "".to_string();
    let mut description = "".to_string();
    let mut meta: HashMap<String, serde_json::Value> = HashMap::new();

    // Check if content starts with YAML front matter (---)
    if let Some(stripped) = content.strip_prefix("---\n") {
        if let Some(end_pos) = stripped.find("\n---\n") {
            let yaml_content = &stripped[..end_pos];
            let markdown_content = stripped[end_pos + 5..].to_string();

            // Parse YAML front matter
            if let Ok(yaml_value) = serde_yaml::from_str::<serde_yaml::Value>(yaml_content)
                && let Some(mapping) = yaml_value.as_mapping()
            {
                for (key, value) in mapping {
                    if let Some(key_str) = key.as_str() {
                        if key_str == "owner" {
                            if let Some(owner_value) = value.as_str() {
                                owner = owner_value.to_string();
                            }
                        } else {
                            // Convert YAML value to JSON value for meta
                            if let Ok(json_value) = serde_json::to_value(value) {
                                meta.insert(key_str.to_string(), json_value);
                            }
                        }
                    }
                }
            }

            // Extract title from markdown content (after frontmatter)
            title = extract_first_title(&markdown_content);
            description = read_readme_content(&markdown_content)
        }
    } else {
        // No frontmatter, extract title and description from full content
        title = extract_first_title(&content);
        description = read_readme_content(&content)
    }

    Ok(ReadmeInfo {
        title,
        owner,
        description,
        meta,
    })
}
