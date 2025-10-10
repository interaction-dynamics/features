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

By following specific guidelines and conventions, it becomes easier to add toolings around this approach and automatize the use cases above.

### Folder structure

The minimum folder structure is as follows:

```bash
examples/rust-basic/src/
├── features
│   ├── feature-1
│   │   ├── file.rs # rust or any other language source code file
│   │   └── README.md
│   └── feature-2
│      └── file.rs
└── ... # other source code
```

> You can find more sophisticated examples in the [examples](./examples) folder.

### Details

| Requirement | Description | Example |
|-------------|-------------|---------|
| **MUST** | Create a folder `features` | `features/` |
| **MUST** | Create your feature folder like `feature-1` | `features/feature-1/` |
| **MUST** | Add the code related to this feature inside the feature folder | `features/feature-1/file.js` |
| **SHOULD** | Add a README.md file into the feature folder | `features/feature-1/README.md` |
| **SHOULD** | Add a line of text describing the feature after the title in the README.md file | `Handles user login and logout functionality` |
| **SHOULD** | Add a [front matter](https://dev.to/dailydevtips1/what-exactly-is-frontmatter-123g) into the README.md with a property `owner` | <pre>---<br/>owner: backend-team<br/>---</pre> |
| **MAY** | Add [MADR files](https://adr.github.io/madr/) into the feature's decisions folder | `features/feature-1/__docs__/decisions/001-use-jwt-tokens.md` |

## Tools

- [Rust CLI](./tools/cli): to parse the code and find the features
- [Web UI](./tools/web): to visualize the features in a dashboard

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## Contributing

We welcome contributions to this project! If you have any ideas or suggestions, please feel free to open an issue or submit a pull request.
