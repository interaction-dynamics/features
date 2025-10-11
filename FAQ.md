# Frequently Asked Questions

## Table of Contents

- [How can I add a decision?](#how-can-i-add-a-decision)

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
