---
status: implemented
date: 2026-11-14
---

# Claude skills to reorganize a codebase into feature folders

## Context

`features-cli` maps a codebase by feature, but only if the code is already organized by feature. Most codebases are organized by technical layer (`components/`, `services/`, `utils/`), so the first step for a team adopting the tool is a refactor that is long, repetitive and easy to get wrong. AI coding agents can do most of this work, but only if they know the conventions the CLI relies on and work in small, verifiable steps.

The README already explains how to give an agent a map of the codebase. This project goes one step further: reusable [Agent Skills](https://docs.claude.com/en/docs/agents-and-tools/agent-skills/overview) that teach Claude how to break a codebase down into readable feature folders, following the repository's own guidelines.

## Requirements

- A new project in `projects/skills`, next to `projects/cli` and `projects/web`.
- One folder per skill under `projects/skills/skills/`, each with a `SKILL.md` that has YAML frontmatter with `name` and `description`, plus optional reference files and scripts.
- The skills follow the conventions documented in `docs/folder-architecture-guidelines.md`, `docs/readme-format.md`, `FAQ.md` and the root `README.md` instead of inventing new ones:
  - features live in a `features` folder or are marked with `feature: true` in their README frontmatter;
  - each feature has a `README.md` with a title, a one-line description after the title and an `owner` in the frontmatter;
  - decision records go in `__docs__/decisions/` (MADR style);
  - code shared by several features stays in common folders outside the features;
  - feature metadata annotations (`--feature-<key>`) are kept when code moves.
- The skills cover the whole workflow:
  - `organize-by-feature`: entry point that runs the workflow below, one feature at a time;
  - `explore-features`: generate the `features scan --json` report into a file and read it through a compact index, so an agent can learn the project (paths, owners, descriptions, decisions, imports between features) before changing it. The other skills reuse the same file;
  - `plan-feature-breakdown`: analyze the codebase and propose a feature breakdown for the user to approve;
  - `extract-feature`: move the code of one feature into its folder in small, safe steps (imports updated, tests passing, behavior unchanged);
  - `document-feature`: write or fix a feature's `README.md` and its decision records;
  - `validate-features`: check the result with `features check`, `features scan` and a report script.
- Every CLI command and flag used by the skills exists in `docs/cli.md`.
- The skills never invent owners or decisions: they ask the user or leave a visible placeholder.
- The skills work with any language supported by the CLI and with any package manager or test runner (they detect the project's own commands).
- The project can be installed as a Claude Code plugin (marketplace at the repository root) or by copying the skill folders into `.claude/skills/`.
- `projects/skills/README.md` explains what each skill does and how to install it.
- The root `README.md` mentions the skills and links to the project, with a small edit that does not conflict with the "AI coding agents" section proposed in interaction-dynamics/features#148.

## Out of scope

- Publishing the skills to a public skills registry.
- Automated tests that run the skills against a real model.
- Changes to the CLI (no new commands or checks).
- Skills for other agents' formats (Cursor rules, Copilot instructions). The `SKILL.md` files are plain Markdown and can be reused by hand.

## Plan

1. Create `projects/skills/` with a `.claude-plugin/plugin.json` manifest and a `skills/` folder.
2. Write the six skills, keeping each `SKILL.md` short and moving templates and details into `references/` files.
3. Add `explore-features/scripts/feature_map.py` (Python standard library only) that prints a compact index of the JSON report, the details of one feature, the feature containing a file, or the features of an owner.
4. Add `validate-features/scripts/feature_report.py` (Python standard library only) that reads `features scan --json --skip-changes` and lists features without description or owner, cross-feature imports, large features and duplicate names.
5. Add `.claude-plugin/marketplace.json` at the repository root so `/plugin marketplace add interaction-dynamics/features` works.
6. Write `projects/skills/README.md` (what each skill does, install as a plugin or by copying).
7. Add a short section to the root `README.md` linking to the project.
8. Check every command and flag used by the skills against `docs/cli.md`, and run both scripts against `examples/demo/src`.
