---
status: accepted
date: 2024-02-12
decisions: [003-use-tailwind]
---

# Shared design tokens

## Context

Routes and libs each define their own colors and spacing, so screens drift apart.

## Requirements

- [x] One source for colors, spacing and typography
- [x] Every route uses the shared tokens
- [ ] Dark mode for every token
- [ ] Lint rule against hard-coded colors

## Out of scope

- A full component library
