---
title: Aggregate repositories
description: Combine the features JSON reports of several repositories into one dashboard with features aggregate, and how features with the same name are merged across repositories.
---

Does a feature live partly in the web app, partly in the API and partly in the mobile app? Export one report per repository and combine them into a single dashboard, where each feature combines the metrics and history of every repository it lives in.

## Export and combine

```bash
features scan ./repo-a/src --json > reports/repo-a.json
features scan ./repo-b/src --json > reports/repo-b.json

features aggregate ./reports                  # static build in ./build (index.html + features.json)
features aggregate ./reports --json           # print the aggregated JSON
```

- A report is either `<repository>.json` or `<repository>/features.json`, such as the output of `features build`. The repository is named after the file or the folder.
- `--build-dir` changes the output folder (default: `build`), and [`--data`](/docs/external-data) adds external data to the aggregated features.
- In [`features.config.yaml`](/docs/configuration), the `aggregate` section sets `build-dir`, `json` and `data`. The `FOLDER` of reports stays on the command line.

In CI, each repository can upload its report as an artifact, and a last job downloads them to one folder and runs `features aggregate`.

## How features are merged

- **Same name, same feature.** Features with the same name are merged. Each aggregated feature lists, in `repositories`, every repository where it was found and its path there.
- **Nesting is kept.** A merged feature takes the parent it has in the repositories where it is nested, and gets the children it has in every repository.
- **Single-repository features keep their place.** Unmatched root features stay at the root.
- **No guessing.** Features are not merged when that would create a loop (`B > C` in one repository and `C > B` in another), or when they have different parents in different repositories. They are kept once per repository, and the conflict is listed in the `errors` of `features.json` and in the _Not merged across repositories_ section of the _Outside features_ page.

Since names are the merge key, run [`features check`](/docs/check-rules#duplicate-names) in each repository: two features with the same name in one repository are an error.

## Output

The aggregated `features.json` is an object:

```json
{
  "repositories": [...],
  "features": [...],
  "errors": [...]
}
```

`features` has the same structure as the output of `features scan --json`, plus `repositories` on each feature.
