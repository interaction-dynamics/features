---
status: implemented
date: 2026-11-21
---

# CI: cut GitHub Actions minutes

## Context

The `features` repository used 55% of the account's GitHub Actions minutes in November. Superseded runs kept going, docs-only changes started the `Valide Code` workflow, the end-to-end suite ran again on every push to `master`, and `release.yml` validated the CLI and built the dashboard on every push to `master` even when nothing was released.

## Requirements

- `validate.yml`
  - `concurrency: { group: ${{ github.workflow }}-${{ github.ref }}, cancel-in-progress: true }` cancels a run when a newer commit lands on the same branch or pull request.
  - Workflow-level `paths` filters on `push` and `pull_request` skip docs-only changes: root `*.md`, `docs/**/*.md`, `projects/*/README.md`, feature `__docs__` folders under `projects/*/src`, and `.claude/**`. Still trigger CI: `docs/changelog.json` (imported by the web app and the website), website content, `projects/skills/**` (embedded in the CLI) and `examples/**` (CLI test fixtures).
  - End-to-end tests run on pull requests and manual runs only, not on pushes to `master`.
  - The `Validate CLI` cache caches `projects/cli/target/` (it pointed at `target/` at the workspace root, so the build was never cached).
  - Cargo caches have one key per build profile (`cargo-validate-` for debug + clippy, `cargo-release-` for `--release`) and `restore-keys`, so a `Cargo.lock` change restores a warm cache and jobs no longer overwrite each other's cache under one key.
  - Playwright browsers are cached by Playwright version; on a cache hit only the system dependencies are installed.
  - Every job has a `timeout-minutes`.
- `release.yml`: `validate` and `build-dashboard` depend on `check` and run only for a release (`release…` commit message or manual run).
- `deploy-demo.yml`: shares the `cargo-release-` cache with the end-to-end job.
- Bun install cache (`~/.bun/install/cache`) stays as it was: already cached in every job.

## Out of scope

- Turbo: the repository doesn't use Turborepo, so there is nothing to cache.
- `deploy-demo.yml` still runs after each successful `Valide Code` run on `master`. A docs-only push to `master` no longer triggers `Valide Code`, so the demo's project records (from `docs/specs` and `docs/decisions`) update with the next code push or a manual run.
- Branch protection: `master` has no required status checks. If some are added later, docs-only pull requests will wait for checks that never start (the workflow-level `paths` filter skips the whole workflow).

## Plan

1. Add `concurrency` and `paths` filters to `validate.yml`.
2. Gate the e2e job on `github.event_name != 'push'`.
3. Fix the CLI cache path, split cargo cache keys per profile, add `restore-keys`, cache Playwright browsers, add job timeouts.
4. Gate `validate` and `build-dashboard` in `release.yml` on the `check` job.
