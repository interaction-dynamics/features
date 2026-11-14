---
status: accepted
date: 2024-02-10
decisions: [003-render-with-webgl]
---

# Glass material editor

A spec can be a folder: `spec.md` is the main file, `plan.md` and `tasks.md` show up as tabs, and the checkboxes of `tasks.md` become the progress bar.

## Context

Designers tune glass materials in Figma, then developers copy the values by hand.

## Requirements

- Preview the material live while editing blur, tint and refraction
- Export the values as design tokens

## Out of scope

- Animations
