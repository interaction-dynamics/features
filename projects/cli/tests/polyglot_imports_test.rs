//! Dependencies between features found from the imports of every supported
//! language, read from `examples/polyglot-imports`

use features_cli::models::Feature;
use features_cli::scan::{ScanConfig, scan_features};
use std::path::PathBuf;

fn scan() -> Vec<Feature> {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/polyglot-imports/src");
    scan_features(&path, ScanConfig::new(&path).skip_changes(true)).unwrap()
}

fn find<'a>(features: &'a [Feature], name: &str) -> &'a Feature {
    fn walk<'a>(features: &'a [Feature], name: &str) -> Option<&'a Feature> {
        features.iter().find_map(|f| {
            (f.name == name)
                .then_some(f)
                .or_else(|| walk(&f.features, name))
        })
    }
    walk(features, name).unwrap_or_else(|| panic!("feature {name} not found"))
}

/// The only dependency of a feature: (source file, target file, target feature path, type)
fn single_dependency(features: &[Feature], name: &str) -> (String, String, String, String) {
    let feature = find(features, name);
    assert_eq!(
        feature.dependencies.len(),
        1,
        "{name}: {:#?}",
        feature.dependencies
    );
    let dep = &feature.dependencies[0];
    (
        dep.source_filename.clone(),
        dep.target_filename.clone(),
        dep.feature_path.clone(),
        format!("{:?}", dep.dependency_type),
    )
}

#[test]
fn test_ruby_require_relative() {
    let features = scan();
    assert_eq!(
        single_dependency(&features, "Checkout"),
        (
            "features/checkout/cart.rb".into(),
            "features/payments/gateway.rb".into(),
            "features/payments".into(),
            "Sibling".into()
        )
    );
}

#[test]
fn test_php_require_once() {
    let features = scan();
    assert_eq!(
        single_dependency(&features, "Payments"),
        (
            "features/payments/charge.php".into(),
            "features/ledger/ledger.php".into(),
            "features/ledger".into(),
            "Sibling".into()
        )
    );
}

#[test]
fn test_shell_source() {
    let features = scan();
    assert_eq!(
        single_dependency(&features, "Ledger"),
        (
            "features/ledger/export.sh".into(),
            "features/reporting/report.sh".into(),
            "features/reporting".into(),
            "Sibling".into()
        )
    );
}

#[test]
fn test_c_local_include() {
    let features = scan();
    assert_eq!(
        single_dependency(&features, "Reporting"),
        (
            "features/reporting/chart.c".into(),
            "features/theme/colors.h".into(),
            "features/theme".into(),
            "Sibling".into()
        )
    );
}

#[test]
fn test_css_import() {
    let features = scan();
    assert_eq!(
        single_dependency(&features, "Theme"),
        (
            "features/theme/theme.css".into(),
            "features/checkout/cart.css".into(),
            "features/checkout".into(),
            "Sibling".into()
        )
    );
}

#[test]
fn test_cpp_include_of_nested_feature_is_a_child_dependency() {
    let features = scan();
    assert_eq!(
        single_dependency(&features, "Catalog"),
        (
            "features/catalog/loader.cpp".into(),
            "features/catalog/features/search/search.h".into(),
            "features/catalog/features/search".into(),
            "Child".into()
        )
    );
}

#[test]
fn test_shell_dot_source_of_parent_feature_is_a_parent_dependency() {
    let features = scan();
    assert_eq!(
        single_dependency(&features, "Search"),
        (
            "features/catalog/features/search/index.sh".into(),
            "features/catalog/catalog.sh".into(),
            "features/catalog".into(),
            "Parent".into()
        )
    );
}

#[test]
fn test_external_package_imports_are_not_dependencies() {
    // Java, Kotlin, C#, Go and Python imports of packages outside the project
    let features = scan();
    let orders = find(&features, "Orders");
    assert!(orders.dependencies.is_empty(), "{:#?}", orders.dependencies);
}
