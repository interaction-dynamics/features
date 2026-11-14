# Frequently Asked Questions

## Table of Contents

- [What is a feature-based architecture?](#what-is-a-feature-based-architecture)
- [Is feature-based architecture the same as Feature-Sliced Design?](#is-feature-based-architecture-the-same-as-feature-sliced-design)
- [How can I add a feature?](#how-can-i-add-a-feature)
- [How can I add a feature description?](#how-can-i-add-a-feature-description)
- [How can I add an owner?](#how-can-i-add-an-owner)
- [How can I add a spec or a decision?](#how-can-i-add-a-spec-or-a-decision)
- [Are GitHub Spec Kit and Kiro specs supported?](#are-github-spec-kit-and-kiro-specs-supported)
- [How can I add a test coverage report?](#how-can-i-add-a-test-coverage-report)
- [What is the technical debt of a feature?](#what-is-the-technical-debt-of-a-feature)
- [How can I generate codeowners file?](#how-can-i-generate-codeowners-file)
- [I'm getting "GLIBC_2.38 not found" error on Linux, how do I fix it?](#im-getting-glibc_238-not-found-error-on-linux-how-do-i-fix-it)

## What is a feature-based architecture?

Feature-based architecture is a software development approach that organizes code and folders around features or business capabilities rather than technical layers (like components, services, or utils). It is similar to domain-driven development (DDD) but more recursive and more structured. Each feature lives in its own self-contained folder with everything it needs. It promotes modularity, maintainability, and scalability by breaking down complex systems into smaller, more manageable components.

This approach is not meant to replace other architecture design like hexagonal architecture but rather complement it. It is an approach easier to understand for beginners. It can also be shared with product managers, designers and stakeholders.

## Is feature-based architecture the same as Feature-Sliced Design?

No. [Feature-Sliced Design](https://feature-sliced.design/) (FSD) is a stricter methodology built on the same idea. It sorts code into fixed layers (`app`, `pages`, `widgets`, `features`, `entities`, `shared`), splits each layer into slices made of fixed segments (`ui`, `model`, `api`, `lib`), and only lets a layer import from the layers below it.

Feature-based architecture keeps only the core idea: each business feature gets its own folder. Features can be nested inside each other, and the layout inside a feature and the import rules are up to you.

Features works with an FSD codebase too: slices in a `features` folder are detected automatically, and you can add `feature: true` to the README front matter of slices in other layers (`entities`, `widgets`, `pages`) to show them as features.

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

<a id="how-can-i-add-a-decision"></a>

## How can I add a spec or a decision?

Specs say **what** to build (requirements, plan, tasks). Decisions say **why** it is built that way. Both appear side by side, sorted by date, in the feature's **Specs & Decisions** tab of the web interface and in the `records` field of the JSON report.

The quickest way is to let the CLI pick the next number:

```bash
features new spec auth "Login flow"         # features/auth/__docs__/specs/001-login-flow.md
features new decision auth "Use JWT tokens" # features/auth/__docs__/decisions/002-use-jwt-tokens.md
features new decision . "Use a monorepo"    # docs/decisions/001-use-a-monorepo.md (project level)
```

Or create the files yourself:

1. In the feature's directory, create a `__docs__/specs/` or `__docs__/decisions/` folder (`.docs/specs/` and `.docs/decisions/` also work, and every folder is read)
2. Add a Markdown file numbered from `001`, e.g. `__docs__/decisions/002-use-jwt.md`
3. Start it with a frontmatter:

   ```markdown
   ---
   status: accepted # draft, proposed, accepted, implemented, rejected, deprecated, superseded
   date: 2026-01-15
   spec: 001-login-flow # on a decision: the spec it belongs to
   supersedes: 001-use-sessions # the record it replaces, if any
   ---

   # Use JWT
   ```

   On a spec, list its decisions with `decisions: [002-use-jwt]`. Links work in both directions and can use the number alone (`decisions: ["002"]`).
4. Structure a decision with the [MADR](https://adr.github.io/madr/) sections: **Context**, **Considered options**, **Decision**, **Consequences**. Classic MADR files without frontmatter work too: the status is read from a `**Status**: Accepted` line or a `## Status` section.
5. Structure a spec with **Context**, **Requirements**, **Out of scope** and **Plan**. `- [ ]` / `- [x]` checklists show as a progress bar.

A large spec can be a folder: `__docs__/specs/003-sso/` with `spec.md` as its main file and other files such as `plan.md` or `tasks.md` next to it.

Records that concern the whole project go in `docs/specs/`, `docs/decisions/` or `docs/adr/` at the root of the scanned folder. They show in the *Not attached to a feature* section of the **Outside features** page of the web interface, and `features records --json` prints them (`{ "records": [...] }`, also served as `records.json`). A root record with `feature: <name or path>` in its frontmatter is shown in that feature instead.

`features check` warns about links to records that don't exist, records without a status, superseded records that nothing replaces, and implemented specs with open tasks.

## Are GitHub Spec Kit and Kiro specs supported?

Yes. At the root of the scanned folder, Features also reads the layouts of two spec-driven tools:

- [GitHub Spec Kit](https://github.com/github/spec-kit): `specs/NNN-name/` folders with a `spec.md`. `plan.md`, `research.md`, `data-model.md`, `quickstart.md`, `tasks.md` and `contracts/*` are shown with it. The status comes from the `**Status**:` line and the date from the `**Created**:` line. Loose `.md` files in `specs/` are read as ordinary specs.
- [Kiro](https://kiro.dev/docs/specs/): `.kiro/specs/<name>/` folders, with `requirements.md` as the main file and `design.md` and `tasks.md` next to it. The title is the folder name in words.

Their task progress comes from `tasks.md`, and they get a **Spec Kit** or **Kiro** badge (`"source": "speckit"` or `"kiro"` in the JSON report). Each one is attached to a feature by a `feature:` frontmatter key, else by its folder name without the `NNN-` prefix matching a feature name or folder (`specs/003-checkout/` goes to the `checkout` feature). The others are project-level records. `features check` doesn't ask them for a status.

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
