# Frequently Asked Questions

## Table of Contents

- [What is a feature-based architecture?](#what-is-a-feature-based-architecture)
- [How can I add a feature?](#how-can-i-add-a-feature)
- [How can I add a feature description?](#how-can-i-add-a-feature-description)
- [How can I add an owner?](#how-can-i-add-an-owner)
- [How can I add a decision?](#how-can-i-add-a-decision)
- [How can I add a test coverage report?](#how-can-i-add-a-test-coverage-report)

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

Do you see the message "No decisions available"? You can add one by following these steps:

1. Navigate to the feature you want to add a decision to
2. In the feature's directory, locate or create a `__docs__/decisions/` folder or `.docs/decisions/` folder
3. Create a new Markdown file with a descriptive name (e.g., `use-typescript.md`)
4. Structure your decision document with the following sections:
   - **Title**: Clear, concise description of the decision
   - **Status**: Current status (proposed, accepted, rejected, deprecated, superseded)
   - **Context**: Background information and the problem being solved
   - **Decision**: The actual decision made
   - **Consequences**: Expected positive and negative outcomes

The decision will automatically appear in the web interface under the feature's decisions section.

This decision template is inspired by the [MADR](https://adr.github.io/madr/) initiative.

## How can I add a test coverage report?

Do you see the message "No test coverage reports available"? 

Generate a test coverage report by running your tests with the `--coverage` flag. For example, if you're using Jest, you can run `npx jest --coverage`. Ensure the report is generated in the folder `coverage` or `.coverage` at the root of the folder you will run `features-cli`. The supported report format is [LCOV](https://wiki.documentfoundation.org/Development/Lcov).
