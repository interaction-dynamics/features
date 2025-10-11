//! HTTP server module for serving features data and embedded static files.
//!
//! This module provides functionality to start an HTTP server that serves:
//! - Features data as JSON at `/features.json`
//! - Embedded static files from the compiled binary
//! - A default index page at the root path
//!
//! The server uses the `warp` web framework and supports CORS for cross-origin requests.
//! Static files are embedded at compile time using the `include_dir` crate.

use anyhow::Result;
use include_dir::{Dir, include_dir};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;
use warp::{Filter, Reply};

use crate::file_scanner::list_files_recursive_with_changes;
use crate::models::Feature;

// Embed the public directory at compile time
static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/public");

/// Configuration for the HTTP server
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Port to run the server on
    pub port: u16,
    /// Host address to bind to
    pub host: [u8; 4],
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            host: [127, 0, 0, 1],
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
}

/// Starts an HTTP server with file watching for a specific directory.
///
/// # Arguments
///
/// * `features` - Initial Feature objects to serve as JSON
/// * `port` - Port number to run the server on
/// * `watch_path` - Path to watch for file changes
///
/// # Returns
///
/// * `Result<()>` - Ok if server starts successfully, Err otherwise
pub async fn serve_features_with_watching(
    features: &[Feature],
    port: u16,
    watch_path: PathBuf,
) -> Result<()> {
    let config = ServerConfig::new(port);
    serve_features_with_config_and_watching(features, config, Some(watch_path)).await
}

/// Starts an HTTP server with custom configuration and optional file watching.
///
/// # Arguments
///
/// * `features` - Slice of Feature objects to serve as JSON
/// * `config` - Server configuration
/// * `watch_path` - Optional path to watch for file changes
///
/// # Returns
///
/// * `Result<()>` - Ok if server starts successfully, Err otherwise
pub async fn serve_features_with_config_and_watching(
    features: &[Feature],
    config: ServerConfig,
    watch_path: Option<PathBuf>,
) -> Result<()> {
    // Create shared state for features
    let features_data = Arc::new(RwLock::new(features.to_vec()));

    // Set up file watching if watch_path is provided
    if let Some(ref path) = watch_path {
        let features_data_clone = Arc::clone(&features_data);
        let watch_path_clone = path.clone();

        tokio::spawn(async move {
            if let Err(e) = setup_file_watcher(features_data_clone, watch_path_clone).await {
                eprintln!("File watcher error: {}", e);
            }
        });

        println!("📁 Watching directory: {}", path.display());
    }

    // Route for features.json with shared state
    let features_data_clone = Arc::clone(&features_data);
    let features_route = warp::path("features.json")
        .and(warp::get())
        .and_then(move || {
            let features_data = Arc::clone(&features_data_clone);
            async move {
                let features = features_data.read().await;
                let features_json = match serde_json::to_string_pretty(&*features) {
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

    // Route for root path to serve index.html
    let index_route = warp::path::end().and(warp::get()).and_then(serve_index);

    // Route for static files from embedded directory
    let static_route = warp::path::tail()
        .and(warp::get())
        .and_then(serve_static_file);

    let routes = features_route
        .or(index_route)
        .or(static_route)
        .with(warp::cors().allow_any_origin())
        .recover(handle_rejection);

    println!(
        "Server running at http://{}:{}",
        config
            .host
            .iter()
            .map(|&b| b.to_string())
            .collect::<Vec<_>>()
            .join("."),
        config.port
    );
    warp::serve(routes).run((config.host, config.port)).await;

    Ok(())
}

/// Sets up file system watching for the specified path.
async fn setup_file_watcher(
    features_data: Arc<RwLock<Vec<Feature>>>,
    watch_path: PathBuf,
) -> Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    // Set up the file watcher in a blocking task
    let watch_path_clone = watch_path.clone();
    let _watcher = tokio::task::spawn_blocking(move || -> Result<RecommendedWatcher> {
        let mut watcher = RecommendedWatcher::new(
            move |res: notify::Result<Event>| {
                match res {
                    Ok(event) => {
                        // Send event through channel
                        if let Err(e) = tx.blocking_send(event) {
                            eprintln!("Failed to send file system event: {}", e);
                        }
                    }
                    Err(e) => eprintln!("File watcher error: {:?}", e),
                }
            },
            Config::default(),
        )?;

        watcher.watch(&watch_path_clone, RecursiveMode::Recursive)?;
        Ok(watcher)
    })
    .await??;

    // Process file system events
    while let Some(event) = rx.recv().await {
        // Check if this is a file we care about (README.md files or directory changes)
        let should_recompute = event.paths.iter().any(|path| {
            path.file_name()
                .map(|name| name == "README.md")
                .unwrap_or(false)
                || event.kind.is_create()
                || event.kind.is_remove()
        });

        if should_recompute {
            // Add a small delay to avoid excessive recomputation during rapid changes
            sleep(Duration::from_millis(500)).await;

            match list_files_recursive_with_changes(&watch_path) {
                Ok(new_features) => {
                    let mut features = features_data.write().await;
                    *features = new_features;
                    println!("✅ Features updated successfully");
                }
                Err(e) => {
                    eprintln!("❌ Failed to recompute features: {}", e);
                }
            }
        }
    }

    Ok(())
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
    let code;
    let message;

    if err.is_not_found() {
        code = warp::http::StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if err
        .find::<warp::filters::body::BodyDeserializeError>()
        .is_some()
    {
        code = warp::http::StatusCode::BAD_REQUEST;
        message = "BAD_REQUEST";
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = warp::http::StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        eprintln!("Unhandled rejection: {:?}", err);
        code = warp::http::StatusCode::INTERNAL_SERVER_ERROR;
        message = "INTERNAL_SERVER_ERROR";
    }

    Ok(warp::reply::with_status(message, code))
}
