---
status: accepted
date: 2023-09-04
spec: 001-feature-folders
---

# Use a feature-based architecture

Project-level decisions live in `docs/decisions` (or `docs/adr`). Use them for choices that are not owned by one feature.

## Context and Problem Statement

How do we group the code so that every part of the product has a clear owner?

## Considered Options

* Group by type (`components/`, `hooks/`, `services/`)
* Group by feature (`features/<name>/`)

## Decision Outcome

Chosen option: "Group by feature", because a feature folder can carry its owner, its docs and its tests, and can be moved or deleted as one unit.
