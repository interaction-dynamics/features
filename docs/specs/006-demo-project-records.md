---
status: implemented
date: 2026-11-14
---

# Project records in the online demo

## Context

The online demo (https://interaction-dynamics.io/features/#/records) showed only the empty state on the Project records page:

- the demo deploy workflow wrote `features.json` but never `records.json` (404), and
- `examples/demo` had no root `docs/specs`, `docs/decisions` or `docs/adr`.

The page is the place for specs and decisions that span several features, so the demo should show it filled.

## Requirements

- `examples/demo/docs/` holds project-level records that show the page's features: a spec with all tasks done, a spec with open tasks, decisions linked to specs (`spec:` / `decisions:`) and a superseded decision with its replacement (`supersedes:` / `superseded_by:`).
- The records parse without warnings from `features check examples/demo`.
- `deploy-demo.yml` writes `projects/web/public/records.json` with `features records ../../examples/demo --skip-changes --json`, fails if it is empty, and checks it is in `dist/`.
- `projects/web/public/records.json` is gitignored, like `features.json`.

## Out of scope

- Fixing the existing `check` errors in the demo (duplicate feature keys, Feature 1 ADR without status).
- Changing the Project records page itself (wording, layout).

## Plan

1. Add 2 specs and 3 decisions to `examples/demo/docs/`.
2. Add a "Generate records data" step to `.github/workflows/deploy-demo.yml` and check `dist/records.json` in the build step.
3. Gitignore `projects/web/public/records.json`.
4. Verify locally: generate both JSON files, run the web dashboard, open `#/records`.
