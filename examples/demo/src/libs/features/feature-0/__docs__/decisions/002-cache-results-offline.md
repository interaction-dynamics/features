---
status: proposed
date: 2025-05-06
spec: 002-offline-mode
---

# Cache results in IndexedDB for offline mode

## Context and Problem Statement

Offline mode needs the latest results on the device. Local storage is limited to a few megabytes.

## Considered Options

* IndexedDB
* Local storage
* Service worker cache only

## Decision Outcome

Chosen option: "IndexedDB", because it stores structured data without a size limit that matters here.
