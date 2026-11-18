---
status: implemented
date: 2026-11-18
---

# Create features with a Claude skill (`create-feature` and `features skills install`)

## Context

AI coding agents create features in minutes, but each one picks its own layout: code dropped in a technical folder, no README, no owner, a name that clashes with an existing feature, imports into the internals of sibling features. The conventions of `features-cli` ([folder architecture guidelines](../folder-architecture-guidelines.md), [README format](../readme-format.md)) are only followed when the agent happens to read them.

The [skills plugin](../../projects/skills/README.md) already teaches Claude to reorganize an existing codebase (`organize-by-feature`, `extract-feature`, `document-feature`...), but nothing covers the most frequent case: building a **new** feature. A skill fits better than a scaffold command: creating a feature is mostly judgment (is it really new, where does it go, who owns it, what is its first spec), and the files to write are already described by `document-feature`. The CLI keeps doing what it does well: listing features, creating numbered records (`features new spec|decision`), checking.

The skills are installed today through the Claude Code plugin marketplace or by cloning the repository and copying folders. A CLI command that installs them makes them one step away for anyone who has `features-cli`, and keeps them in sync with the CLI version they rely on.

## Requirements

### Part 1: `create-feature` skill

- New folder `projects/skills/skills/create-feature/` with a `SKILL.md`, part of the existing `features` plugin.
- Triggers: the user asks to add, build, start or scaffold a new feature or capability ("add a referral program", "create a feature for invoices"). Not for changes inside an existing feature.
- Workflow:
  1. **Look for an existing feature first.** Map the features (`features scan PATH --json --skip-changes` through the `explore-features` index script, or `list_features` when the MCP server is connected) and search names, keys, descriptions and specs for the same capability. If a feature already covers it, stop and propose to extend that feature (or create a sub-feature) instead.
  2. **Choose the location**, and show it to the user before writing:
     - a sub-feature (`<parent>/features/<name>/`) when the new feature only makes sense inside an existing one;
     - otherwise the top-level `features/` folder of the scanned path; if there are several, ask; if there is none, propose `PATH/features/` and the conventions of the guidelines.
     - The folder name follows the naming style of its sibling folders (kebab-case, camelCase...).
     - The name and the key must not clash with another feature (`features check` fails on duplicate names).
  3. **Write the README** with the `document-feature` rules and template: owner (asked, inherited from the parent, or a `# TODO: owner` comment, never invented), title, one-line description for a product manager.
  4. **Write the first spec** when the user described what to build: `features new spec <feature> "<title>"`, filled with the `document-feature` spec template, status `draft`, then `accepted` once the user approves it.
  5. **Build the code inside the folder.** Shared code stays in the shared folders; other features are used through their public entry points, never their internals. Tests live in the feature folder when the project's test setup allows it.
  6. **Record decisions** that come up while building (library choice, data model) with `features new decision`, following `document-feature`.
  7. **Check**: `features check PATH`, the `validate-features` report for the new feature (description, owner, imports between sibling features), the project's build and tests, then `features diff PATH --since <base>` to list the features and owners touched, to mention in the pull request.
- The skill reuses the references of `document-feature` and the scripts of `explore-features` and `validate-features` by linking to them, without copying them.
- `features` is used when installed, `npx features-cli@latest` otherwise, like the other skills.
- Update the skills README (table, "Use" examples), the plugin description and version, and mention `create-feature` in `organize-by-feature` where new features come up.

### Part 2: `features skills install`

- `features skills install [--dir <DIR>] [--global] [--force] [--dry-run]` copies the skills bundled in the CLI binary into a skills folder:
  - default `./.claude/skills/` (shared with the team through git);
  - `--global`: `~/.claude/skills/`;
  - `--dir <DIR>`: any folder, for other agents that read the Agent Skills format.
- Each skill is copied as its own folder (`create-feature/`, `document-feature/`...), with its `references/` and `scripts/`. All the skills are installed together, since they refer to each other by name.
- An existing skill folder is left untouched unless `--force`, which overwrites its files and keeps the files that aren't part of the bundled skill. The command lists the skills installed, skipped and overwritten. `--dry-run` prints the same list without writing.
- `features skills list` prints the bundled skills with their description (from the `SKILL.md` frontmatter; a one-line `description:` that isn't valid YAML, e.g. containing `: `, is read as is).
- The skills are embedded at compile time from `projects/skills/skills/`, so the installed skills always match the CLI version (commands and JSON fields they use). Offline: no network call.
- Packaging: a build script (`projects/cli/build.rs`) embeds `projects/skills/skills` when it exists, otherwise `projects/cli/skills`, a git-ignored copy that the release workflow makes before `cargo publish`. The CLI job of the validate workflow also runs when `projects/skills/**` changes.
- Each installed `SKILL.md` keeps its content as is; the CLI version is written in a `.features-version` file at the root of the skills folder, so a later `install --force` can tell which version it replaces.

## Out of scope

- A `features new <NAME>` scaffold command (it would also clash with `features new spec|decision`).
- An MCP write tool (`create_feature`): the MCP server stays read-only.
- Code templates (components, tests, index files): the skill writes code in the project's own style.
- Installing skills for agents in their own formats (Cursor rules, Copilot instructions): `--dir` covers agents that read Agent Skills; others can paste the `SKILL.md`.
- Updating or uninstalling installed skills beyond `install --force`.

## Plan

1. Write `projects/skills/skills/create-feature/SKILL.md` (workflow above, links to the `document-feature` references and the `explore-features` / `validate-features` scripts).
2. Try it on `examples/demo/src`: ask for a new feature, check the location, README, spec and `features check`; adjust the skill.
3. Update the skills README, `organize-by-feature`, `plugin.json` (version, description) and the marketplace description.
4. CLI: add `src/skills.rs` (embedded skills, list, install with `--dir` / `--global` / `--force` / `--dry-run`), `Command::Skills` in `cli.rs`, wiring in `bin.rs`. Make sure the npm, Homebrew and crates.io packages build with the embedded files (the crate must include `projects/skills/skills`, or a copy made at build time).
5. Tests: unit tests for the install plan (new, existing, forced), integration test installing into a temp folder.
6. Docs: `docs/cli.md`, README (AI coding agents section), website AI and CLI pages, changelog.
