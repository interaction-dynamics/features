# Features CLI

A CLI tool for discovering and managing features in a feature-based architecture project.

## Getting Started

```bash
cargo binstall features-cli

features /path/to/features # list all the features in the directory
```

Commands and their descriptions are listed below:

| Command | Description |
| ------- | ----------- |
| `--json` | Output features as JSON |
| `--flat` | Output features as a flat array instead of nested structure |
| `--description` | Include feature descriptions in the output |
| `--list-owners` | Display only unique list of owners |
| `--check` | Run validation checks on features (e.g., duplicate names) |
| `--serve` | Start an HTTP server to serve features and the web dashboard UI |
| `--serve --port 8080` | Start an HTTP server on specified port |
| `--build` | Build a static version of the web dashboard UI |

## Contributing

From the CLI directory:

```bash

cargo run ../../examples/javascript-basic/src/features

# For the web dashboard
cd ../web
pnpm build_for_cli
cd ../cli

cargo run ../../examples/javascript-basic/src/features --serve # for the Web dashboard
```

## Deploy

```bash
cd ../web
pnpm build_for_cli
cd ../cli

cargo publish --allow-dirty
```
