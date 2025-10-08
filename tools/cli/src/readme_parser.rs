use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn read_readme_content(content: &String) -> String {
    let mut description = String::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            description = trimmed.to_string();
            break;
        }
    }

    return description;
}

pub fn read_readme_info(
    readme_path: &Path,
) -> Result<(String, String, HashMap<String, serde_json::Value>)> {
    if !readme_path.exists() {
        return Ok((
            "Unknown".to_string(),
            "No description available".to_string(),
            HashMap::new(),
        ));
    }

    let content = fs::read_to_string(readme_path)
        .with_context(|| format!("could not read README.md at `{}`", readme_path.display()))?;

    let mut owner = "Unknown".to_string();
    let mut description = "No description available".to_string();
    let mut meta: HashMap<String, serde_json::Value> = HashMap::new();

    // Check if content starts with YAML front matter (---)
    if content.starts_with("---\n") {
        if let Some(end_pos) = content[4..].find("\n---\n") {
            let yaml_content = &content[4..end_pos + 4];
            let markdown_content = content[end_pos + 8..].to_string();

            // Parse YAML front matter
            if let Ok(yaml_value) = serde_yaml::from_str::<serde_yaml::Value>(yaml_content) {
                if let Some(mapping) = yaml_value.as_mapping() {
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
            }

            description = read_readme_content(&markdown_content)
        }
    } else {
        description = read_readme_content(&content)
    }

    Ok((owner, description, meta))
}
