//! Feature keys and links: the first-class fields read from a feature's metadata.
//!
//! The key is the stable identifier external tools use to refer to a feature
//! (e.g. a PostHog event property `feature=checkout`). Links point to the
//! feature in external tools (analytics dashboard, error tracker, design, ...).

use std::collections::BTreeMap;

/// Key used when a folder name has no character that can be kept in a key
const FALLBACK_KEY: &str = "feature";

/// Turn a folder name into a feature key.
///
/// Letters (lowercased), digits, `_` and `-` are kept; any other run of characters
/// becomes a single `-`. Leading and trailing `-` are removed.
pub fn slugify_key(name: &str) -> String {
    let mut key = String::new();
    for c in name.chars() {
        if c.is_alphanumeric() || c == '_' {
            key.extend(c.to_lowercase());
        } else if !key.is_empty() && !key.ends_with('-') {
            key.push('-');
        }
    }
    let key = key.trim_matches('-');
    if key.is_empty() {
        FALLBACK_KEY.to_string()
    } else {
        key.to_string()
    }
}

/// Remove `key` from `meta` and return the feature key: the `key` override when it
/// is a non-empty string, the slugified `folder_name` otherwise.
pub fn take_key(meta: &mut BTreeMap<String, serde_json::Value>, folder_name: &str) -> String {
    let key_override = meta
        .remove("key")
        .and_then(|value| value.as_str().map(|s| s.trim().to_string()))
        .filter(|key| !key.is_empty());

    key_override.unwrap_or_else(|| slugify_key(folder_name))
}

/// Remove `links` from `meta` and return them, keeping only string values.
///
/// When `links` is not a map, it is left in `meta` untouched and no link is returned.
pub fn take_links(meta: &mut BTreeMap<String, serde_json::Value>) -> BTreeMap<String, String> {
    if !meta.get("links").is_some_and(|value| value.is_object()) {
        return BTreeMap::new();
    }

    let Some(serde_json::Value::Object(links)) = meta.remove("links") else {
        return BTreeMap::new();
    };

    links
        .into_iter()
        .filter_map(|(name, url)| url.as_str().map(|url| (name, url.to_string())))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn meta(value: serde_json::Value) -> BTreeMap<String, serde_json::Value> {
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn test_slugify_key() {
        assert_eq!(slugify_key("checkout"), "checkout");
        assert_eq!(slugify_key("Checkout Flow"), "checkout-flow");
        assert_eq!(slugify_key("user_profile"), "user_profile");
        assert_eq!(slugify_key("user-profile"), "user-profile");
        assert_eq!(slugify_key("  Search & Filters!  "), "search-filters");
        assert_eq!(slugify_key("Équipe"), "équipe");
        assert_eq!(slugify_key("v2.api"), "v2-api");
        assert_eq!(slugify_key("---"), "feature");
        assert_eq!(slugify_key(""), "feature");
    }

    #[test]
    fn test_take_key_uses_the_override() {
        let mut m = meta(json!({ "key": " checkout-v2 ", "status": "active" }));
        assert_eq!(take_key(&mut m, "checkout"), "checkout-v2");
        assert!(!m.contains_key("key"));
        assert!(m.contains_key("status"));
    }

    #[test]
    fn test_take_key_ignores_invalid_overrides() {
        let mut m = meta(json!({ "key": "  " }));
        assert_eq!(take_key(&mut m, "Checkout Flow"), "checkout-flow");
        assert!(!m.contains_key("key"));

        let mut m = meta(json!({ "key": 42 }));
        assert_eq!(take_key(&mut m, "search"), "search");

        let mut m = meta(json!({}));
        assert_eq!(take_key(&mut m, "search"), "search");
    }

    #[test]
    fn test_take_links_keeps_string_values() {
        let mut m = meta(json!({
            "links": {
                "posthog": "https://eu.posthog.com/project/1",
                "broken": 42,
            },
            "status": "active",
        }));
        let links = take_links(&mut m);
        assert_eq!(links.len(), 1);
        assert_eq!(links["posthog"], "https://eu.posthog.com/project/1");
        assert!(!m.contains_key("links"));
        assert!(m.contains_key("status"));
    }

    #[test]
    fn test_take_links_leaves_non_map_values_in_meta() {
        let mut m = meta(json!({ "links": "https://example.com" }));
        assert!(take_links(&mut m).is_empty());
        assert_eq!(m["links"], json!("https://example.com"));
    }
}
