---
title: Pull requests and CI
description: Run features-cli in CI. List the features and owners touched by a pull request, comment on it, request reviews from owners, fail the build on check errors and keep CODEOWNERS in sync.
---

Every command runs in CI with `npx features-cli@latest`, no install step needed. The [CLI page](/cli#pull-requests) shows what the pull request comment looks like.

## Features touched by a change

```bash
features diff ./src --since origin/main
```

```text
Features touched since origin/main (merge base 1a2b3c4, including uncommitted changes):

  Auth (features/auth) - owner: team-auth
    M src/features/auth/login.ts
    R src/features/auth/old.ts -> src/features/billing/moved.ts

  Billing (features/billing) - owner: team-billing
    R src/features/auth/old.ts -> src/features/billing/moved.ts
    D src/features/billing/legacy.ts

Files outside any feature:
    M README.md
```

- It compares the merge base of the ref and `HEAD` with the working tree, so uncommitted and untracked files are included. `--committed-only` compares with `HEAD` only.
- Each changed file is mapped to its closest feature, like `find-owner`. A renamed file touches the features of its old and new path; deleted files are mapped by path.
- File paths are relative to the repository root, feature paths to the scanned folder.
- The command exits with `0` even when no feature is touched.

Pick one output format:

| Option          | Output                                                                                                    |
| --------------- | --------------------------------------------------------------------------------------------------------- |
| `--json`        | `features` (name, path, owner, `is_owner_inherited`, changed files) and `files_outside_features`          |
| `--markdown`    | A table ready to post as a pull request comment                                                           |
| `--owners-only` | The unique owners, one per line, with `--codeowners-prefix` added. Features without an owner are left out |

With `diff.since` in [`features.config.yaml`](/docs/configuration), `--since` can be left out:

```yaml
diff:
  since: origin/main
  committed-only: true
```

## Comment on pull requests

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

Pull requests from forks get a read-only token, so commenting on those needs a `pull_request_target` workflow. Read GitHub's security guidance before using it.

## Request reviews from the owners

Feed `--owners-only` to `gh`. Reviewers must be GitHub users or `org/team` slugs, so set the prefix to match, for example `--codeowners-prefix my-org/` when owners are team names:

```bash
features diff src --since origin/main --committed-only --owners-only --codeowners-prefix my-org/ \
  | xargs -r -n1 gh pr edit "$PR_NUMBER" --add-reviewer
```

## Fail the build on invalid features

```yaml
- name: Check features
  run: npx features-cli@latest check ./src --format github
```

`features check` exits with code `1` when a rule set to `error` finds something, and `--format github` turns every finding into an annotation on the files of the pull request. See [Check rules](/docs/check-rules#run-the-checks-in-github-actions) for the full workflow and the rules.

## Keep CODEOWNERS in sync

```bash
features codeowners ./src --project-dir .
```

Regenerates the managed section of `CODEOWNERS` from the owners in the feature READMEs. Run it in a pre-commit hook or in CI, or add `--generate-codeowners` to `check` to update it while validating. Set `codeowners-path` and `codeowners-prefix` in the configuration file to keep the same output everywhere.

## Export a report

```bash
features scan ./src --json > features.json
```

The JSON report lists every feature with its owner, stats, coverage, records and dependencies, for scripts and dashboards of your own. `--skip-changes` leaves out the git history to keep it small.
