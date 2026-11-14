---
status: implemented
date: 2026-11-14
---

# Specs & Decisions tab

## Context

features-cli only reads decision records from `<feature>/__docs__/decisions/` (or `.docs/decisions/`, whichever it finds first) and exposes them as raw markdown strings in `Feature.decisions`. The repository already writes specs as `__docs__/specs/NNN-title.md` and `docs/specs/NNN-title.md` with a YAML frontmatter (`status`, `date`), and many teams keep their specs in tool-specific layouts (GitHub Spec Kit, Kiro), but the CLI ignores all of them. Specs and decisions describe the same feature from two angles (what we build / why we built it this way), so they should be read, linked and shown together.

## Requirements

### Detection

- Per feature, read every one of `__docs__/decisions/`, `__docs__/specs/`, `.docs/decisions/` and `.docs/specs/` (not only the first folder found).
- At the root of the scanned folder, read `docs/decisions/`, `docs/adr/` (decisions) and `docs/specs/` (specs).
- A record is either one Markdown file (`specs/001-x.md`) or a folder (`specs/003-sso/`) whose main file is `spec.md` (falling back to `README.md`, `index.md`, `requirements.md`, then the first `.md` file); the other files of the folder (`plan.md`, `tasks.md`, …) are its sub-files.
- `README.md` and template files (`template.md`, `adr-template.md`, `*-template.md`) are not records.
- Extra readers:
  - **GitHub Spec Kit**: root `specs/NNN-name/` folders that contain a `spec.md` (`plan.md`, `tasks.md`, `research.md`, `data-model.md`, `quickstart.md` and `contracts/*` are sub-files). Status comes from the `**Status**:` line of `spec.md`, tasks from the checkboxes of `tasks.md`. Loose `.md` files in root `specs/` are native specs.
  - **Kiro**: `.kiro/specs/<name>/` folders, `requirements.md` is the main file, `design.md` and `tasks.md` are sub-files. The title is the folder name in words unless the frontmatter or a non-generic heading gives one.

### Record model

Each feature gets an additive `records` array; `decisions: string[]` is unchanged. A record is:

```json
{
  "kind": "spec | decision",
  "id": "001-login-flow",
  "title": "Login flow",
  "status": "accepted",
  "date": "2026-01-02",
  "path": "src/features/auth/__docs__/specs/001-login-flow.md",
  "content": "…markdown of the main file…",
  "files": [{ "name": "plan.md", "path": "…", "content": "…" }],
  "source": "native | speckit | kiro",
  "links": { "decisions": [], "specs": [], "supersedes": [], "superseded_by": [] },
  "tasks": { "done": 3, "total": 5 }
}
```

- `status`, `date`, `tasks` are omitted when unknown, `files` when empty.
- Frontmatter keys: `title`, `status`, `date`, `feature`, `decisions`, `specs` / `spec`, `supersedes`, `superseded_by` (string or list).
- Status falls back to a `**Status**:` / `Status:` line, then to the first line of a `## Status` section (classic MADR / Nygard ADR). It is lowercased and reduced to its first word (`Superseded by 0005` → `superseded`).
- Date falls back to a `**Created**:` / `**Date**:` / `Date:` line.
- Title: frontmatter `title`, else the first `#` heading (Spec Kit's `Feature Specification:` prefix removed), else the id.
- Tasks count `- [ ]` / `- [x]` lines: of `tasks.md` when the record has one, else of the main file (specs only).
- Links are record ids (`002-use-jwt`; `002-use-jwt.md`, a path or a bare number also resolve). They are resolved in both directions: a spec listing a decision gives the decision a link back to the spec, `supersedes` fills the other record's `superseded_by`. Resolution looks in the same feature first, then the project-level records, then any record with that id.

### Mapping root records to features

1. A `feature:` frontmatter key matching a feature path (exact or suffix) or name (case-insensitive, slugified) attaches the record to that feature.
2. Otherwise, for Spec Kit and Kiro records only, the folder name minus its `NNN-` prefix, slugified, matching a feature's slugified name or last path segment attaches it (first match in tree order).
3. Otherwise the record is project-level.

### Project-level records (non-breaking)

`features.json` and `features scan --json` keep their top-level array. Project-level records are exposed:

- by `features serve` at `/records.json` and by `features build` as `records.json`: `{ "records": [Record, …] }`;
- by `features scan --json --project-records`, which prints `{ "features": [...], "records": [...] }` instead of the array.

`features serve` rescans when a text file inside a `specs`, `decisions` or `adr` folder changes, not only when a README changes.

### `features check` warnings

Warnings are printed but do not fail the check:

- a link that resolves to no record;
- a native record with no status;
- a `superseded` record with nothing superseding it;
- an `implemented` spec with open tasks.

### Web

- The Decisions tab becomes **Specs & Decisions**, with the record count in its label.
- Merged timeline sorted by date (newest first, undated last), each card showing a kind badge (Spec / ADR), status badge, source badge (Spec Kit / Kiro), task progress bar, sub-file tabs for folder records, and clickable link chips (navigate to the linked record, in another feature or the project view if needed).
- Filter chips: All / Specs / Decisions, and a toggle to hide superseded and rejected records.
- Empty state with help for both specs and decisions.
- A project-level **Specs & Decisions** page in the sidebar for root records not attached to a feature.
- Older `features.json` without `records` still shows the raw `decisions`; a missing `records.json` (aggregated builds, older servers) means no project-level records.
- The card body drops the leading `# Title` line, already shown on the card.

### Docs

FAQ (adding specs, Spec Kit / Kiro support and the mapping rule), `docs/folder-architecture-guidelines.md` (MAY row for specs), the `document-feature` skill and `explore-features/scripts/feature_map.py` (prints records).

### Website and demo

- The home page advertises the feature: a "New" link in the hero, a Specs & decisions section (linked specs and ADRs, progress, `features check`, Spec Kit and Kiro layouts) and a dashboard screenshot.
- Guidelines, FAQ, CLI and concept pages describe specs, links, Spec Kit / Kiro and project-level records; the `decisions` screenshot is replaced by `specs-decisions`.
- The live demo example (`examples/demo`, feature 0) gets linked specs and decisions, a folder spec with tasks and a superseded decision.

## Out of scope

- Writing or editing records from the dashboard.
- Carrying project-level records through `features aggregate` (feature records are merged; project-level ones are not).
- Name-based mapping of native root records (they need a `feature:` key).
- Spec Kit / Kiro layouts nested inside feature folders.

## Plan

1. CLI: `Record` model in `models.rs`, a `file_scanner/records` pass (native, Spec Kit and Kiro readers, frontmatter/status/tasks parsing, feature mapping, link resolution), `records` on `Feature` and `AggregatedFeature`, every `Feature` constructor updated, project-level records returned next to the features (`ScanOutput`), served as `/records.json`, written by `build`, printed by `scan --json --project-records`.
2. `features check` warnings.
3. Rust tests and fixtures in `examples/specs-and-decisions` (native per-feature and root records, a Spec Kit `specs/` folder and a `.kiro/specs/` folder).
4. Web: Zod schema, `src/features/records` (utils + tests, timeline, card), tab rename, project-level route (`/records`) and sidebar link.
5. Docs (FAQ, `docs/cli.md`, folder guidelines) and skills (`document-feature` with a spec template, `explore-features` and `feature_map.py --records`).
