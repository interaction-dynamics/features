---
status: accepted
date: 2026-11-15
spec: 011-mcp-server
---

# Hand-roll the MCP server instead of using an MCP crate

## Context and problem statement

`features mcp` ([spec 011](../specs/011-mcp-server.md)) runs a Model Context Protocol server over stdio. It must ship in the default binary distributed by npm, Homebrew and crates.io, which we want to keep small and quick to build. Which implementation of the protocol should it use?

## Considered options

- **The official Rust SDK, `rmcp`.** Complete (tools, resources, prompts, sampling, stdio and HTTP transports), maintained by the MCP organization. It pulls in its macro crate, `schemars` for JSON Schema generation, `futures`, `tokio-util` and more, and its API still changes between minor versions.
- **A hand-rolled JSON-RPC loop.** MCP over stdio is newline-delimited JSON-RPC 2.0. A read-only tools server only needs `initialize`, `ping`, `tools/list` and `tools/call`, which is about a hundred lines on top of `serde_json` and `tokio`, both already dependencies.
- **`rmcp` behind a Cargo feature.** Keeps the default binary light, but then the default npm/brew build has no `mcp` command, which defeats the purpose.

## Decision outcome

Chosen option: **a hand-rolled JSON-RPC loop** (`projects/cli/src/mcp_server.rs`), because the subset of the protocol we need is small and stable, it adds no dependency, and it works in every distribution of the CLI.

### Consequences

- Good: no new crate, no impact on binary size, build time or `cargo deny`.
- Good: the dispatch is a plain function from a JSON message to a JSON response, unit-tested without a transport, and the binary is tested end to end over stdin/stdout.
- Bad: new protocol versions have to be followed by hand. The server negotiates the versions it knows (`2024-11-05` to `2025-11-25`) and answers with its latest one otherwise; the methods it implements have not changed across them.
- Bad: resources, prompts, sampling and the HTTP transport are not available. If we need them, switching to `rmcp` is the way, and the tool implementations (plain functions returning JSON) can move over unchanged.
