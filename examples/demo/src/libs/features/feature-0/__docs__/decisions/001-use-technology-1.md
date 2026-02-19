# Decision 1 (open to get more details)

You can add decision files in `__docs__/decisions` folder.

It is very useful for documenting important decisions made during the development process like [Markdown Architectural Decision Records (MADR)](https://adr.github.io/).

# Context and Problem Statement

How to write readable test assertions?
How to write readable test assertions for advanced tests?

## Considered Options

* Plain JUnit5
* Hamcrest
* AssertJ

## Decision Outcome

Chosen option: "Plain JUnit5", because it is a standard framework and the features of the other frameworks do not outweigh the drawbrack of adding a new dependency.
