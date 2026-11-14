# Frequently Asked Questions

## Table of Contents

- [What is a feature-based architecture?](#what-is-a-feature-based-architecture)
- [How can I add a feature?](#how-can-i-add-a-feature)
- [How can I add a feature description?](#how-can-i-add-a-feature-description)
- [How can I add an owner?](#how-can-i-add-an-owner)
- [How can I add a decision?](#how-can-i-add-a-decision)
- [How can I add a spec?](#how-can-i-add-a-spec)
- [How do I link specs and decisions?](#how-do-i-link-specs-and-decisions)
- [Are GitHub Spec Kit and Kiro specs supported?](#are-github-spec-kit-and-kiro-specs-supported)
- [Where do project-level specs and decisions go?](#where-do-project-level-specs-and-decisions-go)
- [How can I add a test coverage report?](#how-can-i-add-a-test-coverage-report)
- [What is the technical debt of a feature?](#what-is-the-technical-debt-of-a-feature)
- [How can I generate codeowners file?](#how-can-i-generate-codeowners-file)
- [I'm getting "GLIBC_2.38 not found" error on Linux, how do I fix it?](#im-getting-glibc_238-not-found-error-on-linux-how-do-i-fix-it)

## What is a feature-based architecture?

Feature-based architecture is a software development approach that organizes code and folders around features or business capabilities rather than technical layers (like components, services, or utils). It is similar to domain-driven development (DDD) but more recursive and more structured. Each feature lives in its own self-contained folder with everything it needs. It promotes modularity, maintainability, and scalability by breaking down complex systems into smaller, more manageable components.

This approach is not meant to replace other architecture design like hexagonal architecture but rather complement it. It is an approach easier to understand for beginners. It can also be shared with product managers, designers and stakeholders.

## How can I add a feature?

To add a new feature to the system, follow these steps:

1. Create a folder `features` where you want in your source code directory. It can be in a subdirectory.
2. Inside the `features` directory, create a folder with the name of the first business feature like `business-feature-name`. It can use snake_case, camelCase, or kebab-case naming conventions.
3. The feature will automatically be detected and displayed in the web interface

## How can I add a feature description?

To add a new feature description to the system, follow these steps:

1. In the feature folder, create a new README.md file
2. In this README.md file, provide a detailed description of the feature, including its purpose, functionality, and any relevant technical details.

## How can I add an owner?

1. In the feature README, add a property `owner` in the [frontmatter]((https://dev.to/dailydevtips1/what-exactly-is-frontmatter-123g)) of the README.md file.

To add a new feature description to the system, follow these steps:

1. In the feature folder, create a new README.md file
2. In this README.md file, provide a detailed description of the feature, including its purpose, functionality, and any relevant technical details.

## How can I add a decision?

Do you see the message "No specs or decisions yet"? You can add a decision by following these steps:

1. Navigate to the feature you want to add a decision to
2. In the feature's directory, locate or create a `__docs__/decisions/` folder or `.docs/decisions/` folder
3. Create a new Markdown file with a numbered, descriptive name (e.g., `001-use-typescript.md`)
4. Optionally start it with a frontmatter: `status` (proposed, accepted, rejected, deprecated, superseded), `date` and links to other records (see [How do I link specs and decisions?](#how-do-i-link-specs-and-decisions))
5. Structure your decision document with the following sections:
   - **Title**: Clear, concise description of the decision
   - **Status**: Current status (proposed, accepted, rejected, deprecated, superseded)
   - **Context**: Background information and the problem being solved
   - **Decision**: The actual decision made
   - **Consequences**: Expected positive and negative outcomes

```markdown
---
status: accepted
date: 2026-01-04
spec: 001-login-flow
supersedes: 001-session-cookies
---

# Use JWT tokens

## Context and Problem Statement
...
```

Without a frontmatter, the status is read from a `**Status**: Accepted` line or from the first line of a `## Status` section, as in classic MADR and Nygard ADRs.

The decision will automatically appear in the web interface under the feature's **Specs & Decisions** tab, and in the `records` of the feature in the JSON report.

This decision template is inspired by the [MADR](https://adr.github.io/madr/) initiative.

## How can I add a spec?

A spec describes what to build; a decision explains why it is built that way. Specs live next to decisions:

1. In the feature's directory, create a `__docs__/specs/` folder (or `.docs/specs/`)
2. Add a numbered Markdown file, e.g. `001-login-flow.md`, starting with a frontmatter:

```markdown
---
status: draft # draft, accepted, implemented, superseded
date: 2026-01-02
decisions: [002-use-jwt]
---

# Login flow

## Context

## Requirements

- [x] Sign in with email and password
- [ ] Lock the account after 5 failed attempts

## Out of scope

## Plan
```

A spec can also be a folder, e.g. `__docs__/specs/003-sso/`, with `spec.md` as its main file and other files next to it (`plan.md`, `tasks.md`, `research.md`…). The dashboard shows each file in its own tab.

The `- [ ]` / `- [x]` checkboxes are counted as tasks (those of `tasks.md` when the spec has one) and shown as a progress bar. `features check` warns about an `implemented` spec that still has open tasks.

## How do I link specs and decisions?

Use these frontmatter keys, with the id of the other record (its file or folder name without `.md`):

| Key | On | Example |
| --- | --- | --- |
| `decisions` | a spec (or a decision, for related decisions) | `decisions: [002-use-jwt]` |
| `spec` / `specs` | a decision | `spec: 001-login-flow` |
| `supersedes` | a record that replaces another one | `supersedes: 001-session-cookies` |

Links only need to be written on one side: features-cli adds the reverse link (a spec listing a decision makes the decision link back to the spec, `supersedes` fills `superseded_by` on the other record). A link resolves to a record of the same feature first, then to a project-level record, then to any record with that id; a bare number like `2` matches `002-use-jwt`.

`features check` warns about links that match no record, native records without a status and `superseded` records that nothing supersedes. Warnings don't fail the check.

## Are GitHub Spec Kit and Kiro specs supported?

Yes. features-cli reads, at the root of the scanned folder:

- [GitHub Spec Kit](https://github.com/github/spec-kit): `specs/NNN-name/` folders with a `spec.md`. `plan.md`, `tasks.md`, `research.md`, `data-model.md`, `quickstart.md` and `contracts/` are shown as tabs of the spec. The status comes from the `**Status**:` line of `spec.md`, the tasks from the checkboxes of `tasks.md`. They show a "Spec Kit" badge.
- [Kiro](https://kiro.dev/docs/specs/): `.kiro/specs/<name>/` folders. `requirements.md` is the main file, `design.md` and `tasks.md` are tabs. They show a "Kiro" badge.

Each of these specs is attached to a feature with this rule:

1. A `feature:` key in the frontmatter of the main file, matching a feature path (or the end of one), name or folder name.
2. Otherwise, the folder name without its `NNN-` prefix, matching a feature name or folder name (case-insensitive, spaces and dashes are equivalent): `specs/003-checkout/` goes to the `checkout` feature, `.kiro/specs/user-profile/` to the "User Profile" feature. When several features match, the first one in the tree (parents first) wins.
3. Otherwise, the spec is project-level.

## Where do project-level specs and decisions go?

Put specs and decisions about the whole project in `docs/specs/`, `docs/decisions/` or `docs/adr/` at the root of the scanned folder. A root record with a `feature:` frontmatter key is shown on that feature instead. The others are shown on the **Specs & Decisions** page of the dashboard's sidebar.

`features.json` stays an array of features, so project-level records are exposed separately: at `/records.json` by `features serve`, in `records.json` by `features build` (`{ "records": [...] }`), and by `features scan --json --project-records`, which prints `{ "features": [...], "records": [...] }`.

## How can I add a test coverage report?

Do you see the message "No test coverage reports available"? 

Generate a test coverage report by running your tests with the `--coverage` flag. For example, if you're using Jest, you can run `npx jest --coverage`. 

Ensure the report is generated in the folder `coverage` or `.coverage`.

The CLI automatically searches for coverage reports in multiple locations:
- `base_path/.coverage` and `base_path/coverage` (path provided as first argument)
- `current_dir/.coverage` and `current_dir/coverage` (path the executable runs)
- `project_dir/.coverage` and `project_dir/coverage` (if `--project-dir` is provided)

If your project uses a different directory structure or naming convention for coverage reports, you can specify the directory using the `--coverage-dir` flag. For example, if your coverage report is located in `reports/coverage`, you can run `features-cli --coverage-dir reports/coverage`.

The supported report formats are [LCOV](https://wiki.documentfoundation.org/Development/Lcov) and [Cobertura](https://cobertura.github.io/cobertura/).

## What is the technical debt of a feature?

Right now the CLI only detects the TODOs in the code. It does not detect other types of technical debt such as code smells or performance issues.

Feel free to suggest new ways to detect technical debt.

## How can I generate codeowners file

Generate or update a CODEOWNERS file based on feature ownership:

```bash
# Generate CODEOWNERS in current directory
features codeowners ./src

# Generate CODEOWNERS in project directory with proper relative paths
features codeowners ./src --project-dir ./

# Can be combined with serve, build and check
features build ./src --generate-codeowners --project-dir ./
```

The generated CODEOWNERS file will have a managed section between markers:
```
# ==== GENERATED BY FEATURES-CLI ====
/src/features/feature-1 @team1
/src/features/feature-2 @team2
# ==== END SECTION GENERATED BY FEATURES-CLI ====
```

Any content outside these markers is preserved when regenerating.

## I'm getting "GLIBC_2.38 not found" error on Linux, how do I fix it?

This error occurs when the Linux binary was built with a newer version of GLIBC than what's available on your system. There are several solutions:

### Option 1: Use the musl build (Recommended)

The musl builds are statically linked and don't depend on GLIBC at all, making them the most compatible option:

```bash
# For x64 (64-bit Intel/AMD) systems
npm install -g @features-cli/feature-cli-linux-x64-musl

# For ARM64 systems
npm install -g @features-cli/feature-cli-linux-arm64-musl

# Note: npx features-cli automatically detects and uses musl on musl-based systems
npx features-cli@latest serve /path/to/project

# Or download the musl binary directly from GitHub releases
# Look for files: features-linux-x64-musl or features-linux-arm64-musl
```

### Option 2: Use cargo/rust to build from source

If you have Rust installed, you can build directly from source which will use your system's GLIBC:

```bash
cargo install features-cli
```

### Option 3: Use npx (which should automatically select the right binary)

```bash
npx features-cli@latest serve /path/to/project
```

### Option 4: Update your system

If possible, update your Linux distribution to a newer version that includes GLIBC 2.38 or higher. This is typically available on:
- Ubuntu 24.04 or later
- Debian 13 (Trixie) or later
- Fedora 39 or later
