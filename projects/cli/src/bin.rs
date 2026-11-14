use anyhow::Result;
use clap::{Parser, ValueEnum};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashSet;

mod build;
mod checker;
mod codeowners;
mod coverage_parser;
mod dependency_resolver;
mod diff;
mod feature_metadata_detector;
mod features_toml_parser;
mod file_scanner;
mod git_helper;
mod http_server;
mod import_detector;
mod models;
mod ownership;
mod printer;
mod readme_parser;
mod scan;
mod skip_dirs;

use build::{BuildConfig, create_build};
use checker::run_checks;
use codeowners::generate_codeowners;
use http_server::serve_features_with_watching;
use models::Feature;
use ownership::find_owner_for_path;
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

    /// Ignore a folder and all its subfolders when scanning for features (can be repeated)
    #[arg(long)]
    ignore_path: Vec<std::path::PathBuf>,

    /// Also scan folders skipped by default (node_modules) for features, stats and imports
    #[arg(long)]
    include_skipped_dirs: bool,

    /// Diff mode: list the features touched since the merge base of this git ref and HEAD,
    /// including uncommitted changes (e.g. --since origin/master)
    #[arg(
        long,
        value_name = "REF",
        conflicts_with_all = ["serve", "build", "check", "generate_codeowners", "list_owners", "find_owner"]
    )]
    since: Option<String>,

    /// With --since: only compare committed changes, ignoring the working tree
    #[arg(long, requires = "since")]
    committed_only: bool,

    /// With --since: output format (text, json or markdown). --json is the same as --format json
    #[arg(long, value_enum, requires = "since")]
    format: Option<DiffFormat>,

    /// With --since: print only the unique owners of the touched features, one per line,
    /// with --codeowners-prefix added
    #[arg(long, requires = "since", conflicts_with = "format")]
    owners_only: bool,
}

/// Output formats for diff mode (`--since`)
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
enum DiffFormat {
    Text,
    Json,
    Markdown,
}

/// Run diff mode: print the features touched since `since`.
fn run_diff(args: &Cli, since: &str) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let base_path = args.path.clone().unwrap_or_else(|| current_dir.clone());
    let ignore_paths = resolve_ignore_paths(&args.ignore_path, &current_dir);
    let config = ScanConfig::new(&current_dir)
        .skip_changes(true)
        .ignore_paths(ignore_paths)
        .ownership_only(true)
        .include_skipped_dirs(args.include_skipped_dirs);
    let features = scan_features(&base_path, config)?;

    let include_uncommitted = !args.committed_only;
    let changes = diff::collect_changes(&base_path, since, include_uncommitted)?;
    let report = diff::build_report(since, include_uncommitted, &changes, &features);

    if args.owners_only {
        for owner in diff::unique_owners(&report, &args.codeowners_prefix) {
            println!("{}", owner);
        }
        return Ok(());
    }

    let format = match (args.format, args.json) {
        (Some(format), _) => format,
        (None, true) => DiffFormat::Json,
        (None, false) => DiffFormat::Text,
    };
    match format {
        DiffFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        DiffFormat::Markdown => print!(
            "{}",
            diff::render_markdown(&report, &args.codeowners_prefix)
        ),
        DiffFormat::Text => print!("{}", diff::render_text(&report)),
    }
    Ok(())
}

/// Resolve `--ignore-path` values to canonical absolute paths.
///
/// Relative paths are resolved against `current_dir`. Paths that don't exist are
/// skipped with a warning rather than causing a hard error.
fn resolve_ignore_paths(
    ignore_paths: &[std::path::PathBuf],
    current_dir: &std::path::Path,
) -> Vec<std::path::PathBuf> {
    ignore_paths
        .iter()
        .filter_map(|raw_path| {
            let candidate = if raw_path.is_absolute() {
                raw_path.clone()
            } else {
                current_dir.join(raw_path)
            };

            match std::fs::canonicalize(&candidate) {
                Ok(canonical) => Some(canonical),
                Err(_) => {
                    eprintln!(
                        "Warning: --ignore-path '{}' does not exist, skipping.",
                        raw_path.display()
                    );
                    None
                }
            }
        })
        .collect()
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

    // Handle diff mode - the path defaults to the current directory
    if let Some(since) = &args.since {
        return run_diff(&args, since);
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
        let ignore_paths = resolve_ignore_paths(&args.ignore_path, &current_dir);
        let config = ScanConfig::new(&current_dir)
            .skip_changes(args.skip_changes)
            .ignore_paths(ignore_paths)
            .ownership_only(true)
            .include_skipped_dirs(args.include_skipped_dirs);

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
    let ignore_paths = resolve_ignore_paths(&args.ignore_path, &current_dir);

    // CODEOWNERS only needs ownership; skip the expensive parts of the scan
    // unless another action that uses the full feature data is requested.
    let ownership_only = args.generate_codeowners && !args.serve && !args.build && !args.check;

    let mut config = ScanConfig::new(&current_dir)
        .skip_changes(args.skip_changes)
        .with_coverage(should_add_coverage)
        .ignore_paths(ignore_paths.clone())
        .ownership_only(ownership_only)
        .include_skipped_dirs(args.include_skipped_dirs);

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
                ignore_paths.clone(),
                args.include_skipped_dirs,
            )
            .await?;
        } else {
            serve_features_with_watching(
                &features,
                args.port,
                path.clone(),
                None,
                args.skip_changes,
                ignore_paths.clone(),
                args.include_skipped_dirs,
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
