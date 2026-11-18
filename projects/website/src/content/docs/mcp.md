---
title: MCP server
description: Run features-cli as a Model Context Protocol server so Claude Code, Cursor and other AI coding agents can list features, find owners and read specs and decisions. Setup and tool reference.
---

```bash
features mcp ./src
```

runs a [Model Context Protocol](https://modelcontextprotocol.io) server over stdio (newline-delimited JSON-RPC 2.0). AI coding agents start it themselves when they need it: register it once. To see how it fits in an agent's workflow, and how to use the plain CLI instead, read [Features for AI coding agents](/ai).

## Register it

Claude Code:

```bash
claude mcp add features -- npx features-cli@latest mcp ./src
```

Cursor, in `.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "features": {
      "command": "npx",
      "args": ["features-cli@latest", "mcp", "./src"]
    }
  }
}
```

Other agents take the same command: `npx features-cli@latest mcp ./src`.

## Tools

| Tool               | Arguments                                                                    | Returns                                                                                                                                                                                                                                                     |
| ------------------ | ---------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `list_features`    | `owner`, `query` (both optional)                                             | `root` (absolute path of the scanned folder) and every feature, flat: `name`, `key`, `path`, `owner`, `parent` (path of the parent feature) and `summary` (first paragraph of the README)                                                                   |
| `get_feature`      | `feature`: key, path, name or folder name                                    | Name, key, path, owner, parent, sub-features, README, `meta`, `links`, `external` (`--data`), stats (files, lines, TODOs, commits, coverage totals), records (summaries with their `path`), dependencies grouped by feature, and the 10 most recent commits |
| `find_owner`       | `path`: absolute, relative to the current directory or to the scanned folder | `owner`, `inherited`, `feature_name`, `feature_key`, `feature_path`. The path does not need to exist                                                                                                                                                        |
| `features_touched` | `since` (git ref), `committed_only` (optional)                               | The same report as `features diff --json`                                                                                                                                                                                                                   |
| `list_records`     | `kind` (`spec` or `decision`), `feature` (both optional)                     | The project-level specs and decision records, or those of a feature, without their content                                                                                                                                                                  |

The `owner` filter of `list_features` ignores case and a leading `@`; `query` searches the name, key, path and README.

## Behavior

- **Read-only and offline.** No tool changes a file or calls the network.
- **Errors are results.** A tool that fails (unknown feature, unknown git ref) returns an error result the agent can read.
- **Scanned on demand.** The project is scanned on the first tool call, with coverage and git history (unless `--skip-changes`), and rescanned after a file that can change the features changes, or a `--data` file is updated.
- **Same options.** The scan options (`--skip-changes`, `--ignore-path`, `--data`, `--strip-emails`, `--coverage-dir`, `--project-dir`...) work as for the other commands, and so does the [configuration file](/docs/configuration).
- **Clean stdout.** stdout only carries protocol messages; logs go to stderr.

On large repositories, `--skip-changes` keeps the first call fast. To expose data from other tools to the agent, add [`--data`](/docs/external-data).
