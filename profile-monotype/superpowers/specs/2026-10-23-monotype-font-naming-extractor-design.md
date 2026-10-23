# Monotype Font Naming Extractor (Rust)

**Date:** 2026-10-23  
**Repo:** `fontspector.monotype`  
**Branch:** `feat/monotype-font-naming-extractor`  
**Source of truth (behavior):** `fonttools-fonttoolkit` — `Extractor` / `get_family_naming_info` / `nc_extractor` / `nc_processor` / `nc_known_names`

## Goal

Port full Python `Extractor` parity into `profile-monotype`: name strings (IDs 4, 6, 21, 22 with fallbacks) plus resolved style parts (`weight`, `width`, `slope`, `design`, `extra`), family name list, product variety, and foundries — using skrifa / `TestFont` for font I/O. Rust API may differ from the Python class layout.

## Non-goals (v1)

- Trademark / copyright parsers
- `get_best_family_and_style` OS/2 scoring and ambiguity resolution beyond what `get_family_naming_info` already does
- New fontspector profile checks (consumers come later)
- Changing `checks/monotype/names.rs` (`monotype/style_in_family_name` — unrelated name-table check)

## Approach

Library module inside `profile-monotype` only (not a shared workspace crate until a second consumer exists).

## Public API

```rust
pub struct NameStrings {
    pub full_name: Option<String>,       // nameID 4
    pub postscript_name: Option<String>, // nameID 6
    pub family_name: Option<String>,     // nameID 21 + fallback
    pub style_name: Option<String>,      // nameID 22 + fallback
}

pub struct StyleParts {
    pub weight: String,
    pub width: String,
    pub slope: String,
    pub design: String,
    pub extra: String,
}

pub struct FontNaming {
    pub names: NameStrings,
    pub style: StyleParts,
    pub family_name: String,        // resolved primary (Python: ps_family)
    pub family_names: Vec<String>,
    pub product_variety: String,    // Python shrinker may yield "" or a label e.g. "Pro"
    pub foundries: Vec<String>,
}

/// Font entry — Python `Extractor(editor)` equivalent.
pub fn extract(font: &TestFont) -> FontNaming;

/// String-only path for unit tests — Python `get_family_naming_info`.
pub fn family_naming_info(
    fullname: &str,
    fontname: &str,
    family: &str,
    style: &str,
    search_design: bool,
) -> (String, Vec<String>, StyleParts, String, Vec<String>);
```

Do not expose Python-only style-dict keys (`style_id`, `is_fixed_pitch`, `italic_angle`, `fonts`) on `StyleParts`.

## Module layout

```
profile-monotype/src/naming/
  mod.rs          // re-exports
  names.rs        // name-ID reads + fallback groups
  token.rs        // nc_extractor
  resolve.rs      // nc_processor::resolve_names (+ fuzzy call site)
  known_names.rs  // nc_known_names data
  merge.rs        // get_family_naming_info + style merge
```

Wire with `mod naming;` in `profile-monotype/src/lib.rs`. Keep the module name `naming` so it does not collide with `checks::monotype::names`.

## Font I/O

- Use `TestFont` / skrifa `StringId` with english-or-first selection consistent with checkapi helpers.
- Match `get_name_str_from_id(..., fallback=True)`:
  - 21 → 21, then 16, then 1
  - 22 → 22, then 17, then 2
  - 4 and 6 → that ID only
- Do not substitute `best_familyname()` / `best_subfamilyname()` unless proven identical to those groups.

## Data and algorithms

- Port `fonttoolkit/data/nc_known_names.py` into `known_names.rs` with keys/aliases byte-identical to Python.
- Behavioral port of `nc_extractor` and `nc_processor::resolve_names` (including `search_design`).
- Merge rules match `get_family_naming_info` / `_merge_style_dicts` for the five style keys above.
- Fuzzy match: only the `rapidfuzz.process.extractOne` site in `nc_processor`. Choose a Rust crate with comparable scoring during implementation; if fixtures diverge, pin Rust expected outputs and document the delta — do not “improve” Python behavior silently.
- Prefer `std` + existing workspace deps (`regex` already present). Add a fuzzy crate only if needed.

## Process

### Graphify (both repositories)

1. **Before planning/implementation:** run `graphify update .` in `fonttools-fonttoolkit` and `fontspector.monotype`. Use `graphify explain` / `graphify path` on `Extractor`, `get_family_naming_info`, `nc_extractor`, `nc_processor`, `TestFont`, and `profile-monotype` to confirm call graphs.
2. **After each meaningful code slice:** `graphify update .` in the repo that changed.
3. Treat `graphify-out/GRAPH_REPORT.md` commit stamp vs `git rev-parse HEAD` as a freshness check.

### Superpowers TDD

Iron law: no production code without a failing test first. Each slice: RED → verify fail → GREEN → refactor.

Slice order:

1. Tokenize / `get_name_tuples` cases from `tests/names/test_nc_extractor.py`
2. `resolve_names` cases from `tests/names/test_nc_processor.py`
3. `family_naming_info` / merge from `tests/names/test_name_collector.py` (e.g. Palatino LT Pro Bold → variety `"Pro"`, foundry Linotype)
4. Name-ID fallbacks + `extract(&TestFont)` smoke on a small fixture

If production code lands before its failing test, delete it and redo.

### Superpowers code review

After each completed TDD slice (and before merge): dispatch requesting-code-review with plan/requirements + `BASE_SHA` / `HEAD_SHA`. Fix Critical and Important before the next slice.

## Done when

- String-path golden tests match the Python cases listed above
- `extract` returns § Public API fields for at least one real font
- `cargo test -p fontspector-profile-monotype` passes
- Clippy denies for unwrap/expect remain intact on new code
- Graphify updated in fontspector after implementation; toolkit graph refreshed if used for navigation/parity helpers

## Risks

- Fuzzy-match score drift vs rapidfuzz
- Incomplete port of `nc_known_names` aliases causing silent style misclassification
- Name-platform / language selection differences between fontio3 and skrifa english-or-first
