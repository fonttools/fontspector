# Monotype FontNaming Extractor Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Port Python `Extractor` / `get_family_naming_info` parity into `profile-monotype` as a Rust `naming` module using skrifa/`TestFont`.

**Architecture:** Pure string pipeline (`token` → `resolve` → `merge`) plus a thin `names` layer that reads name IDs from `TestFont`. Data tables live in `known_names`. No new profile checks in v1. API is Rust-shaped (`FontNaming`, `extract`, `family_naming_info`), not a Python class mirror.

**Tech Stack:** Rust 2021, `fontspector-checkapi` (`TestFont`), `skrifa` (`StringId`), `regex` (workspace), `write-fonts` for font fixtures in tests. No fuzzy crate for v1 (rapidfuzz is only used by OS/2 scoring helpers, which are out of scope).

**Spec:** `profile-monotype/superpowers/specs/2026-10-23-monotype-font-naming-extractor-design.md`

**Python sources (behavior):**
- `C:\projects\fonttools-fonttoolkit\fonttoolkit\names\nc_extractor.py`
- `C:\projects\fonttools-fonttoolkit\fonttoolkit\names\nc_processor.py` (`resolve_names` only)
- `C:\projects\fonttools-fonttoolkit\fonttoolkit\data\nc_known_names.py` (foundries through modifiers; not manufacturer/legal maps)
- `C:\projects\fonttools-fonttoolkit\fonttoolkit\names\name_collector.py` (`get_family_naming_info`, `Extractor`)
- `C:\projects\fonttools-fonttoolkit\fonttoolkit\names\info.py` (`get_name_str_from_id` fallbacks)
- Tests: `tests/names/test_nc_extractor.py`, `test_nc_processor.py`, `test_name_collector.py`

## Global Constraints

- Branch: `feat/monotype-font-naming-extractor` (already created from `main`)
- Crate: `fontspector-profile-monotype`; keep `#![deny(clippy::unwrap_used, clippy::expect_used)]`
- Module name must be `naming` (do not collide with `checks::monotype::names`)
- TDD iron law: no production code without a failing test first
- After each task: Superpowers requesting-code-review; then `graphify . --code-only` in `C:\projects\fontspector.monotype` (and in toolkit if Python helpers change)
- Port Python quirks for parity, including the `product_varieties` list typo `'PRO' 'W1G'` → `'PROW1G'` (adjacent string literals)
- Do not change `checks/monotype/names.rs`

---

## File structure

| File | Responsibility |
|------|----------------|
| `profile-monotype/src/naming/mod.rs` | Re-exports; `extract`, `FontNaming`, `NameStrings`, `StyleParts` |
| `profile-monotype/src/naming/known_names.rs` | Static tables from `nc_known_names.py` |
| `profile-monotype/src/naming/token.rs` | `get_name_tuples` / camel split / foundry / product variety |
| `profile-monotype/src/naming/resolve.rs` | `resolve_names` |
| `profile-monotype/src/naming/merge.rs` | `family_naming_info` + style merge |
| `profile-monotype/src/naming/names.rs` | Name-ID reads with fallback groups |
| `profile-monotype/src/lib.rs` | `mod naming;` |
| `profile-monotype/Cargo.toml` | No new deps expected for v1 |

---

### Task 1: Scaffold + `get_name_tuples` (token + known_names for tokenize)

**Files:**
- Create: `profile-monotype/src/naming/mod.rs`
- Create: `profile-monotype/src/naming/known_names.rs`
- Create: `profile-monotype/src/naming/token.rs`
- Modify: `profile-monotype/src/lib.rs` (add `mod naming;`)

**Interfaces:**
- Consumes: none
- Produces:
  - `pub fn get_name_tuples(name: &str, style: Option<&str>, normalized_names: bool, is_font_name: bool) -> (String, String, Vec<String>, Vec<String>)`
  - Tables: `foundries`, `product_varieties`, `nonsense` (and later style maps in Task 2)

- [ ] **Step 1: Write the failing test** in `token.rs`

```rust
#[cfg(test)]
mod tests {
    use super::get_name_tuples;

    #[test]
    fn present_lt_std_light_italic() {
        let (foundry, pv, family, style) =
            get_name_tuples("Present LT Std", Some("light italic"), true, false);
        assert_eq!(foundry, "Linotype");
        assert_eq!(pv, "Std");
        assert_eq!(family, vec!["Present".to_string()]);
        assert_eq!(style, vec!["Light".to_string(), "Italic".to_string()]);
    }

    #[test]
    fn empty_inputs() {
        let (foundry, pv, family, style) = get_name_tuples("", Some(""), true, false);
        assert_eq!(foundry, "");
        assert_eq!(pv, "");
        assert!(family.is_empty());
        assert!(style.is_empty());
    }

    #[test]
    fn present_linotype_keeps_linotype_in_family() {
        let (foundry, pv, family, style) =
            get_name_tuples("Present Linotype", Some("Light"), true, false);
        assert_eq!(foundry, "Linotype");
        assert_eq!(pv, "");
        assert_eq!(family, vec!["Present".to_string(), "Linotype".to_string()]);
        assert_eq!(style, vec!["Light".to_string()]);
    }

    #[test]
    fn prensa_display_comp_fullname() {
        let (foundry, pv, family, style) =
            get_name_tuples("PrensaDisplayComp SemiBold Italic", None, true, false);
        assert_eq!(foundry, "");
        assert_eq!(pv, "");
        assert_eq!(
            family,
            vec!["Prensa", "Display", "Comp", "Semi", "Bold", "Italic"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
        assert!(style.is_empty());
    }

    #[test]
    fn prensa_font_name_with_hyphen() {
        let (foundry, pv, family, style) =
            get_name_tuples("PrensaDisplayCompLTW1G-Lightitalique", None, true, true);
        assert_eq!(foundry, "");
        assert_eq!(pv, "");
        assert_eq!(
            family,
            vec!["Prensa", "Display", "Comp", "LTW", "1"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            style,
            vec!["Light".to_string(), "Italique".to_string()]
        );
    }
}
```

- [ ] **Step 2: Wire empty module and run test to verify it fails**

In `lib.rs` after `mod checks;`:

```rust
mod naming;
```

`mod.rs`:

```rust
mod known_names;
mod token;

pub use token::get_name_tuples;
```

`token.rs`: declare `pub fn get_name_tuples(...)` that `todo!()` — but crate denies unwrap/expect; use:

```rust
pub fn get_name_tuples(
    _name: &str,
    _style: Option<&str>,
    _normalized_names: bool,
    _is_font_name: bool,
) -> (String, String, Vec<String>, Vec<String>) {
    unimplemented!("get_name_tuples")
}
```

Note: `unimplemented!` is allowed (not unwrap/expect). Alternatively return empty and fail assertions.

Run:

```bash
cd C:\projects\fontspector.monotype
cargo test -p fontspector-profile-monotype naming::token::tests -- --nocapture
```

Expected: FAIL (unimplemented or assertion mismatch).

- [ ] **Step 3: Generate `known_names.rs` tables needed for tokenize**

From toolkit venv/Python, emit Rust for `foundries`, `product_varieties`, `nonsense` only (keep Python adjacent-string quirk). Save to `profile-monotype/src/naming/known_names.rs` as `pub static` / `LazyLock` / `phf` — prefer `std::sync::LazyLock` + `HashMap`/`HashSet` to avoid new deps:

```python
# run from fonttools-fonttoolkit with PYTHONPATH=.
from fonttoolkit.data import nc_known_names as k
# print Rust source for foundries: HashMap, product_varieties: HashSet, nonsense: HashSet
```

Include camelCase fixes in `token.rs` as:

```rust
const CAMEL_CASE_FIXES: &[(&str, &str)] = &[("LTW1G", "LT W1G"), ("Lightitalique", "Light Italique")];
```

- [ ] **Step 4: Implement `token.rs` as behavioral port of `nc_extractor.py`**

Port `_camel_to_tuple`, `split_fontname_tuple`, `_extract_foundry`, `_extract_product_variety`, `_delete_nonsense`, `get_name_tuples` using `regex` crate for the four split patterns and version patterns are **not** required in this task.

- [ ] **Step 5: Run tests — expect PASS**

```bash
cargo test -p fontspector-profile-monotype naming::token::tests -- --nocapture
```

- [ ] **Step 6: Graphify + commit**

```bash
cd C:\projects\fontspector.monotype
graphify . --code-only
git add profile-monotype/src/naming profile-monotype/src/lib.rs
git commit -m "feat(monotype): port get_name_tuples naming tokenizer"
```

- [ ] **Step 7: Code review gate** — Superpowers requesting-code-review on this task's BASE..HEAD; fix Critical/Important before Task 2.

---

### Task 2: `resolve_names`

**Files:**
- Create: `profile-monotype/src/naming/resolve.rs`
- Modify: `profile-monotype/src/naming/known_names.rs` (add weights, widths, slopes, designs, modifiers)
- Modify: `profile-monotype/src/naming/mod.rs`

**Interfaces:**
- Consumes: `get_name_tuples` (optional for tests); known style maps
- Produces:
  - `pub struct ResolvedStyle { pub weight: Option<String>, pub width: Option<String>, pub slope: Option<String>, pub design: Option<String>, pub extra: String }`
  - `pub fn resolve_names(name_tuple: &[String], search_design: bool, normalized_names: bool) -> (String, ResolvedStyle)`

- [ ] **Step 1: Write failing tests** in `resolve.rs` from `test_nc_processor.py::test_resolve_names` (all cases). Example first case:

```rust
#[test]
fn prensa_semi_bold_it() {
    let parts = ["Prensa", "Display", "Comp", "Semi", "Bold", "It"]
        .map(str::to_string);
    let (family, style) = resolve_names(&parts, true, true);
    assert_eq!(family, "Prensa");
    assert_eq!(style.weight.as_deref(), Some("SemiBold"));
    assert_eq!(style.width.as_deref(), Some("Compressed"));
    assert_eq!(style.slope.as_deref(), Some("Italic"));
    assert_eq!(style.design.as_deref(), Some("Display"));
    assert_eq!(style.extra, "");
}
```

Include: `search_design=false` cases (`design: None`, family `"Prensa Display"`), `Itsl` → extra, multi-style accumulation, numeric extras, empty tuple.

- [ ] **Step 2: Run — expect FAIL**

```bash
cargo test -p fontspector-profile-monotype naming::resolve::tests -- --nocapture
```

- [ ] **Step 3: Extend `known_names.rs`** with weights/widths/slopes/designs/modifiers from Python (canonical key → aliases). Implement `_lookup_style` as: match if `name == key || aliases.contains(name)` return canonical key. Special-case modifiers: treat `"Semi"` / `"Sm"` as modifier `"Semi"` before general lookup (mirror Python).

- [ ] **Step 4: Port `resolve_names` loop** from `nc_processor.py` L79–225 into Rust. Keep `Option` for weight/width/slope/design; `extra` is `String` (default `""`).

- [ ] **Step 5: Run — expect PASS**

```bash
cargo test -p fontspector-profile-monotype naming::resolve::tests -- --nocapture
```

- [ ] **Step 6: Graphify + commit + review gate**

```bash
cd C:\projects\fontspector.monotype
graphify . --code-only
git add profile-monotype/src/naming
git commit -m "feat(monotype): port resolve_names style resolution"
```

---

### Task 3: `family_naming_info` (merge)

**Files:**
- Create: `profile-monotype/src/naming/merge.rs`
- Modify: `profile-monotype/src/naming/mod.rs`

**Interfaces:**
- Consumes: `get_name_tuples`, `resolve_names`
- Produces:
  - `pub struct StyleParts { pub weight: String, pub width: String, pub slope: String, pub design: String, pub extra: String }`
  - `pub fn family_naming_info(fullname: &str, fontname: &str, family: &str, style: &str, search_design: bool) -> (String, Vec<String>, StyleParts, String, Vec<String>)`

Merge rules (from Python `_list_shrinker` / `_merge_style_dicts` / `get_family_naming_info`):
1. Call `get_name_tuples` + `resolve_names` on family+style, fullname, fontname (fontname with `is_font_name=true` only for the PS path — Python uses `get_name_tuples(fontname)` **without** `is_font_name=True` in `get_family_naming_info`; match that exactly).
2. `family_names` = unique non-empty of `[family, full_family, ps_family]` sorted.
3. `foundries` = unique non-empty foundry strings sorted.
4. `product_variety` = first of unique non-empty product varieties, else `""`.
5. Style merge: for each of weight/width/slope/design/extra, collect string forms from the three resolved styles (`None` → skip / treat as empty), unique-sort, join with space for `StyleParts` fields (empty string if none).

- [ ] **Step 1: Failing test** (from `test_name_collector.py::test_get_family_naming_info`):

```rust
#[test]
fn palatino_lt_pro_bold() {
    let (ps_family, family_names, style, pv, foundries) = family_naming_info(
        "Palatino LT Pro Bold",
        "PalatinoLTPro-Bold",
        "Palatino LT Pro",
        "Bold",
        true,
    );
    assert_eq!(ps_family, "Palatino");
    assert_eq!(family_names, vec!["Palatino".to_string()]);
    assert_eq!(style.weight, "Bold");
    assert_eq!(style.width, "");
    assert_eq!(style.slope, "");
    assert_eq!(style.design, "");
    assert_eq!(style.extra, "");
    assert_eq!(pv, "Pro");
    assert_eq!(foundries, vec!["Linotype".to_string()]);
}
```

- [ ] **Step 2: Run — expect FAIL**

```bash
cargo test -p fontspector-profile-monotype naming::merge::tests -- --nocapture
```

- [ ] **Step 3: Implement `merge.rs`** porting `get_family_naming_info` + helpers. Map `ResolvedStyle` → strings for merge (`None` → omit).

- [ ] **Step 4: Run — expect PASS**

- [ ] **Step 5: Graphify + commit + review gate**

```bash
git commit -m "feat(monotype): port family_naming_info merge"
```

---

### Task 4: Name-ID reads + `extract`

**Files:**
- Create: `profile-monotype/src/naming/names.rs`
- Modify: `profile-monotype/src/naming/mod.rs` (add `FontNaming`, `NameStrings`, `extract`)

**Interfaces:**
- Consumes: `TestFont`, `family_naming_info`
- Produces:
  - `pub struct NameStrings { pub full_name: Option<String>, pub postscript_name: Option<String>, pub family_name: Option<String>, pub style_name: Option<String> }`
  - `pub struct FontNaming { pub names: NameStrings, pub style: StyleParts, pub family_name: String, pub family_names: Vec<String>, pub product_variety: String, pub foundries: Vec<String> }`
  - `pub fn name_string_with_fallback(font: &TestFont, id: u16, fallback: bool) -> Option<String>`
  - `pub fn extract(font: &TestFont) -> FontNaming`

Fallback groups (Python `get_name_str_from_id`):
- 21 → `[21, 16, 1]`
- 22 → `[22, 17, 2]`
- 4, 6 → single id
Use `font.get_best_name(&[StringId::new(n), ...])` or localized english_or_first per id in order.

- [ ] **Step 1: Failing unit test for fallbacks** — build a minimal font with `write-fonts` (same pattern as `checks/monotype/names.rs` tests) containing only nameIDs 1/2/4/6 (no 21/22). Assert `name_string_with_fallback(..., 21, true)` returns nameID 1 value.

- [ ] **Step 2: Failing extract parity tests** mirroring `test_extractor` / `test_extractor_weight` / `test_extractor_weight_reg`:

| Case | Expected `style.weight` |
|------|-------------------------|
| Garamond Classico Italic (IDs 1,2,4,6) | `""` |
| Helvetica Now XBlk (+16/17) | `"ExtraBlack"` |
| Helvetica Now Display / Reg | `"Regular"` |

Build name tables with Windows platform 3/1/1033 records via `write-fonts`.

- [ ] **Step 3: Run — expect FAIL**

```bash
cargo test -p fontspector-profile-monotype naming:: -- --nocapture
```

- [ ] **Step 4: Implement `names.rs` + `extract`**

```rust
pub fn extract(font: &TestFont) -> FontNaming {
    let full = name_string_with_fallback(font, 4, false);
    let ps = name_string_with_fallback(font, 6, false);
    let fam = name_string_with_fallback(font, 21, true);
    let sty = name_string_with_fallback(font, 22, true);
    let (family_name, family_names, style, product_variety, foundries) = family_naming_info(
        full.as_deref().unwrap_or(""),
        ps.as_deref().unwrap_or(""),
        fam.as_deref().unwrap_or(""),
        sty.as_deref().unwrap_or(""),
        true,
    );
    FontNaming {
        names: NameStrings {
            full_name: full,
            postscript_name: ps,
            family_name: fam,
            style_name: sty,
        },
        style,
        family_name,
        family_names,
        product_variety,
        foundries,
    }
}
```

(Use `Option` without unwrap_or_else panic; empty string defaults are fine.)

- [ ] **Step 5: Run all naming tests — expect PASS**

```bash
cargo test -p fontspector-profile-monotype naming:: -- --nocapture
cargo clippy -p fontspector-profile-monotype -- -D warnings
```

- [ ] **Step 6: Final graphify + commit + review**

```bash
cd C:\projects\fontspector.monotype
graphify . --code-only
cd C:\projects\fonttools-fonttoolkit
graphify . --code-only
cd C:\projects\fontspector.monotype
git add profile-monotype/src/naming profile-monotype/src/lib.rs
git commit -m "feat(monotype): add extract() FontNaming from TestFont"
```

Dispatch final requesting-code-review before considering the feature merge-ready.

---

## Self-review (plan vs spec)

| Spec requirement | Task |
|------------------|------|
| Full Extractor parity (names + styles) | 1–4 |
| Module under `profile-monotype/src/naming/` | 1 |
| skrifa / TestFont I/O | 4 |
| Fallback groups 21/16/1 and 22/17/2 | 4 |
| known_names port | 1–2 |
| No new check | all |
| TDD | every task Steps 1–2 before impl |
| Graphify both repos | Tasks + final Step 6 (`graphify . --code-only`) |
| Code review per slice | Task Steps "review gate" |
| No fuzzy for non-goal path | Global Constraints |

No TBD placeholders. `product_variety` is `String`. Fuzzy crate omitted deliberately (not on Extractor path).
