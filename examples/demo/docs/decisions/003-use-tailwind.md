---
status: accepted
date: 2024-02-20
spec: 002-shared-design-tokens
supersedes: [002-use-css-modules]
---

# Use Tailwind CSS with shared design tokens

## Context and Problem Statement

CSS Modules kept styles scoped, but every module redefined its own colors and spacing. How do we share design tokens across features?

## Considered Options

* CSS Modules with a shared variables file
* Tailwind CSS with a shared theme

## Decision Outcome

Chosen option: "Tailwind CSS", because the theme is the single source for tokens and utility classes make drift easy to spot in review.
