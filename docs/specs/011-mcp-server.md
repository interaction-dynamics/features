---
status: implemented
date: 2026-11-15
---

# MCP server (`features mcp`)

## Context

Phase 3 of the [integrations roadmap](../roadmap/integrations.md). The pitch of the project is giving AI coding agents a map of the codebase by feature. Today an agent has to run `features scan --json` and parse a large report. An MCP (Model Context Protocol) server lets Claude Code, Cursor and other agents ask focused questions instead: which features exist, who owns this file, what does this feature document, which features does my branch touch.

## Requirements

### Command

- `features mcp [PATH]` (default `.`) runs an MCP server over stdio: newline-delimited JSON-RPC 2.0 on stdin/stdout.
- It accepts the shared scan options: `--skip-changes`, `--ignore-path`, `--include-skipped-dirs`, `--coverage-dir`, `--project-dir`, `--strip-emails`, `--data`.
- stdout carries only protocol messages. Logs and warnings go to stderr.
- Offline: no network calls.
- The server stops when stdin is closed.

### Protocol

- Methods: `initialize`, `ping`, `tools/list`, `tools/call`. Notifications (`notifications/initialized`, `notifications/cancelled`...) are accepted and ignored.
- `initialize` answers with the protocol version the client asked for when it is supported (`2024-11-05`, `2025-03-26`, `2025-06-18`, `2025-11-25`), otherwise the latest one. It declares the `tools` capability, the server name (`features`) and version, and short instructions for the agent.
- JSON-RPC errors: parse error (-32700), invalid request (-32600), unknown method (-32601), invalid params or unknown tool (-32602). A JSON array (batch) is answered with an array.
- A tool that fails (unknown feature, git ref not found, scan error) returns a result with `isError: true` and a message, so the agent can read it.
- Tool results are one `text` content item holding pretty-printed JSON.

### Scan and cache

- The project is scanned lazily, on the first tool call, with coverage, and with git history unless `--skip-changes`.
- The result is cached. A file watcher (same filter as `features serve`) marks it stale when files that can change the features change; the next tool call rescans. `--data` files are also checked by modification time on every call.
- If the watcher cannot start, the server rescans on every call and says so on stderr.

### Tools

- `list_features`: flat list of every feature with `name`, `key`, `path`, `owner`, `parent` (path of the parent feature, if nested) and `summary` (first paragraph of the README, at most 200 characters). Optional filters: `owner` (case insensitive, leading `@` ignored) and `query` (case-insensitive substring of name, key, path or README). The response also gives the absolute `root` the paths are relative to.
- `get_feature`: one feature found by `feature` = key, path, name (case insensitive) or last path segment. Returns name, key, path, owner (and whether inherited), parent, sub-features (name, key, path), README, metadata, links, external data (`--data`), stats (files, lines, TODOs, commits, coverage totals without per-file details), records (summaries: kind, id, title, status, date, path, tasks, links), dependencies, and the 10 most recent changes with the total count.
- `find_owner`: owner of `path`, a file or folder. The path can be absolute, relative to the current directory, or relative to the scanned folder; it does not need to exist. Returns owner, inherited, feature name and path, or an error when the path is outside every feature.
- `features_touched`: same as `features diff --since <since>`: the features touched since the merge base of `since` and HEAD, with owners and files, and the files outside features. Optional `committed_only`.
- `list_records`: project-level specs and decisions (root `docs/` records not attached to a feature), as summaries. Optional `kind` (`spec` or `decision`) and `feature` (then the records of that feature instead).

### Dependencies

- No MCP crate: the protocol subset is a small JSON-RPC loop on top of `serde_json` and `tokio`, which the CLI already uses. See [decision 001](../decisions/001-hand-rolled-mcp-server.md).

## Out of scope

- Resources, prompts, sampling and the HTTP transports of MCP.
- Write tools (creating records, editing READMEs).
- Pagination of tool results.

## Plan

1. Add `Command::Mcp` / `Action::Mcp` to `cli.rs` with the shared scan options.
2. Add `src/mcp_server.rs`: JSON-RPC dispatch (pure, testable), tool implementations on top of the scan, `ownership`, `diff` and `records` modules, cache with stale flag, stdio loop generic over reader/writer, watcher task.
3. Wire it in `bin.rs`.
4. Unit tests in the module, integration tests in `tests/mcp_server_test.rs` on `examples/specs-decisions`, and an end-to-end test driving the binary over stdio.
5. Docs: `docs/cli.md`, README section "Give your AI coding agent a map of the codebase", roadmap item marked shipped, decision record.
