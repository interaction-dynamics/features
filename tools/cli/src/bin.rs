use anyhow::Result;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashSet;

mod build;
mod checker;
mod codeowners;
mod coverage_parser;
mod dependency_resolver;
mod feature_metadata_detector;
mod features_toml_parser;
mod file_scanner;
mod git_helper;
mod http_server;
mod import_detector;
mod models;
mod printer;
mod readme_parser;
mod scan;

use build::{BuildConfig, create_build};
use checker::run_checks;
use codeowners::generate_codeowners;
use http_server::serve_features_with_watching;
use models::Feature;
use printer::print_features;
use scan::{ScanConfig, scan_features};

/// A CLI tool for discovering features in a folder by reading README.md or README.mdx files,
/// and serving them via HTTP or static builds.
#[derive(Parser)]
#[command(name = "features")]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
#[command(arg_required_else_help = true)]
struct Cli {
    /// The path to the directory to list
    #[arg(required = false)]
    path: Option<std::path::PathBuf>,

    /// Print version information
    #[arg(short = 'V', long)]
    version: bool,

    /// Output features as JSON
    #[arg(long)]
    json: bool,

    /// Output features as a flat array instead of nested structure
    #[arg(long)]
    flat: bool,

    /// Include descriptions in the output
    #[arg(long)]
    description: bool,

    /// Display only unique list of owners
    #[arg(long)]
    list_owners: bool,

    /// Run checks on features (e.g., duplicate names)
    #[arg(long)]
    check: bool,

    /// Start an HTTP server to serve the features
    #[arg(long)]
    serve: bool,

    /// Port for the HTTP server (default: 3000)
    #[arg(long, default_value = "3000")]
    port: u16,

    /// Create a static build with embedded files and features.json
    #[arg(long)]
    build: bool,

    /// Output directory for the static build
    #[arg(long, default_value = "build")]
    build_dir: std::path::PathBuf,

    /// Skip computing changes (git commits and decisions) in the output
    #[arg(long)]
    skip_changes: bool,

    /// Find the owner of a specific file or folder
    #[arg(long)]
    find_owner: Option<String>,

    /// Path to the coverage directory (overrides default search)
    #[arg(long)]
    coverage_dir: Option<std::path::PathBuf>,

    /// Include coverage information in the output
    #[arg(long)]
    coverage: bool,

    /// Generate or update CODEOWNERS file
    #[arg(long)]
    generate_codeowners: bool,

    /// Project directory for CODEOWNERS file generation
    #[arg(long)]
    project_dir: Option<std::path::PathBuf>,

    /// Custom path and filename for CODEOWNERS file (default: CODEOWNERS)
    #[arg(long)]
    codeowners_path: Option<std::path::PathBuf>,

    /// Custom prefix for owner names in CODEOWNERS file (default: @)
    #[arg(long, default_value = "@")]
    codeowners_prefix: String,
}

fn flatten_features(features: &[Feature]) -> Vec<Feature> {
    let mut flat_features = Vec::new();

    for feature in features {
        // Create a flattened version of this feature (without nested features)
        let flat_feature = Feature {
            name: feature.name.clone(),
            description: feature.description.clone(),
            owner: feature.owner.clone(),
            is_owner_inherited: feature.is_owner_inherited,
            path: feature.path.clone(),
            features: Vec::new(), // Empty for flat structure
            meta: feature.meta.clone(),
            changes: Vec::new(),
            decisions: Vec::new(),
            stats: feature.stats.clone(),
            dependencies: feature.dependencies.clone(),
        };

        flat_features.push(flat_feature);

        // Recursively flatten nested features
        let nested_flat = flatten_features(&feature.features);
        flat_features.extend(nested_flat);
    }

    flat_features
}

#[derive(Debug, serde::Serialize)]
struct OwnerInfo {
    owner: String,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    inherited: bool,
    feature_name: String,
    feature_path: String,
}

fn find_owner_for_path(
    target_path: &std::path::Path,
    features: &[Feature],
    base_path: &std::path::Path,
) -> Option<OwnerInfo> {
    // Canonicalize the target path
    let canonical_target = std::fs::canonicalize(target_path).ok()?;

    // Helper function to recursively search for the closest feature
    fn find_closest_feature(
        target: &std::path::Path,
        features: &[Feature],
        _parent_owner: Option<&str>,
        base_path: &std::path::Path,
    ) -> Option<OwnerInfo> {
        let mut best_match: Option<OwnerInfo> = None;
        let mut best_match_depth = usize::MAX;

        for feature in features {
            let feature_path = base_path.join(&feature.path);
            if let Ok(canonical_feature_path) = std::fs::canonicalize(&feature_path) {
                // Check if target is within this feature's directory
                if target.starts_with(&canonical_feature_path) {
                    // Calculate depth (how many components between feature and target)
                    let depth = target
                        .strip_prefix(&canonical_feature_path)
                        .ok()
                        .map(|p| p.components().count())
                        .unwrap_or(usize::MAX);

                    // Check nested features first
                    if let Some(nested_match) = find_closest_feature(
                        target,
                        &feature.features,
                        Some(&feature.owner),
                        base_path,
                    ) {
                        if best_match.is_none() || depth < best_match_depth {
                            best_match = Some(nested_match);
                            best_match_depth = depth;
                        }
                    } else {
                        // This is the closest feature so far
                        if depth < best_match_depth {
                            let owner = OwnerInfo {
                                owner: feature.owner.clone(),
                                inherited: feature.is_owner_inherited,
                                feature_name: feature.name.clone(),
                                feature_path: feature.path.clone(),
                            };
                            best_match = Some(owner);
                            best_match_depth = depth;
                        }
                    }
                }
            }
        }

        best_match
    }

    find_closest_feature(&canonical_target, features, None, base_path)
}

/// Add coverage data from .coverage and coverage directories to features
/// Add coverage data to features by searching for coverage reports in multiple locations.
///
/// This function searches for coverage reports in the following priority order:
/// 1. If `coverage_dir_override` is provided, only that directory is checked
/// 2. Otherwise, searches in this order (stops at first directory with coverage data):
///    - `base_path/.coverage`
///    - `base_path/coverage`
///    - `current_dir/.coverage`
///    - `current_dir/coverage`
///    - `project_dir/.coverage` (if project_dir is provided)
///    - `project_dir/coverage` (if project_dir is provided)
///
/// # Arguments
///
/// * `features` - Mutable reference to features that will be updated with coverage data
/// * `base_path` - Base path of the project being analyzed
/// * `coverage_dir_override` - Optional explicit coverage directory (takes precedence over all)
/// * `current_dir` - Current working directory where the CLI is executed
/// * `project_dir` - Optional project directory for additional coverage locations
///
/// # Behavior
///
/// - Stops searching after finding coverage data in the first valid directory
/// - Only processes directories that contain actual coverage files
/// - Updates feature stats with coverage information (line/branch coverage percentages)
fn extract_unique_owners(features: &[Feature]) -> Vec<String> {
    let mut owners_set = HashSet::new();

    fn collect_owners(features: &[Feature], owners_set: &mut HashSet<String>) {
        for feature in features {
            owners_set.insert(feature.owner.clone());
            collect_owners(&feature.features, owners_set);
        }
    }

    collect_owners(features, &mut owners_set);

    let mut owners: Vec<String> = owners_set.into_iter().collect();
    owners.sort(); // Sort alphabetically
    owners
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    // Handle version flag
    if args.version {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // Handle find-owner flag - this requires a target path but not the base path
    if let Some(target_file) = &args.find_owner {
        let target_path = std::path::PathBuf::from(target_file);

        if !target_path.exists() {
            eprintln!("Error: Path '{}' does not exist.", target_file);
            std::process::exit(1);
        }

        // Determine the base path to scan for features
        let base_path = if let Some(p) = &args.path {
            p.clone()
        } else {
            // Use current directory or try to find a suitable base
            std::env::current_dir()?
        };

        let current_dir = std::env::current_dir()?;
        let config = ScanConfig::new(&current_dir).skip_changes(args.skip_changes);

        let features = scan_features(&base_path, config)?;

        match find_owner_for_path(&target_path, &features, &base_path) {
            Some(owner_info) => {
                if args.json {
                    let json = serde_json::to_string_pretty(&owner_info)?;
                    println!("{}", json);
                } else {
                    println!(
                        "Owner: {}{}",
                        owner_info.owner,
                        if owner_info.inherited {
                            " (inherited)"
                        } else {
                            ""
                        }
                    );
                    println!("Feature: {}", owner_info.feature_name);
                    println!("Feature Path: {}", owner_info.feature_path);
                }
            }
            None => {
                eprintln!("No feature found for path: {}", target_file);
                std::process::exit(1);
            }
        }

        return Ok(());
    }

    // If no path is provided and no special flags are used, show help
    let path = match args.path {
        Some(p) => p,
        None => {
            // If we get here, it means no version/help flag was used
            // This shouldn't happen with arg_required_else_help, but just in case
            eprintln!("Error: The path argument is required.");
            eprintln!("Try 'features --help' for more information.");
            std::process::exit(1);
        }
    };

    // Handle serve flag - show spinner while loading
    let spinner = if args.serve {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        pb.set_message(format!(
            "Starting Features CLI v{}...",
            env!("CARGO_PKG_VERSION")
        ));
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    // Build scan configuration
    let current_dir = std::env::current_dir()?;
    let should_add_coverage = args.serve || args.build || args.json || args.coverage;

    let mut config = ScanConfig::new(&current_dir)
        .skip_changes(args.skip_changes)
        .with_coverage(should_add_coverage);

    if let Some(ref coverage_dir) = args.coverage_dir {
        config = config.coverage_dir(coverage_dir);
    }

    if let Some(ref project_dir) = args.project_dir {
        config = config.project_dir(project_dir);
    }

    let features = scan_features(&path, config)?;

    // Generate CODEOWNERS file if requested
    if args.generate_codeowners {
        let output_dir = args.project_dir.as_deref().unwrap_or(&current_dir);
        generate_codeowners(
            &features,
            &path,
            args.project_dir.as_deref(),
            output_dir,
            args.codeowners_path.as_deref(),
            &args.codeowners_prefix,
        )?;
    }

    // Handle main actions - these can be combined with generate-codeowners
    if args.serve {
        if let Some(pb) = spinner {
            pb.set_message("Server is starting...");
            let pb_clone = pb.clone();
            serve_features_with_watching(
                &features,
                args.port,
                path.clone(),
                Some(Box::new(move || {
                    pb_clone.finish_and_clear();
                })),
                args.skip_changes,
            )
            .await?;
        } else {
            serve_features_with_watching(
                &features,
                args.port,
                path.clone(),
                None,
                args.skip_changes,
            )
            .await?;
        }
    } else if args.build {
        let build_config = BuildConfig::new(args.build_dir);
        create_build(&features, build_config, args.skip_changes).await?;
    } else if args.check {
        run_checks(&features)?;
    } else if args.generate_codeowners {
        // If only generate-codeowners flag is set, we've already done the work above
        // No additional output needed
    } else if args.list_owners {
        let unique_owners = extract_unique_owners(&features);

        if args.json {
            let json = serde_json::to_string_pretty(&unique_owners)?;
            println!("{}", json);
        } else {
            eprintln!("Unique owners found in {}:", path.display());
            for owner in unique_owners {
                println!("{}", owner);
            }
        }
    } else {
        let output_features = if args.flat {
            flatten_features(&features)
        } else {
            features
        };

        if args.json {
            let json = serde_json::to_string_pretty(&output_features)?;
            println!("{}", json);
        } else {
            eprintln!("Features found in {}:", path.display());
            if output_features.is_empty() {
                eprintln!("No features found.");
            } else {
                print_features(&output_features, 0, args.description);
            }
        }
    }

    Ok(())
}
