---
status: accepted
date: 2025-01-15
---

# Organize the code by feature

## Context and Problem Statement

Code grouped by technical layer (components, hooks, services) spreads one feature over many folders.

## Decision Outcome

Chosen option: one folder per feature, with its README, specs and decisions inside. Project-wide decisions like this one live in `docs/decisions/` at the root and show on the **Project records** page.
