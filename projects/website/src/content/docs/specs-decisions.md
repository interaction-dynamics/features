---
title: Specs and decisions
description: Where features-cli reads specs and decision records, their front matter, links and statuses, GitHub Spec Kit and Kiro layouts, the records command and the JSON format.
---

Specs say **what** a feature must do: requirements, plan and tasks. Decision records say **why** it is built that way. Features reads both from the repository and shows them side by side, sorted by date, in the _Specs & Decisions_ tab of each feature. The [Specs & decisions page](/specs-decisions) shows what it looks like.

## Create a record

Let the CLI pick the next number:

```bash
features new spec auth "Login flow"         # features/auth/__docs__/specs/001-login-flow.md
features new decision auth "Use JWT tokens" # features/auth/__docs__/decisions/002-use-jwt-tokens.md
features new decision . "Use a monorepo"    # docs/decisions/001-use-a-monorepo.md (project level)
```

Or create the file yourself:

1. In the feature folder, create `__docs__/specs/` or `__docs__/decisions/` (`.docs/specs/` and `.docs/decisions/` work too).
2. Add a Markdown file numbered from `001`, such as `__docs__/decisions/002-use-jwt.md`.
3. Start it with a front matter, as below.

## Front matter

```markdown
---
status: accepted # draft, proposed, accepted, implemented, rejected, deprecated, superseded
date: 2026-01-15
spec: 001-login-flow # on a decision: the spec it belongs to
supersedes: 001-use-sessions # the record it replaces, if any
---

# Use JWT
```

- **`status`** is shown as a badge. Specs usually go `draft`, `accepted`, `implemented`; decisions `proposed`, `accepted`, then `deprecated` or `superseded`.
- **`decisions`** on a spec lists its decisions: `decisions: [002-use-jwt]`. Links work both ways and can use the number alone (`decisions: ["002"]`).
- **`supersedes`** retires an older record, which then shows what replaced it.
- **`feature`** on a project-level record shows it in that feature instead (`feature: auth`).

Classic [MADR](https://adr.github.io/madr/) files without front matter work too: the status is read from a `**Status**: Accepted` line or a `## Status` section.

## Content

- **Decisions** follow the MADR sections: _Context_, _Considered options_, _Decision_, _Consequences_.
- **Specs** follow _Context_, _Requirements_, _Out of scope_ and _Plan_. `- [ ]` and `- [x]` checklists become a progress bar.
- **A large spec can be a folder**: `__docs__/specs/003-sso/` with `spec.md` as its main file, and `plan.md`, `tasks.md` or other files shown as tabs.

## Project-level records

Records about the whole project go in `docs/specs/`, `docs/decisions/` or `docs/adr/` at the root of the scanned folder. They show in the _Not attached to a feature_ section of the _Outside features_ page, unless their front matter names a `feature`.

## GitHub Spec Kit and Kiro

At the root of the scanned folder, the layouts of two spec-driven tools are read too:

| Tool                                                  | Layout                | Main file         | Shown with it                                                                         |
| ----------------------------------------------------- | --------------------- | ----------------- | ------------------------------------------------------------------------------------- |
| [GitHub Spec Kit](https://github.com/github/spec-kit) | `specs/NNN-name/`     | `spec.md`         | `plan.md`, `research.md`, `data-model.md`, `quickstart.md`, `tasks.md`, `contracts/*` |
| [Kiro](https://kiro.dev/docs/specs/)                  | `.kiro/specs/<name>/` | `requirements.md` | `design.md`, `tasks.md`                                                               |

Each one is attached to a feature by a `feature:` front matter key, else by its folder name without the `NNN-` prefix (`specs/003-checkout/` goes to the `checkout` feature). The others are project-level records. Their task progress comes from `tasks.md`, and they get a _Spec Kit_ or _Kiro_ badge. For Spec Kit, the status comes from the `**Status**:` line and the date from `**Created**:`.

## List the records

```bash
features records ./          # every spec and decision, per feature
features records ./ --json   # the project-level records, as { "records": [...] }
```

In the JSON report of `features scan --json`, each feature has a `records` array:

```json
{
  "kind": "spec",
  "id": "001-login-flow",
  "title": "Login flow",
  "status": "implemented",
  "date": "2026-01-15",
  "path": "features/auth/__docs__/specs/001-login-flow.md",
  "content": "# Login flow ...",
  "links": {
    "decisions": ["002-use-jwt"],
    "specs": [],
    "supersedes": [],
    "superseded_by": []
  },
  "tasks": { "done": 2, "total": 2 }
}
```

- `status`, `date`, `tasks`, `files` (the other files of a folder spec) and `feature` are only present when known.
- `source` is `speckit` or `kiro` for the specs of those tools, and absent otherwise.
- The project-level records are not in `features scan --json`, which stays an array of features. They are printed by `features records --json`, served at `/records.json` by `features serve` and written to `records.json` by `features build`.
- The older `decisions` array of raw Markdown is kept for compatibility.

## Validate

The [`record-issues`](/docs/check-rules#record-issues) rule of `features check` warns about links to records that don't exist, records without a status, superseded records that nothing replaces and implemented specs with open tasks. Spec Kit and Kiro specs aren't asked for a status.
