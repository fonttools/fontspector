# Design: Configurable `monotype/unnecessary_tables` tag lists

**Date:** 2026-11-09  
**Check:** `monotype/unnecessary_tables`  
**Status:** Approved for implementation planning

## Goal

Make the unnecessary-table tag lists and prefix rules loadable from a Fontspector TOML configuration file, with profile defaults registered in `profile-monotype/src/lib.rs`. The check validates configured values (ASCII, well-formed tags) and fails with `invalid-config` when configuration is malformed. There are no hardcoded fallback tag lists in the check implementation.

## Config shape

Registered via `with_configuration_defaults("monotype/unnecessary_tables", ...)` in `profile-monotype/src/lib.rs`:

| Key | Meaning | Default values |
|-----|---------|----------------|
| `unnecessary_table_tags` | Base tags always considered unnecessary | `LINO`, `SLST`, `FFTM`, `PfEd`, `TeX `, `BDF `, `bdat`, `bloc`, `ADFH`, `SPRK` |
| `unnecessary_table_tags_cff` | Extra tags for CFF/CFF2 (`sfnt` `OTTO`) | `gasp`, `cvt `, `fpgm`, `glyf`, `loca`, `prep` |
| `unnecessary_table_tags_ttf` | Extra tags for TrueType (`sfnt` `0x00010000`) | `VORG` |
| `unnecessary_table_tags_variable` | Extra tags when the font is variable | `kern` |
| `unnecessary_table_prefixes` | Table-name prefixes treated as unnecessary | `TSI`, `MT`, `ASC`, `edt`, `ABA` |

Example TOML override (partial; omitted keys keep profile defaults):

```toml
[monotype/unnecessary_tables]
unnecessary_table_tags = ["LINO", "FFTM"]
unnecessary_table_prefixes = ["TSI", "MT"]
```

## Check behavior

1. Read `context.local_config("monotype/unnecessary_tables")`.
2. For each required key above, require a JSON array of strings.
3. Validate:
   - **Tags:** exactly 4 bytes, ASCII only; convert to `[u8; 4]` for `has_table`.
   - **Prefixes:** ASCII only, length 1–3 characters (`< 4`).
4. On missing key, wrong type, or malformed entry: return Fail with code `invalid-config` and a clear message. Do not silently ignore bad entries and do not fall back to hardcoded lists in the check file.
5. Build the effective tag list: base tags + CFF or TTF tags by sfnt version + variable tags if `is_variable_font()`.
6. Scan font table names for any that start with a configured prefix.
7. Existing failure reporting for found unnecessary tables remains (`unnecessary-tables`).
8. Update the check rationale with a TOML example (same documentation style as `has_features`).

## Files

- `profile-monotype/src/lib.rs` — add `with_configuration_defaults` for this check.
- `profile-monotype/src/checks/monotype/unnecessary_tables.rs` — remove `const` tag arrays; load and validate config; use configured prefixes; update rationale and tests.

## Testing

- Prefer `run_check_with_config` so configuration is explicit (unit tests do not go through profile specialization).
- Keep existing pass/fail coverage for TTF, CFF, CFF2, and injected unnecessary tables, supplying the default config map in tests.
- Add cases for:
  - invalid tag length / non-ASCII → `invalid-config`
  - custom override lists changing which tables fail

## Out of scope

- Changes to other checks, CLI argument parsing, or a global TOML schema beyond this check’s keys.
- Making unknown sfnt-version handling configurable.

## Risks

- Tests that call bare `run_check` without injecting config will fail once consts are removed; they must use `run_check_with_config` (or a shared helper that injects the same default map as `lib.rs`).
- Tag strings with trailing spaces (e.g. `"TeX "`, `"cvt "`) must be preserved exactly in JSON/TOML defaults.
