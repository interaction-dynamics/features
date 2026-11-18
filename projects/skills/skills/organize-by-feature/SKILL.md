---
name: organize-by-feature
description: Reorganizes an existing codebase into feature folders that features-cli can map, one feature at a time, keeping tests green. Runs the full workflow (map, plan the breakdown, move the code, document each feature, validate) by using the explore-features, plan-feature-breakdown, extract-feature, document-feature and validate-features skills. Use when the user asks to restructure, refactor or migrate a codebase to a feature-based (vertical slice) architecture, or to "make the codebase readable by feature".
---

# Organize a codebase by feature

Turn a codebase organized by technical layer (`components/`, `services/`, `utils/`) into feature folders where each feature can be read on its own.

A feature is **readable on its own** when someone who opens its folder can tell, without reading other features:

- what it does (the README title and the line after it),
- who owns it (`owner` in the README frontmatter, or inherited from a parent feature),
- why it is built that way (decision records in `__docs__/decisions/`),
- and where all its code is (inside the folder, importing only shared code and the public entry points of other features).

## The `features` command

All skills in this set use [features-cli](https://github.com/interaction-dynamics/features). Run `features --help` to check it is installed. If it isn't, use `npx features-cli@latest` in place of `features` in every command (for example `npx features-cli@latest check ./src`). Only use the commands and flags listed in the [CLI reference](https://github.com/interaction-dynamics/features/blob/master/docs/cli.md).

## Workflow

Copy this checklist into your reply and keep it up to date:

```
- [ ] 1. Prepare: clean git tree, baseline build and tests pass
- [ ] 2. Plan: feature breakdown approved by the user
- [ ] 3. For each feature: extract, document, validate, commit
- [ ] 4. Final validation and hand-over
```

### 1. Prepare

1. Check `git status` is clean. If it isn't, ask the user to commit or stash their changes first. Never stash or discard them yourself.
2. Create a branch for the refactor if the user works on the default branch (ask first).
3. Find the project's own commands for build, type check, lint and tests (package scripts, `Makefile`, `Cargo.toml`, CI workflows, `CONTRIBUTING.md`). Run them and note the result. This is the baseline: the refactor must not make it worse. If tests already fail, tell the user and list the failing tests before going on.
4. Find the source root to scan (usually `./src`, or each package of a monorepo). It is the `PATH` argument of every `features` command below.
5. Generate the feature map with the **explore-features** skill (`features scan PATH --json --skip-changes` into a file) to see what already exists. Regenerate it after each feature.

### 2. Plan

Use the **plan-feature-breakdown** skill. It writes a breakdown file and asks the user to approve it. Do not move any code before the user approves the plan.

### 3. One feature at a time

Take the features in the order of the approved plan (fewest dependencies first). For each one:

1. **extract-feature** skill: move its code into its folder, update imports, run the baseline commands.
2. **document-feature** skill: write its `README.md` (title, description, owner) and its decision records.
3. **validate-features** skill: run `features check` and the report on the scanned folder. Fix what the report flags for this feature.
4. Commit this feature alone, with a message such as `refactor(checkout): move checkout code into features/checkout`. One feature per commit keeps the history readable and each step easy to revert.
5. Tick the feature in the breakdown file.

Stop and ask the user when a file clearly belongs to several features, when a move would change behavior, or when tests fail and the cause isn't the move.

### 4. Final validation and hand-over

1. Run the **validate-features** skill on the whole source root.
2. If the repository has a `CODEOWNERS` file, or the user wants one, offer to run `features codeowners PATH --project-dir .`. It only rewrites the section between its markers.
3. Offer to add a "Finding your way around" section to `AGENTS.md` or `CLAUDE.md` so future agent sessions use the feature map (see the root README of features-cli, section "Give your AI coding agent a map of the codebase").
4. Summarize for the user: features created, files left in shared folders and why, owners or decisions still marked `TODO`, and the commits made. Suggest `features serve PATH` to browse the result.
5. For features added after the refactor, point the user to the **create-feature** skill, so new code starts in its own feature folder instead of drifting back into technical folders.

## Rules

- **Never change behavior.** Moves, renames and import updates only. If you spot a bug, note it in the summary instead of fixing it.
- **Small steps.** Run the baseline commands after each feature, not only at the end.
- **Don't invent facts.** Owners and decisions come from the user, `CODEOWNERS`, existing docs or git history. When unsure, ask or write a visible `TODO` placeholder.
- **Follow the conventions** in the [folder architecture guidelines](https://github.com/interaction-dynamics/features/blob/master/docs/folder-architecture-guidelines.md) and the [README format](https://github.com/interaction-dynamics/features/blob/master/docs/readme-format.md). Don't add config files the CLI doesn't need.
