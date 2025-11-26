# README format

In order to have your features detected by `features-cli`, you must put your features folder inside a `features` folder or you can rely on README files.

A feature README file should be placed in the root of each feature folder. All the properties in the front matter are optional and you can of course add your own custom properties.

```markdown
---
feature: true # required if not in a folder `features`
owner: team_1
creation_date: 2022-01-01 # to override the first commit date in this feature
status: active # active, deprecated, archived
deprecated: true # true, false
---

# Feature Name (Override the feature folder name if provided)

```
