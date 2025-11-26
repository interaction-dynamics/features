# Guidelines

| Requirement | Description | Example |
|-------------|-------------|---------|
| **MUST** | Either: Create features in a `features` folder OR add `feature: true` to README frontmatter | `features/feature-1/` or any folder with feature flag |
| **MUST** | Add the code related to this feature inside the feature folder | `feature-1/file.js` |
| **SHOULD** | Add a README.md or README.mdx file into the feature folder | `features/feature-1/README.md` |
| **SHOULD** | Add a line of text describing the feature after the title in the README.md file | `Handles user login and logout functionality` |
| **SHOULD** | Add a [front matter](https://dev.to/dailydevtips1/what-exactly-is-frontmatter-123g) into the README.md with a property `owner` | <pre>---<br/>owner: backend-team<br/>---</pre> |
| **MAY** | Add all the properties you want in the [front matter](https://dev.to/dailydevtips1/what-exactly-is-frontmatter-123g) | <pre>---<br/>figma: <url><br/>---</pre> |
| **MAY** | Add [MADR files](https://adr.github.io/madr/) into the feature's decisions folder | `features/feature-1/__docs__/decisions/001-use-jwt-tokens.md` |
