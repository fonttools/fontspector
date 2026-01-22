# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Fontspector is a Rust-based command-line tool for quality control of OpenType fonts, a port of Python's fontbakery. It uses a plugin architecture where checks are organized into profiles that can be built-in, loaded at runtime, or defined via TOML.

## Build Commands

```bash
cargo build --all                    # Build entire workspace
cargo fmt --all                      # Format code
cargo clippy --all -- -D warnings    # Lint check (required before commits)
cargo test --all                     # Run Rust tests
```

### Python Test Suite (Primary Tests)

The main test suite uses pytest via the Python module:

```bash
cd fontspector-py
python3 -m venv venv && . venv/bin/activate
pip install maturin pytest
maturin develop
pytest                               # Run all tests
pytest tests/test_mandatory_glyphs.py  # Run single test file
pytest -k "test_name"                # Run tests matching pattern
```

### External Plugins

```bash
npm install
npm run build-microsoft              # Build Microsoft profile plugin
npm run build-adobe                  # Build Adobe profile plugin
```

### Web Version (WASM)

```bash
cd fontspector-web
wasm-pack build
cd www && npm install && npm run build  # Output in ../docs/
```

## Architecture

### Workspace Crates

- **fontspector-checkapi**: Core API defining `Check`, `Profile`, `Registry`, `Context`, `Status`, `Testable`, `TestFont`
- **fontspector-checkhelper**: Procedural macro crate providing the `#[check(...)]` attribute
- **fontspector-cli**: Main binary that orchestrates check execution with parallel processing (rayon)
- **profile-{opentype,universal,googlefonts,iso15008,fontwerk}**: Built-in profile crates
- **profile-{microsoft,adobe,designspace}**: External plugins (loaded at runtime)
- **fontbakery-bridge**: Python fontbakery integration
- **fontspector-py**: Python module via PyO3/maturin
- **fontspector-web**: WASM version

### Core Concepts

- **Check**: A test with metadata (id, title, rationale) and implementation function
- **Profile**: Collection of checks organized in sections; can be Rust crate or TOML file
- **Testable**: Wrapper for files (fonts, metadata, etc.) to be tested
- **TestFont**: Parsed font with helper methods for check implementations
- **Context**: Runtime context with cache, configuration, and overrides
- **Status**: Individual result (PASS/WARN/INFO/FAIL/SKIP) with code and message

### Writing a Check

```rust
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, DEFAULT_LOCATION};

#[check(
    id = "profile/category/check_name",
    title = "Human readable title",
    rationale = "Why this check exists",
    proposal = "https://github.com/..."
)]
fn check_name(f: &Testable, context: &Context) -> CheckFnResult {
    let font = testfont!(f);  // Extract TestFont or return error
    let mut problems = vec![];

    // Check logic...
    if something_wrong {
        problems.push(Status::fail("code", "message"));
    }

    return_result(problems)
}
```

Key macros:
- `testfont!(f)`: Extract TestFont from Testable
- `skip!(code, message)` or `skip!(condition, code, message)`: Return skip status
- `return_result(problems)`: Convert Vec<Status> to CheckFnResult

## Code Quality Requirements

- **Clippy rules**: `unwrap_used`, `expect_used`, `indexing_slicing` are denied
- **Documentation**: `#![deny(missing_docs)]` is enforced
- **Formatting**: Must pass `cargo fmt --all`
- **Conventional Commits**: Required for all commits (use `cog commit` helper)

## Commit Convention

Use Conventional Commits format. PR titles must also follow this format (squash-and-merge):

```
<type>(<scope>): <short description>

Types: feat, fix, docs, style, refactor, perf, test, chore
Example: feat(check-api): Add support for variable font axis checks
```

Do NOT manually bump versions or edit CHANGELOG.md - automated via `cargo-smart-release`.
