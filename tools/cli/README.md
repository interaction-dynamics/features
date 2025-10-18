# Features CLI

A CLI tool for discovering and managing features in a feature-based architecture project by detecting README.md or README.mdx files in directories.

## Getting Started

```bash
npm install -g features-cli
# or
cargo binstall features-cli


features /path/to/project # list all features in the directory and subdirectories
```

## Feature Detection

The CLI automatically detects features by scanning directories for README.md or README.mdx files. Any directory containing a README file is considered a feature, with the following rules:

- **README formats**: Supports both README.md and README.mdx files
- **Metadata parsing**: Extracts owner and metadata from YAML frontmatter
- **Description extraction**: Uses content after the first heading as feature description
- **Nested features**: Supports hierarchical feature organization
- **Documentation exclusion**: Ignores README files in documentation directories (docs, __docs__, decisions, etc.)

### README Format Example

```markdown
---
owner: Team Name
figma: https://www.figma.com/file/1234567890/Feature-Name?node-id=0%3A1
status: active
version: 1.0.0
tags: ["authentication", "security"]
---

# Feature Name

This is the feature description that will be extracted by the CLI.

## Additional sections are included in the description
```

Commands and their descriptions are listed below:

| Command | Description |
| ------- | ----------- |
| `--json` | Output features as JSON |
| `--flat` | Output features as a flat array instead of nested structure |
| `--description` | Include feature descriptions in the output. The description is automatically included in the json format |
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
