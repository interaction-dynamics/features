//! Build module for creating static file builds from embedded resources.
//!
//! This module provides functionality to extract embedded static files and features data
//! to a build directory, creating a complete static website that can be deployed.

use anyhow::Result;
use include_dir::{Dir, include_dir};
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::models::Feature;

// Embed the public directory at compile time
static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/public");

/// Configuration for the build process
#[derive(Debug, Clone)]
pub struct BuildConfig {
    /// Output directory for the build
    pub output_dir: PathBuf,
    /// Whether to clean the output directory before building
    pub clean: bool,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("build"),
            clean: true,
        }
    }
}

impl BuildConfig {
    /// Create a new build configuration with custom output directory
    pub fn new<P: Into<PathBuf>>(output_dir: P) -> Self {
        Self {
            output_dir: output_dir.into(),
            ..Default::default()
        }
    }

    /// Set whether to clean the output directory before building
    #[allow(dead_code)]
    pub fn with_clean(mut self, clean: bool) -> Self {
        self.clean = clean;
        self
    }
}

/// Creates a static build by extracting embedded files and generating features.json
///
/// # Arguments
///
/// * `features` - Slice of Feature objects to include in the build
/// * `config` - Build configuration
///
/// # Returns
///
/// * `Result<()>` - Ok if build succeeds, Err otherwise
///
/// # Build Output
///
/// The build directory will contain:
/// * All static files from the embedded public directory
/// * `features.json` - Generated features data
///
/// # Example
///
/// ```rust,no_run
/// use your_crate::build::{create_build, BuildConfig};
/// use your_crate::models::Feature;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let features = vec![]; // Your features data
///     let config = BuildConfig::new("dist");
///     create_build(&features, config).await
/// }
/// ```
pub async fn create_build(features: &[Feature], config: BuildConfig) -> Result<()> {
    println!(
        "Creating build in directory: {}",
        config.output_dir.display()
    );

    // Clean output directory if requested
    if config.clean && config.output_dir.exists() {
        println!("Cleaning existing build directory...");
        fs::remove_dir_all(&config.output_dir).await?;
    }

    // Create output directory
    fs::create_dir_all(&config.output_dir).await?;

    // Extract all embedded static files
    extract_embedded_files(&config.output_dir).await?;

    // Generate features.json
    generate_features_json(features, &config.output_dir).await?;

    println!("Build completed successfully!");
    println!("Output directory: {}", config.output_dir.display());

    Ok(())
}

/// Extracts all embedded static files to the output directory
async fn extract_embedded_files(output_dir: &Path) -> Result<()> {
    println!("Extracting embedded static files...");

    extract_dir_recursive(&STATIC_DIR, output_dir, "").await?;

    Ok(())
}

/// Recursively extracts files from an embedded directory
fn extract_dir_recursive<'a>(
    dir: &'a Dir<'a>,
    output_base: &'a Path,
    relative_path: &'a str,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
    Box::pin(async move {
        // Create the current directory in output
        let current_output_dir = output_base.join(relative_path);
        if !relative_path.is_empty() {
            fs::create_dir_all(&current_output_dir).await?;
        }

        // Extract all files in the current directory
        for file in dir.files() {
            let file_path = current_output_dir.join(file.path().file_name().unwrap());
            println!("  Extracting: {}", file_path.display());

            fs::write(&file_path, file.contents()).await?;
        }

        // Recursively extract subdirectories
        for subdir in dir.dirs() {
            let subdir_name = subdir.path().file_name().unwrap().to_string_lossy();
            let new_relative_path = if relative_path.is_empty() {
                subdir_name.to_string()
            } else {
                format!("{}/{}", relative_path, subdir_name)
            };

            extract_dir_recursive(subdir, output_base, &new_relative_path).await?;
        }

        Ok(())
    })
}

/// Generates the features.json file
async fn generate_features_json(features: &[Feature], output_dir: &Path) -> Result<()> {
    println!("Generating features.json...");

    let features_json = serde_json::to_string_pretty(features)
        .map_err(|e| anyhow::anyhow!("Failed to serialize features to JSON: {}", e))?;

    let features_path = output_dir.join("features.json");
    fs::write(&features_path, features_json).await?;

    println!("  Created: {}", features_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_create_build() {
        let temp_dir = tempdir().unwrap();
        let config = BuildConfig::new(temp_dir.path().join("test_build"));
        let features = vec![];

        let result = create_build(&features, config).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_build_files() {
        let files = list_build_files();
        assert!(files.contains(&"features.json".to_string()));
    }
}
