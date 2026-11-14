---
status: implemented
date: 2026-11-14
---

# Bun, oxlint and oxfmt for the web dashboard and website

## Context

`projects/web` and `projects/website` used pnpm for packages and Biome for linting and formatting (`projects/web` also carried unused ESLint dependencies). We want one faster toolchain: Bun for installs and scripts, oxlint for linting and oxfmt for formatting.

## Requirements

- Both projects install with `bun install` from a committed `bun.lock`. `pnpm-lock.yaml` is removed.
- The lockfile is migrated from `pnpm-lock.yaml` so resolved versions stay the same.
- `packageManager` is set to `bun@<version>` in each `package.json`; CI reads the Bun version from it.
- The `rolldown-vite` override moves from `pnpm.overrides` to the top-level `overrides` field.
- Biome, its config and the unused ESLint packages are removed.
- Scripts in both projects:
  - `lint`: `oxlint`
  - `lint:fix`: `oxlint --fix`
  - `format`: `oxfmt` (writes files)
  - `format:check`: `oxfmt --check` (used by CI)
- oxfmt keeps the previous Biome style: 2 spaces, single quotes, no semicolons, trailing commas, 80 columns. Imports are sorted without blank lines between groups (like Biome's organize imports).
- oxlint runs the `correctness` category as errors. The web dashboard adds the React, jsx-a11y, import and Vitest plugins. React Compiler rules (`set-state-in-effect`, `refs`, `preserve-manual-memoization`) and `no-array-index-key` are warnings, so existing code passes without behavior changes.
- `biome-ignore` comments become `oxlint-disable-next-line` comments. The CSS ones are dropped (oxlint does not lint CSS).
- CI (`validate.yml`, `deploy-demo.yml`, the web build in `release.yml`) uses `oven-sh/setup-bun` and caches `~/.bun/install/cache`. Node.js stays installed because Vite, Vitest and Playwright run on Node.
- Playwright starts the dev server with `bun run dev`.
- Docs (READMEs, CONTRIBUTING) refer to Bun commands.

## Out of scope

- The npm publishing step of the CLI in `release.yml` (still pnpm).
- `examples/` projects.
- Fixing the new oxlint warnings.

## Plan

1. Rewrite both `package.json` files (scripts, devDependencies, overrides, `packageManager`).
2. Migrate `pnpm-lock.yaml` to `bun.lock` with `bun install`, then delete the pnpm lockfiles.
3. Generate `.oxfmtrc.json` with `oxfmt --migrate=biome`, add import sorting, and write `.oxlintrc.json`.
4. Port suppression comments, fix the few trivial lint errors, and reformat.
5. Update CI workflows, Playwright config and docs.
