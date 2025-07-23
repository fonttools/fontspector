# Contributing to Fontspector

First off, thank you for considering contributing to Fontspector! We're excited you're here. Every contribution, from a small typo fix to a new feature, is valuable.

This document provides guidelines to help you through the contribution process.

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior.

## Getting Started

Fontspector is a Rust workspace containing multiple crates. To get started:

1.  Fork the repository on GitHub.
2.  Clone your fork locally:
    ```sh
    git clone https://github.com/YOUR-USERNAME/fontspector.git
    cd fontspector
    ```
3.  Build the project to ensure everything is set up correctly:
    ```sh
    cargo build --all
    ```

## How to Contribute

We welcome many types of contributions, including:

*   New checks and features
*   Bug fixes
*   Documentation improvements
*   Performance enhancements

### Submitting a Pull Request

1.  **Create a branch** for your changes:
    ```sh
    git checkout -b feat/my-awesome-feature
    ```
2.  **Make your changes.**
3.  **Ensure Code Quality:** Before committing, please run the standard Rust formatting and linting tools across the entire workspace.
    ```sh
    # Format your code
    cargo fmt --all

    # Run clippy to catch common mistakes and style issues
    cargo clippy --all -- -D warnings
    ```
4.  **Run Tests:** Make sure all existing tests pass and, if you're adding a new feature, please add tests for it.
    ```sh
    cargo test --all
    ```
5.  **Commit Your Changes with Conventional Commits:** We use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) to automate changelogs and versioning. Your commit messages MUST follow this specification.

    The commit message should be structured as follows:
    ```
    <type>(<scope>): <short description>
    <BLANK LINE>
    <optional body>
    <BLANK LINE>
    <optional footer>
    ```

    **Common types:**
    *   `feat`: A new feature.
    *   `fix`: A bug fix.
    *   `docs`: Documentation only changes.
    *   `style`: Changes that do not affect the meaning of the code (white-space, formatting, etc).
    *   `refactor`: A code change that neither fixes a bug nor adds a feature.
    *   `perf`: A code change that improves performance.
    *   `test`: Adding missing tests or correcting existing tests.
    *   `chore`: Changes to the build process or auxiliary tools.

    **Example:**
    ```
    feat(check-api): Add support for variable font axis checks
    ```

    **Using `cog` for commits:**
    To simplify creating conventional commits, we strongly encourage using `cog` (Cocogitto), which is configured for this project in `cog.toml`. Instead of `git commit`, you can simply run:
    ```sh
    cog commit
    ```
    This will guide you through creating a compliant commit message.

6.  **Push to your fork** and **open a Pull Request** against the `main` branch.

7.  **PR Title:** Because we squash and merge pull requests, the **title of your PR must also be a valid Conventional Commit message**. The title will become the commit message in the `main` branch.

### Automated Changelogs and Versioning

Please **do not** bump version numbers in `Cargo.toml` files or manually edit `CHANGELOG.md` files. This is handled automatically.

Our release process is automated using `cargo-smart-release` in a GitHub Action. When a release is triggered, the tool analyzes all Conventional Commit messages since the last tag. It then determines the correct semantic version bump (patch, minor, or major) for each affected crate and generates the corresponding changelog entries.

This is why your commit messages and PR titles are so important—they directly control the release process!

Thank you for contributing!
