---
status: implemented
date: 2026-11-14
---

# Specs & Decisions tab

## Context

Features only reads architecture decision records (`__docs__/decisions/*.md` or
`.docs/decisions`, stopping at the first folder found) as raw markdown strings,
and the web dashboard shows them as collapsible cards titled by their first line.
The repository already writes specs (`__docs__/specs/NNN-title.md`,
`docs/specs/NNN-title.md`) with YAML frontmatter, but the CLI ignores them.

Specs say what to build, decisions say why it was built that way. Showing them
side by side, linked to each other and to their feature, gives the full story of
a feature in one place.

## Requirements

### Detection

- Per feature, read every one of these folders (not only the first one found):
  `__docs__/decisions/`, `__docs__/specs/`, `.docs/decisions/`, `.docs/specs/`.
- At the root of the scanned folder, read `docs/decisions/`, `docs/adr/` and `docs/specs/`.
  - A root record with a `feature: <name or path>` frontmatter field is attached to that feature.
  - Every other root record is a project-level record.
- A spec is either one file (`specs/001-login.md`) or a folder (`specs/003-sso/`) whose
  main file is `spec.md` (else `README.md`, else `index.md`); the folder's other markdown
  files (`plan.md`, `tasks.md`...) are its sub-files.
- `README.md` files directly inside a records folder are not records.

### Record model

Each record is serialized as:

```json
{
  "kind": "spec | decision",
  "id": "002-use-jwt",
  "title": "Use JWT",
  "status": "accepted",
  "date": "2026-01-02",
  "path": "features/auth/__docs__/decisions/002-use-jwt.md",
  "content": "markdown without frontmatter",
  "files": [{ "name": "tasks.md", "path": "...", "content": "..." }],
  "links": { "decisions": [], "specs": [], "supersedes": [], "superseded_by": [] },
  "tasks": { "done": 1, "total": 3 }
}
```

- `id` is the file stem, or the folder name for a folder spec.
- `title` comes from the `title` frontmatter field, else the first `#` heading, else the id.
- `status` comes from the `status` frontmatter field, else a `Status: x` line (with optional
  bold and list markers, e.g. `**Status**: Accepted`, `* Status: proposed`), else the first
  line of a `## Status` section. It is lowercased. Missing statuses are omitted.
- `date` comes from the `date` frontmatter field, else a `Date: x` line, else omitted.
- Links come from frontmatter: `decisions` (on specs), `spec` / `specs` (on decisions),
  `supersedes`. Each accepts a single id or a list. A link matches a record whose id equals it,
  or whose number prefix equals it (`002` matches `002-use-jwt`).
- Links are resolved in both directions: a spec listing a decision adds the spec to the
  decision's `specs`, and `supersedes` fills the target's `superseded_by`. Links resolve within
  the feature's records first, then the project-level records (so `001` in a feature points to the
  feature's own `001-...` even when a project record also starts with `001`).
- `tasks` counts `- [ ]` / `- [x]` lines (outside code fences) in the main file and the
  sub-files. It is omitted when there are none. `files` is omitted when empty.

### JSON output

- Each feature gets an additive `records` field (sorted by date then id). The existing
  `decisions: string[]` field is unchanged.
- The top-level JSON stays an array of features. Project-level records are exposed separately,
  without changing that shape:
  - `features serve` serves them at `/records.json` and `features build` writes `records.json`,
    both as `{ "records": [Record...] }`.
  - `features records [path] --json` prints the same object.
- Aggregated reports (`features aggregate`) keep each feature's records, deduplicated by path.

### `features check`

Prints warnings (they don't fail the check) for:

- a link to a record that does not exist,
- a record without a status,
- a superseded record that no record supersedes,
- an `implemented` spec with open tasks.

### `features new`

`features new spec|decision <feature> "<title>" [--path <dir>]` writes the next numbered file
(`NNN-kebab-title.md`) in `<feature>/__docs__/specs|decisions/`, where `<feature>` is a feature
name or path. `.` writes in the project-level `docs/specs|decisions/` folder.

### Web dashboard

- The "Decisions" tab becomes "Specs & Decisions", with the number of records in its label.
- It shows a merged timeline sorted by date (newest first) with a kind badge (Spec / ADR), a status
  badge, a task progress bar, clickable link chips that scroll to and open the linked record, and
  filter chips: All / Specs / Decisions, plus "Hide superseded/rejected".
- The empty state explains how to add both kinds.
- A "Project records" page (sidebar link with the number of records, route `#/records`) shows the
  project-level records with the same timeline.
- The expanded card shows the markdown without the heading that repeats the title, then each
  sub-file of a folder spec.
- Reports from older CLI versions (no `records` field) still show their raw `decisions`.
- The Zod schema in `projects/web/src/models/feature.ts` describes `records`.

### Docs

- FAQ (repository and website): "How can I add a decision?" becomes "How can I add a spec or a
  decision?"; the old anchor keeps working.
- `docs/folder-architecture-guidelines.md`: MAY row for `__docs__/specs`.
- The `document-feature` skill and `explore-features/scripts/feature_map.py` know about records.
- The website advertises it: a "Specs & decisions" section on the homepage with a dashboard
  screenshot (demo data in `examples/demo`), plus the guidelines, CLI, AI and architecture pages.

## Out of scope

- Readers for Spec Kit, Kiro or OpenSpec layouts (built in another branch).
- Editing records from the web dashboard.

## Plan

1. Save this spec.
2. CLI: `Record` model, scanner pass reading feature and root folders, frontmatter / MADR parsing,
   link resolution, task counting; add `records` everywhere a `Feature` is built; tests and an
   `examples/` fixture.
3. `records.json` in serve and build, `features records`, watcher rescans on record edits.
4. `features check` warnings and `features new`.
5. Web: Zod schema, Specs & Decisions tab, project records page.
6. Docs, skill and `feature_map.py`.
7. `cargo test`, `cargo clippy`, web lint/tests/build, format, commit.
