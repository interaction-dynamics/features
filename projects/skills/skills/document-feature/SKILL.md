---
name: document-feature
description: Writes or fixes the README.md of a feature folder (frontmatter with owner and other metadata, title, one-line description), its specs in __docs__/specs and its decision records in __docs__/decisions, following the features-cli formats, so the feature can be understood on its own. Use when the user asks to document a feature, add an owner or a description, write a spec, record a design decision (ADR, MADR), or after moving code into a feature folder.
---

# Document a feature

Give the feature a README and, when there is something worth writing down, specs (what the feature must do) and decision records (why it is built this way). features-cli reads them to show the feature's name, description, owner, specs and decisions in its dashboard and JSON report.

If `features` isn't installed, use `npx features-cli@latest` in place of `features` below.

## 1. Gather the facts

Read the feature's code, then collect, without guessing:

- **What it does**: entry points (routes, endpoints, commands, exported functions), the user-facing behavior, the main concepts.
- **Owner**: in this order, the user's answer, the approved breakdown file, `CODEOWNERS` lines that covered the files before they moved, a team named in existing docs. Commit authors (`git shortlog -sn --no-merges -- <path>`) are only a hint to show the user, not an owner. If you can't find a team, ask. If the user doesn't know, leave `owner` out and add a `# TODO: owner` comment line in the frontmatter. Never write a placeholder value such as `owner: TODO`: `features codeowners` would write it into `CODEOWNERS`.
- **Creation date**: features-cli dates a feature from the first commit in its folder, so a feature that was just moved looks brand new. Find when its oldest file was added with `git log --follow --diff-filter=A --format=%as -- <file> | tail -1` for each file, and keep the earliest date for `creation_date`.
- **Decisions**: "why" comments in the code, commit messages and pull request descriptions (`git log --follow -- <file>`), existing ADRs elsewhere in the repository (`docs/adr/`, `docs/decisions/`), linked issues.
- **Specs**: existing specs in `docs/specs/`, a GitHub Spec Kit `specs/NNN-name/` folder or a Kiro `.kiro/specs/<name>/` folder about this feature, issues or product documents describing the expected behavior.

Check whether the feature inherits an owner: run `features scan PATH --json --skip-changes` and look at the feature's `owner` and `is_owner_inherited`. A nested feature inherits its parent's owner when it has none: leave `owner` out when it's the same as the parent's.

## 2. Write the README

Create or update `README.md` at the root of the feature folder, using [references/readme-template.md](references/readme-template.md). The rules that matter for features-cli:

- The frontmatter must be the **very first thing** in the file: `---` on line 1, then YAML, then `---`.
- `owner` is a team or user handle without `@` (`team-payments`, or `my-org/payments` for a GitHub team). `features codeowners` adds the `@` prefix.
- Add `feature: true` only when the folder isn't directly inside a `features` folder.
- The first `#` heading is the feature's display name. Make it unique across the scanned folder: `features check` fails on duplicate names.
- Right after the heading, write one sentence that describes the feature. Write it for a product manager: what the user can do, not how it's coded.
- Everything after the heading becomes the feature's description in the dashboard and the JSON report. Keep it short: purpose, entry points, main concepts, links to related features. No code dumps; the code is next door.
- Extra frontmatter keys are allowed and shown as metadata (`status`, `deprecated`, `figma`, `docs`…). Only add values you can source.

If a README already exists, keep its content and fix only what's missing or wrong. Don't remove custom frontmatter keys.

## 3. Write decision records

Decision records are optional. Write one only for a real choice with evidence (a library picked over another, a data model, a security or performance trade-off, a deliberate limitation). Don't write one for every file.

- Path: `__docs__/decisions/NNN-kebab-case-title.md` inside the feature folder, numbered from `001`.
- Format: [references/decision-template.md](references/decision-template.md) (MADR style: Title, Status, Context, Decision, Consequences).
- When an existing ADR elsewhere in the repository is only about this feature, move it into the feature's `__docs__/decisions/` with `git mv` and update links to it.
- When you infer a decision from the code without written evidence, set its status to `proposed` and ask the user to confirm it. Never present a guess as an accepted decision.
- Give it a frontmatter with `status` and `date` so features-cli can sort and filter it (a `## Status` section also works).

## 4. Write specs

Specs are optional too. Write one when the user describes a change to build, or when a spec exists elsewhere and only concerns this feature.

- Path: `__docs__/specs/NNN-kebab-case-title.md` inside the feature folder, numbered from `001`. A large spec can be a folder, `__docs__/specs/NNN-title/`, with `spec.md` as its main file and `plan.md`, `tasks.md`… next to it.
- Format: [references/spec-template.md](references/spec-template.md): frontmatter with `status` (`draft`, `accepted`, `implemented`, `superseded`) and `date`, then Context, Requirements, Out of scope and Plan.
- Write tasks as `- [ ]` / `- [x]` checkboxes: features-cli shows them as a progress bar. Don't mark a spec `implemented` while it has open tasks.
- Leave Spec Kit (`specs/NNN-name/`) and Kiro (`.kiro/specs/<name>/`) specs where they are: features-cli reads them and attaches them to the feature whose name matches the folder name. Add `feature: <feature name or path>` to their frontmatter when the folder name doesn't match.

## 5. Link specs and decisions

Link records with frontmatter keys holding the other record's id (file or folder name without `.md`): `decisions: [002-use-jwt]` on a spec, `spec: 001-login-flow` on a decision, `supersedes: 001-old-choice` on a record that replaces another one. Write each link once: features-cli adds the reverse link.

## 6. Check

```bash
features scan PATH --description --skip-changes
features check PATH
```

The feature must show its new name, owner and description, and `check` must pass. `check` also prints warnings about specs and decisions (broken links, missing status, superseded records nothing supersedes, implemented specs with open tasks): fix them. Then tell the user which values you couldn't source (every `TODO`) and which decisions are `proposed`.
