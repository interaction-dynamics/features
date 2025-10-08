use crate::models::Feature;
use colored::*;

pub fn print_features(features: &[Feature], indent: usize, show_description: bool) {
    let prefix = "  ".repeat(indent);

    for feature in features {
        let is_deprecated = feature
            .meta
            .get("deprecated")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let feature_name = if is_deprecated {
            feature.name.truecolor(255, 165, 0).bold()
        } else {
            feature.name.bold()
        };

        println!(
            "{}{} {} -> {}",
            prefix,
            feature_name,
            format!("[{}]", feature.owner).blue(),
            feature.path.dimmed()
        );
        if show_description {
            println!("{}Description: {}", prefix, feature.description);
        }

        // Recursively print nested features
        if !feature.features.is_empty() {
            print_features(&feature.features, indent + 1, show_description);
        }
    }
}
