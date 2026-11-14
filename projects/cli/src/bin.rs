use anyhow::Result;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use features_cli::aggregate;
use features_cli::build::{BuildConfig, create_build, write_records_json};
use features_cli::checker::run_checks;
use features_cli::cli::{
    Action, AggregateArgs, Cli, CodeownersOptions, DiffArgs, Invocation, ScanArgs,
};
use features_cli::codeowners::generate_codeowners;
use features_cli::diff;
use features_cli::http_server::{ServerConfig, serve_features_with_watching};
use features_cli::models::{Feature, Record};
use features_cli::ownership::find_owner_for_path;
use features_cli::printer::print_features;
use features_cli::records::{create_record, project_records};
use features_cli::scan::{ScanConfig, scan_features_with};

/// Resolve `--ignore-path` values to canonical absolute paths.
///
/// Relative paths are resolved against `current_dir`. Paths that don't exist are
/// skipped with a warning rather than causing a hard error.
fn resolve_ignore_paths(ignore_paths: &[PathBuf], current_dir: &Path) -> Vec<PathBuf> {
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
            records: Vec::new(),
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
    let invocation = match Cli::parse().into_invocation() {
        Ok(invocation) => invocation,
        Err(err) => err.exit(),
    };
    run(invocation).await
}

/// Scan configuration from the scan options shared by every action
fn scan_config(scan: &ScanArgs, current_dir: &Path) -> ScanConfig {
    let mut config = ScanConfig::new(current_dir)
        .skip_changes(scan.skip_changes)
        .ignore_paths(resolve_ignore_paths(&scan.ignore_path, current_dir))
        .include_skipped_dirs(scan.include_skipped_dirs)
        .strip_emails(scan.strip_emails);

    if let Some(coverage_dir) = &scan.coverage_dir {
        config = config.coverage_dir(coverage_dir);
    }
    if let Some(project_dir) = &scan.project_dir {
        config = config.project_dir(project_dir);
    }

    config
}

fn serve_spinner() -> ProgressBar {
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
    pb
}

async fn run(invocation: Invocation) -> Result<()> {
    let Invocation {
        action,
        scan,
        codeowners,
    } = invocation;
    let current_dir = std::env::current_dir()?;

    // Scan the features, then generate CODEOWNERS if requested alongside the action
    let scan_path = |path: &Path, config: &ScanConfig| -> Result<Vec<Feature>> {
        let features = scan_features_with(path, config)?;
        if let Some(options) = &codeowners {
            write_codeowners(&features, path, &scan, options, &current_dir)?;
        }
        Ok(features)
    };

    match action {
        Action::Version => println!("{}", env!("CARGO_PKG_VERSION")),

        Action::FindOwner { target, path, json } => {
            let base_path = path.unwrap_or_else(|| current_dir.clone());
            find_owner(&target, &base_path, &scan, &current_dir, json)?;
        }

        Action::List(args) => {
            let config = scan_config(&scan, &current_dir).with_coverage(args.json || args.coverage);
            let features = scan_path(&args.path, &config)?;

            let output_features = if args.flat {
                flatten_features(&features)
            } else {
                features
            };

            if args.json {
                let json = serde_json::to_string_pretty(&output_features)?;
                println!("{}", json);
            } else {
                eprintln!("Features found in {}:", args.path.display());
                if output_features.is_empty() {
                    eprintln!("No features found.");
                } else {
                    print_features(&output_features, 0, args.description);
                }
            }
        }

        Action::Serve(args) => {
            let spinner = serve_spinner();
            let config = scan_config(&scan, &current_dir).with_coverage(true);
            let features = scan_path(&args.path, &config)?;

            spinner.set_message("Server is starting...");
            let server_config =
                ServerConfig::new(args.port).with_allowed_origins(args.allow_origin);
            serve_features_with_watching(
                &features,
                server_config,
                args.path,
                Some(Box::new(move || spinner.finish_and_clear())),
                config,
            )
            .await?;
        }

        Action::Build(args) => {
            let config = scan_config(&scan, &current_dir).with_coverage(true);
            let features = scan_path(&args.path, &config)?;
            let records = project_records(&features, &args.path);
            create_build(
                &features,
                BuildConfig::new(args.build_dir.clone()),
                scan.skip_changes,
            )
            .await?;
            write_records_json(&records, &args.build_dir).await?;
        }

        Action::Check { path } => {
            let features = scan_path(&path, &scan_config(&scan, &current_dir))?;
            run_checks(&features, &project_records(&features, &path))?;
        }

        Action::Records { path, json } => {
            let config = scan_config(&scan, &current_dir).skip_changes(true);
            let features = scan_path(&path, &config)?;
            let records = project_records(&features, &path);
            if json {
                let json =
                    serde_json::to_string_pretty(&serde_json::json!({ "records": records }))?;
                println!("{}", json);
            } else {
                print_records(&features, &records);
            }
        }

        Action::New(args) => {
            let config = scan_config(&scan, &current_dir).ownership_only(true);
            let features = scan_path(&args.path, &config)?;
            let today = chrono::Local::now().format("%Y-%m-%d").to_string();
            let created = create_record(
                &args.path,
                &features,
                args.kind.into(),
                &args.feature,
                &args.title,
                &today,
            )?;
            println!("{}", created.display());
        }

        Action::Codeowners { path } => {
            // CODEOWNERS only needs ownership: skip the expensive parts of the scan
            let config = scan_config(&scan, &current_dir).ownership_only(true);
            scan_path(&path, &config)?;
        }

        Action::Diff(args) => run_diff(&args, &scan, &current_dir)?,

        Action::Aggregate(args) => run_aggregate(args).await?,

        Action::Owners { path, json } => {
            let config = scan_config(&scan, &current_dir).ownership_only(true);
            let features = scan_path(&path, &config)?;
            let unique_owners = extract_unique_owners(&features);

            if json {
                let json = serde_json::to_string_pretty(&unique_owners)?;
                println!("{}", json);
            } else {
                eprintln!("Unique owners found in {}:", path.display());
                for owner in unique_owners {
                    println!("{}", owner);
                }
            }
        }
    }

    Ok(())
}

/// Print the project-level records, then the records of every feature
fn print_records(features: &[Feature], project: &[Record]) {
    fn print_list(records: &[Record]) {
        for record in records {
            let mut details: Vec<String> = Vec::new();
            if let Some(status) = &record.status {
                details.push(status.clone());
            }
            if let Some(date) = &record.date {
                details.push(date.clone());
            }
            if let Some(tasks) = record.tasks {
                details.push(format!("{}/{} tasks", tasks.done, tasks.total));
            }
            let details = if details.is_empty() {
                String::new()
            } else {
                format!(" ({})", details.join(", "))
            };
            println!(
                "  [{}] {}: {}{}",
                record.kind.label(),
                record.id,
                record.title,
                details
            );
        }
    }

    fn visit(features: &[Feature], found: &mut bool) {
        for feature in features {
            if !feature.records.is_empty() {
                *found = true;
                println!("{} ({}):", feature.name, feature.path);
                print_list(&feature.records);
            }
            visit(&feature.features, found);
        }
    }

    let mut found = !project.is_empty();
    if found {
        println!("Project:");
        print_list(project);
    }
    visit(features, &mut found);
    if !found {
        eprintln!("No specs or decision records found.");
    }
}

fn write_codeowners(
    features: &[Feature],
    base_path: &Path,
    scan: &ScanArgs,
    options: &CodeownersOptions,
    current_dir: &Path,
) -> Result<()> {
    let output_dir = scan.project_dir.as_deref().unwrap_or(current_dir);
    generate_codeowners(
        features,
        base_path,
        scan.project_dir.as_deref(),
        output_dir,
        options.codeowners_path.as_deref(),
        &options.codeowners_prefix,
    )
}

/// Aggregate the features reports of several repositories
async fn run_aggregate(args: AggregateArgs) -> Result<()> {
    let report = aggregate::aggregate_folder(&args.folder)?;

    eprintln!(
        "Aggregated {} repositories: {}",
        report.repositories.len(),
        report.repositories.join(", ")
    );
    if !report.errors.is_empty() {
        eprintln!("{} reconciliation issue(s):", report.errors.len());
        for error in &report.errors {
            eprintln!("  - {}", error.message);
        }
    }

    if args.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        create_build(&report, BuildConfig::new(args.build_dir), false).await?;
    }

    Ok(())
}

/// Diff mode: print the features touched since `args.since`.
fn run_diff(args: &DiffArgs, scan: &ScanArgs, current_dir: &Path) -> Result<()> {
    let base_path = args
        .path
        .clone()
        .unwrap_or_else(|| current_dir.to_path_buf());
    let config = scan_config(scan, current_dir).ownership_only(true);
    let features = scan_features_with(&base_path, &config)?;

    let include_uncommitted = !args.committed_only;
    let changes = diff::collect_changes(&base_path, &args.since, include_uncommitted)?;
    let report = diff::build_report(&args.since, include_uncommitted, &changes, &features);

    if args.owners_only {
        for owner in diff::unique_owners(&report, &args.codeowners_prefix) {
            println!("{}", owner);
        }
    } else if args.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else if args.markdown {
        print!(
            "{}",
            diff::render_markdown(&report, &args.codeowners_prefix)
        );
    } else {
        print!("{}", diff::render_text(&report));
    }
    Ok(())
}

fn find_owner(
    target: &Path,
    base_path: &Path,
    scan: &ScanArgs,
    current_dir: &Path,
    json: bool,
) -> Result<()> {
    if !target.exists() {
        eprintln!("Error: Path '{}' does not exist.", target.display());
        std::process::exit(1);
    }

    let config = scan_config(scan, current_dir).ownership_only(true);
    let features = scan_features_with(base_path, &config)?;

    match find_owner_for_path(target, &features, base_path) {
        Some(owner_info) => {
            if json {
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
            eprintln!("No feature found for path: {}", target.display());
            std::process::exit(1);
        }
    }

    Ok(())
}
