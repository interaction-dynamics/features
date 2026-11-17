---
status: implemented
date: 2026-11-17
---

# CLI install with a shell script

## Context

The CLI page offered npx, npm, Homebrew and cargo. Each needs a toolchain or package manager (Node.js, Homebrew, Rust). A one-line `curl … | sh` install gets the prebuilt binary on any macOS or Linux machine with nothing else installed, the usual way to install a single-binary CLI.

## Requirements

- The website serves a POSIX `sh` script at `/install.sh`, as `text/plain` so it can be read in the browser before running it.
- `curl -fsSL <site>/install.sh | sh`:
  - detects the platform: macOS x64 / arm64 (arm64 under Rosetta too), Linux x64 / arm64 with the statically linked musl build so it runs whatever the glibc version. Windows and other systems fail with a message pointing to npm.
  - downloads `features-<platform>` and `features-<platform>.sha256` from the latest GitHub release, or from `v$FEATURES_VERSION` when set (a leading `v` is accepted).
  - refuses to install when the SHA-256 does not match.
  - installs the binary as `features` in `$FEATURES_INSTALL_DIR`, default `~/.local/bin`, without `sudo`, and prints how to add the folder to `PATH` when it is missing.
  - works with `curl` or `wget`, and `sha256sum` or `shasum`.
  - runs nothing until the whole script is downloaded (everything in a `main` called on the last line).
- The install picker on `/cli` gets a "Shell script" option with the command built from the site URL, and the plain-text version (`/cli.md`, `llms.txt`) lists it too.

## Out of scope

- Windows (PowerShell) install script.
- Updating or uninstalling through the script (re-running it updates).
- Changing the README, FAQ or GitHub release notes, whose wording needs the production domain of the website.

## Plan

1. Add `projects/website/public/install.sh`.
2. Serve it as `text/plain` in `projects/website/vercel.json`.
3. Add the "Shell script" option to the install picker in `projects/website/src/pages/cli.astro`, and mention it on the landing page and in `llms.txt`.
4. Document it in `projects/website/README.md` and add an unreleased changelog entry.
