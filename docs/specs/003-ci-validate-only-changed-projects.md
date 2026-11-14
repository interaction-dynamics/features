---
status: implemented
date: 2026-11-14
---

# CI: validate only the projects that changed

## Context

The `Valide Code` workflow (`.github/workflows/validate.yml`) ran every job on every push and pull request. A change limited to `projects/website` still rebuilt and tested the CLI and the web app and ran the end-to-end suite, which wastes CI time.

## Requirements

- A `changes` job detects changed paths with `dorny/paths-filter` (pinned by SHA) and exposes one output per job.
- Each validation job runs only when its inputs changed:

  | Job | Runs when these change |
  | --- | --- |
  | Validate UI | `projects/web/**` |
  | Validate Website | `projects/website/**` |
  | Validate CLI | `projects/cli/**`, `examples/**` (test fixtures) |
  | Audit CLI dependencies | `projects/cli/Cargo.toml`, `Cargo.lock`, `deny.toml` |
  | End-to-end Tests | `projects/cli/**`, `projects/web/**`, `examples/tests-skip-changes/**` |

- Changing `.github/workflows/validate.yml` runs every job.
- A manual run (`workflow_dispatch`) runs every job.
- Jobs are skipped with a job-level `if`, so they report as skipped (treated as passing) and required status checks don't block pull requests.
- On push to `master`, changes are compared with the previous commit of the push.

## Out of scope

- `release.yml` and `deploy-demo.yml` are unchanged.
- No scheduled dependency audit: new RustSec advisories are only checked when CLI dependencies change or on a manual run (Dependabot still opens weekly cargo updates).

## Plan

1. Add the `changes` job with path filters to `validate.yml`.
2. Add `needs: changes` and an `if` on its output to each existing job.
