---
name: plan-feature-breakdown
description: Analyzes a codebase and proposes how to split it into feature folders (feature names, target paths, files to move, shared code, owners and migration order), then writes the plan to a file for the user to approve. Uses features-cli to see the features that already exist. Use before reorganizing a codebase by feature, when the user asks which features a codebase has, or how to split a folder into features. Read-only: it never moves code.
---

# Plan a feature breakdown

Propose a breakdown of the codebase into features, write it to a file and get the user's approval. Do not move or edit source code in this skill.

If `features` isn't installed, use `npx features-cli@latest` in place of `features` below.

## 1. See what already exists

```bash
features scan PATH --json --skip-changes > "${TMPDIR:-/tmp}/features.json"
```

`PATH` is the source root (e.g. `./src`). The **explore-features** skill explains how to read this report without loading it all at once (`scripts/feature_map.py` prints a compact index). The report is a tree of features, each with `name`, `path`, `owner`, `is_owner_inherited`, `description`, nested `features`, `decisions` (raw markdown), `records` (specs and decision records with `kind`, `id`, `title`, `status`, `date`, `links`, `tasks`), `stats` (`files_count`, `lines_count`, `todos_count`) and `dependencies` (imports into other features, typed `parent`, `child` or `sibling`). An empty array means no feature exists yet.

Keep existing features: extend them rather than replacing them, unless the user asks.

## 2. Explore the code

Look for the business capabilities, not the technical layers:

- **Entry points**: routes and pages, API endpoints, CLI commands, jobs and queue consumers, public exports of a library.
- **Domain words**: model and table names, folder and file names repeated across layers (`OrderForm.tsx`, `orderService.ts`, `order.sql` → an `order` feature).
- **Files that change together**: `git log --name-only --format= -n 300 -- PATH` shows which files are usually committed together.
- **Existing ownership**: `CODEOWNERS`, package manifests, team names in docs. Note them as owner candidates; never guess an owner.
- **Product docs**: README, docs folder, issue labels.

Skip folders that are not source code (`node_modules`, build output, generated code, vendored code).

## 3. Decide the breakdown

Read [references/breakdown-rules.md](references/breakdown-rules.md) for how to name features, where shared code goes, when to nest features and which layout to pick for frameworks with a fixed folder structure.

The essentials:

- A feature is a business capability that a product manager would recognize (`checkout`, `user-profile`), not a technical layer (`helpers`, `hooks`).
- Each file goes in exactly one place: inside one feature, or in a shared folder when two or more features use it, or where it is when it is application glue (entry point, router setup, dependency injection, global config).
- Tests, styles, fixtures and stories go with the code they cover.
- Feature names must be unique across the whole source root: `features check` fails on duplicates.
- Order the migration so that features with the fewest dependencies on other planned features come first.

## 4. Write the plan and ask for approval

Write the plan with [references/breakdown-template.md](references/breakdown-template.md). Save it where the repository keeps specs or plans (e.g. `docs/specs/`), or `docs/feature-breakdown.md` if there is no such place.

Then show the user a short summary: the list of features with their target path and owner, the shared folders, and the open questions. Ask them to approve or adjust it. Update the file with their answers and set `status: accepted` once they approve. Only then may code move (with the **extract-feature** skill, or the **organize-by-feature** workflow).
