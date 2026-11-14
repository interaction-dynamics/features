<div align="center">
    <img width="100" src="https://raw.githubusercontent.com/interaction-dynamics/features/master/projects/web/public/feature-icon.svg" alt="features-cli logo" />
</div>
<h1 align="center">Features</h1>

<p align="center">
    <strong>See your codebase the way your product team does: by feature.</strong>
</p>

<p align="center">
    Map every feature in your codebase: who owns it, how well it's tested and how much debt it carries.<br/>
    Works with any language. One command, no config.
</p>

<div align="center">
    <a href="https://crates.io/crates/features-cli"><img src="https://img.shields.io/crates/v/features-cli.svg?color=green" alt="Crates.io version" /></a>
    <a href="https://www.npmjs.com/package/features-cli"><img src="https://img.shields.io/npm/v/features-cli.svg?color=green" alt="npm version" /></a>
    <a href="https://www.npmjs.com/package/features-cli"><img src="https://img.shields.io/npm/dm/features-cli.svg?color=green" alt="npm downloads" /></a>
    <a href="https://crates.io/crates/features-cli"><img src="https://img.shields.io/crates/d/features-cli.svg?color=green" alt="Crates.io downloads" /></a>
    <a href="https://github.com/interaction-dynamics/features/actions/workflows/validate.yml"><img src="https://github.com/interaction-dynamics/features/actions/workflows/validate.yml/badge.svg" alt="CI status" /></a>
    <a href="https://github.com/interaction-dynamics/features/blob/master/LICENSE"><img src="https://img.shields.io/badge/license-MIT-green.svg" alt="MIT license" /></a>
    <a href="https://github.com/interaction-dynamics/features/stargazers"><img src="https://img.shields.io/github/stars/interaction-dynamics/features?style=flat&color=green" alt="GitHub stars" /></a>
</div>

<p align="center">
    <a href="https://interaction-dynamics.io/features/"><strong>Live demo</strong></a> ·
    <a href="#quick-start">Quick start</a> ·
    <a href="https://github.com/interaction-dynamics/features/blob/master/docs/cli.md">CLI reference</a> ·
    <a href="https://github.com/interaction-dynamics/features/blob/master/FAQ.md">FAQ</a>
</p>

<a href="https://interaction-dynamics.io/features/">
    <img src="https://github.com/user-attachments/assets/e5e76d65-53a2-4469-9736-76daec884839" width="100%" alt="features-cli web dashboard showing the features of a project with their owners, metrics and test coverage" />
</a>

## Quick start

```bash
npx features-cli@latest serve ./src
```

Then open [http://localhost:3000](http://localhost:3000). That's it.

Prefer a global install?

```bash
npm install -g features-cli   # or: cargo binstall features-cli
features serve ./src
```

## Why Features?

Most codebases are organized by technical layer (`components/`, `services/`, `utils/`), so the features your users care about are scattered everywhere. As a result:

- **Nobody knows who owns what.** Questions and bugs bounce between teams.
- **CODEOWNERS files drift** out of sync with the code.
- **Tech debt and missing tests stay invisible** until something breaks.
- **New joiners struggle** to find where a feature lives and why it was built that way.
- **Features span several repositories.** The web app, the API and the mobile app each hold a piece of the same feature, and nobody sees the whole of it.

`features-cli` scans your repository, finds every feature and turns it into an interactive dashboard and a JSON report, using the folders, READMEs and git history you already have.

## What you get

**For engineering managers**
- See which team owns each feature, and find the owner of any file or folder
- Generate and keep a `CODEOWNERS` file in sync automatically
- See which features and owners a pull request touches, and [ping the right teams automatically](#ping-the-right-teams-on-every-pull-request)
- See every feature across all your repositories in [one dashboard](#one-dashboard-for-all-your-repositories)

**For tech leads**
- Spot risky areas with [technical debt per feature](https://github.com/interaction-dynamics/features/blob/master/FAQ.md#what-is-the-technical-debt-of-a-feature)
- Track test coverage per feature
- Compare feature, fix and refactor commits, lines of code and file count per feature

**For new joiners**
- Browse every feature with its description, docs and decision records
- Read a feature's history from its git log

**For product and QA**
- List feature flags, experiments and other feature metadata across the codebase

> Have another use case? [Open an issue](https://github.com/interaction-dynamics/features/issues). We'd love to hear about it.

## Share a dashboard with your team

Build a static version of the dashboard and host it anywhere:

```bash
features build ./src --build-dir ./dashboard
```

Add `--strip-emails` to remove commit author emails before publishing it.

To publish it to GitHub Pages on every push, see the [GitHub Actions setup](#publish-the-dashboard-to-github-pages).

## One dashboard for all your repositories

Does a feature live partly in your web app, partly in your API and partly in your mobile app? Export one report per repository and combine them:

```bash
features scan ./web/src --json > reports/web.json
features scan ./api/src --json > reports/api.json
features aggregate ./reports --build-dir ./dashboard
```

Features with the same name become one feature that combines the metrics and history of every repository and lists the repositories it lives in. If two repositories disagree on how features are nested, `features-cli` keeps them apart and shows the conflict on a *Reconciliation issues* page instead of guessing.

See how features are merged in the [CLI reference](https://github.com/interaction-dynamics/features/blob/master/docs/cli.md#aggregate-several-repositories).

## Use it in CI and scripts

```bash
features check ./src                                     # fail the build on invalid features (e.g. duplicate names)
features codeowners ./src --project-dir .                # regenerate CODEOWNERS
features scan ./src --json > features.json               # export a report for scripts, dashboards or AI coding agents
features find-owner src/features/checkout/api.ts         # who owns this file?
features diff ./src --since origin/main                  # which features (and owners) does my change touch?
```

All options are listed in the [CLI reference](https://github.com/interaction-dynamics/features/blob/master/docs/cli.md).

## Ping the right teams on every pull request

`features diff --since <ref>` lists the features a change touches, with their owners. It compares the merge base of `<ref>` and `HEAD` with your working tree, so uncommitted and untracked files are included (add `--committed-only` to compare with `HEAD` only). The path defaults to the current directory.

```bash
features diff src --since origin/master
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

Each changed file is mapped to its closest feature, like `find-owner`. A renamed file touches the features of both its old and new path, and deleted files are mapped by path. File paths are relative to the repository root; feature paths are relative to the scanned folder, as in the other outputs.

- `--json` gives a report with `features` (name, path, owner, `is_owner_inherited`, changed files) and `files_outside_features`.
- `--markdown` gives a table ready to post as a pull request comment.
- `--owners-only` prints the unique owners, one per line, with `--codeowners-prefix` added. Features without an owner are left out.

The command exits with `0` even when no feature is touched.

To post this list on every pull request and request reviews from the owning teams, see the [GitHub Actions setup](#comment-on-pull-requests).

## How it finds your features

`features-cli` follows a few simple conventions. No config file is required.

### Option 1: a `features` folder (recommended)

Put each feature in its own folder inside a `features` folder:

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
│   │   └── README.md # description, owner and other metadata
│   └── feature-2
│       └── ...
├── components # common components used across features
├── utils # common utils/hooks used across features
└── ... # other source code
```

> Read the detailed [guidelines](https://github.com/interaction-dynamics/features/blob/master/docs/folder-architecture-guidelines.md), or just run `features serve`: the dashboard guides you through the methodology.

### Option 2: README frontmatter

Mark any folder as a feature by adding `feature: true` to its README frontmatter. This lets you keep features anywhere in your codebase:

```yaml
---
feature: true
owner: backend-team
---
```

Conversely, `feature: false` stops a folder from being detected as a feature, even if it sits directly inside a `features` folder. Unlike `--ignore-path`, its subfolders are still scanned, so any nested features are still discovered:

```yaml
---
feature: false
---
```

### Feature metadata annotations

Annotate your code with special comments to attach metadata (feature flags, experiments, anything) to a feature. Annotations can live **anywhere in your codebase**:

```typescript
// --feature-flag feature: feature-1, name: feature-flag1, type: experiment, owner: team-a, introduced_on: 2025-01-01

export function experimentalLoginFlow() {
  // implementation
}
```

The syntax is `--feature-<metadata-key> feature:<feature-folder-basename>, property: value, ...`

- **`feature:`** is optional when the comment is inside a feature folder: it links to that feature automatically
- **`<metadata-key>`** becomes a tab in the web UI (e.g., `flag`, `experiment`)
- Properties are free-form: add any key-value pairs you need

It works the same in every language:

```python
# --feature-flag name: feature-flag1, type: processing, language: python
```

```rust
// --feature-flag name: feature-flag1, type: core, status: stable
```

Annotations are supported in Rust, C/C++, Java, JavaScript/TypeScript, C#, Go, Swift, Kotlin, Scala, CSS/SCSS/Less, Python, Bash/Shell, Ruby, Perl, YAML, TOML, SQL, Lua and HTML/XML/SVG.

## GitHub Actions setup

### Publish the dashboard to GitHub Pages

```yaml
# .github/workflows/features.yml
name: Features dashboard
on:
  push:
    branches: [main]
permissions:
  contents: read
  pages: write
  id-token: write
jobs:
  deploy:
    runs-on: ubuntu-latest
    environment:
      name: github-pages
    steps:
      - uses: actions/checkout@v5
        with:
          fetch-depth: 0 # full git history for feature metrics
      - run: npx features-cli@latest build ./src --build-dir ./dashboard --strip-emails
      - uses: actions/upload-pages-artifact@v5
        with:
          path: ./dashboard
      - uses: actions/deploy-pages@v5
```

### Comment on pull requests

```yaml
# .github/workflows/features-touched.yml
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
        run: npx features-cli@latest diff src --since origin/${{ github.base_ref }} --committed-only --markdown > features.md

      - name: Comment on the pull request
        env:
          GH_TOKEN: ${{ github.token }}
        # Updates the bot's last comment on each push. If other workflows comment as the same bot,
        # use a sticky-comment action instead so they don't overwrite each other.
        run: gh pr comment ${{ github.event.pull_request.number }} --body-file features.md --edit-last --create-if-none
```

To also request reviews from the owning teams, feed `--owners-only` to `gh`. Reviewers must be GitHub users or `org/team` slugs, so set the prefix accordingly, e.g. `--codeowners-prefix my-org/` when owners are team names:

```bash
features diff src --since origin/master --committed-only --owners-only --codeowners-prefix my-org/ \
  | xargs -r -n1 gh pr edit "$PR_NUMBER" --add-reviewer
```

Pull requests from forks get a read-only token, so commenting from those needs a `pull_request_target` workflow; read GitHub's security guidance before using it.

## How it compares

| | Features | CODEOWNERS alone | Service catalogs (e.g. Backstage) |
| --- | --- | --- | --- |
| Granularity | Features inside a codebase | Files and folders | Services and repositories |
| Setup | One command, no config | Hand-written file | Hosted platform + YAML per service |
| Ownership | ✅ (and generates CODEOWNERS) | ✅ | ✅ |
| Tests, debt and git metrics per feature | ✅ | ❌ | Via plugins |
| One view across repositories | ✅ | ❌ | ✅ |
| Works offline / in CI | ✅ | ✅ | ❌ |

## Learn more about feature-based architecture

> Feature-based architecture is about how you organize **code**: each feature lives in its own folder. It is not the same as feature-based **design**, which is about how you plan the product.

- [FAQ](https://github.com/interaction-dynamics/features/blob/master/FAQ.md)
- [Why I Recommend a Feature-Driven Approach to Software Design](https://khalilstemmler.com/articles/software-design-architecture/feature-driven/): the big picture of feature-oriented development
- [Scalable React Projects with Feature-Based Architecture](https://dev.to/naserrasouli/scalable-react-projects-with-feature-based-architecture-117c)
- [Bulletproof React folder architecture](https://github.com/alan2207/bulletproof-react/blob/master/docs/project-structure.md)
- [Reddit discussion about feature-based architecture](https://www.reddit.com/r/reactjs/comments/1afywy4/codebase_examples_of_featuredriven_or_vertical/)

## Contributing

Contributions are welcome! Read the [contributing guide](https://github.com/interaction-dynamics/features/blob/master/CONTRIBUTING.md), pick a [good first issue](https://github.com/interaction-dynamics/features/labels/good%20first%20issue) or check the [roadmap](https://github.com/orgs/interaction-dynamics/projects/18/views/4).

If `features-cli` helps you, **[give it a ⭐ on GitHub](https://github.com/interaction-dynamics/features)**. It helps other developers find it.

## License

MIT. See [LICENSE](https://github.com/interaction-dynamics/features/blob/master/LICENSE).
