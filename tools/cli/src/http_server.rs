//! HTTP server module for serving features data and static files.
//!
//! This module provides functionality to start an HTTP server that serves:
//! - Features data as JSON at `/features.json`
//! - Static files from the `public/` directory
//! - A default index page at the root path
//!
//! The server uses the `warp` web framework and supports CORS for cross-origin requests.

use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;
use warp::Filter;

use crate::models::Feature;

/// Configuration for the HTTP server
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Port to run the server on
    pub port: u16,
    /// Directory to serve static files from
    pub public_dir: PathBuf,
    /// Host address to bind to
    pub host: [u8; 4],
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            public_dir: PathBuf::from("public"),
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

    /// Set the public directory for static files
    #[allow(dead_code)]
    pub fn with_public_dir<P: Into<PathBuf>>(mut self, dir: P) -> Self {
        self.public_dir = dir.into();
        self
    }

    /// Set the host address to bind to
    #[allow(dead_code)]
    pub fn with_host(mut self, host: [u8; 4]) -> Self {
        self.host = host;
        self
    }
}

/// Starts an HTTP server to serve features data and static files.
///
/// # Arguments
///
/// * `features` - Slice of Feature objects to serve as JSON
/// * `port` - Port number to run the server on
///
/// # Returns
///
/// * `Result<()>` - Ok if server starts successfully, Err otherwise
///
/// # Server Routes
///
/// * `GET /` - Serves index.html or default HTML page
/// * `GET /features.json` - Serves features data as JSON
/// * `GET /*` - Serves static files from `public/` directory
///
/// # Example
///
/// ```rust,no_run
/// use your_crate::http_server::serve_features;
/// use your_crate::models::Feature;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let features = vec![]; // Your features data
///     serve_features(&features, 3000).await
/// }
/// ```
pub async fn serve_features(features: &[Feature], port: u16) -> Result<()> {
    let config = ServerConfig::new(port);
    serve_features_with_config(features, config).await
}

/// Starts an HTTP server with custom configuration.
///
/// # Arguments
///
/// * `features` - Slice of Feature objects to serve as JSON
/// * `config` - Server configuration
///
/// # Returns
///
/// * `Result<()>` - Ok if server starts successfully, Err otherwise
pub async fn serve_features_with_config(features: &[Feature], config: ServerConfig) -> Result<()> {
    let features_json = serde_json::to_string_pretty(features)
        .map_err(|e| anyhow::anyhow!("Failed to serialize features to JSON: {}", e))?;

    // Route for features.json
    let features_json_clone = features_json.clone();
    let features_route = warp::path("features.json").and(warp::get()).map(move || {
        warp::reply::with_header(
            features_json_clone.clone(),
            "content-type",
            "application/json",
        )
    });

    // Route for static files from public folder
    let static_route = warp::fs::dir(config.public_dir.clone());

    // Route for root path to serve index.html if it exists
    let config_clone = config.clone();
    let index_route = warp::path::end()
        .and(warp::get())
        .and_then(move || serve_index(config_clone.clone()));

    let routes = features_route
        .or(static_route)
        .or(index_route)
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

/// Serves the index page for the root path.
///
/// If `public/index.html` exists, it will be served. Otherwise, a default
/// HTML page with navigation links will be returned.
///
/// # Arguments
///
/// * `config` - Server configuration containing public directory path
///
/// # Returns
///
/// * `Result<impl warp::Reply, warp::Rejection>` - HTML response or rejection
///
/// # Behavior
///
/// 1. Check if `public/index.html` exists
/// 2. If it exists, read and serve the file with `text/html` content type
/// 3. If it doesn't exist or can't be read, serve a default HTML page
/// 4. If file reading fails, return a 404 not found error
async fn serve_index(config: ServerConfig) -> Result<impl warp::Reply, warp::Rejection> {
    let index_path = config.public_dir.join("index.html");
    if index_path.exists() {
        match fs::read_to_string(&index_path).await {
            Ok(content) => Ok(warp::reply::with_header(
                content,
                "content-type",
                "text/html",
            )),
            Err(e) => {
                eprintln!("Warning: Failed to read {}: {}", index_path.display(), e);
                Err(warp::reject::not_found())
            }
        }
    } else {
        let html = create_default_index_html();
        Ok(warp::reply::with_header(html, "content-type", "text/html"))
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
        <p><small>This server provides features data and serves static files from the public directory.</small></p>
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
