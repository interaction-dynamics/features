---
title: Publish the dashboard
description: Serve the features-cli web dashboard locally, build a static version for any host, publish it to GitHub Pages, name it and remove commit emails before sharing it.
---

The web dashboard shows every feature with its owner, README, history, coverage, specs and decisions, dependencies and imported data. Try it on the [live demo](https://interaction-dynamics.io/features/).

## Serve it locally

```bash
features serve ./src
```

Opens the dashboard on `http://localhost:3000` and rescans when files change. `--port` changes the port.

The server only answers requests addressed to `localhost`, `127.0.0.1` or `[::1]`, and no other origin can read its data. To let a tool running on another origin read it, allow that origin, or `*` for any:

```bash
features serve ./src --allow-origin http://localhost:5173
```

## Build a static version

```bash
features build ./src --build-dir ./dashboard --strip-emails
```

Writes `index.html`, `features.json` and `records.json` to `dashboard` (default: `build`). Deploy the folder to any static host. `--strip-emails` removes the commit author emails and keeps the names: use it before publishing outside the team.

For the history and change metrics, build from a full clone of the repository (`fetch-depth: 0` on GitHub Actions). `--skip-changes` leaves the history out, which is faster on large repositories.

## Publish to GitHub Pages

```yaml
# .github/workflows/features.yml
name: Features dashboard
on:
  push:
    branches: [main]
permissions:
  contents: read
  pages: write
  id-token: write
jobs:
  deploy:
    runs-on: ubuntu-latest
    environment:
      name: github-pages
    steps:
      - uses: actions/checkout@v5
        with:
          fetch-depth: 0 # full git history for feature metrics
      - run: npx features-cli@latest build ./src --build-dir ./dashboard --strip-emails
      - uses: actions/upload-pages-artifact@v5
        with:
          path: ./dashboard
      - uses: actions/deploy-pages@v5
```

## Name the dashboard

The dashboard shows the project name at the top left. By default it comes from the manifest of the project (`package.json`, `Cargo.toml`, `pyproject.toml`, `go.mod`, `composer.json`, `pom.xml`, `settings.gradle(.kts)`, `pubspec.yaml` or `deno.json`), looked up in the scanned folder, then in its parents up to the root of the git repository. Without one, it shows "Features".

To choose another name, set `name` in [`features.config.yaml`](/docs/configuration):

```yaml
name: Acme Store
```

or pass `--project-name` to `serve` or `build`, which wins over the file:

```bash
features build ./src --project-name "Acme Store"
```

## Experimental pages

```bash
features serve ./src --experimental
```

Shows the parts of the dashboard that are still experimental: the History page, owner pages linked from owner names, and project records in _Outside features_. They are hidden by default and marked as experimental when shown.

## Keep the settings in the repository

The options of `serve` and `build` can live in the [configuration file](/docs/configuration), so everyone gets the same dashboard:

```yaml
# features.config.yaml
name: Acme Store
path: src
strip-emails: true

serve:
  port: 4000
  experimental: true

build:
  build-dir: dashboard
  generate-codeowners: true
```

To combine the dashboards of several repositories, see [Aggregate repositories](/docs/aggregate).
