---
status: implemented
date: 2026-01-05
decisions: [002-use-jwt]
---

# Login flow

## Context

Users sign in with an email and a password.

## Requirements

- [x] Sign in with email and password
- [x] Lock the account after 5 failed attempts

## Plan

1. Build the form
2. Issue a token
