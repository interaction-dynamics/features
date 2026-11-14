# Contributing to Features CLI

Thank you for your interest in contributing to the Features CLI project! This document outlines the process for contributing and setting up the development environment.

## Development Setup

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Node.js](https://nodejs.org/) (version 14 or higher)
- [pnpm](https://pnpm.io/) for package management
- Git

### Local Development

1. Clone the repository:
   ```bash
   git clone https://github.com/interaction-dynamics/features.git
   cd features
   ```

2. Build the CLI:
   ```bash
   cd projects/cli
   cargo build
   ```

3. Run tests:
   ```bash
   cargo test
   ```

4. Test the CLI locally:
   ```bash
   cargo run -- ../../examples/javascript-basic
   ```

### Code Quality

Before submitting a PR, ensure your code passes all checks:

```bash
# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test
```

## Release Process

The project uses automated releases triggered by git tags. Here's how releases work:

### Publishing to crates.io and npm

When a new version tag (e.g., `v0.5.0`) is pushed, the GitHub Actions workflow automatically:

1. **Validates the code** - runs tests, linting, and formatting checks
2. **Builds binaries** - creates binaries for all supported platforms
3. **Publishes to crates.io** - publishes the Rust crate
4. **Publishes to npm** - creates and publishes npm packages for all platforms
5. **Creates GitHub release** - with release notes and binary attachments

### Publishing credentials (trusted publishing)

The release workflow does not use long-lived registry tokens. Both registries
use [trusted publishing](https://docs.github.com/en/actions/security-for-github-actions/security-hardening-your-deployments/about-security-hardening-with-openid-connect):
the job exchanges a short-lived GitHub OIDC token for a publish token that
expires on its own.

Maintainers must configure, once:

#### crates.io
- On [crates.io](https://crates.io/crates/features-cli) → Settings → Trusted Publishing, add a GitHub publisher:
  - Repository owner: `interaction-dynamics`
  - Repository name: `features`
  - Workflow filename: `release.yml`
  - Environment: `crates-publish`
- Then delete the old `CRATES_IO_TOKEN` repository secret and revoke the token on crates.io.

#### npm
- For `features-cli` and every `@features-cli/*` platform package, on npmjs.com → package Settings → Trusted Publisher, add GitHub Actions with
  workflow `release.yml` and environment `npm-publish`.
- An `NPM_TOKEN` secret is not needed.

#### GitHub Environments
The workflow uses these environments (configure in repository settings). Restrict
both to the `master` branch so only the release workflow on `master` can publish:
- `crates-publish`: crates.io publishing
- `npm-publish`: npm publishing

#### Pinned actions
Third-party actions are pinned to full commit SHAs, with the version in a
comment. Dependabot (`.github/dependabot.yml`) opens PRs to update them.

### Version Management

1. Update the version in `projects/cli/Cargo.toml`
2. Commit the change: `git commit -am "chore: bump version to X.Y.Z"`
3. Create and push a tag: `git tag vX.Y.Z && git push origin vX.Y.Z`
4. The GitHub Actions workflow will handle the rest

### Manual Publishing (Emergency)

If automated publishing fails, you can publish manually:

#### To crates.io:
```bash
cd projects/cli
cargo publish
```

#### To npm:
Follow the steps in the GitHub workflow or use the existing npm publishing scripts.

## Project Structure

```
features/
├── projects/
│   ├── cli/          # Rust CLI source code
│   └── web/          # Web dashboard
├── examples/         # Example projects
├── .github/
│   └── workflows/    # CI/CD workflows
└── docs/             # Documentation
```

## Pull Request Guidelines

1. **Create a feature branch** from `main`
2. **Write descriptive commit messages** following conventional commits
3. **Add tests** for new functionality
4. **Update documentation** as needed
5. **Ensure CI passes** - all checks must be green
6. **Keep PRs focused** - one feature/fix per PR

### Commit Message Format

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat: add new feature`
- `fix: resolve bug`
- `docs: update documentation`
- `chore: maintenance tasks`
- `test: add or update tests`

## Getting Help

- **Issues**: Report bugs or request features via [GitHub Issues](https://github.com/interaction-dynamics/features/issues)
- **Discussions**: Ask questions in [GitHub Discussions](https://github.com/interaction-dynamics/features/discussions)
- **Documentation**: Check the [README](README.md) and inline code comments

## Screenshot 

To update the screenshot in the README, follow these steps:

1. Run the project locally using `features serve /path/to/project --project-dir ./repository`
2. Take a screenshot of the UI
3. Use https://postspark.app/ to add a frame
4. Upload the framed screenshot to your GitHub account
5. Update the README with the new screenshot URL

## Code of Conduct

This project follows a standard code of conduct. Please be respectful and professional in all interactions.

## License

By contributing to this project, you agree that your contributions will be licensed under the MIT License.
