# Feature-based architecture

The feature-based architecture consists into grouping all the code related to a specific business requirement or user story into a dedicated folder.

This approach is not meant to replace other architecture design like hexagonal architecture but rather complement it. It is an approach easier to understand and maintain for beginners. It is also easier to share with product managers and stakeholders.

This approach is technology agnostic and can be used with any programming language or framework.

> This approach is not innovative but lacks documentation online so this repository is a good starting point.

## Guidelines

By following specific guidelines and conventions, it becomes easier to add toolings around this approach.

1. **[MUST]**    create a folder `features`
2. **[MUST]**    create your feature folder like `feature-1`
3. **[MUST]**    add the code related to this feature inside `features/feature-1`
4. **[SHOULD]**  add a README.md file into `features/feature-1`
5. **[SHOULD]**  add a [front matter](https://dev.to/dailydevtips1/what-exactly-is-frontmatter-123g) into the README.md with a property `owner` and the name of the team who has ownership of this feature
6. **[MAY]**     add [MADR files](https://adr.github.io/madr/) into `features/feature-1/__docs__/decisions`

> You can find examples in the [examples](./examples) folder.

## CLI

In progress

## Dashboard

In progress
