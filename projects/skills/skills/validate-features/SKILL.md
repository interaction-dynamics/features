---
name: validate-features
description: Validates the feature folders of a codebase with features-cli (features check, features scan) and a report script that flags features without description or owner, imports between sibling features, oversized features, duplicate names and code left outside any feature. Use after moving or documenting features, before committing a feature refactor, or when the user asks whether the codebase follows the feature-based architecture guidelines.
---

# Validate features

Check that the features are detected as intended and that each one can be read on its own.

If `features` isn't installed, use `npx features-cli@latest` in place of `features` below. `PATH` is the scanned source root (e.g. `./src`).

## 1. Run the checks

```bash
features check PATH
```

It fails (exit code 1) on duplicate feature names and lists their paths. A failing check is a blocker: rename a feature (its README heading, or its folder when there is no heading) until it passes.

## 2. Look at the feature tree

```bash
features scan PATH --description --skip-changes
```

Compare it with the approved breakdown: every planned feature is listed, at the planned path, with the right name, and there is no unexpected feature (a folder inside `features` that should be shared code gets a README with `feature: false`).

## 3. Run the report

```bash
features scan PATH --json --skip-changes > "${TMPDIR:-/tmp}/features.json"
python3 scripts/feature_report.py "${TMPDIR:-/tmp}/features.json" --root PATH
```

The same file is the feature map read by the **explore-features** skill.

`scripts/feature_report.py` is in this skill's folder; use its full path. It needs Python 3 and nothing else. It prints a table of the features (owner, files, lines, decisions, TODOs), then:

- **Errors**: duplicate names. Same as `features check`.
- **Warnings**:
  - *no description*: the feature has no README, or nothing after its heading. Use the **document-feature** skill.
  - *no owner*: no `owner` in the README and no parent to inherit from. Ask the user; don't invent one.
  - *imports from sibling feature*: the feature reaches into another feature's files. `[circular]` means both import each other. See "Fixing cross-feature imports" below.
- **Notes**: features larger than `--max-files` (40) or `--max-lines` (4000). Consider nested features, only along product boundaries.
- **Files outside any feature**, per top-level folder: shared code and application glue are expected there; code used by a single feature isn't.

Add `--strict` to exit with 1 on warnings too (useful in CI once the refactor is done).

## 4. Fix, or report

Fix what belongs to the feature you're working on, then run the checks again. Leave the rest as a list for the user, with the report's exact lines.

### Fixing cross-feature imports

Some coupling is normal; the goal is that a reader of one feature doesn't need to open another feature's internals. In order of preference:

1. The imported code is used by several features → move it to a shared folder outside `features/`.
2. The imported code is really part of the importing feature → move it there.
3. The import is a deliberate use of another feature → keep it, and point it at that feature's public entry point if the project uses them (`index.ts`, `__init__.py`, `mod.rs`).
4. Circular imports → tell the user; breaking a cycle often means changing code, which is out of scope for a move-only refactor.

Every move follows the **extract-feature** rules: `git mv`, update imports, run the tests.

## 5. Before opening a pull request

```bash
features diff PATH --since origin/main
```

Use the repository's default branch in place of `main`. It lists the features touched by the branch and their owners: include it in the pull request description so the owners know their features moved.
