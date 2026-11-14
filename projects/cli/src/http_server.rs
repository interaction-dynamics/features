//! HTTP server module for serving features data and embedded static files.
//!
//! This module provides functionality to start an HTTP server that serves:
//! - Features data as JSON at `/features.json`
//! - Project-level specs and decisions at `/records.json` (`{ "records": [...] }`)
//! - Embedded static files from the compiled binary
//! - A default index page at the root path
//!
//! The server uses the `warp` web framework. It only answers requests whose
//! `Host` header names the loopback interface (protection against DNS
//! rebinding) and sends no CORS headers unless origins are explicitly allowed
//! with `--allow-origin`, so other websites cannot read the features data.
//! Static files are embedded at compile time using the `include_dir` crate.

use anyhow::Result;
use include_dir::{Dir, include_dir};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::build::{current_build_timestamp, records_json};
use crate::file_watcher::{self, MAX_WAIT, QUIET_PERIOD, WatchFilter};
use crate::git_helper::get_repository_url;
use crate::scan::{ScanConfig, ScanOutput, scan_project_with};

// Embed the public directory at compile time
static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/public");

/// Configuration for the HTTP server
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Port to run the server on
    pub port: u16,
    /// Host address to bind to
    pub host: [u8; 4],
    /// Origins allowed to read the server's responses cross-origin (CORS).
    /// Empty means no CORS headers are sent. `*` allows any origin.
    pub allowed_origins: Vec<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            host: [127, 0, 0, 1],
            allowed_origins: Vec::new(),
        }
    }
}

impl ServerConfig {
    /// Create a new server configuration with custom port
    pub fn new(port: u16) -> Self {
        Self {
            port,
            ..Default::default()
        }
    }

    /// Set the host address to bind to
    #[allow(dead_code)]
    pub fn with_host(mut self, host: [u8; 4]) -> Self {
        self.host = host;
        self
    }

    /// Set the origins allowed to make cross-origin requests
    pub fn with_allowed_origins(mut self, origins: Vec<String>) -> Self {
        self.allowed_origins = origins;
        self
    }
}

/// Validates and normalizes a value passed to `--allow-origin`.
///
/// Accepts `*` or an origin of the form `http(s)://host[:port]` (a trailing
/// slash is dropped). Paths, queries, fragments and credentials are rejected.
pub fn parse_allowed_origin(value: &str) -> std::result::Result<String, String> {
    let value = value.trim();
    if value == "*" {
        return Ok(value.to_string());
    }

    let normalized = value.strip_suffix('/').unwrap_or(value);
    let authority = normalized
        .strip_prefix("http://")
        .or_else(|| normalized.strip_prefix("https://"))
        .ok_or_else(|| format!("invalid origin '{value}': must start with http:// or https://"))?;

    if authority.is_empty()
        || authority.contains(['/', '?', '#', '@'])
        || authority.chars().any(char::is_whitespace)
        || authority.parse::<warp::http::uri::Authority>().is_err()
    {
        return Err(format!(
            "invalid origin '{value}': expected scheme://host[:port], e.g. http://localhost:5173"
        ));
    }

    Ok(normalized.to_string())
}

/// Returns true if a `Host` header value names the loopback interface:
/// `localhost`, `127.0.0.1` or `[::1]`, with or without a port.
fn is_allowed_host(host: &str) -> bool {
    let (name, port) = if let Some(rest) = host.strip_prefix('[') {
        let Some((name, after)) = rest.split_once(']') else {
            return false;
        };
        let port = match after {
            "" => None,
            after => match after.strip_prefix(':') {
                Some(digits) => Some(digits),
                None => return false,
            },
        };
        (name, port)
    } else {
        match host.rsplit_once(':') {
            Some((name, digits)) => (name, Some(digits)),
            None => (host, None),
        }
    };

    let port_ok =
        port.is_none_or(|digits| !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()));
    let name_ok = name.eq_ignore_ascii_case("localhost") || name == "127.0.0.1" || name == "::1";

    port_ok && name_ok
}

/// Rejection for requests whose `Host` header is not a loopback name
#[derive(Debug)]
struct ForbiddenHost;
impl warp::reject::Reject for ForbiddenHost {}

/// Filter rejecting requests whose `Host` header is missing or not a loopback name.
///
/// This blocks DNS rebinding attacks, where a malicious website points its own
/// domain at 127.0.0.1 to read the server's responses as same-origin.
fn host_guard() -> impl Filter<Extract = (), Error = warp::Rejection> + Clone {
    warp::header::optional::<String>("host")
        .and_then(|host: Option<String>| async move {
            match host {
                Some(host) if is_allowed_host(&host) => Ok(()),
                _ => Err(warp::reject::custom(ForbiddenHost)),
            }
        })
        .untuple_one()
}

/// Starts an HTTP server with file watching for a specific directory.
///
/// # Arguments
///
/// * `features` - Initial Feature objects to serve as JSON
/// * `config` - Server configuration (port, allowed CORS origins)
/// * `watch_path` - Path to watch for file changes
/// * `on_ready` - Optional callback to be called when server is ready
/// * `scan_config` - Options the features were scanned with, reused when the
///   file watcher recomputes them
///
/// # Returns
///
/// * `Result<()>` - Ok if server starts successfully, Err otherwise
pub async fn serve_features_with_watching(
    features: &ScanOutput,
    config: ServerConfig,
    watch_path: PathBuf,
    on_ready: Option<Box<dyn FnOnce() + Send>>,
    scan_config: ScanConfig,
) -> Result<()> {
    serve_features_with_config_and_watching(
        features,
        config,
        Some(watch_path),
        on_ready,
        scan_config,
    )
    .await
}

/// Starts an HTTP server with custom configuration and optional file watching.
///
/// # Arguments
///
/// * `features` - Slice of Feature objects to serve as JSON
/// * `config` - Server configuration
/// * `watch_path` - Optional path to watch for file changes
/// * `on_ready` - Optional callback to be called when server is ready
/// * `scan_config` - Options the features were scanned with, reused when the
///   file watcher recomputes them
///
/// # Returns
///
/// * `Result<()>` - Ok if server starts successfully, Err otherwise
pub async fn serve_features_with_config_and_watching(
    features: &ScanOutput,
    config: ServerConfig,
    watch_path: Option<PathBuf>,
    on_ready: Option<Box<dyn FnOnce() + Send>>,
    scan_config: ScanConfig,
) -> Result<()> {
    let skip_changes = scan_config.skip_changes;

    // Create shared state for features
    let features_data = Arc::new(RwLock::new(features.clone()));

    // When the features data was last built, refreshed by the file watcher
    let built_at = Arc::new(RwLock::new(current_build_timestamp()));

    // Get repository URL from git config
    let repository_url = watch_path
        .as_ref()
        .and_then(|path| get_repository_url(path));

    // Set up file watching if watch_path is provided
    if let Some(ref path) = watch_path {
        let features_data_clone = Arc::clone(&features_data);
        let built_at_clone = Arc::clone(&built_at);
        let watch_path_clone = path.clone();

        tokio::spawn(async move {
            if let Err(e) = setup_file_watcher(
                features_data_clone,
                built_at_clone,
                watch_path_clone,
                scan_config,
            )
            .await
            {
                eprintln!("File watcher error: {}", e);
            }
        });
    }

    let app = build_app(
        features_data,
        built_at,
        repository_url,
        skip_changes,
        &config,
    );

    // Call the ready callback if provided
    if let Some(callback) = on_ready {
        callback();
    }

    println!(
        "Server running at http://{}:{}",
        config
            .host
            .iter()
            .map(|&b| b.to_string())
            .collect::<Vec<_>>()
            .join("."),
        config.port,
    );
    warp::serve(app).run((config.host, config.port)).await;

    Ok(())
}

/// Builds the complete request handler: routes, `Host` header guard, optional
/// CORS and rejection handling.
fn build_app(
    features_data: Arc<RwLock<ScanOutput>>,
    built_at: Arc<RwLock<String>>,
    repository_url: Option<String>,
    skip_changes: bool,
    config: &ServerConfig,
) -> BoxedFilter<(warp::reply::Response,)> {
    // Route for records.json: the project-level records
    let records_data = Arc::clone(&features_data);
    let records_route = warp::path("records.json")
        .and(warp::path::end())
        .and(warp::get())
        .and_then(move || {
            let records_data = Arc::clone(&records_data);
            async move {
                let output = records_data.read().await;
                let json = match records_json(&output.records) {
                    Ok(json) => json,
                    Err(e) => {
                        eprintln!("Failed to serialize records: {}", e);
                        return Err(warp::reject::custom(SerializationError));
                    }
                };
                Ok::<_, warp::Rejection>(warp::reply::with_header(
                    json,
                    "content-type",
                    "application/json",
                ))
            }
        });

    // Route for features.json with shared state
    let features_route = warp::path("features.json")
        .and(warp::path::end())
        .and(warp::get())
        .and_then(move || {
            let features_data = Arc::clone(&features_data);
            async move {
                let output = features_data.read().await;
                let features_json = match serde_json::to_string_pretty(&output.features) {
                    Ok(json) => json,
                    Err(e) => {
                        eprintln!("Failed to serialize features: {}", e);
                        return Err(warp::reject::custom(SerializationError));
                    }
                };

                Ok::<_, warp::Rejection>(warp::reply::with_header(
                    features_json,
                    "content-type",
                    "application/json",
                ))
            }
        });

    // Route for metadata.json with version info and repository URL
    let metadata_route = warp::path("metadata.json")
        .and(warp::path::end())
        .and(warp::get())
        .and_then(move || {
            let repo_url = repository_url.clone();
            let built_at = Arc::clone(&built_at);
            async move {
                let built_at = built_at.read().await.clone();
                let mut metadata = serde_json::json!({
                    "version": env!("CARGO_PKG_VERSION"),
                    "skipChanges": skip_changes,
                    "builtAt": built_at
                });

                // Add repository URL if available
                if let Some(url) = repo_url {
                    metadata["repository"] = serde_json::json!(url);
                }

                let metadata_json = match serde_json::to_string_pretty(&metadata) {
                    Ok(json) => json,
                    Err(e) => {
                        eprintln!("Failed to serialize metadata: {}", e);
                        return Err(warp::reject::custom(SerializationError));
                    }
                };

                Ok::<_, warp::Rejection>(warp::reply::with_header(
                    metadata_json,
                    "content-type",
                    "application/json",
                ))
            }
        });

    // Route for root path to serve index.html
    let index_route = warp::path::end().and(warp::get()).and_then(serve_index);

    // Route for static files from embedded directory
    let static_route = warp::path::tail()
        .and(warp::get())
        .and_then(serve_static_file);

    let routes = host_guard().and(
        features_route
            .or(records_route)
            .or(metadata_route)
            .or(index_route)
            .or(static_route),
    );

    if config.allowed_origins.is_empty() {
        return routes
            .recover(handle_rejection)
            .map(Reply::into_response)
            .boxed();
    }

    let cors = if config.allowed_origins.iter().any(|origin| origin == "*") {
        warp::cors().allow_any_origin()
    } else {
        // The dashboard's own origins must stay allowed: once CORS is enabled,
        // warp rejects any request carrying an Origin header that is not listed.
        let own_origins = [
            format!("http://localhost:{}", config.port),
            format!("http://127.0.0.1:{}", config.port),
            format!("http://[::1]:{}", config.port),
        ];
        warp::cors().allow_origins(
            config
                .allowed_origins
                .iter()
                .map(String::as_str)
                .chain(own_origins.iter().map(String::as_str)),
        )
    }
    .allow_methods(vec!["GET"]);

    routes
        .with(cors)
        .recover(handle_rejection)
        .map(Reply::into_response)
        .boxed()
}

/// Watches `watch_path` and recomputes the features when a relevant change
/// happens, once per burst of changes.
async fn setup_file_watcher(
    features_data: Arc<RwLock<ScanOutput>>,
    built_at: Arc<RwLock<String>>,
    watch_path: PathBuf,
    scan_config: ScanConfig,
) -> Result<()> {
    let filter = WatchFilter::new(
        &watch_path,
        scan_config.ignore_paths.clone(),
        scan_config.include_skipped_dirs,
    );
    // Dropping the watcher would stop it, so keep it for the whole loop
    let (_watcher, mut changes) = file_watcher::watch(&watch_path, filter)?;

    while file_watcher::next_change(&mut changes, QUIET_PERIOD, MAX_WAIT).await {
        // Scanning reads the whole project, so keep it off the async workers.
        // Changes made meanwhile leave one pending signal: one more rescan.
        let path = watch_path.clone();
        let config = scan_config.clone();
        let result = tokio::task::spawn_blocking(move || rescan_features(&path, &config)).await?;

        match result {
            Ok(new_features) => {
                let mut features = features_data.write().await;
                *features = new_features;
                *built_at.write().await = current_build_timestamp();
                println!("✅ Features updated successfully");
            }
            Err(e) => {
                eprintln!("❌ Failed to recompute features: {}", e);
            }
        }
    }

    Ok(())
}

/// Recomputes the features after a file change, with the same options as the
/// initial scan (git history or not, coverage, ignored folders, emails...)
fn rescan_features(watch_path: &Path, scan_config: &ScanConfig) -> Result<ScanOutput> {
    scan_project_with(watch_path, scan_config)
}

/// Custom error type for serialization failures
#[derive(Debug)]
struct SerializationError;
impl warp::reject::Reject for SerializationError {}

/// Serves the index page for the root path.
///
/// If embedded `index.html` exists, it will be served. Otherwise, a default
/// HTML page with navigation links will be returned.
///
/// # Returns
///
/// * `Result<impl warp::Reply, warp::Rejection>` - HTML response or rejection
async fn serve_index() -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(file) = STATIC_DIR.get_file("index.html") {
        let content = file.contents_utf8().unwrap_or("");
        Ok(
            warp::reply::with_header(content, "content-type", "text/html; charset=utf-8")
                .into_response(),
        )
    } else {
        let html = create_default_index_html();
        Ok(
            warp::reply::with_header(html, "content-type", "text/html; charset=utf-8")
                .into_response(),
        )
    }
}

/// Serves static files from the embedded directory.
///
/// # Arguments
///
/// * `path` - The requested file path
///
/// # Returns
///
/// * `Result<impl warp::Reply, warp::Rejection>` - File content or rejection
async fn serve_static_file(path: warp::path::Tail) -> Result<impl warp::Reply, warp::Rejection> {
    let path_str = path.as_str();

    // Try to get the file from the embedded directory
    if let Some(file) = STATIC_DIR.get_file(path_str) {
        let content_type = get_content_type(path_str);

        if let Some(contents) = file.contents_utf8() {
            // Text file
            Ok(warp::reply::with_header(contents, "content-type", content_type).into_response())
        } else {
            // Binary file
            Ok(
                warp::reply::with_header(file.contents(), "content-type", content_type)
                    .into_response(),
            )
        }
    } else {
        Err(warp::reject::not_found())
    }
}

/// Determines the content type based on file extension.
///
/// # Arguments
///
/// * `path` - The file path
///
/// # Returns
///
/// * `&'static str` - The appropriate MIME type
fn get_content_type(path: &str) -> &'static str {
    let extension = Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    match extension.to_lowercase().as_str() {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "ico" => "image/x-icon",
        "txt" => "text/plain; charset=utf-8",
        "pdf" => "application/pdf",
        "xml" => "application/xml; charset=utf-8",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "eot" => "application/vnd.ms-fontobject",
        _ => "application/octet-stream",
    }
}

/// Creates the default HTML page when no index.html is found.
fn create_default_index_html() -> String {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Features Dashboard</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            margin: 40px;
            line-height: 1.6;
            background: #f5f5f5;
        }
        .container {
            max-width: 800px;
            margin: 0 auto;
            background: white;
            padding: 30px;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        h1 { color: #333; }
        .links { list-style: none; padding: 0; }
        .links li { margin: 10px 0; }
        .links a {
            color: #007acc;
            text-decoration: none;
            padding: 8px 15px;
            border: 1px solid #007acc;
            border-radius: 4px;
            display: inline-block;
        }
        .links a:hover { background: #007acc; color: white; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🏗️ Features Dashboard</h1>
        <p>Welcome to the feature-based architecture server!</p>
        <ul class="links">
            <li><a href="/features.json">📊 View Features JSON</a></li>
            <li><a href="/metadata.json">ℹ️ View Metadata JSON</a></li>
        </ul>
        <p><small>This server provides features data and serves embedded static files from the binary.</small></p>
    </div>
</body>
</html>"#.to_string()
}

/// Handles HTTP request rejections and converts them to appropriate responses.
async fn handle_rejection(
    err: warp::Rejection,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    let (code, message) = if err.find::<ForbiddenHost>().is_some() {
        (warp::http::StatusCode::FORBIDDEN, "FORBIDDEN_HOST")
    } else if err.find::<warp::cors::CorsForbidden>().is_some() {
        (warp::http::StatusCode::FORBIDDEN, "CORS_FORBIDDEN")
    } else if err.is_not_found() {
        (warp::http::StatusCode::NOT_FOUND, "NOT_FOUND")
    } else if err
        .find::<warp::filters::body::BodyDeserializeError>()
        .is_some()
    {
        (warp::http::StatusCode::BAD_REQUEST, "BAD_REQUEST")
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (
            warp::http::StatusCode::METHOD_NOT_ALLOWED,
            "METHOD_NOT_ALLOWED",
        )
    } else {
        eprintln!("Unhandled rejection: {:?}", err);
        (
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_SERVER_ERROR",
        )
    };

    Ok(warp::reply::with_status(message, code))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Feature;

    fn app(allowed_origins: Vec<String>) -> BoxedFilter<(warp::reply::Response,)> {
        let config = ServerConfig::new(3000).with_allowed_origins(allowed_origins);
        build_app(
            Arc::new(RwLock::new(ScanOutput::default())),
            Arc::new(RwLock::new(String::from("2026-01-01T00:00:00Z"))),
            None,
            false,
            &config,
        )
    }

    #[test]
    fn test_is_allowed_host() {
        for host in [
            "localhost",
            "localhost:3000",
            "LocalHost:8080",
            "127.0.0.1",
            "127.0.0.1:3000",
            "[::1]",
            "[::1]:3000",
        ] {
            assert!(is_allowed_host(host), "{host} should be allowed");
        }

        for host in [
            "",
            "evil.com",
            "evil.com:3000",
            "localhost.evil.com",
            "127.0.0.1.nip.io:3000",
            "192.168.1.10:3000",
            "0.0.0.0:3000",
            "localhost:",
            "localhost:abc",
            "[::1]x",
            "[::1",
            "::1",
        ] {
            assert!(!is_allowed_host(host), "{host} should be rejected");
        }
    }

    #[test]
    fn test_parse_allowed_origin() {
        assert_eq!(
            parse_allowed_origin("http://localhost:5173").unwrap(),
            "http://localhost:5173"
        );
        assert_eq!(
            parse_allowed_origin("https://example.com/").unwrap(),
            "https://example.com"
        );
        assert_eq!(parse_allowed_origin("*").unwrap(), "*");

        for invalid in [
            "localhost:5173",
            "ftp://example.com",
            "http://",
            "http://example.com/path",
            "http://example.com?q=1",
            "http://user:pass@example.com",
            "http://exa mple.com",
        ] {
            assert!(
                parse_allowed_origin(invalid).is_err(),
                "{invalid} should be rejected"
            );
        }
    }

    #[tokio::test]
    async fn test_rejects_non_loopback_host() {
        let res = warp::test::request()
            .path("/metadata.json")
            .header("host", "evil.com")
            .reply(&app(Vec::new()))
            .await;
        assert_eq!(res.status(), 403);
    }

    #[tokio::test]
    async fn test_rejects_missing_host() {
        let res = warp::test::request()
            .path("/metadata.json")
            .reply(&app(Vec::new()))
            .await;
        assert_eq!(res.status(), 403);
    }

    #[tokio::test]
    async fn test_records_json_lists_project_records() {
        let res = warp::test::request()
            .path("/records.json")
            .header("host", "localhost:3000")
            .reply(&app(Vec::new()))
            .await;
        assert_eq!(res.status(), 200);
        let body: serde_json::Value = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(body, serde_json::json!({ "records": [] }));
    }

    #[tokio::test]
    async fn test_no_cors_headers_by_default() {
        let res = warp::test::request()
            .path("/features.json")
            .header("host", "localhost:3000")
            .header("origin", "https://evil.com")
            .reply(&app(Vec::new()))
            .await;
        assert_eq!(res.status(), 200);
        assert!(res.headers().get("access-control-allow-origin").is_none());
    }

    #[tokio::test]
    async fn test_cors_allows_listed_origin_only() {
        let app = app(vec![String::from("http://localhost:5173")]);

        let allowed = warp::test::request()
            .path("/features.json")
            .header("host", "localhost:3000")
            .header("origin", "http://localhost:5173")
            .reply(&app)
            .await;
        assert_eq!(allowed.status(), 200);
        assert_eq!(
            allowed
                .headers()
                .get("access-control-allow-origin")
                .unwrap(),
            "http://localhost:5173"
        );

        let own_origin = warp::test::request()
            .path("/features.json")
            .header("host", "localhost:3000")
            .header("origin", "http://localhost:3000")
            .reply(&app)
            .await;
        assert_eq!(own_origin.status(), 200);

        let denied = warp::test::request()
            .path("/features.json")
            .header("host", "localhost:3000")
            .header("origin", "https://evil.com")
            .reply(&app)
            .await;
        assert_eq!(denied.status(), 403);
        assert!(
            denied
                .headers()
                .get("access-control-allow-origin")
                .is_none()
        );
    }

    #[tokio::test]
    async fn test_cors_wildcard_allows_any_origin() {
        let res = warp::test::request()
            .path("/features.json")
            .header("host", "localhost:3000")
            .header("origin", "https://example.com")
            .reply(&app(vec![String::from("*")]))
            .await;
        assert_eq!(res.status(), 200);
        assert!(res.headers().get("access-control-allow-origin").is_some());
    }

    fn all_features(features: &[Feature]) -> Vec<&Feature> {
        features
            .iter()
            .flat_map(|f| std::iter::once(f).chain(all_features(&f.features)))
            .collect()
    }

    /// A rescan must keep the options of the initial scan: it used to always
    /// compute git history and never add coverage.
    #[test]
    fn test_rescan_keeps_scan_options() {
        let base = PathBuf::from("../../examples/rust-basic");
        let config = ScanConfig::new(&base)
            .skip_changes(true)
            .with_coverage(true);

        let initial = scan_project_with(&base, &config).unwrap().features;
        let rescanned = rescan_features(&base, &config).unwrap().features;

        for features in [&initial, &rescanned] {
            let features = all_features(features);
            assert!(!features.is_empty());
            assert!(
                features.iter().all(|f| f.changes.is_empty()),
                "--skip-changes must survive a rescan"
            );
            assert!(
                features
                    .iter()
                    .any(|f| f.stats.as_ref().is_some_and(|s| s.coverage.is_some())),
                "coverage must survive a rescan"
            );
        }
        assert_eq!(
            serde_json::to_value(&initial).unwrap(),
            serde_json::to_value(&rescanned).unwrap()
        );
    }

    #[test]
    fn test_rescan_keeps_strip_emails() {
        let base = PathBuf::from("../../examples/rust-basic");
        let config = ScanConfig::new(&base).strip_emails(true);

        let rescanned = rescan_features(&base, &config).unwrap().features;
        let changes: Vec<_> = all_features(&rescanned)
            .into_iter()
            .flat_map(|f| &f.changes)
            .collect();
        assert!(!changes.is_empty(), "Should find git history");
        assert!(changes.iter().all(|c| c.author_email.is_empty()));
    }
}
