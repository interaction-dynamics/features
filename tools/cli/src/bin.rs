use anyhow::Result;
use clap::Parser;
use std::collections::HashSet;

mod build;
mod checker;
mod file_scanner;
mod git_helper;
mod http_server;
mod models;
mod printer;
mod readme_parser;

use build::{BuildConfig, create_build};
use checker::run_checks;
use file_scanner::{list_files_recursive, list_files_recursive_with_changes};
use http_server::serve_features_with_watching;
use models::Feature;
use printer::print_features;

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
}

fn flatten_features(features: &[Feature]) -> Vec<Feature> {
    let mut flat_features = Vec::new();

    for feature in features {
        // Create a flattened version of this feature (without nested features)
        let flat_feature = Feature {
            name: feature.name.clone(),
            description: feature.description.clone(),
            owner: feature.owner.clone(),
            path: feature.path.clone(),
            features: Vec::new(), // Empty for flat structure
            meta: feature.meta.clone(),
            changes: Vec::new(),
            decisions: Vec::new(),
            stats: feature.stats.clone(),
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

fn find_owner_for_path(target_path: &std::path::Path, features: &[Feature]) -> Option<OwnerInfo> {
    use std::path::PathBuf;

    // Canonicalize the target path
    let canonical_target = std::fs::canonicalize(target_path).ok()?;

    // Helper function to recursively search for the closest feature
    fn find_closest_feature(
        target: &std::path::Path,
        features: &[Feature],
        parent_owner: Option<&str>,
    ) -> Option<OwnerInfo> {
        let mut best_match: Option<OwnerInfo> = None;
        let mut best_match_depth = usize::MAX;

        for feature in features {
            let feature_path = PathBuf::from(&feature.path);
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
                    if let Some(nested_match) =
                        find_closest_feature(target, &feature.features, Some(&feature.owner))
                    {
                        if best_match.is_none() || depth < best_match_depth {
                            best_match = Some(nested_match);
                            best_match_depth = depth;
                        }
                    } else {
                        // This is the closest feature so far
                        if depth < best_match_depth {
                            let owner = if feature.owner == "Unknown" {
                                // Try to inherit from parent
                                if let Some(parent) = parent_owner {
                                    if parent != "Unknown" {
                                        OwnerInfo {
                                            owner: parent.to_string(),
                                            inherited: true,
                                            feature_name: feature.name.clone(),
                                            feature_path: feature.path.clone(),
                                        }
                                    } else {
                                        OwnerInfo {
                                            owner: "Unknown".to_string(),
                                            inherited: false,
                                            feature_name: feature.name.clone(),
                                            feature_path: feature.path.clone(),
                                        }
                                    }
                                } else {
                                    OwnerInfo {
                                        owner: "Unknown".to_string(),
                                        inherited: false,
                                        feature_name: feature.name.clone(),
                                        feature_path: feature.path.clone(),
                                    }
                                }
                            } else {
                                OwnerInfo {
                                    owner: feature.owner.clone(),
                                    inherited: false,
                                    feature_name: feature.name.clone(),
                                    feature_path: feature.path.clone(),
                                }
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

    find_closest_feature(&canonical_target, features, None)
}

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

        let features = if args.skip_changes {
            list_files_recursive(&base_path)?
        } else {
            list_files_recursive_with_changes(&base_path)?
        };

        match find_owner_for_path(&target_path, &features) {
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

    let features = if args.skip_changes {
        list_files_recursive(&path)?
    } else {
        list_files_recursive_with_changes(&path)?
    };

    if args.serve {
        eprintln!("Watching directory: {}", path.display());
        serve_features_with_watching(&features, args.port, path.clone()).await?;
    } else if args.build {
        let build_config = BuildConfig::new(args.build_dir);
        create_build(&features, build_config).await?;
    } else if args.check {
        run_checks(&features)?;
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
