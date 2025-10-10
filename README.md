# Feature-based architecture

Feature-based architecture organizes the codebase by features rather than technical layers (like components, services, or utils). Each feature lives in its own self-contained folder with everything it needs — making the code more modular, discoverable, and maintainable as the project grows.

This approach is not meant to replace other architecture design like hexagonal architecture but rather complement it. It is an approach easier to understand for beginners. It also has the benefits to be shareable with product managers, designers and stakeholders.

This approach is technology agnostic and can be used with any programming language or framework.

> This documentation is not innovative but feature-based architecture lacks documentation online so this repository is a good starting point.

## The benefits

This architecture brings a clear separation of concerns and makes it easier to understand and maintain the codebase.

You can also easier:

- list all the features in a project
- find the team 'owning' a specific feature
- get the history of a feature
- share the list of features with product managers and stakeholders
- find the code related to a feature
- find the documentation related to the feature

> If you see an other use case, feel free to contribute to this repository.

## Guidelines

By following specific guidelines and conventions, it becomes easier to add toolings around this approach and automatize the use cases above:

1. **[MUST]**    create a folder `features`
2. **[MUST]**    create your feature folder like `feature-1`
3. **[MUST]**    add the code related to this feature inside `features/feature-1`
4. **[SHOULD]**  add a README.md file into `features/feature-1`
5. **[SHOULD]**  add a line of text describing the feature after the title in the README.md file
6. **[SHOULD]**  add a [front matter](https://dev.to/dailydevtips1/what-exactly-is-frontmatter-123g) into the README.md with a property `owner` and the name of the team who has ownership of this feature
7. **[MAY]**     add [MADR files](https://adr.github.io/madr/) into `features/feature-1/__docs__/decisions`

> You can find examples in the [examples](./examples) folder.

## Tools

- [Rust CLI](./tools/cli): to parse the code and find the features
- [Web UI](./tools/web): to visualize the features in a dashboard

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## Contributing

We welcome contributions to this project! If you have any ideas or suggestions, please feel free to open an issue or submit a pull request.
