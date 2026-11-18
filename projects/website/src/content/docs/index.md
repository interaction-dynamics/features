---
title: Introduction
description: Technical documentation of features-cli, the open-source CLI that maps the features of a codebase, their owners, history, coverage and records, and serves them in a web dashboard.
---

Features finds the features of a codebase from its folder structure and README files, then reports them: in the terminal, as JSON, in a web dashboard, in a CODEOWNERS file, in a pull request comment or to an AI coding agent. These pages are the reference for the CLI and its conventions. For an overview, start with [the concept](/feature-based-architecture) or the [CLI page](/cli).

## Install

Try it without installing anything:

```bash
npx features-cli@latest serve ./src
```

Or install the `features` binary with the tool you prefer:

```bash
npm install -g features-cli                     # npm
brew install interaction-dynamics/tap/features  # Homebrew, on macOS or Linux
cargo binstall features-cli                     # prebuilt binary from crates.io
```

Without a package manager, the [install script](/cli) downloads the binary for your OS and CPU, checks it and puts it in `~/.local/bin`; the CLI page shows its one-line command. The binary is written in Rust and has no runtime dependency.

## Quick start

1. Put each feature in a folder inside a `features` folder, or add `feature: true` to the front matter of its README. See [Folder structure](/docs/folder-structure).
2. Describe it in a `README.md` with an `owner`. See [README format](/docs/readme-format).
3. Run a command on the folder that holds your features:

```bash
features serve ./src             # open the dashboard on http://localhost:3000
features scan ./src              # list the features in the terminal
features check ./src             # validate them, for CI
features diff ./src --since main # features and owners touched by a change
```

4. Save the options your team uses in a [`features.config.yaml`](/docs/configuration) at the root of the repository, so every command stays short.

## How the CLI works

```bash
features <command> [PATH] [options]
```

- **Each action is a command.** `scan` is the default, so `features ./src` lists the features.
- **`PATH` is the folder to scan.** It defaults to the current directory, or to `path` in the configuration file.
- **Options have a long name** (`--skip-changes`, `--port 4000`) and the same name in the configuration file. Run `features <command> --help` for the options of a command.
- **Everything runs locally.** No account and no server to set up: the CLI reads the files and the git history of the repository.

## Where to go next

- [Commands](/docs/commands) and [Options](/docs/options): the full reference.
- [Configuration file](/docs/configuration): set every option once in `features.config.yaml`.
- [Check rules](/docs/check-rules): what `features check` validates and how to tune it.
- [Pull requests and CI](/docs/pull-requests-ci): comment on pull requests, request reviews from owners and fail builds on errors.
- [MCP server](/docs/mcp): give Claude Code, Cursor and other agents a map of the codebase.
