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

        // Display coverage stats if available
        if let Some(stats) = &feature.stats
            && let Some(coverage) = &stats.coverage
        {
            let coverage_color = if coverage.line_coverage_percent >= 80.0 {
                "green"
            } else if coverage.line_coverage_percent >= 60.0 {
                "yellow"
            } else {
                "red"
            };

            let coverage_str = format!(
                "  {}Coverage: {:.1}% lines ({}/{})",
                prefix,
                coverage.line_coverage_percent,
                coverage.lines_covered,
                coverage.lines_total
            );

            println!("{}", coverage_str.color(coverage_color));

            if let Some(branch_percent) = coverage.branch_coverage_percent {
                let branch_str = format!(
                    "  {}         {:.1}% branches ({}/{})",
                    prefix,
                    branch_percent,
                    coverage.branches_covered.unwrap_or(0),
                    coverage.branches_total.unwrap_or(0)
                );
                println!("{}", branch_str.color(coverage_color));
            }
        }

        if show_description {
            println!("{}Description: {}", prefix, feature.description);
        }

        // Recursively print nested features
        if !feature.features.is_empty() {
            print_features(&feature.features, indent + 1, show_description);
        }
    }
}
