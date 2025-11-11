<div align="center">
    <img width="100" src="./tools/web/public/feature-icon.svg" alt="feature logo" />
</div>
<h1 align="center">Feature-based architecture</h1>

<div align="center">
    <a href="https://crates.io/crates/features-cli"><img src="https://img.shields.io/crates/v/features-cli.svg" alt="Crates.io version" /></a>
    <a href="https://www.npmjs.com/package/features-cli"><img src="https://img.shields.io/npm/v/features-cli.svg" alt="Npm version" /></a>
</div>

Feature-based architecture organizes the codebase by features rather than technical layers (like components, services, or utils). It is similar to domain-driven development (DDD) but more recursive and a more structured approach. Each feature lives in its own self-contained folder with everything it needs — making the code more modular, discoverable, and maintainable as the project grows.

This approach is not meant to replace other architecture design like hexagonal architecture but rather complement it. It is an approach easier to understand for beginners. It can also be shared with product managers, designers and stakeholders.

This repository includes guidelines, examples and tools to boost your productivity with feature-based architecture. Explore a live [demo](http://interaction-dynamics.io/features/) of the dashboard built with these tools.

> This approach is technology agnostic and can be used with any programming language or framework.

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

Features can be organized in two ways:

#### Method 1: Features folder (recommended)
Place features in a dedicated `features` folder:

```bash
src/
├── features
│   ├── saveUser
│   │   ├── components/
│   │   │   ├── UserForm.ts
│   │   │   └── EmailInput.ts
│   │   ├── utils/
│   │   │   ├── useSaveUser.ts
│   │   │   ├── useFetchUser.ts
│   │   │   └── transformUser.ts
│   │   └── README.md # a README.md file for large features helps
│   └── feature-2
│       └── ... # ...
├── components # common components used across features
├── utils # common utils/hooks used across features
└── ... # other source code
```

> You can read the detailed [guidelines](./docs/guidelines.md) or just jump into trying the CLI with the `--serve` options. The UI will guide you through the methodology.

#### Method 2: Feature flag
Mark any folder as a feature by adding `feature: true` in its README frontmatter:

```yaml
---
feature: true
owner: backend-team
---
```

This allows features to be organized anywhere in your codebase while still being discoverable by the tools.

> You can find more sophisticated examples in the [examples](./examples) folder.

## Tools

- [Rust CLI](./tools/cli): Command-line tool to parse the code, find the features and serve the web dashboard UI
- [Web UI](./tools/web): Dashboard to visualize the features in your project. Try the demo [here](http://interaction-dynamics.io/features/). A watch mode version and a static build version are integrated into the CLI.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## Contributing

We welcome contributions to this project! If you have any ideas or suggestions, please feel free to open an issue or submit a pull request.

## Roadmap

Check [here](https://github.com/orgs/interaction-dynamics/projects/18/views/4).

## Bibliography about features-based architecture

> Feature based architecture shouldn't be confused with feature-based design.

- [Scalable React Projects with Feature-Based Architecture](https://dev.to/naserrasouli/scalable-react-projects-with-feature-based-architecture-117c)
- [Reddit discussion about feature-based architecture](https://www.reddit.com/r/reactjs/comments/1afywy4/codebase_examples_of_featuredriven_or_vertical/)
- [Why I Recommend a Feature-Driven Approach to Software Design](https://khalilstemmler.com/articles/software-design-architecture/feature-driven/): global vision of feature oriented development
- [Bullet-proof react folder architecture](https://github.com/alan2207/bulletproof-react/blob/master/docs/project-structure.md)
