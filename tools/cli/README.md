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

#### Real-time Web Interface Updates

When combined with the built-in web interface, the file watching system provides seamless real-time updates:

- **Automatic Data Refresh**: The web interface polls `/features.json` every 2 seconds to check for updates
- **Smart Current Feature Tracking**: If you have a specific feature selected in the web UI, it will automatically update with fresh data when the underlying files change
- **Visual Update Indicators**: A small "Updating..." indicator appears in the header during data refresh
- **Intelligent Polling**: The system uses optimized SWR configuration with deduplication and smart caching
- **Seamless User Experience**: Selected features remain selected but show updated content immediately

#### Development Workflow Integration

Perfect for teams working on feature-driven architectures:

1. **Start the server**: `cargo run -- /path/to/features --serve`
2. **Open web interface**: Visit `http://localhost:3000` in your browser
3. **Edit feature docs**: Modify README.md files in your feature directories
4. **See instant updates**: Changes appear automatically in the web interface without page refresh
5. **Navigate seamlessly**: Selected features stay selected but show fresh content

This creates a powerful documentation-driven development experience where your feature documentation and web dashboard stay perfectly synchronized.

## Examples

### Analyze JavaScript Basic Example

```bash
cargo run -- ../../examples/javascript-basic
```

### Generate JSON for Web Dashboard

```bash
cargo run -- ../../examples/javascript-basic --json > ../web/public/features.json
```

### Serve Features Dashboard with Real-time Updates

```bash
cargo run -- ../../examples/javascript-basic --serve --port 3000
```

Then visit http://localhost:3000 to view the dashboard. The web interface will automatically update when you modify feature files.

### Live Development Workflow

```bash
# Start the server with file watching
cargo run -- /path/to/your/features --serve --port 3000

# In another terminal, make changes to feature documentation
echo "# Updated Feature\n**Owner:** new-team\n\nThis feature has been updated!" > /path/to/your/features/some-feature/README.md

# The web interface at http://localhost:3000 will automatically show the changes
```

### Testing the File Watching System

Run the comprehensive test script to see all features in action:

```bash
# From the CLI directory
./test_complete_system.sh
```

This interactive test demonstrates:
- Real-time file system monitoring
- Automatic web interface updates
- Feature creation, modification, and deletion
- Nested feature support
- Decision document integration
- Performance optimization with debouncing

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