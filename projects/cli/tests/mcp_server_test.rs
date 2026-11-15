//! MCP server (`features mcp`), driven over JSON-RPC like a real client:
//! in-process on `examples/specs-decisions` and on temporary projects, and
//! end to end through the binary's stdin/stdout

use features_cli::mcp_server::{McpServer, serve};
use features_cli::scan::ScanConfig;
use serde_json::{Value, json};
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::Ordering;

fn example_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/specs-decisions")
}

fn example_server() -> McpServer {
    let path = example_path();
    McpServer::new(&path, ScanConfig::new(&path).skip_changes(true))
}

/// Send `messages` (one per line) and return the responses
fn exchange(server: &mut McpServer, messages: &[Value]) -> Vec<Value> {
    let input: String = messages.iter().map(|m| format!("{}\n", m)).collect();
    let mut output = Vec::new();
    serve(server, Cursor::new(input), &mut output).unwrap();
    String::from_utf8(output)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

/// Call a tool and return its parsed JSON result, or its error message
fn call(server: &mut McpServer, name: &str, arguments: Value) -> Result<Value, String> {
    let request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": { "name": name, "arguments": arguments },
    });
    let response = exchange(server, &[request]).remove(0);
    let result = &response["result"];
    let text = result["content"][0]["text"].as_str().unwrap().to_string();
    if result["isError"] == true {
        Err(text)
    } else {
        Ok(serde_json::from_str(&text).unwrap())
    }
}

#[test]
fn test_initialize_then_list_tools() {
    let responses = exchange(
        &mut example_server(),
        &[
            json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {
                "protocolVersion": "2025-06-18",
                "capabilities": {},
                "clientInfo": { "name": "test", "version": "1.0" },
            }}),
            json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }),
            json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/list" }),
        ],
    );

    // No response to the notification
    assert_eq!(responses.len(), 2);
    assert_eq!(responses[0]["result"]["protocolVersion"], "2025-06-18");
    assert_eq!(responses[1]["id"], 2);
    assert_eq!(responses[1]["result"]["tools"].as_array().unwrap().len(), 5);
}

#[test]
fn test_list_features() {
    let mut server = example_server();
    let result = call(&mut server, "list_features", json!({})).unwrap();
    assert_eq!(result["count"], 2);
    assert_eq!(
        result["features"][0],
        json!({
            "name": "Auth",
            "key": "auth",
            "path": "features/auth",
            "owner": "team-auth",
            "parent": null,
            "summary": "Signs users in.",
        })
    );
    assert!(
        Path::new(result["root"].as_str().unwrap())
            .join("features/auth")
            .is_dir()
    );

    let by_owner = call(
        &mut server,
        "list_features",
        json!({ "owner": "@Team-Billing" }),
    )
    .unwrap();
    assert_eq!(by_owner["count"], 1);
    assert_eq!(by_owner["features"][0]["key"], "billing");

    let by_query = call(
        &mut server,
        "list_features",
        json!({ "query": "signs users" }),
    )
    .unwrap();
    assert_eq!(by_query["count"], 1);
    assert_eq!(by_query["features"][0]["key"], "auth");
}

#[test]
fn test_get_feature() {
    let mut server = example_server();
    let auth = call(
        &mut server,
        "get_feature",
        json!({ "feature": "features/auth" }),
    )
    .unwrap();
    assert_eq!(auth["name"], "Auth");
    assert_eq!(auth["owner"], "team-auth");
    assert_eq!(auth["readme"], "Signs users in.");

    let records = auth["records"].as_array().unwrap();
    let ids: Vec<&str> = records.iter().map(|r| r["id"].as_str().unwrap()).collect();
    assert!(ids.contains(&"002-use-jwt"));
    // The project-level spec attached with `feature: auth`
    assert!(ids.contains(&"001-auth-overhaul"));
    // Summaries only: the agent reads the file at `path`
    assert!(records.iter().all(|r| r.get("content").is_none()));
    let sso = records.iter().find(|r| r["id"] == "003-sso").unwrap();
    assert_eq!(sso["tasks"], json!({ "done": 1, "total": 3 }));

    // By name, case insensitive
    let billing = call(&mut server, "get_feature", json!({ "feature": "BILLING" })).unwrap();
    assert_eq!(billing["key"], "billing");

    let missing = call(&mut server, "get_feature", json!({ "feature": "payments" })).unwrap_err();
    assert!(missing.contains("payments"), "{}", missing);
    let no_argument = call(&mut server, "get_feature", json!({})).unwrap_err();
    assert!(no_argument.contains("feature"), "{}", no_argument);
}

#[test]
fn test_find_owner() {
    let mut server = example_server();

    // Relative to the scanned folder; the file does not need to exist
    let owner = call(
        &mut server,
        "find_owner",
        json!({ "path": "features/billing/invoice.ts" }),
    )
    .unwrap();
    assert_eq!(owner["owner"], "team-billing");
    assert_eq!(owner["feature_key"], "billing");
    assert_eq!(owner["feature_path"], "features/billing");

    // Absolute
    let readme = example_path().join("features/auth/README.md");
    let owner = call(
        &mut server,
        "find_owner",
        json!({ "path": readme.to_string_lossy() }),
    )
    .unwrap();
    assert_eq!(owner["owner"], "team-auth");
    assert_eq!(owner["path"], "features/auth/README.md");

    let outside = call(&mut server, "find_owner", json!({ "path": "docs/specs" })).unwrap_err();
    assert!(outside.contains("not inside any feature"), "{}", outside);
}

#[test]
fn test_list_records() {
    let mut server = example_server();
    let project = call(&mut server, "list_records", json!({})).unwrap();
    assert_eq!(project["scope"], "project");
    let ids: Vec<&str> = project["records"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["id"].as_str().unwrap())
        .collect();
    assert_eq!(ids, vec!["0001-use-a-monorepo", "002-design-system"]);

    let specs = call(&mut server, "list_records", json!({ "kind": "spec" })).unwrap();
    assert_eq!(specs["count"], 1);
    assert_eq!(specs["records"][0]["kind"], "spec");

    let billing = call(
        &mut server,
        "list_records",
        json!({ "feature": "billing", "kind": "decision" }),
    )
    .unwrap();
    assert_eq!(billing["scope"], "features/billing");
    assert_eq!(billing["count"], 2);

    let bad_kind = call(&mut server, "list_records", json!({ "kind": "note" })).unwrap_err();
    assert!(bad_kind.contains("kind"), "{}", bad_kind);
}

#[test]
fn test_features_touched() {
    let mut server = example_server();
    // Nothing is committed between HEAD and itself
    let report = call(
        &mut server,
        "features_touched",
        json!({ "since": "HEAD", "committed_only": true }),
    )
    .unwrap();
    assert_eq!(report["since"], "HEAD");
    assert_eq!(report["features"], json!([]));
    assert_eq!(report["files_outside_features"], json!([]));
    assert_eq!(report["scan_root"], "examples/specs-decisions");

    let bad_ref = call(
        &mut server,
        "features_touched",
        json!({ "since": "no-such-ref-for-sure" }),
    )
    .unwrap_err();
    assert!(!bad_ref.is_empty());
}

fn write_feature(root: &Path, name: &str, owner: &str) {
    let dir = root.join("features").join(name);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("README.md"),
        format!(
            "---\nowner: {}\n---\n\n# {}\n\nDoes {}.\n",
            owner, name, name
        ),
    )
    .unwrap();
}

#[test]
fn test_cache_is_refreshed_when_stale() {
    let dir = tempfile::tempdir().unwrap();
    write_feature(dir.path(), "alpha", "team-a");
    let mut server = McpServer::new(dir.path(), ScanConfig::new(dir.path()).skip_changes(true));

    let count =
        |server: &mut McpServer| call(server, "list_features", json!({})).unwrap()["count"].clone();
    assert_eq!(count(&mut server), 1);

    // Cached until the watcher marks the features as stale
    write_feature(dir.path(), "beta", "team-b");
    assert_eq!(count(&mut server), 1);
    server.stale_flag().store(true, Ordering::SeqCst);
    assert_eq!(count(&mut server), 2);
}

#[test]
fn test_data_files_are_read_again_when_they_change() {
    let dir = tempfile::tempdir().unwrap();
    write_feature(dir.path(), "alpha", "team-a");
    let data = dir.path().join("usage.json");
    std::fs::write(&data, r#"{ "alpha": { "weekly_users": 10 } }"#).unwrap();

    let config = ScanConfig::new(dir.path())
        .skip_changes(true)
        .data_files(vec![data.clone()]);
    let mut server = McpServer::new(dir.path(), config);

    let users = |server: &mut McpServer| {
        call(server, "get_feature", json!({ "feature": "alpha" })).unwrap()["external"]
            ["weekly_users"]
            .clone()
    };
    assert_eq!(users(&mut server), 10);

    std::fs::write(&data, r#"{ "alpha": { "weekly_users": 25 } }"#).unwrap();
    // Make sure the modification time changes, whatever the file system precision
    let later = std::time::SystemTime::now() + std::time::Duration::from_secs(5);
    std::fs::File::options()
        .write(true)
        .open(&data)
        .unwrap()
        .set_modified(later)
        .unwrap();
    assert_eq!(users(&mut server), 25);
}

#[test]
fn test_binary_speaks_json_rpc_over_stdio() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_features"))
        .args(["mcp", "--skip-changes"])
        .arg(example_path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let messages = [
        json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {
            "protocolVersion": "2025-03-26",
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "1.0" },
        }}),
        json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }),
        json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/list" }),
        json!({ "jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {
            "name": "find_owner",
            "arguments": { "path": "features/auth/login.ts" },
        }}),
    ];
    {
        let mut stdin = child.stdin.take().unwrap();
        for message in &messages {
            writeln!(stdin, "{}", message).unwrap();
        }
        // Closing stdin stops the server
    }

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());

    // Every stdout line is a JSON-RPC response: logs go to stderr
    let responses: Vec<Value> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).expect("stdout must only carry JSON-RPC"))
        .collect();
    let ids: Vec<&Value> = responses.iter().map(|r| &r["id"]).collect();
    assert_eq!(ids, vec![&json!(1), &json!(2), &json!(3)]);
    assert!(responses.iter().all(|r| r["jsonrpc"] == "2.0"));
    assert_eq!(responses[0]["result"]["protocolVersion"], "2025-03-26");

    let text = responses[2]["result"]["content"][0]["text"]
        .as_str()
        .unwrap();
    let owner: Value = serde_json::from_str(text).unwrap();
    assert_eq!(owner["owner"], "team-auth");

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("MCP server"), "{}", stderr);
}
