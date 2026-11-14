//! Feature metadata annotations (`--feature-<key>` comments) found anywhere in
//! the codebase, merged into each feature's `meta`.

use super::for_each_feature_mut;
use crate::feature_metadata_detector::FeatureMetadataMap;
use crate::models::Feature;

/// Merge the annotations matched to each feature (by its path) into its `meta`,
/// appending to any array already set from the README or FEATURES.toml
pub(super) fn apply_feature_metadata(
    features: &mut [Feature],
    feature_metadata_map: &FeatureMetadataMap,
) {
    for_each_feature_mut(features, &mut |feature| {
        let Some(metadata_map) = feature_metadata_map.get(&feature.path) else {
            return;
        };

        // Iterate through each metadata key (e.g., "feature-flag", "feature-experiment")
        for (metadata_key, flags) in metadata_map {
            // Convert Vec<HashMap<String, String>> to JSON array
            let flags_json: Vec<serde_json::Value> = flags
                .iter()
                .map(|flag_map| {
                    let json_map: serde_json::Map<String, serde_json::Value> = flag_map
                        .iter()
                        .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                        .collect();
                    serde_json::Value::Object(json_map)
                })
                .collect();

            // Check if this metadata key already exists, append if it does
            feature
                .meta
                .entry(metadata_key.clone())
                .and_modify(|existing| {
                    if let serde_json::Value::Array(arr) = existing {
                        arr.extend(flags_json.clone());
                    }
                })
                .or_insert_with(|| serde_json::Value::Array(flags_json));
        }
    });
}
