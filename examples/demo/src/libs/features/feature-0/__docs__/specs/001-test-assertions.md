---
status: implemented
date: 2023-09-20
decisions: [001-use-technology-1]
---

# Readable test assertions

Specs live next to decisions in `__docs__/specs`. A spec says **what** to build; a decision says **why** it is built that way. Link them with `decisions:` in the frontmatter.

## Context

Feature 0 tests are hard to read when they check several values at once.

## Requirements

- [x] Assertions read as one sentence
- [x] Failures print the expected and actual values
- [x] No new test dependency unless it pays off

## Out of scope

- Snapshot testing
