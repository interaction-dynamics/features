# Features skills for Claude

[Agent Skills](https://docs.claude.com/en/docs/agents-and-tools/agent-skills/overview) that teach Claude how to reorganize an existing codebase into feature folders, so each feature can be read on its own: its code in one folder, a README that says what it does and who owns it, and decision records that explain why it's built that way.

The skills follow the conventions of [features-cli](https://github.com/interaction-dynamics/features) ([folder architecture guidelines](../../docs/folder-architecture-guidelines.md), [README format](../../docs/readme-format.md), [FAQ](../../FAQ.md)) and use the CLI to check their work.

## Skills

| Skill | What it does |
| ----- | ------------ |
| [`organize-by-feature`](skills/organize-by-feature/SKILL.md) | Entry point. Runs the whole refactor: checks the baseline, plans the breakdown, then extracts, documents, validates and commits one feature at a time. |
| [`explore-features`](skills/explore-features/SKILL.md) | Generates the `features scan --json` report into a file and reads it through a compact index script: every feature's path, owner, description, decision records and imports into other features. Useful on its own to get to know any codebase before changing it. |
| [`plan-feature-breakdown`](skills/plan-feature-breakdown/SKILL.md) | Analyzes the codebase (`features scan`, entry points, domain words, files that change together, `CODEOWNERS`) and writes a breakdown plan for you to approve: features, target paths, files to move, shared code, owners and migration order. Read-only. |
| [`extract-feature`](skills/extract-feature/SKILL.md) | Moves one feature's files into `features/<name>/` with `git mv`, updates imports and other path references, and runs your build and tests. No behavior change. |
| [`document-feature`](skills/document-feature/SKILL.md) | Writes or fixes the feature's `README.md` (owner, creation date, title, description) and its decision records in `__docs__/decisions/`. Never invents an owner or a decision. |
| [`validate-features`](skills/validate-features/SKILL.md) | Runs `features check` and `features scan`, plus a report script that flags features without description or owner, imports between sibling features, oversized features and code left outside any feature. |

## Install

The skills need [features-cli](https://github.com/interaction-dynamics/features#quick-start). They use `features` when it's installed and `npx features-cli@latest` otherwise. The scripts need Python 3.

### As a Claude Code plugin

In Claude Code, add this repository as a plugin marketplace and install the `features` plugin:

```
/plugin marketplace add interaction-dynamics/features
/plugin install features@interaction-dynamics
```

The skills are then available in every project, as `features:organize-by-feature`, `features:plan-feature-breakdown` and so on.

### By copying the skills

Copy the skill folders into your project's `.claude/skills/` folder (shared with your team through git), or into `~/.claude/skills/` for all your projects:

```bash
git clone --depth 1 https://github.com/interaction-dynamics/features.git /tmp/features
mkdir -p .claude/skills
cp -R /tmp/features/projects/skills/skills/* .claude/skills/
```

Copy all the folders: the skills refer to each other by name.

### Other agents

Each `SKILL.md` is plain Markdown. Agents that support the Agent Skills format can use the folders as they are; for others, paste the content of a `SKILL.md` into the agent's instructions.

## Use

Ask Claude in your own words, for example:

- "Reorganize `./src` by feature."
- "Map the features of this repository before we start." / "Who owns `src/features/checkout/api.ts`, and why is it built this way?"
- "Which features does this codebase have? Propose a breakdown, don't move anything yet."
- "Move the checkout code into its own feature folder."
- "Document the `billing` feature and record why we use Stripe."
- "Check that our features follow the guidelines."

Or call a skill directly, e.g. `/organize-by-feature` (or `/features:organize-by-feature` with the plugin).

Claude works on one feature at a time and runs your tests after each one, so every step is a small commit you can review or revert. It asks before anything that would change behavior, and leaves visible `TODO`s for owners and decisions it can't source.

## Contributing

Each skill is a folder in [`skills/`](skills) with a `SKILL.md` (YAML frontmatter with `name` and `description`, then instructions), and optional `references/` and `scripts/`. Keep `SKILL.md` short and move details into reference files.

When the CLI changes, check that every command and flag used by the skills still exists in the [CLI reference](../../docs/cli.md), and that the fields read by the scripts still exist in the JSON report. The scripts can be tried on the demo project:

```bash
cd projects/cli && cargo run -- scan ../../examples/demo/src --json --skip-changes > /tmp/features.json
python3 ../skills/skills/explore-features/scripts/feature_map.py /tmp/features.json
python3 ../skills/skills/validate-features/scripts/feature_report.py /tmp/features.json --root ../../examples/demo/src
```
