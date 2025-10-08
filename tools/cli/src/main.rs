use anyhow::Result;
use clap::Parser;
use std::collections::HashSet;

mod checker;
mod file_scanner;
mod models;
mod printer;
mod readme_parser;

use checker::run_checks;
use file_scanner::list_files_recursive;
use models::Feature;
use printer::print_features;

/// Recursively list all files in a directory.
#[derive(Parser)]
struct Cli {
    /// The path to the directory to list
    path: std::path::PathBuf,

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

fn main() -> Result<()> {
    let args = Cli::parse();

    let features = list_files_recursive(&args.path)?;

    if args.check {
        run_checks(&features)?;
    } else if args.list_owners {
        let unique_owners = extract_unique_owners(&features);

        if args.json {
            let json = serde_json::to_string_pretty(&unique_owners)?;
            println!("{}", json);
        } else {
            eprintln!("Unique owners found in {}:", args.path.display());
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
            eprintln!("Features found in {}:", args.path.display());
            if output_features.is_empty() {
                eprintln!("No features found.");
            } else {
                print_features(&output_features, 0, args.description);
            }
        }
    }

    Ok(())
}
