//! MCP (Model Context Protocol) server over stdio.
//!
//! `features mcp` lets AI coding agents (Claude Code, Cursor...) query the
//! feature map instead of parsing `features scan --json`. The protocol is
//! newline-delimited JSON-RPC 2.0 on stdin/stdout; only the subset needed for
//! tools is implemented (`initialize`, `ping`, `tools/list`, `tools/call`), so
//! no MCP crate is needed (see `docs/decisions/001-hand-rolled-mcp-server.md`).
//!
//! stdout carries only protocol messages: every log goes to stderr.
//!
//! The project is scanned on the first tool call and cached. A file watcher
//! marks the cache stale when files that can change the features change, and
//! the next tool call rescans.

use std::io::{BufRead, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::SystemTime;

use anyhow::{Result, bail};
use serde_json::{Map, Value, json};

use crate::diff;
use crate::file_watcher::{self, WatchFilter};
use crate::models::{Feature, Record, RecordKind};
use crate::ownership::find_owner_for_relative_path;
use crate::records::project_records;
use crate::scan::{ScanConfig, scan_features_with};

/// Protocol versions the server can speak, latest first
pub const SUPPORTED_PROTOCOL_VERSIONS: &[&str] =
    &["2025-11-25", "2025-06-18", "2025-03-26", "2024-11-05"];

/// Name of the server in the `initialize` response
pub const SERVER_NAME: &str = "features";

const PARSE_ERROR: i64 = -32700;
const INVALID_REQUEST: i64 = -32600;
const METHOD_NOT_FOUND: i64 = -32601;
const INVALID_PARAMS: i64 = -32602;

/// Longest README summary in `list_features`, in characters
const SUMMARY_MAX_CHARS: usize = 200;

/// Number of recent changes (commits) in `get_feature`
const RECENT_CHANGES: usize = 10;

const INSTRUCTIONS: &str = "\
Map of this codebase by feature: folders under `features/` or with a README \
that has `feature: true`, each with an owner, a README, specs and decision \
records, coverage, git history and data imported from other tools. \
Use `list_features` to discover the features and their owners, `get_feature` \
before changing a feature (README, specs, decisions, coverage, external data), \
`find_owner` to know which feature and team own a file, `features_touched` to \
see which features and owners a branch affects, and `list_records` for the \
project-level specs and decisions. Paths are relative to the scanned folder \
(`root` in `list_features`).";

/// The scanned features, and what is needed to know when they are outdated
struct Snapshot {
    features: Vec<Feature>,
    /// Project-level records (root records not attached to a feature)
    records: Vec<Record>,
    /// Modification times of the `--data` files when they were read
    data_mtimes: Vec<Option<SystemTime>>,
}

/// Answers MCP messages about the features of one folder
pub struct McpServer {
    base_path: PathBuf,
    config: ScanConfig,
    snapshot: Option<Snapshot>,
    /// Set by the file watcher when the features may have changed
    stale: Arc<AtomicBool>,
    /// Rescan on every tool call, when no watcher can tell about changes
    always_rescan: bool,
}

impl McpServer {
    /// A server for the features of `base_path`, scanned with `config`
    pub fn new(base_path: impl Into<PathBuf>, config: ScanConfig) -> Self {
        Self {
            base_path: base_path.into(),
            config,
            snapshot: None,
            stale: Arc::new(AtomicBool::new(false)),
            always_rescan: false,
        }
    }

    /// Flag to set when the features may have changed: the next tool call rescans
    pub fn stale_flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.stale)
    }

    /// Rescan on every tool call instead of caching the features
    pub fn rescan_every_call(&mut self) {
        self.always_rescan = true;
    }

    /// Answer one line of input: a JSON-RPC message or batch.
    ///
    /// Returns the serialized response, or `None` for notifications.
    pub fn handle_line(&mut self, line: &str) -> Option<String> {
        let response = match serde_json::from_str::<Value>(line) {
            Ok(message) => self.handle_message(&message)?,
            Err(err) => error_response(Value::Null, PARSE_ERROR, format!("Parse error: {}", err)),
        };
        Some(response.to_string())
    }

    /// Answer a JSON-RPC message or batch. Returns `None` when nothing must be
    /// sent back (notifications, responses).
    pub fn handle_message(&mut self, message: &Value) -> Option<Value> {
        match message {
            Value::Array(batch) if batch.is_empty() => Some(error_response(
                Value::Null,
                INVALID_REQUEST,
                "Invalid request: empty batch",
            )),
            Value::Array(batch) => {
                let responses: Vec<Value> = batch
                    .iter()
                    .filter_map(|message| self.handle_single(message))
                    .collect();
                (!responses.is_empty()).then_some(Value::Array(responses))
            }
            message => self.handle_single(message),
        }
    }

    fn handle_single(&mut self, message: &Value) -> Option<Value> {
        let Some(object) = message.as_object() else {
            return Some(error_response(
                Value::Null,
                INVALID_REQUEST,
                "Invalid request: expected an object",
            ));
        };

        let id = object.get("id").cloned();
        let Some(method) = object.get("method") else {
            // A response to a request we never sent: nothing to answer
            if object.contains_key("result") || object.contains_key("error") {
                return None;
            }
            return Some(error_response(
                id.unwrap_or(Value::Null),
                INVALID_REQUEST,
                "Invalid request: missing method",
            ));
        };
        let Some(method) = method.as_str() else {
            return Some(error_response(
                id.unwrap_or(Value::Null),
                INVALID_REQUEST,
                "Invalid request: method must be a string",
            ));
        };

        // Notifications (initialized, cancelled...) need no answer
        let id = id?;

        let params = object.get("params").cloned().unwrap_or(Value::Null);
        let result = match method {
            "initialize" => Ok(initialize_result(&params)),
            "ping" => Ok(json!({})),
            "tools/list" => Ok(json!({ "tools": tool_definitions() })),
            "tools/call" => self.call_tool(&params),
            _ => Err((METHOD_NOT_FOUND, format!("Method not found: {}", method))),
        };

        Some(match result {
            Ok(result) => json!({ "jsonrpc": "2.0", "id": id, "result": result }),
            Err((code, message)) => error_response(id, code, message),
        })
    }

    /// `tools/call`: a protocol error for a malformed call or an unknown tool,
    /// otherwise a result, with `isError` when the tool failed
    fn call_tool(&mut self, params: &Value) -> std::result::Result<Value, (i64, String)> {
        let Some(name) = params.get("name").and_then(Value::as_str) else {
            return Err((INVALID_PARAMS, "Invalid params: missing tool name".into()));
        };
        let empty = Map::new();
        let args = match params.get("arguments") {
            None | Some(Value::Null) => &empty,
            Some(Value::Object(args)) => args,
            Some(_) => {
                return Err((
                    INVALID_PARAMS,
                    "Invalid params: arguments must be an object".into(),
                ));
            }
        };
        if !TOOLS.iter().any(|tool| tool.name == name) {
            return Err((INVALID_PARAMS, format!("Unknown tool: {}", name)));
        }

        Ok(match self.run_tool(name, args) {
            Ok(value) => json!({
                "content": [{
                    "type": "text",
                    "text": serde_json::to_string_pretty(&value).unwrap_or_default(),
                }],
                "isError": false,
            }),
            Err(err) => json!({
                "content": [{ "type": "text", "text": format!("{:#}", err) }],
                "isError": true,
            }),
        })
    }

    fn run_tool(&mut self, name: &str, args: &Map<String, Value>) -> Result<Value> {
        self.refresh()?;
        let snapshot = self
            .snapshot
            .as_ref()
            .expect("refresh keeps a snapshot on success");
        let base_path = self.base_path.as_path();

        match name {
            "list_features" => Ok(list_features(snapshot, base_path, args)),
            "get_feature" => get_feature(snapshot, base_path, args),
            "find_owner" => find_owner(snapshot, base_path, &self.config.current_dir, args),
            "features_touched" => features_touched(snapshot, base_path, args),
            "list_records" => list_records(snapshot, args),
            _ => bail!("Unknown tool: {}", name),
        }
    }

    /// Scan the features if they were never scanned or may have changed
    fn refresh(&mut self) -> Result<()> {
        let data_mtimes = modification_times(&self.config.data_files);
        // Clear the flag before scanning, so a change during the scan is not lost
        let stale = self.stale.swap(false, Ordering::SeqCst);
        let fresh = !self.always_rescan
            && !stale
            && self
                .snapshot
                .as_ref()
                .is_some_and(|snapshot| snapshot.data_mtimes == data_mtimes);
        if fresh {
            return Ok(());
        }

        match scan_features_with(&self.base_path, &self.config) {
            Ok(features) => {
                let records = project_records(&features, &self.base_path);
                self.snapshot = Some(Snapshot {
                    features,
                    records,
                    data_mtimes,
                });
                Ok(())
            }
            Err(err) => {
                // Try again on the next call
                self.stale.store(true, Ordering::SeqCst);
                Err(err.context("failed to scan the features"))
            }
        }
    }
}

/// Read JSON-RPC messages from `reader`, one per line, and write the
/// responses to `writer`, one per line, until the input ends
pub fn serve<R: BufRead, W: Write>(server: &mut McpServer, reader: R, mut writer: W) -> Result<()> {
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        if let Some(response) = server.handle_line(&line) {
            writeln!(writer, "{}", response)?;
            writer.flush()?;
        }
    }
    Ok(())
}

/// Run the MCP server on stdin/stdout for the features of `base_path`, until
/// stdin is closed
pub async fn run_stdio(base_path: PathBuf, config: ScanConfig) -> Result<()> {
    if !base_path.is_dir() {
        bail!("'{}' is not a folder", base_path.display());
    }

    let mut server = McpServer::new(base_path.clone(), config.clone());

    // Dropping the watcher would stop it, so keep it until the server stops
    let filter = WatchFilter::new(
        &base_path,
        config.ignore_paths.clone(),
        config.include_skipped_dirs,
    );
    let _watcher = match file_watcher::watch(&base_path, filter) {
        Ok((watcher, mut changes)) => {
            let stale = server.stale_flag();
            tokio::spawn(async move {
                while changes.recv().await.is_some() {
                    stale.store(true, Ordering::SeqCst);
                }
            });
            Some(watcher)
        }
        Err(err) => {
            eprintln!(
                "Warning: cannot watch '{}' for changes ({}), rescanning on every call.",
                base_path.display(),
                err
            );
            server.rescan_every_call();
            None
        }
    };

    eprintln!(
        "Features MCP server v{} serving {} over stdio",
        env!("CARGO_PKG_VERSION"),
        base_path.display()
    );

    tokio::task::spawn_blocking(move || {
        let stdin = std::io::stdin();
        let stdout = std::io::stdout();
        serve(&mut server, stdin.lock(), stdout.lock())
    })
    .await??;

    Ok(())
}

fn error_response(id: Value, code: i64, message: impl Into<String>) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": { "code": code, "message": message.into() },
    })
}

fn initialize_result(params: &Value) -> Value {
    let requested = params.get("protocolVersion").and_then(Value::as_str);
    let version = requested
        .filter(|version| SUPPORTED_PROTOCOL_VERSIONS.contains(version))
        .unwrap_or(SUPPORTED_PROTOCOL_VERSIONS[0]);

    json!({
        "protocolVersion": version,
        "capabilities": { "tools": { "listChanged": false } },
        "serverInfo": {
            "name": SERVER_NAME,
            "title": "Features",
            "version": env!("CARGO_PKG_VERSION"),
        },
        "instructions": INSTRUCTIONS,
    })
}

struct ToolDefinition {
    name: &'static str,
    title: &'static str,
    description: &'static str,
    input_schema: fn() -> Value,
}

const TOOLS: &[ToolDefinition] = &[
    ToolDefinition {
        name: "list_features",
        title: "List features",
        description: "List every feature of the codebase (flat), with its key, path, owner, \
            parent feature and a one-line summary of its README. Filter by owner or by a \
            text query.",
        input_schema: || {
            json!({
                "type": "object",
                "properties": {
                    "owner": {
                        "type": "string",
                        "description": "Only the features of this owner (case insensitive, leading @ ignored)",
                    },
                    "query": {
                        "type": "string",
                        "description": "Only the features whose name, key, path or README contains this text (case insensitive)",
                    },
                },
            })
        },
    },
    ToolDefinition {
        name: "get_feature",
        title: "Get feature",
        description: "Everything known about one feature: owner, README, metadata, links, \
            data imported from other tools, stats (files, lines, TODOs, coverage), specs and \
            decision records, dependencies on other features, sub-features and recent commits. \
            Read it before changing a feature.",
        input_schema: || {
            json!({
                "type": "object",
                "properties": {
                    "feature": {
                        "type": "string",
                        "description": "Key, path, name or folder name of the feature",
                    },
                },
                "required": ["feature"],
            })
        },
    },
    ToolDefinition {
        name: "find_owner",
        title: "Find owner",
        description: "The feature that contains a file or folder, and its owner. The path \
            can be absolute, relative to the current directory or relative to the scanned \
            folder, and does not need to exist.",
        input_schema: || {
            json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "File or folder",
                    },
                },
                "required": ["path"],
            })
        },
    },
    ToolDefinition {
        name: "features_touched",
        title: "Features touched",
        description: "The features touched since the merge base of a git ref and HEAD \
            (including uncommitted changes unless committed_only), with their owners and \
            changed files, and the changed files outside every feature. Same as \
            `features diff --since <ref>`.",
        input_schema: || {
            json!({
                "type": "object",
                "properties": {
                    "since": {
                        "type": "string",
                        "description": "Git ref to compare with, e.g. origin/main",
                    },
                    "committed_only": {
                        "type": "boolean",
                        "description": "Ignore the working tree (staged, unstaged and untracked files)",
                    },
                },
                "required": ["since"],
            })
        },
    },
    ToolDefinition {
        name: "list_records",
        title: "List records",
        description: "Specs and decision records (ADRs): the project-level ones by default, \
            or the ones of a feature. Paths are relative to the scanned folder.",
        input_schema: || {
            json!({
                "type": "object",
                "properties": {
                    "kind": {
                        "type": "string",
                        "enum": ["spec", "decision"],
                        "description": "Only specs or only decision records",
                    },
                    "feature": {
                        "type": "string",
                        "description": "Key, path, name or folder name of a feature, to list its records instead",
                    },
                },
            })
        },
    },
];

fn tool_definitions() -> Vec<Value> {
    TOOLS
        .iter()
        .map(|tool| {
            json!({
                "name": tool.name,
                "title": tool.title,
                "description": tool.description,
                "inputSchema": (tool.input_schema)(),
                "annotations": {
                    "title": tool.title,
                    "readOnlyHint": true,
                    "openWorldHint": false,
                },
            })
        })
        .collect()
}

fn modification_times(paths: &[PathBuf]) -> Vec<Option<SystemTime>> {
    paths
        .iter()
        .map(|path| std::fs::metadata(path).and_then(|m| m.modified()).ok())
        .collect()
}

fn string_arg<'a>(args: &'a Map<String, Value>, name: &str) -> Result<Option<&'a str>> {
    match args.get(name) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => Ok(Some(value.as_str())),
        Some(_) => bail!("'{}' must be a string", name),
    }
}

fn required_string_arg<'a>(args: &'a Map<String, Value>, name: &str) -> Result<&'a str> {
    match string_arg(args, name)? {
        Some(value) if !value.trim().is_empty() => Ok(value.trim()),
        _ => bail!("missing required argument '{}'", name),
    }
}

fn bool_arg(args: &Map<String, Value>, name: &str) -> Result<bool> {
    match args.get(name) {
        None | Some(Value::Null) => Ok(false),
        Some(Value::Bool(value)) => Ok(*value),
        Some(_) => bail!("'{}' must be a boolean", name),
    }
}

/// Every feature with its parent, in tree order
fn all_features(features: &[Feature]) -> Vec<(&Feature, Option<&Feature>)> {
    fn visit<'a>(
        features: &'a [Feature],
        parent: Option<&'a Feature>,
        all: &mut Vec<(&'a Feature, Option<&'a Feature>)>,
    ) {
        for feature in features {
            all.push((feature, parent));
            visit(&feature.features, Some(feature), all);
        }
    }
    let mut all = Vec::new();
    visit(features, None, &mut all);
    all
}

fn normalize_feature_path(path: &str) -> &str {
    let path = path.trim();
    let path = path.strip_prefix("./").unwrap_or(path);
    path.trim_end_matches('/')
}

/// Find a feature by key, path, name (case insensitive) or folder name, in
/// that order of priority
pub fn find_feature<'a>(
    features: &'a [Feature],
    query: &str,
) -> Option<(&'a Feature, Option<&'a Feature>)> {
    let query = normalize_feature_path(query);
    let all = all_features(features);

    all.iter()
        .find(|(f, _)| f.key == query)
        .or_else(|| all.iter().find(|(f, _)| f.path == query))
        .or_else(|| all.iter().find(|(f, _)| f.name.eq_ignore_ascii_case(query)))
        .or_else(|| all.iter().find(|(f, _)| f.key.eq_ignore_ascii_case(query)))
        .or_else(|| {
            all.iter()
                .find(|(f, _)| folder_name(f).eq_ignore_ascii_case(query))
        })
        .copied()
}

fn folder_name(feature: &Feature) -> &str {
    feature.path.rsplit('/').next().unwrap_or_default()
}

fn feature_not_found(query: &str) -> anyhow::Error {
    anyhow::anyhow!(
        "No feature matches '{}'. Use list_features to see the features.",
        query
    )
}

/// First paragraph of a README body, on one line, at most `SUMMARY_MAX_CHARS`
pub fn summarize(readme: &str) -> String {
    let paragraph: Vec<&str> = readme
        .lines()
        .map(str::trim)
        .skip_while(|line| line.is_empty() || line.starts_with('#'))
        .take_while(|line| !line.is_empty())
        .collect();
    let summary = paragraph.join(" ");
    if summary.chars().count() <= SUMMARY_MAX_CHARS {
        return summary;
    }
    let truncated: String = summary.chars().take(SUMMARY_MAX_CHARS - 1).collect();
    format!("{}…", truncated.trim_end())
}

fn normalize_owner(owner: &str) -> String {
    owner.trim().trim_start_matches('@').to_lowercase()
}

fn list_features(snapshot: &Snapshot, base_path: &Path, args: &Map<String, Value>) -> Value {
    let owner = string_arg(args, "owner")
        .ok()
        .flatten()
        .map(normalize_owner);
    let query = string_arg(args, "query")
        .ok()
        .flatten()
        .map(|query| query.trim().to_lowercase())
        .filter(|query| !query.is_empty());

    let features: Vec<Value> = all_features(&snapshot.features)
        .into_iter()
        .filter(|(feature, _)| {
            owner
                .as_ref()
                .is_none_or(|owner| normalize_owner(&feature.owner) == *owner)
        })
        .filter(|(feature, _)| {
            query.as_ref().is_none_or(|query| {
                [
                    &feature.name,
                    &feature.key,
                    &feature.path,
                    &feature.description,
                ]
                .iter()
                .any(|field| field.to_lowercase().contains(query))
            })
        })
        .map(|(feature, parent)| {
            json!({
                "name": feature.name,
                "key": feature.key,
                "path": feature.path,
                "owner": feature.owner,
                "parent": parent.map(|parent| parent.path.clone()),
                "summary": summarize(&feature.description),
            })
        })
        .collect();

    json!({
        "root": absolute(base_path).display().to_string(),
        "count": features.len(),
        "features": features,
    })
}

fn get_feature(snapshot: &Snapshot, base_path: &Path, args: &Map<String, Value>) -> Result<Value> {
    let query = required_string_arg(args, "feature")?;
    let (feature, parent) =
        find_feature(&snapshot.features, query).ok_or_else(|| feature_not_found(query))?;

    let mut stats = serde_json::to_value(&feature.stats)?;
    if let Some(coverage) = stats.get_mut("coverage").and_then(Value::as_object_mut) {
        // Per-file coverage can be huge: keep the totals
        coverage.remove("files");
    }

    let sub_features: Vec<Value> = feature
        .features
        .iter()
        .map(|child| {
            json!({
                "name": child.name,
                "key": child.key,
                "path": child.path,
                "owner": child.owner,
            })
        })
        .collect();

    Ok(json!({
        "name": feature.name,
        "key": feature.key,
        "path": feature.path,
        "absolute_path": absolute(&base_path.join(&feature.path)).display().to_string(),
        "owner": feature.owner,
        "is_owner_inherited": feature.is_owner_inherited,
        "parent": parent.map(|parent| json!({ "name": parent.name, "key": parent.key, "path": parent.path })),
        "features": sub_features,
        "readme": feature.description,
        "meta": feature.meta,
        "links": feature.links,
        "external": feature.external,
        "stats": stats,
        "records": feature.records.iter().map(record_summary).collect::<Vec<_>>(),
        "dependencies": dependency_summary(feature),
        "changes_count": feature.changes.len(),
        "recent_changes": feature.changes.iter().take(RECENT_CHANGES).collect::<Vec<_>>(),
    }))
}

/// Imports of other features, grouped by feature: the full list of import
/// lines is too long to be useful to an agent
fn dependency_summary(feature: &Feature) -> Vec<Value> {
    let mut grouped: Vec<(String, Value, usize)> = Vec::new();
    for dependency in &feature.dependencies {
        match grouped
            .iter_mut()
            .find(|(path, _, _)| *path == dependency.feature_path)
        {
            Some((_, _, count)) => *count += 1,
            None => grouped.push((
                dependency.feature_path.clone(),
                serde_json::to_value(&dependency.dependency_type).unwrap_or(Value::Null),
                1,
            )),
        }
    }
    grouped
        .into_iter()
        .map(|(path, kind, imports)| json!({ "feature": path, "type": kind, "imports": imports }))
        .collect()
}

/// A record without its content: the agent can read the file at `path`
fn record_summary(record: &Record) -> Value {
    let mut summary = json!({
        "kind": record.kind,
        "id": record.id,
        "title": record.title,
        "path": record.path,
    });
    let object = summary.as_object_mut().expect("summary is an object");
    if let Some(status) = &record.status {
        object.insert("status".into(), json!(status));
    }
    if let Some(date) = &record.date {
        object.insert("date".into(), json!(date));
    }
    if let Some(feature) = &record.feature {
        object.insert("feature".into(), json!(feature));
    }
    if let Some(tasks) = &record.tasks {
        object.insert("tasks".into(), json!(tasks));
    }
    if !record.source.is_native() {
        object.insert("source".into(), json!(record.source));
    }
    if !record.links.is_empty() {
        object.insert("links".into(), json!(record.links));
    }
    if !record.files.is_empty() {
        let files: Vec<&str> = record.files.iter().map(|f| f.path.as_str()).collect();
        object.insert("files".into(), json!(files));
    }
    summary
}

fn absolute(path: &Path) -> PathBuf {
    std::fs::canonicalize(path)
        .or_else(|_| std::path::absolute(path))
        .unwrap_or_else(|_| path.to_path_buf())
}

/// Drop `.` components and resolve `..` lexically
fn lexical_normalize(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other),
        }
    }
    normalized
}

/// `target` relative to the scanned folder. Absolute paths and paths relative
/// to the current directory are used when they are inside the scanned folder;
/// other relative paths are taken as relative to the scanned folder.
pub fn relative_to_base(target: &str, base_path: &Path, current_dir: &Path) -> Option<PathBuf> {
    let target_path = Path::new(target.trim());
    let candidate = if target_path.is_absolute() {
        target_path.to_path_buf()
    } else {
        current_dir.join(target_path)
    };

    let absolute_base = lexical_normalize(&current_dir.join(base_path));
    let mut bases = vec![absolute_base.clone()];
    if let Ok(canonical) = std::fs::canonicalize(&absolute_base) {
        bases.push(canonical);
    }
    let mut candidates = vec![lexical_normalize(&candidate)];
    if let Ok(canonical) = std::fs::canonicalize(&candidate) {
        candidates.push(canonical);
    }

    for candidate in &candidates {
        for base in &bases {
            if let Ok(relative) = candidate.strip_prefix(base) {
                return Some(relative.to_path_buf());
            }
        }
    }

    (!target_path.is_absolute()).then(|| lexical_normalize(target_path))
}

fn find_owner(
    snapshot: &Snapshot,
    base_path: &Path,
    current_dir: &Path,
    args: &Map<String, Value>,
) -> Result<Value> {
    let target = required_string_arg(args, "path")?;
    let Some(relative) = relative_to_base(target, base_path, current_dir) else {
        bail!(
            "'{}' is outside the scanned folder '{}'",
            target,
            absolute(base_path).display()
        );
    };
    let Some(info) = find_owner_for_relative_path(&relative, &snapshot.features) else {
        bail!("'{}' is not inside any feature", target);
    };
    let key = find_feature(&snapshot.features, &info.feature_path)
        .map(|(feature, _)| feature.key.clone())
        .unwrap_or_default();

    Ok(json!({
        "path": relative.to_string_lossy().replace('\\', "/"),
        "owner": info.owner,
        "inherited": info.inherited,
        "feature_name": info.feature_name,
        "feature_key": key,
        "feature_path": info.feature_path,
    }))
}

fn features_touched(
    snapshot: &Snapshot,
    base_path: &Path,
    args: &Map<String, Value>,
) -> Result<Value> {
    let since = required_string_arg(args, "since")?;
    let include_uncommitted = !bool_arg(args, "committed_only")?;
    let changes = diff::collect_changes(base_path, since, include_uncommitted)?;
    let report = diff::build_report(since, include_uncommitted, &changes, &snapshot.features);
    Ok(serde_json::to_value(report)?)
}

fn list_records(snapshot: &Snapshot, args: &Map<String, Value>) -> Result<Value> {
    let kind = match string_arg(args, "kind")?.map(|kind| kind.trim().to_lowercase()) {
        None => None,
        Some(kind) if kind.is_empty() => None,
        Some(kind) => Some(match kind.as_str() {
            "spec" | "specs" => RecordKind::Spec,
            "decision" | "decisions" | "adr" => RecordKind::Decision,
            other => bail!("'kind' must be 'spec' or 'decision', not '{}'", other),
        }),
    };

    let (scope, records) = match string_arg(args, "feature")?.filter(|f| !f.trim().is_empty()) {
        Some(query) => {
            let (feature, _) =
                find_feature(&snapshot.features, query).ok_or_else(|| feature_not_found(query))?;
            (feature.path.clone(), &feature.records)
        }
        None => ("project".to_string(), &snapshot.records),
    };

    let records: Vec<Value> = records
        .iter()
        .filter(|record| kind.is_none_or(|kind| record.kind == kind))
        .map(record_summary)
        .collect();

    Ok(json!({
        "scope": scope,
        "count": records.len(),
        "records": records,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn feature(name: &str, path: &str, children: Vec<Feature>) -> Feature {
        Feature {
            name: name.to_string(),
            key: crate::feature_key::slugify_key(path.rsplit('/').next().unwrap()),
            description: String::new(),
            owner: "team".to_string(),
            is_owner_inherited: false,
            path: path.to_string(),
            features: children,
            meta: BTreeMap::new(),
            links: BTreeMap::new(),
            changes: Vec::new(),
            decisions: Vec::new(),
            records: Vec::new(),
            stats: None,
            dependencies: Vec::new(),
            external: BTreeMap::new(),
        }
    }

    fn server() -> McpServer {
        McpServer::new(".", ScanConfig::new("."))
    }

    fn request(server: &mut McpServer, method: &str, params: Value) -> Value {
        server
            .handle_message(
                &json!({ "jsonrpc": "2.0", "id": 1, "method": method, "params": params }),
            )
            .unwrap()
    }

    #[test]
    fn initialize_echoes_a_supported_protocol_version() {
        let response = request(
            &mut server(),
            "initialize",
            json!({ "protocolVersion": "2025-03-26", "capabilities": {}, "clientInfo": { "name": "test", "version": "1" } }),
        );
        let result = &response["result"];
        assert_eq!(response["id"], 1);
        assert_eq!(result["protocolVersion"], "2025-03-26");
        assert_eq!(result["serverInfo"]["name"], "features");
        assert!(result["capabilities"]["tools"].is_object());
        assert!(
            result["instructions"]
                .as_str()
                .unwrap()
                .contains("list_features")
        );
    }

    #[test]
    fn initialize_falls_back_to_the_latest_protocol_version() {
        let response = request(
            &mut server(),
            "initialize",
            json!({ "protocolVersion": "1999-01-01" }),
        );
        assert_eq!(
            response["result"]["protocolVersion"],
            SUPPORTED_PROTOCOL_VERSIONS[0]
        );
    }

    #[test]
    fn ping_returns_an_empty_result() {
        assert_eq!(
            request(&mut server(), "ping", Value::Null)["result"],
            json!({})
        );
    }

    #[test]
    fn notifications_get_no_response() {
        let mut server = server();
        let notification = json!({ "jsonrpc": "2.0", "method": "notifications/initialized" });
        assert!(server.handle_message(&notification).is_none());
        // Responses from the client are ignored too
        assert!(
            server
                .handle_message(&json!({ "jsonrpc": "2.0", "id": 3, "result": {} }))
                .is_none()
        );
    }

    #[test]
    fn unknown_method_is_an_error() {
        let response = request(&mut server(), "resources/list", Value::Null);
        assert_eq!(response["error"]["code"], METHOD_NOT_FOUND);
    }

    #[test]
    fn invalid_json_is_a_parse_error() {
        let response: Value =
            serde_json::from_str(&server().handle_line("{not json").unwrap()).unwrap();
        assert_eq!(response["error"]["code"], PARSE_ERROR);
        assert_eq!(response["id"], Value::Null);
    }

    #[test]
    fn invalid_requests_are_errors() {
        let mut server = server();
        let response = server.handle_message(&json!(42)).unwrap();
        assert_eq!(response["error"]["code"], INVALID_REQUEST);
        let response = server.handle_message(&json!({ "id": 2 })).unwrap();
        assert_eq!(response["error"]["code"], INVALID_REQUEST);
        assert_eq!(response["id"], 2);
        let response = server.handle_message(&json!([])).unwrap();
        assert_eq!(response["error"]["code"], INVALID_REQUEST);
    }

    #[test]
    fn batches_are_answered_with_an_array() {
        let mut server = server();
        let response = server
            .handle_message(&json!([
                { "jsonrpc": "2.0", "id": 1, "method": "ping" },
                { "jsonrpc": "2.0", "method": "notifications/initialized" },
                { "jsonrpc": "2.0", "id": 2, "method": "ping" },
            ]))
            .unwrap();
        let ids: Vec<&Value> = response
            .as_array()
            .unwrap()
            .iter()
            .map(|r| &r["id"])
            .collect();
        assert_eq!(ids, vec![&json!(1), &json!(2)]);
    }

    #[test]
    fn tools_list_declares_every_tool_as_read_only() {
        let response = request(&mut server(), "tools/list", Value::Null);
        let tools = response["result"]["tools"].as_array().unwrap();
        let names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
        assert_eq!(
            names,
            vec![
                "list_features",
                "get_feature",
                "find_owner",
                "features_touched",
                "list_records"
            ]
        );
        for tool in tools {
            assert_eq!(tool["inputSchema"]["type"], "object");
            assert_eq!(tool["annotations"]["readOnlyHint"], true);
            assert!(!tool["description"].as_str().unwrap().is_empty());
        }
    }

    #[test]
    fn calling_an_unknown_tool_is_an_invalid_params_error() {
        let response = request(
            &mut server(),
            "tools/call",
            json!({ "name": "delete_everything", "arguments": {} }),
        );
        assert_eq!(response["error"]["code"], INVALID_PARAMS);
        let response = request(&mut server(), "tools/call", json!({}));
        assert_eq!(response["error"]["code"], INVALID_PARAMS);
        let response = request(
            &mut server(),
            "tools/call",
            json!({ "name": "list_features", "arguments": [] }),
        );
        assert_eq!(response["error"]["code"], INVALID_PARAMS);
    }

    #[test]
    fn find_feature_prefers_key_then_path_then_name_then_folder() {
        let features = vec![
            feature(
                "Auth",
                "features/auth",
                vec![feature(
                    "Login Flow",
                    "features/auth/features/login",
                    vec![],
                )],
            ),
            feature("Billing", "features/billing", vec![]),
        ];

        let (found, parent) = find_feature(&features, "login").unwrap();
        assert_eq!(found.name, "Login Flow");
        assert_eq!(parent.unwrap().name, "Auth");

        let by_path = find_feature(&features, "./features/billing/").unwrap().0;
        assert_eq!(by_path.name, "Billing");
        let by_name = find_feature(&features, "login flow").unwrap().0;
        assert_eq!(by_name.path, "features/auth/features/login");
        let by_key_case = find_feature(&features, "AUTH").unwrap().0;
        assert_eq!(by_key_case.name, "Auth");
        assert!(find_feature(&features, "nope").is_none());
    }

    #[test]
    fn summarize_keeps_the_first_paragraph() {
        assert_eq!(summarize(""), "");
        assert_eq!(
            summarize("\nSigns users in.\nWith a password.\n\n## Details\nMore."),
            "Signs users in. With a password."
        );
        assert_eq!(summarize("## Usage\n\nRun it."), "Run it.");
        let long = "a".repeat(300);
        let summary = summarize(&long);
        assert_eq!(summary.chars().count(), SUMMARY_MAX_CHARS);
        assert!(summary.ends_with('…'));
    }

    #[test]
    fn relative_to_base_accepts_every_kind_of_path() {
        let cwd = Path::new("/repo");
        let base = Path::new("src");
        // Relative to the current directory, inside the scanned folder
        assert_eq!(
            relative_to_base("src/features/a/x.ts", base, cwd),
            Some(PathBuf::from("features/a/x.ts"))
        );
        // Absolute
        assert_eq!(
            relative_to_base("/repo/src/features/a", base, cwd),
            Some(PathBuf::from("features/a"))
        );
        // Relative to the scanned folder
        assert_eq!(
            relative_to_base("./features/a/../b/y.ts", base, cwd),
            Some(PathBuf::from("features/b/y.ts"))
        );
        // Absolute, outside the scanned folder
        assert_eq!(relative_to_base("/elsewhere/x.ts", base, cwd), None);
    }
}
