---
status: implemented
date: 2023-09-01
decisions: [001-feature-based-architecture]
---

# Feature folders

Project-level specs live in `docs/specs`. Unlike the specs in a feature's `__docs__/specs`, they cover the whole project or several features at once, so they show on the **Outside features** page.

## Context

Code for one feature is spread across `components/`, `hooks/` and `services/`. Nobody knows who owns what.

## Requirements

- [x] Every feature lives in its own folder under a `features/` folder
- [x] A feature folder has a `README.md` with an owner
- [x] Nested features are allowed (`libs/features/feature-1/features/...`)

## Out of scope

- Moving shared UI components (`components/`) into features
