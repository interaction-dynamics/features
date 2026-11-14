<div align="center">
    <img width="100" src="./projects/web/public/feature-icon.svg" alt="feature logo" />
</div>
<h1 align="center">Features</h1>

<div align="center">
    <a href="https://crates.io/crates/features-cli"><img src="https://img.shields.io/crates/v/features-cli.svg?color=green" alt="Crates.io version" /></a>
    <a href="https://www.npmjs.com/package/features-cli"><img src="https://img.shields.io/npm/v/features-cli.svg?color=green" alt="Npm version" /></a>
</div>

`Features` is a Rust-based Web and CLI tool to explore the features in your projects, visualize their metrics and understand the ownership of each of these features.

It is technology agnostic and can be used with any programming language or framework.

You can: 

- list all the features in a project
- find the team 'owning' a specific feature
- get the history of a feature (git log)
- find the documentation related to the feature
- list the test coverage by feature
- check [the technical debt by feature](./FAQ.md#what-is-the-technical-debt-of-a-feature)
- see statistics about the features like the repartition of feature, fix and refactor commits, lines of code, and files
- generate CODEOWNERS file
- list the features (and owners) touched by a change, to ping the right teams on a pull request
- list the feature flags and other feature dependencies

> If you see an other use case, feel free to contribute to this repository.

It also provides a UI to explore the feature. Check a live [demo](http://interaction-dynamics.io/features/).

<a href="http://interaction-dynamics.io/features">
    <img src="https://github.com/user-attachments/assets/e5e76d65-53a2-4469-9736-76daec884839" width="100%" alt="screenshot" />
</a>

`features-cli` provides a Web Dashboard and also a CLI to generate json reports.
    
## Getting started 

```bash
npx features-cli@latest /path/to/project --project-dir ./repository --serve

# or with installation
npm install -g features-cli
# or
cargo binstall features-cli
# then 
features /path/to/project --project-dir ./repository --serve
```

Commands and their descriptions are listed below:

| Command | Description |
| ------- | ----------- |
| `--json` | Output features as JSON |
| `--flat` | Output features as a flat array instead of nested structure |
| `--description` | Include feature descriptions in the output (automatically enabled for `--serve`, `--build`, and `--json`) |
| `--coverage` | Include coverage information in the output (automatically enabled for `--serve`, `--build`, and `--json`) |
| `--list-owners` | Display only unique list of owners |
| `--find-owner <path>` | Find the owner of a specific file or folder |
| `--check` | Run validation checks on features (e.g., duplicate names) |
| `--skip-changes` | Skip computing git commit history (faster for large repos) |
| `--ignore-path <path>` | Ignore a folder and all its subfolders when scanning for features. Can be repeated to ignore multiple folders |
| `--include-skipped-dirs` | Also scan folders that are skipped by default (`node_modules`) for features, stats and imports |
| `--serve` | Start an HTTP server to serve features and the web dashboard UI |
| `--port <port>` | Change the port (default: 3000). Should be used with `--serve` |
| `--build` | Build a static version of the web dashboard UI |
| `--build-dir <path>` | Output directory for the static build (default: `build`) |
| `--coverage-dir <path>` | Specify a custom coverage directory (overrides automatic search) |
| `--generate-codeowners` | Generate or update a CODEOWNERS file with feature ownership information |
| `--project-dir <path>` | Project directory for CODEOWNERS generation and additional coverage search locations |
| `--codeowners-path <path>` | Custom path and filename for CODEOWNERS file (default: `CODEOWNERS` in project directory) |
| `--codeowners-prefix <prefix>` | Custom prefix for owner names in CODEOWNERS file and in `--since --markdown` / `--owners-only` output (default: `@`) |
| `--since <ref>` | Diff mode: list the features touched since the merge base of `<ref>` and `HEAD`, including uncommitted changes. See [Features touched by a change](#features-touched-by-a-change) |
| `--committed-only` | With `--since`: ignore uncommitted and untracked files |
| `--markdown` | With `--since`: output a markdown table for pull request comments (use `--json` for JSON) |
| `--owners-only` | With `--since`: print only the unique owners of the touched features, one per line |

## Features touched by a change

`--since <ref>` lists the features a change touches, with their owners. It compares the merge base of `<ref>` and `HEAD` with your working tree, so uncommitted and untracked files are included (add `--committed-only` to compare with `HEAD` only). The path defaults to the current directory.

```bash
features src --since origin/master
```

```text
Features touched since origin/master (merge base 1a2b3c4, including uncommitted changes):

  Auth (features/auth) - owner: team-auth
    M src/features/auth/login.ts
    R src/features/auth/old.ts -> src/features/billing/moved.ts

  Billing (features/billing) - owner: team-billing
    R src/features/auth/old.ts -> src/features/billing/moved.ts
    D src/features/billing/legacy.ts

Files outside any feature:
    M README.md
```

Each changed file is mapped to its closest feature, like `--find-owner`. A renamed file touches the features of both its old and new path, and deleted files are mapped by path. File paths are relative to the repository root; feature paths are relative to the scanned folder, as in the other outputs.

- `--json` gives a report with `features` (name, path, owner, `is_owner_inherited`, changed files) and `files_outside_features`.
- `--markdown` gives a table ready to post as a pull request comment.
- `--owners-only` prints the unique owners, one per line, with `--codeowners-prefix` added. Features without an owner are left out.

The command exits with `0` even when no feature is touched.

### Comment on pull requests with GitHub Actions

```yaml
name: Features touched
on: pull_request

permissions:
  contents: read
  pull-requests: write

jobs:
  features:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5
        with:
          fetch-depth: 0 # the merge base with the target branch is needed

      - name: List touched features
        run: npx features-cli@latest src --since origin/${{ github.base_ref }} --committed-only --markdown > features.md

      - name: Comment on the pull request
        env:
          GH_TOKEN: ${{ github.token }}
        # Updates the bot's last comment on each push. If other workflows comment as the same bot,
        # use a sticky-comment action instead so they don't overwrite each other.
        run: gh pr comment ${{ github.event.pull_request.number }} --body-file features.md --edit-last --create-if-none
```

To also request reviews from the owning teams, feed `--owners-only` to `gh`. Reviewers must be GitHub users or `org/team` slugs, so set the prefix accordingly, e.g. `--codeowners-prefix my-org/` when owners are team names:

```bash
features src --since origin/master --committed-only --owners-only --codeowners-prefix my-org/ \
  | xargs -r -n1 gh pr edit "$PR_NUMBER" --add-reviewer
```

Pull requests from forks get a read-only token, so commenting from those needs a `pull_request_target` workflow; read GitHub's security guidance before using it.

## Guidelines

By following specific guidelines and conventions, it becomes easier for `features` to find your feature:

### Folder structure

Features can be organized in two ways:

#### Method 1: Features folder (recommended)

Place your features folder in a dedicated `features` folder:

```bash
src/
├── features
│   ├── saveUser
│   │   ├── components/
│   │   │   ├── UserForm.ts
│   │   │   └── EmailInput.ts
│   │   ├── utils/
│   │   │   ├── useSaveUser.ts
│   │   │   ├── useFetchUser.ts
│   │   │   └── transformUser.ts
│   │   └── README.md # a README.md file for large features helps
│   └── feature-2
│       └── ... # ...
├── components # common components used across features
├── utils # common utils/hooks used across features
└── ... # other source code
```

> You can read the detailed [guidelines](./docs/folder-architecture-guidelines.md) or just jump into trying the CLI with the `--serve` options. The UI will guide you through the methodology.

#### Method 2: Feature flag
Mark any folder as a feature by adding `feature: true` in its README frontmatter:

```yaml
---
feature: true
owner: backend-team
---
```

This allows features to be organized anywhere in your codebase while still being discoverable by the tools.

Conversely, adding `feature: false` skips a folder from being detected as a feature, even if it's a direct subfolder of a `features` folder. Unlike `--ignore-path`, its subfolders are still scanned, so any nested features it contains are still discovered:

```yaml
---
feature: false
---
```

### Feature Metadata Annotations

You can annotate your code with feature metadata using special comments. These annotations can be placed **anywhere in your codebase** and will be automatically associated with the matching feature:

```typescript
// --feature-flag feature: feature-1, name: feature-flag1, type: experiment, owner: team-a, introduced_on: 2025-01-01

export function experimentalLoginFlow() {
  // implementation
}
```

The syntax is: `--feature-<metadata-key> feature:<feature-folder-basename>, property: value, ...`

- **`feature:`** property is optional if the comment is inside a feature folder - it will automatically link to that feature
- **`<metadata-key>`** becomes a tab in the web UI (e.g., `flag`, `experiment`)
- Properties are flexible - add any key-value pairs you need

This works in any programming language:

```python
# --feature-flag name: feature-flag1, type: processing, language: python
```

```rust
// --feature-flag name: feature-flag1, type: core, status: stable
```

```bash
# --feature-flag name: feature-flag1, environment: production, region: us-west-2
```

## Supported Programming Languages

The tool supports feature metadata annotations in the following programming languages: Rust, C/C++, Java, JavaScript/TypeScript, C#, Go, Swift, Kotlin, Scala, CSS/SCSS/Less, Python, Bash/Shell, Ruby, Perl, YAML, TOML, SQL, Lua, HTML/XML/SVG.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## Contributing

We welcome contributions to this project! If you have any ideas or suggestions, please feel free to open an issue or submit a pull request.

## Roadmap

Check [here](https://github.com/orgs/interaction-dynamics/projects/18/views/4).

## Bibliography about features-based architecture

> Feature based architecture shouldn't be confused with feature-based design.

- [FAQ](./FAQ.md)
- [Scalable React Projects with Feature-Based Architecture](https://dev.to/naserrasouli/scalable-react-projects-with-feature-based-architecture-117c)
- [Reddit discussion about feature-based architecture](https://www.reddit.com/r/reactjs/comments/1afywy4/codebase_examples_of_featuredriven_or_vertical/)
- [Why I Recommend a Feature-Driven Approach to Software Design](https://khalilstemmler.com/articles/software-design-architecture/feature-driven/): global vision of feature oriented development
- [Bullet-proof react folder architecture](https://github.com/alan2207/bulletproof-react/blob/master/docs/project-structure.md)
