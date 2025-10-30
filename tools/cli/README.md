# Features CLI

A zero-config CLI tool for discovering and managing features in a feature-based architecture project by detecting README.md or README.mdx files in directories.

## Getting Started

```bash
npm install -g features-cli
# or
cargo binstall features-cli


features /path/to/project # list all features in the directory and subdirectories
```

## Feature Detection

The CLI automatically detects features using two methods:

### Method 1: Features folder (default)
Any directory that is a direct subfolder of a `features` directory is considered a feature.

### Method 2: Feature flag in README
Any directory with a README.md or README.mdx file containing `feature: true` in the YAML frontmatter is considered a feature.

Both methods support:
- **README formats**: Both README.md and README.mdx files
- **Metadata parsing**: Extracts owner and metadata from YAML frontmatter
- **Description extraction**: Uses content after the first heading as feature description
- **Nested features**: Supports hierarchical feature organization (via `features` subfolder or nested directories with `feature: true`)
- **Documentation exclusion**: Ignores README files in documentation directories (docs, __docs__, decisions, etc.)

### README Format Examples

#### Standard feature (in features folder):
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

#### Explicit feature (anywhere with feature flag):
```markdown
---
feature: true
owner: Team Name
status: active
---

# Feature Name

This feature can be located anywhere in your codebase, not just in a features folder.
```

Commands and their descriptions are listed below:

| Command | Description |
| ------- | ----------- |
| `--json` | Output features as JSON |
| `--flat` | Output features as a flat array instead of nested structure |
| `--description` | Include feature descriptions in the output. The description is automatically included in the json format |
| `--list-owners` | Display only unique list of owners |
| `--check` | Run validation checks on features (e.g., duplicate names) |
| `--skip-changes` | Skip computing git commit history (faster for large repos) |
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
