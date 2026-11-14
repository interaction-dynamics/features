---
status: implemented
date: 2026-11-14
---

# Spec Kit and Kiro readers

## Context

[002-specs-and-decisions-tab](./002-specs-and-decisions-tab.md) reads native specs and decision records (`__docs__/`, `.docs/`, root `docs/`). Many teams write their specs with spec-driven AI tools that use their own layout at the root of the repository: GitHub Spec Kit (`specs/NNN-name/`) and Kiro (`.kiro/specs/<name>/`). Features should read them where they are and show them with the feature they describe, without asking teams to move them.

## Requirements

### Detection (root of the scanned folder only)

- **GitHub Spec Kit**: `specs/NNN-name/` folders containing a `spec.md` (the main file). Every other text file of the folder is a sub-file, nested ones included (`contracts/login.yaml`), ordered `plan.md`, `research.md`, `data-model.md`, `quickstart.md`, `tasks.md`, then the others by name. Loose `.md` files in `specs/` are native specs; `README.md` and `*template.md` files are skipped.
- **Kiro**: `.kiro/specs/<name>/` folders containing a `requirements.md` (the main file), with `design.md`, `tasks.md` and the other text files as sub-files.
- Folders without their main file, and hidden folders, are skipped.

### Record fields

- `source`: `speckit` or `kiro`; omitted for native records, so existing JSON reports and snapshots don't change.
- Title: frontmatter `title`, else the first heading without Spec Kit's `Feature Specification:` prefix; for Kiro, a generic heading (`Requirements Document`, `Design`, …) is replaced by the folder name in words (`notifications-center` → `Notifications center`).
- Status: from the `**Status**:` line (existing rules). Date: frontmatter `date`, else a `Date:` line, else Spec Kit's `**Created**:` line.
- Tasks: counted in `tasks.md` only, not in the review checklist of `spec.md`.

### Mapping to features

1. A `feature:` frontmatter key, as for native root records.
2. Otherwise the folder name without its `NNN-` prefix, slugified, matching a feature's slugified name or the slugified last segment of its path (first match in tree order): `specs/003-auth/` goes to the `auth` feature.
3. Otherwise the record is project-level (`features records --json`, `records.json`, the Project records page).

### Checks

- `features check` doesn't report a missing status for Spec Kit and Kiro records (Kiro has none). The other warnings apply.

### Web

- A **Spec Kit** or **Kiro** badge on the record card.
- No "no status" pill on Spec Kit and Kiro records without a status.
- Non-markdown sub-files (contracts) are shown as a code block.

### Docs, website and demo

- FAQ entry "Are GitHub Spec Kit and Kiro specs supported?", `docs/cli.md` (`source` field), folder guidelines (MAY row), `explore-features` skill.
- Website: a "New" link in the home hero, a Specs & decisions section on the home page with the Spec Kit and Kiro layouts and a dashboard screenshot, the `records` and `new` commands on the CLI page, the Spec Kit and Kiro FAQ entry, guidelines updates.
- The live demo example (`examples/demo`, feature 0) gets linked specs and decisions, a folder spec with tasks and a superseded decision.

## Out of scope

- Spec Kit and Kiro layouts nested inside feature folders.
- Name-based mapping of native root records (they still need a `feature:` key).
- Other spec tools (Tessl, BMAD…).

## Plan

1. CLI: `RecordSource` on `Record` (skipped when native), `records/layouts.rs` with the Spec Kit and Kiro readers called by `read_root_records`, folder-name mapping in `split_root_records`, missing-status warning for native records only.
2. Tests: unit tests in `layouts.rs`, `tests/spec_tools_test.rs` on the `examples/spec-tools` fixture.
3. Web: `source` in the Zod schema, source badge, code blocks for non-markdown sub-files, tests.
4. Docs, website and demo.
