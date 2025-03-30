# Contributing to Brush

First off, thank you for considering contributing to Brush! We welcome contributions from everyone.

This document provides guidelines for contributing to the project.

## How Can I Contribute?

### Reporting Bugs

*   **Ensure the bug was not already reported** by searching on GitHub under [Issues](https://github.com/ArthurBrussee/brush/issues).
*   If you're unable to find an open issue addressing the problem, [open a new one](https://github.com/ArthurBrussee/brush/issues/new). Be sure to include a **title and clear description**, as much relevant information as possible, and a **code sample or an executable test case** demonstrating the expected behavior that is not occurring.

### Suggesting Enhancements

*   Open a new issue, clearly describing the enhancement proposal and the reasoning behind it.
*   Explain why this enhancement would be useful and provide examples if possible.

### Pull Requests

1.  Fork the repository and create your branch from `main`.
2.  If you've added code that should be tested, add tests.
3.  If you've changed APIs, update the documentation (`cargo doc --workspace --no-deps`).
4.  Ensure the test suite passes (`cargo test --workspace`).
5.  Format your code using `cargo fmt`.
6.  Ensure your code adheres to the linter rules (`cargo clippy --workspace --all-targets --all-features -- -D warnings`). The specific lint rules are configured in the root `Cargo.toml` under `[workspace.lints.*]`.
7.  Make sure your commit messages are clear and descriptive.
8.  Open a pull request to the `main` branch.

## Style Guides

### Rust Code

*   Follow the standard [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).
*   Use `cargo fmt` to automatically format your code before committing.
*   Address warnings reported by `cargo clippy` (using the command above). The project aims to be free of clippy warnings.

### Commit Messages

*   Use the present tense ("Add feature" not "Added feature").
*   Use the imperative mood ("Move cursor to..." not "Moves cursor to...").
*   Limit the first line to 72 characters or less.
*   Reference issues and pull requests liberally after the first line.

Thank you for contributing! 