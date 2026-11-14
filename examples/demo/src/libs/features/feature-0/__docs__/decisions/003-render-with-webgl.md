---
status: accepted
date: 2024-02-12
spec: 002-glass-material-editor
supersedes: 002-render-with-canvas
---

# Render the preview with WebGL

## Context and Problem Statement

Canvas 2D can't render refraction at 60 fps on a laptop.

## Considered Options

* Canvas 2D
* WebGL
* CSS backdrop-filter

## Decision Outcome

Chosen option: "WebGL", because it renders blur and refraction in a shader at 60 fps.
