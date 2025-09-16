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

## Data structures and core concepts

Fontspector is a font testing framework written in Rust. It has a number of core concepts:

- A _check_ is a single test that can be run on a font. It is implemented as a `Check` structure,
  which brings together the check implementation (a function) and metadata about the check.
  The `#[check]` attribute macro is used to define checks. We use the term "check" rather than
  "test" to avoid confusion with the unit tests; we check a font, and we test the checks. The `Check` structure is defined in `fontspector-checkapi/src/check.rs`.
- Each check has a check ID. The mapping between check IDs and their implementation files can be found in `checks.txt`.
- A _profile_ is a collection of checks that can be run together. Individual font manufacturers
  define which checks they would like to run on their fonts by creating a profile.
  Profiles are implemented as Rust crates that depend on the `fontspector-checkapi` crate. The
  check implementations have a "home" within a specific profile, but can be reused in other
  profiles. The `Profile` structure is defined in `fontspector-checkapi/src/profile.rs`.
- A check returns a `CheckResult`. Fontspector will run multiple checks on multiple fonts, and  
  each check may discover one or more problems with the font. `CheckResult` is a structure which wraps up everything that you need to report the result of a check: which file it was run on,
  which check was run, metadata to be displayed about the check (such as the reasoning behind the check and a user-friendly title), together with all of the results of the check (which may be zero or more problems found). The `CheckResult` structure is defined in `fontspector-checkapi/src/checkresult.rs`.
- A `Status` is a single reported problem found by a check. A check may return zero or more
  `Status` items, each of which has a _severity_ (error, warning, info, or pass), optionally a _message_ (to be reported to the user) and a _code_ (a short string that identifies the specific problem found, for example for unit tests or for advanced users who know what to expect).
- The severity mentioned above is represented by the `StatusCode` enum. We should probably have
  called it `Severity`, and maybe one day we will. `Status` and `StatusCode` are defined in `fontspector-checkapi/src/status.rs`.
- We don't simply check fonts. Fontspector can check HTML files, metadata files, and so on. But obviously only some checks apply to certain file types. Checks declare which `FileType` they apply to.
- Files are wrapped up in `Testable` structs, which include their contents and file name. To determine if the file can be converted into a `FileType` to be run by a particular check, we call `.from_testable` on the `FileType` enum. If it returns `Some`, we can run the check; if it returns `None`, we skip the check for that file. The `FileTypeConvert` trait also tells us what kind of representation the file can be converted into. For example, `Testable`s which are TTF files can be converted into `TestFont`s.
- `TestFont` in `fontspector-checkapi/src/font.rs` contains a number of helper methods which tests can use to manipulate the font and extract data from it.
- Each check runs in a `Context`. (`fontspector-checkapi/src/context.rs`) The context contains a general-purpose cache that checks can use to avoid recomputing things, user-defined per-check configuration, some free-form metadata, and `Override`s which change the return values of a check.


## Running the test suite

We export the Fontspector check runner to a Python module, and then use
`pytest` to run (a modified version of) the fontbakery test suite. To
do this:

```
pip3 install -U maturin
cd fontspector-py
python3 -m venv venv ; . venv/bin/activate
pip install maturin
maturin develop
pytest
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
