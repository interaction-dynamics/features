---
status: implemented
date: 2026-11-14
---

# Website: CLI page selectors and AI page

## Context

The CLI page showed the four install methods as four cards and the options as one long list, which made it hard to find the command for your tool or the options of a given command. The site also had no page explaining how to use features-cli with AI coding agents, though the README covers it.

## Requirements

- CLI page hero: a combo box (native `<select>`) picks the install tool (npx, npm, Homebrew, cargo) and shows only that tool's command and a one-line hint. Without JavaScript, every command is shown and the picker is hidden.
- CLI page "Options at a glance": a tab bar (`All commands` plus one tab per command) filters the option rows to the ones the selected command accepts. Options marked `all` show under every command; groups left empty are hidden. Groups are stacked in a single column. Tabs follow the ARIA tabs pattern (arrow keys, Home, End).
- New `/ai` page, linked in the main navigation between CLI and FAQ: why a feature map helps agents, the snippet for `AGENTS.md` / `CLAUDE.md` / `.cursor/rules`, a per-task command loop (scan, find-owner, check, diff), the Claude skills plugin with its six skills and example prompts, and keeping decisions next to each feature.
- Landing page: the badge reads "Open source · Any language" (no "Rust"), and the AI section lead ends with "Tracking, documenting and maintaining what you built is."
- No horizontal page scroll at phone width.

## Out of scope

- Copy-to-clipboard buttons.
- Changing the README wording.

## Plan

1. Replace the install cards in `projects/website/src/pages/cli.astro` with the combo box and a small inline script.
2. Add the tab bar and row filtering to the options section of the same page.
3. Add `projects/website/src/pages/ai.astro` from the README's AI sections and `projects/skills/README.md`; add the nav entry in `src/data/site.ts`.
4. Edit the landing badge and AI lead in `src/pages/index.astro`.
5. Build the site and check the pages in the browser at desktop and phone width.
