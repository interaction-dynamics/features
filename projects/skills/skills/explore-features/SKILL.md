---
name: explore-features
description: Generates the features-cli JSON report of a codebase (features, paths, owners, descriptions, decision records, imports between features) into a file and reads it to understand the project before working on it. Use at the start of a task in an unfamiliar codebase, when looking for where a feature lives or who owns it, before changing a feature (to read its description and decisions), or when the other feature skills need the current feature map.
---

# Explore the features of a codebase

Build a map of the codebase by feature and use it to find your way, instead of searching the repository file by file.

If `features` isn't installed, use `npx features-cli@latest` in place of `features` below. `PATH` is the source root (e.g. `./src`); in a monorepo, scan each package or the repository root.

## 1. Generate the report

Write the report to a file outside the repository, so it isn't committed by accident:

```bash
features scan PATH --json --skip-changes > "${TMPDIR:-/tmp}/features.json"
```

`--skip-changes` leaves out the git history, which keeps the report small and fast. Drop it only when you need the commits of each feature.

Generate it again after moving code or editing READMEs: the report is a snapshot. If the user wants the report kept in the repository (for other tools or agents), ask where to save it and whether to add it to `.gitignore` or regenerate it in CI.

An empty array (`[]`) means no feature is detected yet: use the **plan-feature-breakdown** skill to propose some.

## 2. Read the map

The report contains each feature's full README and decision records, so it can be large. Start with the compact index from `scripts/feature_map.py` (in this skill's folder; use its full path, Python 3 only):

```bash
python3 scripts/feature_map.py "${TMPDIR:-/tmp}/features.json"
```

It prints one line per feature: name, path, owner, first sentence of the description and number of decision records, indented by nesting. Then zoom in:

```bash
python3 scripts/feature_map.py "${TMPDIR:-/tmp}/features.json" --feature checkout     # description, decisions, metadata, imports
python3 scripts/feature_map.py "${TMPDIR:-/tmp}/features.json" --file src/features/checkout/api.ts  # the feature that contains a file
python3 scripts/feature_map.py "${TMPDIR:-/tmp}/features.json" --owner team-payments  # the features of an owner
```

File paths in the report are relative to `PATH`. `features find-owner <file> PATH` also gives the owner of a single file without a report.

## 3. Use it

- **Before changing a feature**, read its description and decision records (`--feature`). Respect the decisions, or tell the user when your change goes against one.
- **To find code**, go to the feature's folder first; open other features only when the feature imports them (the `Imports into other features` list).
- **To know who to ask or who reviews**, use the owner. Before opening a pull request, `features diff PATH --since origin/main` (with the repository's default branch) lists the features and owners a change touches.

## Fields of the JSON report

Each feature has:

| Field | Meaning |
| ----- | ------- |
| `name` | First heading of the feature's README, or the folder name |
| `path` | Folder, relative to the scanned folder |
| `owner`, `is_owner_inherited` | `owner` from the README frontmatter; inherited from the parent feature when it has none |
| `description` | README content after the first heading |
| `meta` | Other frontmatter keys (`status`, `deprecated`, `creation_date`…) and feature metadata annotations (`--feature-<key>` comments) |
| `decisions` | Content of each file in `__docs__/decisions/` or `.docs/decisions/` |
| `features` | Nested features (same fields) |
| `dependencies` | Imports from this feature into other features: `sourceFilename`, `targetFilename`, `line`, `featurePath` and `type` (`parent`, `child` or `sibling`) |
| `stats` | `files_count`, `lines_count`, `todos_count`, `coverage` when a coverage report is found, and `commits` |
| `changes` | Git commits touching the feature (empty with `--skip-changes`) |
