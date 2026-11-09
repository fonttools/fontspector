# Configurable `monotype/unnecessary_tables` Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Load unnecessary-table tags and prefixes from TOML/check config, with defaults registered in the Monotype profile (`lib.rs`), validating ASCII/4-byte tags in the check and failing with `invalid-config` on bad config.

**Architecture:** Profile defaults are registered with `with_configuration_defaults("monotype/unnecessary_tables", ...)` so CLI runs merge them via `Context::specialize`. The check reads `context.local_config(...)`, parses/validates five array keys, builds the effective tag list (base + CFF/TTF + variable), and scans table-name prefixes. Unit tests inject the same default map via `run_check_with_config` because they do not go through profile specialization.

**Tech Stack:** Rust, `fontspector-checkapi` (`Context::local_config`, `run_check_with_config`), `serde_json`

**Spec:** `profile-monotype/superpowers/specs/2026-11-09-unnecessary-tables-config-design.md`

---

## File map

| File | Responsibility |
|------|----------------|
| `profile-monotype/src/lib.rs` | Register default JSON arrays for the five config keys |
| `profile-monotype/src/checks/monotype/unnecessary_tables.rs` | Parse/validate config; apply tags/prefixes; rationale; tests + shared default-config helper |

No new files.

---

### Task 1: Failing tests for config validation

**Files:**
- Modify: `profile-monotype/src/checks/monotype/unnecessary_tables.rs` (tests module only)

- [ ] **Step 1: Add test helper and invalid-config tests**

In the `#[cfg(test)] mod tests` block, add imports and helpers (keep existing consts imports for now — Task 3 removes them):

```rust
use std::collections::HashMap;

use fontspector_checkapi::{
    codetesting::{
        add_table, assert_messages_contain, assert_pass, assert_results_contain, remove_table,
        run_check_with_config, test_able,
    },
    StatusCode, TestableType,
};
use serde_json::json;

fn default_unnecessary_tables_config() -> HashMap<String, serde_json::Value> {
    HashMap::from([(
        "monotype/unnecessary_tables".to_string(),
        json!({
            "unnecessary_table_tags": [
                "LINO", "SLST", "FFTM", "PfEd", "TeX ", "BDF ", "bdat", "bloc", "ADFH", "SPRK"
            ],
            "unnecessary_table_tags_cff": ["gasp", "cvt ", "fpgm", "glyf", "loca", "prep"],
            "unnecessary_table_tags_ttf": ["VORG"],
            "unnecessary_table_tags_variable": ["kern"],
            "unnecessary_table_prefixes": ["TSI", "MT", "ASC", "edt", "ABA"],
        }),
    )])
}

#[test]
fn test_invalid_tag_length_fails_config() {
    let testable = test_able("mada/Mada-Regular.ttf");
    let config = HashMap::from([(
        "monotype/unnecessary_tables".to_string(),
        json!({
            "unnecessary_table_tags": ["TOOLONG"],
            "unnecessary_table_tags_cff": [],
            "unnecessary_table_tags_ttf": [],
            "unnecessary_table_tags_variable": [],
            "unnecessary_table_prefixes": [],
        }),
    )]);
    let results = run_check_with_config(
        unnecessary_tables,
        TestableType::Single(&testable),
        config,
    );
    assert_results_contain(
        &results,
        StatusCode::Fail,
        Some("invalid-config".to_string()),
    );
}

#[test]
fn test_non_ascii_tag_fails_config() {
    let testable = test_able("mada/Mada-Regular.ttf");
    let config = HashMap::from([(
        "monotype/unnecessary_tables".to_string(),
        json!({
            "unnecessary_table_tags": ["Täg "],
            "unnecessary_table_tags_cff": [],
            "unnecessary_table_tags_ttf": [],
            "unnecessary_table_tags_variable": [],
            "unnecessary_table_prefixes": [],
        }),
    )]);
    let results = run_check_with_config(
        unnecessary_tables,
        TestableType::Single(&testable),
        config,
    );
    assert_results_contain(
        &results,
        StatusCode::Fail,
        Some("invalid-config".to_string()),
    );
}

#[test]
fn test_missing_config_key_fails() {
    let testable = test_able("mada/Mada-Regular.ttf");
    let config = HashMap::from([(
        "monotype/unnecessary_tables".to_string(),
        json!({
            "unnecessary_table_tags": ["LINO"],
            // missing other required keys
        }),
    )]);
    let results = run_check_with_config(
        unnecessary_tables,
        TestableType::Single(&testable),
        config,
    );
    assert_results_contain(
        &results,
        StatusCode::Fail,
        Some("invalid-config".to_string()),
    );
}
```

Also change the four existing `run_check(...)` pass tests and the two fail-injection tests to use:

```rust
run_check_with_config(
    unnecessary_tables,
    TestableType::Single(&testable), // or &test_font
    default_unnecessary_tables_config(),
)
```

Keep using the consts in the injection tests for the list of tags to add/remove for now.

- [ ] **Step 2: Run the new invalid-config tests — expect FAIL (RED)**

Run:

```bash
cargo test -p fontspector-profile-monotype -- unnecessary_tables::tests::test_invalid_tag_length_fails_config unnecessary_tables::tests::test_non_ascii_tag_fails_config unnecessary_tables::tests::test_missing_config_key_fails -- --nocapture
```

Expected: tests FAIL because the check still ignores config (pass or no `invalid-config`).

- [ ] **Step 3: Commit test scaffolding**

```bash
git add profile-monotype/src/checks/monotype/unnecessary_tables.rs
git commit -m "test(monotype): add failing config validation tests for unnecessary_tables"
```

---

### Task 2: Implement config parse/validate + wire the check

**Files:**
- Modify: `profile-monotype/src/checks/monotype/unnecessary_tables.rs`

- [ ] **Step 1: Replace consts and check body with config-driven implementation**

Remove `UNNECESSARY_TABLE_TAGS`, `UNNECESSARY_TABLE_TAGS_CFF`, `UNNECESSARY_TABLE_TAGS_TTF`.

Add helpers and update the check (full intended shape):

```rust
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::{json, Value};

fn parse_table_tags(config: &Value, key: &str) -> Result<Vec<[u8; 4]>, Status> {
    let Some(arr) = config.get(key).and_then(|v| v.as_array()) else {
        return Err(Status::fail(
            "invalid-config",
            &format!("Missing or non-array config key '{key}' for monotype/unnecessary_tables."),
        ));
    };
    let mut tags = Vec::with_capacity(arr.len());
    for entry in arr {
        let Some(s) = entry.as_str() else {
            return Err(Status::fail(
                "invalid-config",
                &format!("Config key '{key}' must contain only strings."),
            ));
        };
        if s.len() != 4 || !s.is_ascii() {
            return Err(Status::fail(
                "invalid-config",
                &format!(
                    "Invalid table tag '{s}' in '{key}': must be exactly 4 ASCII bytes."
                ),
            ));
        }
        let bytes = s.as_bytes();
        tags.push([bytes[0], bytes[1], bytes[2], bytes[3]]);
    }
    Ok(tags)
}

fn parse_prefixes(config: &Value, key: &str) -> Result<Vec<String>, Status> {
    let Some(arr) = config.get(key).and_then(|v| v.as_array()) else {
        return Err(Status::fail(
            "invalid-config",
            &format!("Missing or non-array config key '{key}' for monotype/unnecessary_tables."),
        ));
    };
    let mut prefixes = Vec::with_capacity(arr.len());
    for entry in arr {
        let Some(s) = entry.as_str() else {
            return Err(Status::fail(
                "invalid-config",
                &format!("Config key '{key}' must contain only strings."),
            ));
        };
        if s.is_empty() || !s.is_ascii() {
            return Err(Status::fail(
                "invalid-config",
                &format!(
                    "Invalid table prefix '{s}' in '{key}': must be non-empty ASCII."
                ),
            ));
        }
        prefixes.push(s.to_string());
    }
    Ok(prefixes)
}

#[check(
    id = "monotype/unnecessary_tables",
    title = "Font contains unnecessary tables?",
    rationale = "
        Unnecessary tables (private tables and embedded bitmap tables).

        This check loads tag and prefix lists from configuration. Profile defaults
        apply when keys are omitted. Example:

        ```
        [monotype/unnecessary_tables]
        unnecessary_table_tags = [ \"LINO\", \"FFTM\" ]
        unnecessary_table_tags_cff = [ \"gasp\", \"cvt \" ]
        unnecessary_table_tags_ttf = [ \"VORG\" ]
        unnecessary_table_tags_variable = [ \"kern\" ]
        unnecessary_table_prefixes = [ \"TSI\", \"MT\", \"ASC\", \"edt\", \"ABA\" ]
        ```
    ",
    proposal = "https://github.com/Monotype/fontspector/issues/17"
)]
fn unnecessary_tables(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let config = context.local_config("monotype/unnecessary_tables");

    let base_tags = match parse_table_tags(&config, "unnecessary_table_tags") {
        Ok(v) => v,
        Err(status) => return return_result(vec![status]),
    };
    let cff_tags = match parse_table_tags(&config, "unnecessary_table_tags_cff") {
        Ok(v) => v,
        Err(status) => return return_result(vec![status]),
    };
    let ttf_tags = match parse_table_tags(&config, "unnecessary_table_tags_ttf") {
        Ok(v) => v,
        Err(status) => return return_result(vec![status]),
    };
    let variable_tags = match parse_table_tags(&config, "unnecessary_table_tags_variable") {
        Ok(v) => v,
        Err(status) => return return_result(vec![status]),
    };
    let prefixes = match parse_prefixes(&config, "unnecessary_table_prefixes") {
        Ok(v) => v,
        Err(status) => return return_result(vec![status]),
    };

    let mut unnecessary_table_tags = base_tags;
    if f.is_variable_font() {
        unnecessary_table_tags.extend(variable_tags);
    }

    let version = f.font().table_directory.sfnt_version();
    if version == 0x4F54544F {
        unnecessary_table_tags.extend(cff_tags);
    } else if version == 0x00010000 {
        unnecessary_table_tags.extend(ttf_tags);
    } else {
        skip!("unknown-sfnt-version", "Unknown sfnt version encountered");
    }

    let mut problems: Vec<Status> = vec![];
    let mut found_unnecessary_tables: Vec<String> = vec![];

    for tag in &unnecessary_table_tags {
        if f.has_table(tag) {
            found_unnecessary_tables.push(String::from_utf8(tag.to_vec()).map_err(|_| {
                FontspectorError::General(format!("Font tag '{:?}' wasn't UTF8?", tag.to_vec()))
            })?);
        }
    }

    let font_tables = f
        .font()
        .table_directory
        .table_records()
        .iter()
        .map(|record| record.tag().to_string())
        .collect::<Vec<_>>();
    for table in font_tables {
        if prefixes.iter().any(|p| table.starts_with(p)) {
            found_unnecessary_tables.push(table);
        }
    }

    if !found_unnecessary_tables.is_empty() {
        let message = format!(
            "This font contains the following unnecessary tables:\n\n    {}",
            found_unnecessary_tables.join("\n    ")
        );
        let mut status = Status::fail("unnecessary-tables", &message);
        status.add_metadata(Metadata::FontProblem {
            message: message.clone(),
            context: Some(json!({ "unnecessary_tables": found_unnecessary_tables })),
        });
        problems.push(status);
    }

    return_result(problems)
}
```

Use `return_result(vec![status])` for invalid-config (same multi-status style as the rest of this check). Do not use `Ok(status.into())`.

- [ ] **Step 2: Fix injection tests to not import removed consts**

In `test_unnecessary_tables_ttf` / `test_unnecessary_tables_cff`, build tag lists from string literals (same defaults as the helper), convert with a small test-local helper:

```rust
fn tags_from_strs(tags: &[&str]) -> Vec<[u8; 4]> {
    tags.iter()
        .map(|s| {
            let b = s.as_bytes();
            [b[0], b[1], b[2], b[3]]
        })
        .collect()
}
```

Use those instead of `UNNECESSARY_TABLE_TAGS` / `_CFF` / `_TTF`.

- [ ] **Step 3: Run all unnecessary_tables tests — expect PASS (GREEN)**

```bash
cargo test -p fontspector-profile-monotype -- unnecessary_tables -- --nocapture
```

Expected: all tests in the module PASS.

- [ ] **Step 4: Commit**

```bash
git add profile-monotype/src/checks/monotype/unnecessary_tables.rs
git commit -m "feat(monotype): load unnecessary_tables tags/prefixes from check config"
```

---

### Task 3: Register profile configuration defaults in `lib.rs`

**Files:**
- Modify: `profile-monotype/src/lib.rs` (after `.add_and_register_check(checks::monotype::unnecessary_tables)`)

- [ ] **Step 1: Add `with_configuration_defaults`**

Immediately after registering the check:

```rust
.add_and_register_check(checks::monotype::unnecessary_tables)
.with_configuration_defaults(
    "monotype/unnecessary_tables",
    HashMap::from([
        (
            "unnecessary_table_tags".to_string(),
            json!([
                "LINO", "SLST", "FFTM", "PfEd", "TeX ", "BDF ",
                "bdat", "bloc", "ADFH", "SPRK"
            ]),
        ),
        (
            "unnecessary_table_tags_cff".to_string(),
            json!(["gasp", "cvt ", "fpgm", "glyf", "loca", "prep"]),
        ),
        (
            "unnecessary_table_tags_ttf".to_string(),
            json!(["VORG"]),
        ),
        (
            "unnecessary_table_tags_variable".to_string(),
            json!(["kern"]),
        ),
        (
            "unnecessary_table_prefixes".to_string(),
            json!(["TSI", "MT", "ASC", "edt", "ABA"]),
        ),
    ]),
)
```

Preserve trailing spaces in `"TeX "`, `"BDF "`, `"cvt "` exactly.

- [ ] **Step 2: Ensure crate still builds**

```bash
cargo test -p fontspector-profile-monotype -- unnecessary_tables -- --nocapture
```

Expected: PASS (unit tests unchanged; defaults matter for CLI specialize path).

- [ ] **Step 3: Commit**

```bash
git add profile-monotype/src/lib.rs
git commit -m "feat(monotype): register default config for unnecessary_tables"
```

---

### Task 4: Override-behavior test

**Files:**
- Modify: `profile-monotype/src/checks/monotype/unnecessary_tables.rs` (tests only)

- [ ] **Step 1: Add override coverage test**

(Regression/coverage — expect PASS if Task 2 already wired prefixes.)

```rust
#[test]
fn test_custom_prefix_override() {
    let mut testable = test_able("mada/Mada-Regular.ttf");
    // Ensure no default-prefix tables; add a custom-prefix table
    for tag in [b"MT01", b"TSI0", b"ASC0", b"edt0", b"ABA0"] {
        remove_table(&mut testable, tag);
    }
    add_table(&mut testable, b"ZZZ0");

    let mut config = default_unnecessary_tables_config();
    config.insert(
        "monotype/unnecessary_tables".to_string(),
        json!({
            "unnecessary_table_tags": [],
            "unnecessary_table_tags_cff": [],
            "unnecessary_table_tags_ttf": [],
            "unnecessary_table_tags_variable": [],
            "unnecessary_table_prefixes": ["ZZZ"],
        }),
    );

    let results = run_check_with_config(
        unnecessary_tables,
        TestableType::Single(&testable),
        config,
    );
    assert_results_contain(
        &results,
        StatusCode::Fail,
        Some("unnecessary-tables".to_string()),
    );
    assert_messages_contain(&results, "ZZZ0");
}
```

- [ ] **Step 2: Run test**

```bash
cargo test -p fontspector-profile-monotype -- unnecessary_tables::tests::test_custom_prefix_override -- --nocapture
```

Expected: PASS. If it fails, fix prefix scanning until green.

- [ ] **Step 3: Commit**

```bash
git add profile-monotype/src/checks/monotype/unnecessary_tables.rs
git commit -m "test(monotype): cover custom unnecessary_tables prefix override"
```

---

### Task 5: Final verification

- [ ] **Step 1: Run full monotype profile tests**

```bash
cargo test -p fontspector-profile-monotype
```

Expected: all tests PASS.

- [ ] **Step 2: Clippy on touched crate (optional but recommended)**

```bash
cargo clippy -p fontspector-profile-monotype -- -D warnings
```

Expected: clean, or only pre-existing unrelated warnings.

- [ ] **Step 3: Confirm no leftover consts**

Grep `UNNECESSARY_TABLE_TAGS` under `profile-monotype/` — expect no matches in `src/` (tests must not reintroduce production consts).

---

## Execution notes

- Follow @superpowers:test-driven-development: do not implement Task 2 before Task 1 tests have been observed failing.
- Defaults in `lib.rs` and `default_unnecessary_tables_config()` must stay in sync; if they drift, prefer extracting a shared `pub fn default_unnecessary_tables_config_value() -> Value` later — YAGNI for this plan unless drift becomes painful during implementation.
- Empty arrays are valid config (means “no tags/prefixes for that key”), distinct from missing keys.
