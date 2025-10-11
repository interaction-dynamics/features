# Features CLI

A CLI tool for discovering and managing features in a feature-based architecture project.

## Installation

From the CLI directory:

```bash
cargo build --release
```

## Usage

### Basic Usage

List all features in a directory:

```bash
cargo run -- /path/to/features
```

### JSON Output

Output features as JSON:

```bash
cargo run -- /path/to/features --json
```

### Flat Structure

Output features as a flat array instead of nested structure:

```bash
cargo run -- /path/to/features --flat
```

### Include Descriptions

Include feature descriptions in the output:

```bash
cargo run -- /path/to/features --description
```

### List Owners

Display only unique list of owners:

```bash
cargo run -- /path/to/features --list-owners
```

### Run Checks

Run validation checks on features (e.g., duplicate names):

```bash
cargo run -- /path/to/features --check
```

### HTTP Server

Start an HTTP server to serve features and static files:

```bash
cargo run -- /path/to/features --serve
```

By default, the server runs on port 3000. You can specify a different port:

```bash
cargo run -- /path/to/features --serve --port 8080
```

The server provides:
- `/features.json` - Features data in JSON format
- Static files from the `public/` directory
- Default index.html if no public/index.html exists

#### File Watching

When using `--serve`, the server automatically watches the specified directory for changes and recomputes features in real-time. The file watcher will:

- Monitor all `README.md` files for changes
- Detect when directories are created or removed
- Automatically update the features data served at `/features.json`
- Provide console feedback with status indicators:
  - 🔄 When changes are detected and recomputation starts
  - ✅ When features are successfully updated
  - ❌ When an error occurs during recomputation

This makes it perfect for development workflows where you're actively editing feature documentation and want to see changes immediately in connected dashboards or applications.

## Examples

### Analyze JavaScript Basic Example

```bash
cargo run -- ../../examples/javascript-basic
```

### Generate JSON for Web Dashboard

```bash
cargo run -- ../../examples/javascript-basic --json > ../web/public/features.json
```

### Serve Features Dashboard

```bash
cargo run -- ../../examples/javascript-basic --serve --port 3000
```

Then visit http://localhost:3000 to view the dashboard.

## Development

### Run Tests

```bash
cargo test
```

### Format Code

```bash
cargo fmt
```

### Lint Code

```bash
cargo clippy
```

## Contributing

1. Make sure all tests pass
2. Format your code with `cargo fmt`
3. Run `cargo clippy` to check for issues
4. Update documentation as needed