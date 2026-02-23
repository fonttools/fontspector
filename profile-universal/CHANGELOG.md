# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.7.0 (2026-02-23)

### New Features

 - <csr-id-568958e9b33f5c11076dde02e89ce0a73bc6a07e/> Add machine-readable metadata to (almost) all checks
   * chore(api): Add structured metadata enum
   
   * feat(googlefonts/outline): Add machine-readable metadata to all outline checks
   
   * feat(interpolation_issues): Add machine-readable metadata to interpolation issues check
   
   * chore(api): Fix metadata type
   
   * feat(universal): Add machine-readable metadata to (almost) all checks
   
   * feat(opentype): Add machine-readable metadata to (almost) all checks
   
   * feat(googlefonts): Add machine-readable metadata to (almost) all checks

### Bug Fixes

 - <csr-id-eb142594b74d95a8deda5bdd16faf71dbdf34d0a/> Update file size checks
   * refactor(profile-universal/src/checks/file_size): include FATAL level
   
   * feat(googlefonts): add check to ensure families are under 25mb
   
   * chore: lint

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 3 calendar days.
 - 16 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#628](https://github.com/fonttools/fontspector/issues/628), [#629](https://github.com/fonttools/fontspector/issues/629)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#628](https://github.com/fonttools/fontspector/issues/628)**
    - Update file size checks ([`eb14259`](https://github.com/fonttools/fontspector/commit/eb142594b74d95a8deda5bdd16faf71dbdf34d0a))
 * **[#629](https://github.com/fonttools/fontspector/issues/629)**
    - Add machine-readable metadata to (almost) all checks ([`568958e`](https://github.com/fonttools/fontspector/commit/568958e9b33f5c11076dde02e89ce0a73bc6a07e))
</details>

## v1.6.0 (2026-02-06)

<csr-id-19b91cdf15a6d8ea7e60b616199d28e03c76ba4d/>
<csr-id-275df045d7ee4ba95e8183c48982201531dad633/>

### Chore

 - <csr-id-19b91cdf15a6d8ea7e60b616199d28e03c76ba4d/> Reformat

### New Features

<csr-id-5ee341e08c41001139a7914f346bfe70a875cc3a/>

 - <csr-id-04a449bd24ff2519f5c0300cd013ffcdbf363bf7/> Add check (implements #557)
   * chore(checkapi): Add fontdrasil and axes function
* feat(suspicious_sidebearings): Add check (implements #557)

### Refactor

 - <csr-id-275df045d7ee4ba95e8183c48982201531dad633/> check for double spaces
   * refactor(name/trailing_spaces): check for double spaces
   
   * refactor: format, make lint happy
   
   ---------

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 1 calendar day.
 - 2 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#582](https://github.com/fonttools/fontspector/issues/582), [#611](https://github.com/fonttools/fontspector/issues/611), [#612](https://github.com/fonttools/fontspector/issues/612), [#618](https://github.com/fonttools/fontspector/issues/618)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#582](https://github.com/fonttools/fontspector/issues/582)**
    - Check for double spaces ([`275df04`](https://github.com/fonttools/fontspector/commit/275df045d7ee4ba95e8183c48982201531dad633))
 * **[#611](https://github.com/fonttools/fontspector/issues/611)**
    - Reformat ([`19b91cd`](https://github.com/fonttools/fontspector/commit/19b91cdf15a6d8ea7e60b616199d28e03c76ba4d))
 * **[#612](https://github.com/fonttools/fontspector/issues/612)**
    - Improve warning message, fixes #609 ([`5ee341e`](https://github.com/fonttools/fontspector/commit/5ee341e08c41001139a7914f346bfe70a875cc3a))
 * **[#618](https://github.com/fonttools/fontspector/issues/618)**
    - Add check (implements #557) ([`04a449b`](https://github.com/fonttools/fontspector/commit/04a449bd24ff2519f5c0300cd013ffcdbf363bf7))
 * **Uncategorized**
    - Release fontspector-checkapi v1.4.0, fontspector-profile-fontwerk v1.2.3, fontspector-profile-googlefonts v1.6.1, fontspector-profile-opentype v1.3.1, fontspector-profile-universal v1.6.0, fontspector v1.5.4 ([`b27d3e3`](https://github.com/fonttools/fontspector/commit/b27d3e3ab7e2cf650a02883d776808bde0611edd))
</details>

<csr-unknown>
 Improve warning message, fixes #609<csr-unknown/>

## v1.5.0 (2026-02-04)

<csr-id-905b344c2702bc78927fd7407978959c32dabbfe/>
<csr-id-c8bc460e7ee09451e8cb86ce9463fcbea3703c86/>

### New Features

 - <csr-id-1680b05ab268e7acabf4444823585128ed36e2e6/> warn when variable font with vmtx lacks VVAR table
   * feat(universal): warn when variable font with vmtx lacks VVAR table
* test(universal): add Rust tests for VVAR table check
- test_vvar_missing: variable font with vmtx but no VVAR triggers WARN
     and message contains "vmtx"
- test_vvar_present: variable font with vmtx AND VVAR has no warning

### Refactor

 - <csr-id-905b344c2702bc78927fd7407978959c32dabbfe/> add context and improve report
   * refactor(family_and_style_max_length): add info '({} characters too long)'
   
   * refactor(required_name_ids): removing leftover print
   
   * refactor(family_and_style_max_length): adding unittest + context
   
   * refactor(family_and_style_max_length): add missing context "INSTANCE_NAME"
   
   ---------

### Test

 - <csr-id-c8bc460e7ee09451e8cb86ce9463fcbea3703c86/> Port all remaining `required_table` tests to rust
   * feat(universal): warn when variable font with vmtx lacks VVAR table
   
   Variable fonts that include a vmtx (vertical metrics) table should also
   include a VVAR table.
   
   As noted by Behdad (at https://github.com/notofonts/noto-cjk/issues/307),
   inclusion of the VVAR table speeds up processing of vertical typesetting
   significantly with only a minor file size increase, even in cases where
   there is no variation in the vertical metrics (vmtx) across the
   designspace. Fonttools automatically adds this table if the vmtx table
   exists, and it isn't on the exclude list, but other build systems do
   not at present.
   
   (Closes #516)
   
   * test(universal): add Rust tests for VVAR table check
   
   Add unit tests for the missing-vvar warning in required_tables check:
   - test_vvar_missing: variable font with vmtx but no VVAR triggers WARN
     and message contains "vmtx"
   - test_vvar_present: variable font with vmtx AND VVAR has no warning
   
   Add ShantellSans test font to resources/test/ for VVAR present case.
   Remove equivalent Python tests in favor of Rust implementation.
   
   * test(universal): complete required_tables Python to Rust test port
   
   - Add remove_table and add_table helpers to codetesting.rs
   - Port remaining Python tests for required_tables check
   - Tests cover TrueType, CFF, and CFF2 fonts
   - Tests validate required tables, optional tables detection, and VVAR check
   - Remove Python test file since Rust tests are now complete
   - Note: maxp removal test skipped as it causes TestFont initialization to fail
   
   * refactor(check-api): use skrifa/write-fonts for table manipulation helpers
   
   Replace raw byte manipulation in remove_table and add_table test helpers
   with skrifa FontRef and write-fonts FontBuilder APIs. This reduces code
   from ~110 lines each to ~15 lines while maintaining the same functionality.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 25 calendar days.
 - 48 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#566](https://github.com/fonttools/fontspector/issues/566), [#585](https://github.com/fonttools/fontspector/issues/585), [#586](https://github.com/fonttools/fontspector/issues/586)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#566](https://github.com/fonttools/fontspector/issues/566)**
    - Add context and improve report ([`905b344`](https://github.com/fonttools/fontspector/commit/905b344c2702bc78927fd7407978959c32dabbfe))
 * **[#585](https://github.com/fonttools/fontspector/issues/585)**
    - Warn when variable font with vmtx lacks VVAR table ([`1680b05`](https://github.com/fonttools/fontspector/commit/1680b05ab268e7acabf4444823585128ed36e2e6))
 * **[#586](https://github.com/fonttools/fontspector/issues/586)**
    - Port all remaining `required_table` tests to rust ([`c8bc460`](https://github.com/fonttools/fontspector/commit/c8bc460e7ee09451e8cb86ce9463fcbea3703c86))
 * **Uncategorized**
    - Release fontspector-checkapi v1.3.2, fontspector-profile-fontwerk v1.2.2, fontspector-profile-googlefonts v1.6.0, fontspector-profile-universal v1.5.0, fontspector-profile-iso15008 v1.0.5, fontspector v1.5.3 ([`43eb125`](https://github.com/fonttools/fontspector/commit/43eb125ce74b3424af90f5f9aa8082fdb4e2e742))
</details>

## v1.4.0 (2025-12-17)

<csr-id-3aff895fb75d510fa826e19339424347b5d3ff61/>
<csr-id-fe45168dca4d0764eb65dfddb75b2a8013bd6326/>
<csr-id-4befd6c88900e2e06c363a8a6b1cdfc9518e9c91/>

### Chore

 - <csr-id-3aff895fb75d510fa826e19339424347b5d3ff61/> Refresh dependencies
   * chore: Refresh dependencies
   
   * chore: Fixup fontc API
 - <csr-id-fe45168dca4d0764eb65dfddb75b2a8013bd6326/> Not my fail
 - <csr-id-4befd6c88900e2e06c363a8a6b1cdfc9518e9c91/> More from rustybuzz to harfrust
   * chore: Move from rustybuzz to harfrust
   
   * test(googlefonts/shaping/forbidden): Move tests to Rust
   
   * test: Pass full config to tests
   
   * chore: Missing docstrings
   
   * chore: not my fail

### New Features

 - <csr-id-6d1aa3c5839e5c76fe8510c029ab0c89eda81327/> check required unicodes (fixes #526)
   * feat(has_unicodes): a new universal check for required unicodes
* style(has_unicodes): make title shorter for better UX

### Bug Fixes

 - <csr-id-79be9c4d732825e2158ad9a1ff9c98b184eba7d4/> Improve output (fixes #527)
 - <csr-id-19710dd4360d090522da6d68387067988103f9c2/> ignore fonts without name ID 16 and 17
   * fix(family/uniqueness_first_31_characters): first add failing unittest
* fix(family/uniqueness_first_31_characters): skip if font has no name ID 16 and 17
* fix(family/uniqueness_first_31_characters): make lint happy

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 55 calendar days.
 - 64 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#503](https://github.com/fonttools/fontspector/issues/503), [#528](https://github.com/fonttools/fontspector/issues/528), [#531](https://github.com/fonttools/fontspector/issues/531), [#535](https://github.com/fonttools/fontspector/issues/535), [#536](https://github.com/fonttools/fontspector/issues/536), [#552](https://github.com/fonttools/fontspector/issues/552)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#503](https://github.com/fonttools/fontspector/issues/503)**
    - Ignore fonts without name ID 16 and 17 ([`19710dd`](https://github.com/fonttools/fontspector/commit/19710dd4360d090522da6d68387067988103f9c2))
 * **[#528](https://github.com/fonttools/fontspector/issues/528)**
    - Improve output (fixes #527) ([`79be9c4`](https://github.com/fonttools/fontspector/commit/79be9c4d732825e2158ad9a1ff9c98b184eba7d4))
 * **[#531](https://github.com/fonttools/fontspector/issues/531)**
    - More from rustybuzz to harfrust ([`4befd6c`](https://github.com/fonttools/fontspector/commit/4befd6c88900e2e06c363a8a6b1cdfc9518e9c91))
 * **[#535](https://github.com/fonttools/fontspector/issues/535)**
    - Fix(required_name_ids): make universal (fixes #534) ([`ad73d4c`](https://github.com/fonttools/fontspector/commit/ad73d4cf0a8e9627723b25e2861057e7d75ad49b))
 * **[#536](https://github.com/fonttools/fontspector/issues/536)**
    - Check required unicodes (fixes #526) ([`6d1aa3c`](https://github.com/fonttools/fontspector/commit/6d1aa3c5839e5c76fe8510c029ab0c89eda81327))
 * **[#552](https://github.com/fonttools/fontspector/issues/552)**
    - Refresh dependencies ([`3aff895`](https://github.com/fonttools/fontspector/commit/3aff895fb75d510fa826e19339424347b5d3ff61))
 * **Uncategorized**
    - Release fontspector-checkapi v1.3.1, fontspector-profile-fontwerk v1.2.1, fontspector-profile-googlefonts v1.5.0, fontspector-profile-universal v1.4.0, fontspector v1.5.2 ([`2779526`](https://github.com/fonttools/fontspector/commit/2779526c00f235ea93e95882b4ebd2b41786c715))
    - Not my fail ([`fe45168`](https://github.com/fonttools/fontspector/commit/fe45168dca4d0764eb65dfddb75b2a8013bd6326))
</details>

## v1.3.0 (2025-10-14)

<csr-id-56e2f3f9167f15b2cb8cba8377403b8472514a7c/>

### New Features

 - <csr-id-008b6fa8eab5ed5e9bb9cff5a6d4e51f822ce0be/> Adding new check
   * feat(family/uniqueness_first_31_characters): Adding new check (#472)
* fix: some typos
* fix: some formatting and panic issues
* refactor: moving get_name_entry_string and get_name_pel_codes to font.rs
* refactor: rename get_name_pel_codes -> get_name_platform_tuples
* refactor: get_name_platform_tuples (return set, not sorted list)
* refactor: remove redundant first_31_char_collection.contains_key
* refactor: add struct PlatformSelector
* refactor(family_uniqueness_first_31_characters): unittest

### Bug Fixes

<csr-id-7a7f2a6fe346597071b8099a481023b24afa094d/>
<csr-id-523021c7fbf321d5f3b715955f1f59ea81b8749f/>

 - <csr-id-a059bae9570e98cffe2f6596edb752feba568731/> Fix typo in codepoint
   * fix(arabic_high_hamza): Typo in high hamza codepoint
* test(arabic_high_hamza): Add tests
* fix(whitespace_ink): Don't check for whitespace ink in format characters
* chore: Update Cargo.lock
* fix(hinting_impact): We can now dehint glyf tables
* chore: Change Rust CI workflow

### Test

 - <csr-id-56e2f3f9167f15b2cb8cba8377403b8472514a7c/> Allow testing of collections

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 15 calendar days.
 - 29 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#471](https://github.com/fonttools/fontspector/issues/471), [#472](https://github.com/fonttools/fontspector/issues/472), [#473](https://github.com/fonttools/fontspector/issues/473), [#478](https://github.com/fonttools/fontspector/issues/478), [#486](https://github.com/fonttools/fontspector/issues/486), [#494](https://github.com/fonttools/fontspector/issues/494)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#471](https://github.com/fonttools/fontspector/issues/471)**
    - We can now dehint glyf tables ([`523021c`](https://github.com/fonttools/fontspector/commit/523021c7fbf321d5f3b715955f1f59ea81b8749f))
 * **[#472](https://github.com/fonttools/fontspector/issues/472)**
    - Adding new check ([`008b6fa`](https://github.com/fonttools/fontspector/commit/008b6fa8eab5ed5e9bb9cff5a6d4e51f822ce0be))
 * **[#473](https://github.com/fonttools/fontspector/issues/473)**
    - Adding new check ([`008b6fa`](https://github.com/fonttools/fontspector/commit/008b6fa8eab5ed5e9bb9cff5a6d4e51f822ce0be))
 * **[#478](https://github.com/fonttools/fontspector/issues/478)**
    - Allow testing of collections ([`56e2f3f`](https://github.com/fonttools/fontspector/commit/56e2f3f9167f15b2cb8cba8377403b8472514a7c))
 * **[#486](https://github.com/fonttools/fontspector/issues/486)**
    - Don't check for whitespace ink in format characters ([`7a7f2a6`](https://github.com/fonttools/fontspector/commit/7a7f2a6fe346597071b8099a481023b24afa094d))
 * **[#494](https://github.com/fonttools/fontspector/issues/494)**
    - Fix typo in codepoint ([`a059bae`](https://github.com/fonttools/fontspector/commit/a059bae9570e98cffe2f6596edb752feba568731))
 * **Uncategorized**
    - Release fontspector-checkapi v1.3.0, fontspector-profile-googlefonts v1.4.1, fontspector-profile-universal v1.3.0, fontspector v1.5.1 ([`7b3d29e`](https://github.com/fonttools/fontspector/commit/7b3d29e9dab0c4bf11506345219e59b378291be2))
</details>

## v1.2.0 (2025-09-15)

<csr-id-0c6365a1d3383dad9d12de3984989ee5747b35cf/>
<csr-id-90205a8089d1394f957cdf13cdcc461b73824425/>
<csr-id-abd4d4cf426666d0dac52706a763fb01d0e85d2c/>

### Chore

 - <csr-id-0c6365a1d3383dad9d12de3984989ee5747b35cf/> get working again
   * chore: Upgrade fontations deps
   
   * chore: API changed
   
   * chore: Comply with new elided lifetime rules

### New Features

<csr-id-fe2405db54f4da13ae03673150e91163f4f6d889/>
<csr-id-74f8cf3206b0b74227b9e22a5f0071a016ad9973/>

 - <csr-id-36399a0f334f5348bf7c5dc951c3d03fb1fe0afe/> New check
 - <csr-id-6726d1fc9f61577ad6020127b0135146abfe507b/> colr_glyph_entry_with_no_layers
   * test: Add testdata –> color font with glyph entry within the COLR table with no layers (modified with ttx)
* Update .gitignore
* feat: colr_glyph_entry_with_no_layers
* refactor: remove comment
* refactor: run cargo fmt
* fix: this `if let` can be collapsed into the outer `if let`
* feat(opentype/field_values): New check
* chore: improve fvar serialization

### Bug Fixes

 - <csr-id-d58c9ef800041bb3ec8cd266907d4c03f1b548c9/> Improve formatting of bulleted lists, fixes #352
 - <csr-id-1fd17e6a7e44c5d31bf560d02c6cf846f4fc59b3/> Rebuild desired glyph data, fixes #284
 - <csr-id-0c50d3bf0c259394384857c2908b59f5da279170/> Only run check on hinted fonts, fixes #347

### Other

 - <csr-id-90205a8089d1394f957cdf13cdcc461b73824425/> fontspector-checkapi v1.1.2, fontspector-fontbakery-bridge v1.2.0, fontspector-profile-fontwerk v1.1.0, fontspector-profile-googlefonts v1.3.0, fontspector-profile-opentype v1.2.1, fontspector-profile-universal v1.1.2, fontspector v1.4.0

### Test

 - <csr-id-abd4d4cf426666d0dac52706a763fb01d0e85d2c/> Move the tests from Python to Rust
   * chore(codetesting): Improve codetesting story slightly
   
   * test(contour_count): WIP move test to Rust
   
   * chore: Helper function for check testing
   
   * test: Move many tests to Rust, by some strange magic
   
   * test: add test files
   
   * chore: Helper function for check testing with parameters
   
   * test: Move many more tests to Rust, thanks to my little electronic friend
   
   * chore: Change test helper functions to take references
   
   * test: Two more tests to Rust
   
   * chore: Fix up warnings
   
   * test: Four more tests to Rust

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 59 calendar days.
 - 10 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 9 unique issues were worked on: [#348](https://github.com/fonttools/fontspector/issues/348), [#349](https://github.com/fonttools/fontspector/issues/349), [#357](https://github.com/fonttools/fontspector/issues/357), [#371](https://github.com/fonttools/fontspector/issues/371), [#405](https://github.com/fonttools/fontspector/issues/405), [#407](https://github.com/fonttools/fontspector/issues/407), [#409](https://github.com/fonttools/fontspector/issues/409), [#417](https://github.com/fonttools/fontspector/issues/417), [#418](https://github.com/fonttools/fontspector/issues/418)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#348](https://github.com/fonttools/fontspector/issues/348)**
    - Rebuild desired glyph data, fixes #284 ([`1fd17e6`](https://github.com/fonttools/fontspector/commit/1fd17e6a7e44c5d31bf560d02c6cf846f4fc59b3))
 * **[#349](https://github.com/fonttools/fontspector/issues/349)**
    - Only run check on hinted fonts, fixes #347 ([`0c50d3b`](https://github.com/fonttools/fontspector/commit/0c50d3bf0c259394384857c2908b59f5da279170))
 * **[#357](https://github.com/fonttools/fontspector/issues/357)**
    - Improve formatting of bulleted lists, fixes #352 ([`d58c9ef`](https://github.com/fonttools/fontspector/commit/d58c9ef800041bb3ec8cd266907d4c03f1b548c9))
 * **[#371](https://github.com/fonttools/fontspector/issues/371)**
    - Get working again ([`0c6365a`](https://github.com/fonttools/fontspector/commit/0c6365a1d3383dad9d12de3984989ee5747b35cf))
 * **[#405](https://github.com/fonttools/fontspector/issues/405)**
    - New check ([`74f8cf3`](https://github.com/fonttools/fontspector/commit/74f8cf3206b0b74227b9e22a5f0071a016ad9973))
 * **[#407](https://github.com/fonttools/fontspector/issues/407)**
    - New check ([`fe2405d`](https://github.com/fonttools/fontspector/commit/fe2405db54f4da13ae03673150e91163f4f6d889))
 * **[#409](https://github.com/fonttools/fontspector/issues/409)**
    - Colr_glyph_entry_with_no_layers ([`6726d1f`](https://github.com/fonttools/fontspector/commit/6726d1fc9f61577ad6020127b0135146abfe507b))
 * **[#417](https://github.com/fonttools/fontspector/issues/417)**
    - New check ([`36399a0`](https://github.com/fonttools/fontspector/commit/36399a0f334f5348bf7c5dc951c3d03fb1fe0afe))
 * **[#418](https://github.com/fonttools/fontspector/issues/418)**
    - Move the tests from Python to Rust ([`abd4d4c`](https://github.com/fonttools/fontspector/commit/abd4d4cf426666d0dac52706a763fb01d0e85d2c))
 * **Uncategorized**
    - Release fontspector-checkapi v1.2.0, fontspector-profile-fontwerk v1.2.0, fontspector-profile-googlefonts v1.4.0, fontspector-profile-opentype v1.3.0, fontspector-profile-universal v1.2.0, fontspector-profile-iso15008 v1.0.4, fontspector v1.5.0 ([`de4a966`](https://github.com/fonttools/fontspector/commit/de4a966105bea222ea98da69793ddbfbdd590f9d))
    - Fontspector-checkapi v1.1.2, fontspector-fontbakery-bridge v1.2.0, fontspector-profile-fontwerk v1.1.0, fontspector-profile-googlefonts v1.3.0, fontspector-profile-opentype v1.2.1, fontspector-profile-universal v1.1.2, fontspector v1.4.0 ([`90205a8`](https://github.com/fonttools/fontspector/commit/90205a8089d1394f957cdf13cdcc461b73824425))
</details>

## v1.1.2 (2025-08-11)

<csr-id-0c6365a1d3383dad9d12de3984989ee5747b35cf/>

### Chore

 - <csr-id-0c6365a1d3383dad9d12de3984989ee5747b35cf/> get working again
   * chore: Upgrade fontations deps
   
   * chore: API changed
   
   * chore: Comply with new elided lifetime rules

### Bug Fixes

 - <csr-id-d58c9ef800041bb3ec8cd266907d4c03f1b548c9/> Improve formatting of bulleted lists, fixes #352
 - <csr-id-1fd17e6a7e44c5d31bf560d02c6cf846f4fc59b3/> Rebuild desired glyph data, fixes #284
 - <csr-id-0c50d3bf0c259394384857c2908b59f5da279170/> Only run check on hinted fonts, fixes #347

## v1.1.1 (2025-07-02)

<csr-id-138edbbfba88008d71d9247eccbdfc017fef8b81/>
<csr-id-a6b7ffc4f39c6b1c1bd92cd9b07f4ba22d54ef2e/>

### Chore

 - <csr-id-138edbbfba88008d71d9247eccbdfc017fef8b81/> Update fontations dependencies
   * chore: Update fontations dependencies
   
   * test(italic_angle): Update bounds check

### Bug Fixes

 - <csr-id-46e90e51624979590af83272f96cbcfc521b7d0a/> Improve rationale rewrapping
   * fix(cli): Improve rationale rewrapping
* chore: Style fixes for new clippy

### Style

 - <csr-id-a6b7ffc4f39c6b1c1bd92cd9b07f4ba22d54ef2e/> deny indexing slicing
   * chore: More lints into Cargo.toml
   
   * style: Deny indexing slicing

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 11 calendar days.
 - 12 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#287](https://github.com/fonttools/fontspector/issues/287), [#291](https://github.com/fonttools/fontspector/issues/291), [#299](https://github.com/fonttools/fontspector/issues/299)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#287](https://github.com/fonttools/fontspector/issues/287)**
    - Deny indexing slicing ([`a6b7ffc`](https://github.com/fonttools/fontspector/commit/a6b7ffc4f39c6b1c1bd92cd9b07f4ba22d54ef2e))
 * **[#291](https://github.com/fonttools/fontspector/issues/291)**
    - Update fontations dependencies ([`138edbb`](https://github.com/fonttools/fontspector/commit/138edbbfba88008d71d9247eccbdfc017fef8b81))
 * **[#299](https://github.com/fonttools/fontspector/issues/299)**
    - Improve rationale rewrapping ([`46e90e5`](https://github.com/fonttools/fontspector/commit/46e90e51624979590af83272f96cbcfc521b7d0a))
 * **Uncategorized**
    - Release fontspector-checkapi v1.1.1, fontspector-profile-opentype v1.2.0, fontspector-profile-googlefonts v1.1.1, fontspector-profile-universal v1.1.1, fontspector v1.2.0 ([`f407a9a`](https://github.com/fonttools/fontspector/commit/f407a9aaf0aae501443842311f1b5c27eab007b6))
</details>

## v1.1.0 (2025-06-19)

<csr-id-f44be5515dcaea17b96b1df7a4b11407561d0c17/>

### Chore

 - <csr-id-f44be5515dcaea17b96b1df7a4b11407561d0c17/> Improve error handling
   * chore: Better error handling
   
   * chore: Better error handling for fix functions too

### New Features

 - <csr-id-ea5107c15304c96b035aab80551ce8ddb7e7e98b/> Add fix function
   * chore(api): add_table is not polymorphic so this idea just didn't work
* feat(cjk_vertical_metrics): Add fix function

### Bug Fixes

 - <csr-id-bc48fb1bc721e5b8b59780900e75c6e3ed177409/> Move to tabled to display tables correctly

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 22 calendar days.
 - 30 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#137](https://github.com/fonttools/fontspector/issues/137), [#275](https://github.com/fonttools/fontspector/issues/275), [#280](https://github.com/fonttools/fontspector/issues/280)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#137](https://github.com/fonttools/fontspector/issues/137)**
    - Move to tabled to display tables correctly ([`bc48fb1`](https://github.com/fonttools/fontspector/commit/bc48fb1bc721e5b8b59780900e75c6e3ed177409))
 * **[#275](https://github.com/fonttools/fontspector/issues/275)**
    - Improve error handling ([`f44be55`](https://github.com/fonttools/fontspector/commit/f44be5515dcaea17b96b1df7a4b11407561d0c17))
 * **[#280](https://github.com/fonttools/fontspector/issues/280)**
    - Add fix function ([`ea5107c`](https://github.com/fonttools/fontspector/commit/ea5107c15304c96b035aab80551ce8ddb7e7e98b))
 * **Uncategorized**
    - Release fontspector-checkapi v1.1.0, fontspector-profile-opentype v1.1.0, fontspector-profile-googlefonts v1.1.0, fontspector-profile-universal v1.1.0 ([`b126546`](https://github.com/fonttools/fontspector/commit/b12654669b361af01b98615c288f3bb816cfe0f6))
    - Release fontspector-profile-opentype v1.1.0, fontspector-profile-googlefonts v1.1.0, fontspector-profile-universal v1.1.0 ([`7cd073b`](https://github.com/fonttools/fontspector/commit/7cd073b65714dc47fc6a007308bc7f466612010c))
</details>

## v1.0.2 (2025-05-19)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 7 calendar days.
 - 11 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#107](https://github.com/fonttools/fontspector/issues/107), [#111](https://github.com/fonttools/fontspector/issues/111)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#107](https://github.com/fonttools/fontspector/issues/107)**
    - Move to fontations crate ([`da2830b`](https://github.com/fonttools/fontspector/commit/da2830ba694bf3379142a81dad043031e1c39f35))
 * **[#111](https://github.com/fonttools/fontspector/issues/111)**
    - Include most of the fixes from gftools-fix ([`2de6875`](https://github.com/fonttools/fontspector/commit/2de68751c8c4da8c29f9e46d444280cdf478c6b2))
 * **Uncategorized**
    - Release fontspector-profile-universal v1.0.2 ([`768ee53`](https://github.com/fonttools/fontspector/commit/768ee534822c8a8a9bce8cc6f80e8bc0a702d0ed))
    - Release fontspector-profile-googlefonts v1.0.2, fontspector-profile-universal v1.0.2 ([`d26f105`](https://github.com/fonttools/fontspector/commit/d26f105058189e6baa1ccd726d2151851e4e7d85))
    - Release fontspector-profile-opentype v1.0.2, fontspector-profile-googlefonts v1.0.2, fontspector-profile-universal v1.0.2 ([`fd2d019`](https://github.com/fonttools/fontspector/commit/fd2d0197d3918c1f74890b69fed4fe49ac8a3408))
</details>

## v1.0.1 (2025-05-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#105](https://github.com/fonttools/fontspector/issues/105)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#105](https://github.com/fonttools/fontspector/issues/105)**
    - Fix font_version parsing ([`cbf7b4b`](https://github.com/fonttools/fontspector/commit/cbf7b4bdd0cc30ddda32c919cbbae9a5e0e09cd2))
 * **Uncategorized**
    - Release fontspector-checkhelper v1.0.1, fontspector-profile-opentype v1.0.1, fontspector-profile-googlefonts v1.0.1, fontspector-profile-universal v1.0.1 ([`6ee7aed`](https://github.com/fonttools/fontspector/commit/6ee7aeda28e6961710b748e346cc1cc8c3e26b82))
    - Add changelogs ([`8b511ed`](https://github.com/fonttools/fontspector/commit/8b511eda27d0f3c7bb9e1f21d9749585e35c2fce))
</details>

## v1.0.0 (2025-05-08)

<csr-id-7cc0e15f42ffbf1d512f2fa50d42fe12ba3aca44/>
<csr-id-d7968d62b6271d79869a3ebf34c1d20365482c6c/>

### Other

 - <csr-id-7cc0e15f42ffbf1d512f2fa50d42fe12ba3aca44/> move to Universal profile
   Still needs to be ported, though.
   
   (https://github.com/fonttools/fontbakery/pull/4937)
 - <csr-id-d7968d62b6271d79869a3ebf34c1d20365482c6c/> sync with latest fontbakery

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 212 commits contributed to the release over the course of 301 calendar days.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #102 from fonttools/release-prep ([`e5435f4`](https://github.com/fonttools/fontspector/commit/e5435f4ab282338ccc818daca8dacf543de27022))
    - Prepare for release ([`7bc8c9f`](https://github.com/fonttools/fontspector/commit/7bc8c9ff57581c560d64c092c210255ab47247b7))
    - Read profile cargo files for release ([`5fe1c5a`](https://github.com/fonttools/fontspector/commit/5fe1c5aff636944c257ec25b19004426660db0c2))
    - Prep for 1.0.0 release ([`c1ef822`](https://github.com/fonttools/fontspector/commit/c1ef822c860b8dd53b363c9b69201981c75f757c))
    - Merge pull request #99 from fonttools/rich-metadata ([`dfd2c49`](https://github.com/fonttools/fontspector/commit/dfd2c49e542a5c5def5929c6c5e5dbd30e5015bb))
    - Stupid newline fix ([`1a30fa0`](https://github.com/fonttools/fontspector/commit/1a30fa05c32bf1e6cf0bfbf2f274a09f8170772e))
    - Merge pull request #96 from fonttools/non-ink-characters ([`1577008`](https://github.com/fonttools/fontspector/commit/15770084eaa140071658b5b6157ceb8174c8eb3a))
    - Move AnythingPen -> HasInkPen ([`3ca0531`](https://github.com/fonttools/fontspector/commit/3ca05318046d24109aa3404c9158c55ed9293159))
    - Merge pull request #88 from fonttools/reduce-false-positives ([`dcf298d`](https://github.com/fonttools/fontspector/commit/dcf298d93ad3abe68d4f520f8e980914eb74c008))
    - Only warn once per feature ([`18e2ccd`](https://github.com/fonttools/fontspector/commit/18e2ccd33a5ff938ffa30a0bb50f0c1bde080fe0))
    - Merge pull request #80 from fonttools/dependency-hell ([`b8ec37d`](https://github.com/fonttools/fontspector/commit/b8ec37d7d52f440fc2d6a9470ee2d3056df2d94c))
    - Reformat ([`ab0a4e4`](https://github.com/fonttools/fontspector/commit/ab0a4e4a5bbd316783438d0337782090a03e0a3f))
    - Use skrifa::raw instead of read_fonts, pin deps ([`76eacb7`](https://github.com/fonttools/fontspector/commit/76eacb755b79772e761b832b8fe8983af81e07fa))
    - Merge pull request #63 from LuxxxLucy/lucy-multiple-proposal-br ([`2d675d5`](https://github.com/fonttools/fontspector/commit/2d675d5bfe5cdb3de99e1a2cf8c65964c144bc52))
    - Add fontwerk check ([`c01b612`](https://github.com/fonttools/fontspector/commit/c01b612302e66f7da9d59e33f42f84e8ed345aea))
    - Unreachable_subsetting ([`3bd965f`](https://github.com/fonttools/fontspector/commit/3bd965f6874955e4a45fb782f50eaff3a6862997))
    - Warn about decomposing components in variable font ([`6984523`](https://github.com/fonttools/fontspector/commit/6984523892f273e660d0ab5f9af165b30de5fff8))
    - Allow for chaining hotfixes ([`9543e3d`](https://github.com/fonttools/fontspector/commit/9543e3da857864027bc6e69d86b52b2d6fd4500b))
    - Handle transforms correctly when decomposing ([`2ee4431`](https://github.com/fonttools/fontspector/commit/2ee44315895bac888a0520e1d6654d3e556e46df))
    - Make nested components fix depth first (still other problems to fix here) ([`0463a58`](https://github.com/fonttools/fontspector/commit/0463a586161221f4498b96df0068b1a97217ca13))
    - Update the checks ([`4110dcf`](https://github.com/fonttools/fontspector/commit/4110dcfd1c79131aea9893523b50e0b0bdfd2f95))
    - Hotfix nested components ([`f928774`](https://github.com/fonttools/fontspector/commit/f928774222d364f0fa48428fa16725139a282f54))
    - Latest read-fonts API ([`47a5310`](https://github.com/fonttools/fontspector/commit/47a531036503433ae38f78ef4fad98cf76536bf7))
    - Redo the way configuration files work to match fontbakery ([`bf2dac6`](https://github.com/fonttools/fontspector/commit/bf2dac6551472828c04519afe502440f870945f0))
    - Transform decomposed components hotfix ([`53583ba`](https://github.com/fonttools/fontspector/commit/53583ba214dcfbaac529391df056a5692d8ce4bb))
    - Use namecheck API ([`2be2e5d`](https://github.com/fonttools/fontspector/commit/2be2e5d8853687a1ae66916ad1d17713b5e6e302))
    - New clippy found new lints! ([`1933d0a`](https://github.com/fonttools/fontspector/commit/1933d0a7835610c4c59e2ca272696789320992e9))
    - Run cargo fmt ([`a97b2a9`](https://github.com/fonttools/fontspector/commit/a97b2a96d2ffbf6fab861b842096159d666a4dc9))
    - Neater repo organization (universal profile) ([`20504e5`](https://github.com/fonttools/fontspector/commit/20504e5488f0a59a01a6812f6cbb7cbec5c3d59b))
    - Hashbrown is just faster ([`dbf0907`](https://github.com/fonttools/fontspector/commit/dbf09072a4541938696440ef7a394dbc644146fc))
    - Oops missing checks ([`56a1792`](https://github.com/fonttools/fontspector/commit/56a1792cf3fed7fbc27b0c80725fa44585e0f10e))
    - Use profile builder in Universal profile ([`d42afd6`](https://github.com/fonttools/fontspector/commit/d42afd60144ed575f26cddcb42f4ba77d8bd4314))
    - Move alt_caron to universal ([`d3f4035`](https://github.com/fonttools/fontspector/commit/d3f4035c1b67c7749600d124e5e97f7fd0f19d2c))
    - Share some crates, add axis registry ([`683ec0e`](https://github.com/fonttools/fontspector/commit/683ec0eeb3a0b1d34fc13c4935d448489be0fd58))
    - Vttclean is now merged into unwanted_tables ([`1fbf4f8`](https://github.com/fonttools/fontspector/commit/1fbf4f841c2dc7dcb87c77bc81a956caddbc8e66))
    - Left to port: 'overlapping_path_segments' ([`8d4c2ea`](https://github.com/fonttools/fontspector/commit/8d4c2ea1e8504c4d69e75af2f219ba383c26d485))
    - Move to Universal profile ([`7cc0e15`](https://github.com/fonttools/fontspector/commit/7cc0e15f42ffbf1d512f2fa50d42fe12ba3aca44))
    - Varfont/duplexed_axis_reflow was already ported ([`8da0603`](https://github.com/fonttools/fontspector/commit/8da060360bc2d3709d671e7319bca281e7e174aa))
    - Googlefonts/varfont/bold_wght_coord => varfont/bold_wght_coord ([`c040fe0`](https://github.com/fonttools/fontspector/commit/c040fe032d87f2db4f2546f2b9ae53fc524c4181))
    - Migrate render_own_name to GoogleFonts profile ([`220d710`](https://github.com/fonttools/fontspector/commit/220d71044c2ee91c7ff7b78b71231b04a4e3bdcb))
    - No_debugging_tables was merged into unwanted_tables ([`03880af`](https://github.com/fonttools/fontspector/commit/03880af0747dff248a689cd085da1860ba6bdbb9))
    - [name/italic_names] on Universal profile seems to be Google Fonts specific ([`c9408e9`](https://github.com/fonttools/fontspector/commit/c9408e9fff0c0429c5fc3b4882560f45fe66b5fa))
    - Sort ([`48c0b78`](https://github.com/fonttools/fontspector/commit/48c0b7839da8e5b581ee022cabb0e638facfb81f))
    - Stat_has_axis_value_tables / inconsistencies_between_fvar_STAT ([`ed2aa43`](https://github.com/fonttools/fontspector/commit/ed2aa43f514c6f340a8db6a5b9924f4ebd431c55))
    - Opentype/gpos_kerning_info => gpos_kerning_info (Universal profile) ([`854af14`](https://github.com/fonttools/fontspector/commit/854af14cacb2c785ac5f27e8fc64bd2b8c966743))
    - Rename glyf_nested_components to nested_components ([`e3ec242`](https://github.com/fonttools/fontspector/commit/e3ec242efc48a0b936333e38078e47e90fd184c9))
    - Sync with latest fontbakery ([`d7968d6`](https://github.com/fonttools/fontspector/commit/d7968d62b6271d79869a3ebf34c1d20365482c6c))
    - Use cache to determine codepoints in font ([`0514efc`](https://github.com/fonttools/fontspector/commit/0514efcf5e99d3c157fad5795816183d8f84e091))
    - Hey, unique_glyphnames is unnecessary now ([`70db0ef`](https://github.com/fonttools/fontspector/commit/70db0eff4f05e27f3b69500e49b3598c0eb6a54a))
    - Remove noise ([`bbd6894`](https://github.com/fonttools/fontspector/commit/bbd6894ea17cf58f986826acf846b0cb3048dd51))
    - Expose the checks so docs can be built ([`911e557`](https://github.com/fonttools/fontspector/commit/911e55758ecf2fc7c21f62eb470f0d42c9668723))
    - Another super-long title ([`85c016b`](https://github.com/fonttools/fontspector/commit/85c016b610c66e7a35d89700114496e0609c5be7))
    - Don't use cmap manually, use skrifa ([`b2357df`](https://github.com/fonttools/fontspector/commit/b2357dfa77de6b2e1016dc5d812c036b929cf156))
    - Duplexed_axes_reflow urgh urgh urgh ([`d5ed74b`](https://github.com/fonttools/fontspector/commit/d5ed74b8bfea97aebfed0fa7e6b74e0237e5d424))
    - Isolate the *madness* behind an API function ([`8235ad1`](https://github.com/fonttools/fontspector/commit/8235ad10aa928c5d45e0bcc0f4b4b0ef09ee17ae))
    - Give up on that check ([`eaa5244`](https://github.com/fonttools/fontspector/commit/eaa52447ddc4a42e26b6430841a43026870d8a48))
    - Ligature carets ([`907d865`](https://github.com/fonttools/fontspector/commit/907d86521eb790c234ca8b471753549c400fdfad))
    - Contour_count ([`c4b7e2b`](https://github.com/fonttools/fontspector/commit/c4b7e2bd20bcb691b9dd431eedf8c2be962fc6ba))
    - Reorg profile ([`2e97ee1`](https://github.com/fonttools/fontspector/commit/2e97ee14fda61b24d055ad6f8cbd6c8a951189c9))
    - No fontdata_namecheck for WASM ([`3b0e5f0`](https://github.com/fonttools/fontspector/commit/3b0e5f03b889f4710c583c74ca46122312866260))
    - Wrong title ([`f3a66dc`](https://github.com/fonttools/fontspector/commit/f3a66dc110237463ab5bd25992ebcad49d20cfa0))
    - Fontdata_namecheck ([`669b9ad`](https://github.com/fonttools/fontspector/commit/669b9adc66079ae21bea5f9754c1e304cda62d17))
    - Tabular_kerning ([`8253254`](https://github.com/fonttools/fontspector/commit/8253254bbd099e1c4aee1f5ae63f749fc30b27d9))
    - Varfont_instances_in_order ([`b41e00a`](https://github.com/fonttools/fontspector/commit/b41e00a9d38998f8372bc487e8046989319436b2))
    - File_size ([`a74c5e4`](https://github.com/fonttools/fontspector/commit/a74c5e401c4f588dc27fa0a4cb8b839500c1b80d))
    - Vtt_volt_data, reorg profile ([`0de3e26`](https://github.com/fonttools/fontspector/commit/0de3e268cf9ecfd5c151c76add3e6d2755750d15))
    - Smallcaps_before_ligatures ([`b118298`](https://github.com/fonttools/fontspector/commit/b118298fcb50315f4c4b52132ed140b3f08b5673))
    - Missing_small_caps_glyphs implementation ([`2c585f4`](https://github.com/fonttools/fontspector/commit/2c585f41a61eab465923d584ed8a152c749e1404))
    - Use new GetSubstitutionMap trait ([`d43a5e2`](https://github.com/fonttools/fontspector/commit/d43a5e2945fbbf9ceb0ee1d6a53ec09e109e77d3))
    - Unreachable glyphs check ([`9d227ef`](https://github.com/fonttools/fontspector/commit/9d227efee1d5f39130813158c1be52f602b166e9))
    - Reorder profile ([`1c6cfeb`](https://github.com/fonttools/fontspector/commit/1c6cfeb9f7211965ad25edd8a8980f1c467e846f))
    - Improve reporting ([`6d9c838`](https://github.com/fonttools/fontspector/commit/6d9c8386979b699fd8da4ee46a0cf65ddc341bcc))
    - More dehinting ([`3edb308`](https://github.com/fonttools/fontspector/commit/3edb3086741e084900986211a8d1bb6e74d04331))
    - Bit more progress but stymied by the API ([`0512dbb`](https://github.com/fonttools/fontspector/commit/0512dbb630c1f46ce0749da5f37b5426bce61147))
    - Don't freetype on wasm ([`2e8740a`](https://github.com/fonttools/fontspector/commit/2e8740a79574151040092f41e19f18783127ac9f))
    - Freetype_rasterizer ([`faa2aba`](https://github.com/fonttools/fontspector/commit/faa2aba767a5404ef1cd5d2850b74d02eaa26a18))
    - Empty_letters ([`51b0b33`](https://github.com/fonttools/fontspector/commit/51b0b337b08ddd4391df6dbea58aa59a9eaf8be5))
    - Hinting_impact ([`5537d2a`](https://github.com/fonttools/fontspector/commit/5537d2a8245805bc2eaf41d70db8f55fcfcbdec9))
    - Legacy_accents ([`bedab08`](https://github.com/fonttools/fontspector/commit/bedab08451ca212e82560a75dc5831506f9810fc))
    - Math_signs_width ([`0a47714`](https://github.com/fonttools/fontspector/commit/0a47714d0fee21fab0149c0d1e82f7c08a45c657))
    - Rearrange some checks ([`d271eb0`](https://github.com/fonttools/fontspector/commit/d271eb022dfbf27ee90827ab01a10a1b5b23c3ce))
    - More micro-optimizations ([`7886854`](https://github.com/fonttools/fontspector/commit/788685487526a9d2d10a4b4466c59ebe307bd432))
    - Optimizations ([`2c52e34`](https://github.com/fonttools/fontspector/commit/2c52e3460b55d399054917b42b8a5c0eeb6a4ea7))
    - Optimizations ([`dc71848`](https://github.com/fonttools/fontspector/commit/dc7184813e71e56c302d84bb18a06f9ae37747c8))
    - Empty_glyph_on_gid1_for_colrv0 ([`483c66c`](https://github.com/fonttools/fontspector/commit/483c66ce8aca43af4d40ef4e649490176e03eff6))
    - Cjk_not_enough_glyphs ([`b40cc36`](https://github.com/fonttools/fontspector/commit/b40cc3684954fdc1c134f84f83a6b963c2900479))
    - Arabic_high_hamza ([`7b794ca`](https://github.com/fonttools/fontspector/commit/7b794ca4e377286722f2fb5724ccc4c05271461f))
    - Export a DEFAULT_LOCATION setting ([`1ab59e9`](https://github.com/fonttools/fontspector/commit/1ab59e9064181e168765ea3f6cab9d8a28ddac5c))
    - Move all pens to a utility module in checkapi ([`8f86fd5`](https://github.com/fonttools/fontspector/commit/8f86fd56087c660943f39957d5471d865d2755fd))
    - Tidy up universal profile ([`595db25`](https://github.com/fonttools/fontspector/commit/595db25e1f4804b91a28db3905248b328fdbd3f7))
    - Ckj_chws_feature ([`d79455a`](https://github.com/fonttools/fontspector/commit/d79455a0132e072bad3a08cd67acb715e801f375))
    - Fix warning about unused import ([`58660c1`](https://github.com/fonttools/fontspector/commit/58660c15d1f0f776efbd666e4f56ac4cb0977148))
    - Mandatory_glyphs (just notdef these days) ([`f041dee`](https://github.com/fonttools/fontspector/commit/f041deebbba37f3ca2403ed37e6e4b3b1149d485))
    - Typoascender_exceeds_agrave ([`cf38b4d`](https://github.com/fonttools/fontspector/commit/cf38b4d8b29043734100d9906af76c3b24473d55))
    - Smart_dropout ([`378482a`](https://github.com/fonttools/fontspector/commit/378482abb37f7bceea31ab9d424e3192d0f81f47))
    - Gpos7 ([`9448970`](https://github.com/fonttools/fontspector/commit/94489707017b9877b7c147d2e2df490328b74070))
    - Family and style max length ([`6a3a2df`](https://github.com/fonttools/fontspector/commit/6a3a2df4a1e8329c6e815370c5225de25a508f97))
    - Family_vertical_metrics ([`3e2be97`](https://github.com/fonttools/fontspector/commit/3e2be970c57992e566f1e95a90b9f43345cd14ad))
    - STAT_in_statics ([`d54f0df`](https://github.com/fonttools/fontspector/commit/d54f0dfb6d23161ff2bbb532cbd3949ba3c275ff))
    - Stat_strings ([`d8aea2f`](https://github.com/fonttools/fontspector/commit/d8aea2fb3891e4dfb66a0cc56610be7a95cdf020))
    - Family_win_ascent_and_descent ([`4eaddac`](https://github.com/fonttools/fontspector/commit/4eaddac11088b9b963e64acbe05778f8a0dc6299))
    - Update check to new API ([`22e9516`](https://github.com/fonttools/fontspector/commit/22e9516358cf82b4eb57dc9ac50106753451161c))
    - Dependency hell ([`83ec39e`](https://github.com/fonttools/fontspector/commit/83ec39e3a9cd61d63f8b3b6ce977dd66870283ce))
    - New “base has width” check ([`564e18c`](https://github.com/fonttools/fontspector/commit/564e18c8deb779f47474411927e6bdbe3427500b))
    - Reorder check list, reflecting recently ported checks ([`440d664`](https://github.com/fonttools/fontspector/commit/440d6645ad55917051c3535419a9cfb041392198))
    - Glyf_nested_components ([`e6f05b3`](https://github.com/fonttools/fontspector/commit/e6f05b316546036701df66c9d99067f50ea97601))
    - Color check tests ([`7b0d4a8`](https://github.com/fonttools/fontspector/commit/7b0d4a86b29570db2ef6baa5db2accdc58e99a27))
    - Name char restrictions ([`8ca7f3f`](https://github.com/fonttools/fontspector/commit/8ca7f3f03acdffc0fb2fad39d886b8b3aef732bd))
    - No mac entries ([`de8a783`](https://github.com/fonttools/fontspector/commit/de8a7835e16dc08505416798580248ba268305e9))
    - Two color tests ([`6830f15`](https://github.com/fonttools/fontspector/commit/6830f15d8f9268f4125850a8687f8fd109e955b6))
    - Unique_glyphnames ([`845f537`](https://github.com/fonttools/fontspector/commit/845f53797855d0a086e96adc90980a5142461472))
    - Oops missing ([`7343b74`](https://github.com/fonttools/fontspector/commit/7343b74b890c3cd54d1d83a0b2e841a4b2b0a348))
    - Cmap format 12 ([`9ba1a0e`](https://github.com/fonttools/fontspector/commit/9ba1a0e27a6c0a41ed7ba1703b04a3c3007102df))
    - Control chars ([`34fe8a8`](https://github.com/fonttools/fontspector/commit/34fe8a8426a79334e4b6377e84b952075ce946b7))
    - Typographic_family_name ([`f7cf15a`](https://github.com/fonttools/fontspector/commit/f7cf15ad462cc66e5647ffca9d1c5894eadca391))
    - Linegaps ([`1e2e501`](https://github.com/fonttools/fontspector/commit/1e2e501163c10ff27fe5edcd502dafe6e076bbfb))
    - Integer_ppem_if_hinted ([`7a41ac8`](https://github.com/fonttools/fontspector/commit/7a41ac8ef9a2cfbb66dc76e4f743970da7ba2fcb))
    - New check: unwanted_aat_tables ([`c692652`](https://github.com/fonttools/fontspector/commit/c6926527db87518ed1c8a106ced717fc74572df4))
    - Update legacy checks proposal field ([`ad3861e`](https://github.com/fonttools/fontspector/commit/ad3861e292ef2e1cbf118d5df8329c961123c90c))
    - Add interpolation issues check ([`7671c6b`](https://github.com/fonttools/fontspector/commit/7671c6bc9c045ff6842356ba5437d48ae3f3d313))
    - Os2_metrics_match_hhea ([`709ff4c`](https://github.com/fonttools/fontspector/commit/709ff4c79c9612a2df9abf89257dcde2e025c62a))
    - Soft_hyphen ([`b4cf4b5`](https://github.com/fonttools/fontspector/commit/b4cf4b5066eb5c861fa9e7253018ba1e95c3f7df))
    - Sfnt_version ([`000987b`](https://github.com/fonttools/fontspector/commit/000987bf46bba698c5558dda113b1629d89b3660))
    - Four more checkcs ([`8970345`](https://github.com/fonttools/fontspector/commit/8970345d53d7d388fa696be118ff74a870b29e0a))
    - More unwanted tables ([`38ea16d`](https://github.com/fonttools/fontspector/commit/38ea16dbb08a59c015e5c1d226dcb7ea185c6245))
    - Two more checks ([`567d91a`](https://github.com/fonttools/fontspector/commit/567d91a87f6e410d7927c6b66c1f5aa21e5afaf0))
    - Add stylistic set check ([`9e54b7a`](https://github.com/fonttools/fontspector/commit/9e54b7a8c61349c7b9698ca4a35b50a21744fb97))
    - Share itertools versions ([`71e6f81`](https://github.com/fonttools/fontspector/commit/71e6f81d35e3fbe8540a38ec532e382effa87459))
    - More tests ([`c0a40cd`](https://github.com/fonttools/fontspector/commit/c0a40cdb6b0c6bdf9b69ab807d99f69c0f9a1ea1))
    - Case mapping check ([`18d34bb`](https://github.com/fonttools/fontspector/commit/18d34bbf5e3902448e99d009d8e90a9fc9de95f0))
    - Whitespace widths check ([`11891cd`](https://github.com/fonttools/fontspector/commit/11891cd5e31e868945919759525b7a10cab8adb3))
    - Add whitespace ink check ([`1341cd5`](https://github.com/fonttools/fontspector/commit/1341cd5560280623016ddd55d15d82b28c1bb817))
    - Everyone gets unicode-properties! ([`6218042`](https://github.com/fonttools/fontspector/commit/621804215cf361f7d515be71b71e8bd84bc481d7))
    - Bump read/write/skrifa versions, dump font-types, deal with fallout ([`d2fd7e4`](https://github.com/fonttools/fontspector/commit/d2fd7e4be7f70b014776c6a56ec035b5156692c0))
    - Improve glyph name API - move unwraps into API lib ([`2a094be`](https://github.com/fonttools/fontspector/commit/2a094bea6bbe22e15320c521aebbe493f3bb4c3c))
    - Use read-fonts' glyph class constants ([`3c41053`](https://github.com/fonttools/fontspector/commit/3c41053289a71d555710a66acc7cfc61cc2402ab))
    - More passes ([`d61590b`](https://github.com/fonttools/fontspector/commit/d61590b39cc724ef546ff66ee5753c2a3d6815e3))
    - Rupee check ([`b720c74`](https://github.com/fonttools/fontspector/commit/b720c74f60efef8a183abb1457f31db3ed2a6002))
    - Move 'name/italic_names' to Universal profile. ([`0ce2aef`](https://github.com/fonttools/fontspector/commit/0ce2aef4e63fb53b278176bcb6516f86b6117e93))
    - Move 'name/no_copyright_on_description' to Universal profile. ([`0eaff9d`](https://github.com/fonttools/fontspector/commit/0eaff9d86043c79b29a3035722e1520fd67e6644))
    - Some leftovers from universal/opentype split ([`c71f0da`](https://github.com/fonttools/fontspector/commit/c71f0da94f981a4bc69bda022ad1a2039a17f0d9))
    - Merge pull request #15 from felipesanches/issue_14 ([`57a2274`](https://github.com/fonttools/fontspector/commit/57a2274c13a2ac02292eaf60ec37f7cb63098304))
    - Split profiles Universal and OpenType ([`72550af`](https://github.com/fonttools/fontspector/commit/72550af9c9c8f9a9f4dad37a52f789290b4f6fb8))
    - Several more checks ([`9512dfd`](https://github.com/fonttools/fontspector/commit/9512dfded101ee67c6c7413109db97517a783826))
    - Three more checks ([`35db31f`](https://github.com/fonttools/fontspector/commit/35db31f26fdf3640a5be7397e97bce6b5dd48906))
    - Weighted average check ([`4fe626b`](https://github.com/fonttools/fontspector/commit/4fe626bf621b1fb7dd8944289778eb5368a50bc0))
    - Fix to float parsing ([`0633a88`](https://github.com/fonttools/fontspector/commit/0633a88d5d3396600fe3908ada0608095c050579))
    - Rustfmt ([`44db2b0`](https://github.com/fonttools/fontspector/commit/44db2b0f70cf4ed96b1f43b7368d54a41fabccba))
    - Another one ([`1c082a3`](https://github.com/fonttools/fontspector/commit/1c082a37ad1ff30439dc45c9d990889d3cb66a8b))
    - Fix check ID ([`8a18ce0`](https://github.com/fonttools/fontspector/commit/8a18ce055d4fdab50d13cbc14e55d5eb9ed7a91b))
    - Error fixups ([`c5fba48`](https://github.com/fonttools/fontspector/commit/c5fba480c620ad99f9e4842900dc6879070a09a7))
    - Another check ([`172fea4`](https://github.com/fonttools/fontspector/commit/172fea494a2aef8530c9418c17f3a45d14ee6544))
    - A bunch more checks ([`c47194b`](https://github.com/fonttools/fontspector/commit/c47194b6132888d7a6e2372aff68c430dc909ffe))
    - Slant direction check ([`174c9a9`](https://github.com/fonttools/fontspector/commit/174c9a9831ae1476ee9ff89de1d9360a2aba0ab3))
    - Plus register some checks we forgot... ([`c52ca71`](https://github.com/fonttools/fontspector/commit/c52ca7144bae331e214e957151b06affdc9c16a4))
    - Another stat check ([`2fe9d95`](https://github.com/fonttools/fontspector/commit/2fe9d95670a75a10fbf2e124d5e4342cdf8011b3))
    - Mild speed improvement ([`c65ba86`](https://github.com/fonttools/fontspector/commit/c65ba8611d3f2d6e2a609cb0b0b2e5cde8a3bf24))
    - Rework Python bridge ([`e357d73`](https://github.com/fonttools/fontspector/commit/e357d73000b82b71ee93f28f71c5b16c5ca819d1))
    - Port another seven opentype checks ([`f11d58a`](https://github.com/fonttools/fontspector/commit/f11d58a7569cf32a15091880901923c49b62d534))
    - Empty result is a pass ([`c4cff3b`](https://github.com/fonttools/fontspector/commit/c4cff3b51774733b19e6cb83d9e6390e75ef284d))
    - Merge pull request #10 from felipesanches/more_checks_2024_sep_20 ([`8cfb898`](https://github.com/fonttools/fontspector/commit/8cfb898458a69666f439676be4d02e7f115bf7a0))
    - Code-tests for opentype/code_pages ([`77f0008`](https://github.com/fonttools/fontspector/commit/77f00085df2b7422c22f2f1e19707d2f2957065d))
    - Added code-tests for opentype/name/empty_records ([`432d0e3`](https://github.com/fonttools/fontspector/commit/432d0e3b9b47ab719499d7d13da28cf7976a6826))
    - Comment out unfinished check ([`cf856a1`](https://github.com/fonttools/fontspector/commit/cf856a183aa29344ef67384068b6f894998fb819))
    - Fixup ([`d5389fb`](https://github.com/fonttools/fontspector/commit/d5389fba16ed6dacea06ffba4487da12dc3db736))
    - Some name checks ([`12a4163`](https://github.com/fonttools/fontspector/commit/12a4163175d185d20568a982d6045a96f8a187ee))
    - Glyf table checks ([`0ef8110`](https://github.com/fonttools/fontspector/commit/0ef81104a58d51bf4c1adc959e45240e7d6aaaec))
    - Panose_familytype ([`2d10caf`](https://github.com/fonttools/fontspector/commit/2d10cafd4f35b31cdc2559d5f6078b174cc89fea))
    - Clippy ([`1a9d3b4`](https://github.com/fonttools/fontspector/commit/1a9d3b44c85bafd9c4e3ca903d7b07a8b037f639))
    - Merge pull request #9 from felipesanches/a_few_additional_ports_of_checks ([`16bc0b0`](https://github.com/fonttools/fontspector/commit/16bc0b01713e1ace3cf5aade9415614a2a39c488))
    - Split the registering of checks between opentype and universal profiles ([`a6a5e35`](https://github.com/fonttools/fontspector/commit/a6a5e3553aa6f1cc4f0a37a61334af984a6dc155))
    - New check: 'opentype/code_pages' ([`a2eb17c`](https://github.com/fonttools/fontspector/commit/a2eb17c953cf8fc634ca496b89d0a00dee747d36))
    - More checks! ([`65d31a3`](https://github.com/fonttools/fontspector/commit/65d31a31feea5b48e853d1962b2c122b65d79a6f))
    - Implement three more checks ([`6264892`](https://github.com/fonttools/fontspector/commit/6264892c82030579f178ca5421f36811589b0a86))
    - Merge pull request #6 from felipesanches/new_check_ids ([`4fdc7c5`](https://github.com/fonttools/fontspector/commit/4fdc7c52a7582dbc984f89d8d0b35f6a58748cbd))
    - Reorganization of profiles ([`d48de7a`](https://github.com/fonttools/fontspector/commit/d48de7a018f9b46ebf44fc03f6a3ce3d4ae486c4))
    - Update check-ID following FontBakery's new naming scheme ([`64e3e5d`](https://github.com/fonttools/fontspector/commit/64e3e5d452fec3f6c86cff9f34e33816951af3d5))
    - Another stat check ([`4ab581e`](https://github.com/fonttools/fontspector/commit/4ab581eb8cfeb8aebd9f7e3110b1334d7f5a2874))
    - Merge pull request #2 from felipesanches/new_check_arabic_spacing_symbols ([`e49cfed`](https://github.com/fonttools/fontspector/commit/e49cfed72bf775ee70d0abce5621a33c5a1cd299))
    - Simplify ([`c19cb45`](https://github.com/fonttools/fontspector/commit/c19cb45d6cb7a19c7272a81b00d3da831f4cd2bd))
    - Syntax and type fixes ([`d5082b2`](https://github.com/fonttools/fontspector/commit/d5082b25bb24c7dc5d811d670aa1f9c05f8b21c1))
    - Run rustfmt ([`fd0cbff`](https://github.com/fonttools/fontspector/commit/fd0cbffdbd2cc883e873f07cc0fad2ed4a7b6ba7))
    - Check that Arabic spacing symbols aren't classified as marks ([`dd4af2c`](https://github.com/fonttools/fontspector/commit/dd4af2c5e4631c1a1cba8815bb7368b346c23d8e))
    - A couple more checks ([`b8c42b4`](https://github.com/fonttools/fontspector/commit/b8c42b42a140065f264918c7ad1e9e8f42b128a5))
    - Move to the hellish procmacro ([`20d9a48`](https://github.com/fonttools/fontspector/commit/20d9a48838d57250cac9e84c8d7e00ac6359b4bd))
    - More universal/opentype checks ([`f5750bd`](https://github.com/fonttools/fontspector/commit/f5750bdf9cdfcf5b1e5fefb76bc34a600046b488))
    - Fencepost error ([`8f4e609`](https://github.com/fonttools/fontspector/commit/8f4e60982e9fffd5aebb145205d352b9be478bec))
    - Regexes are slow, use optimised glyph name access ([`7ba0913`](https://github.com/fonttools/fontspector/commit/7ba09133812a73d425dd35b1536e1fbdd811bdd2))
    - Checks for correctness of axis ranges ([`fc1c923`](https://github.com/fonttools/fontspector/commit/fc1c9238904fcef076689da65f2f402e24393dfb))
    - Space name checks ([`8735cea`](https://github.com/fonttools/fontspector/commit/8735cea7aa45490624dc204901f57c88e8108077))
    - Valid/unique glyph names check ([`dcbe80f`](https://github.com/fonttools/fontspector/commit/dcbe80f504e1105813454f0b0ea9e0d23ca18c59))
    - Make check implementation (one/all) an enum ([`d57b5c8`](https://github.com/fonttools/fontspector/commit/d57b5c8a08433ecb0ac60330c35df94a91461541))
    - Improve error/skip story, add fvar regular coords check ([`c23b8b0`](https://github.com/fonttools/fontspector/commit/c23b8b0eae9f7f97a15c2d70092196ab1175fe9b))
    - Tidy up dependencies ([`395112f`](https://github.com/fonttools/fontspector/commit/395112f646b53d446dd082174026fa3ce381f095))
    - Make checks serializable, add check flags ([`c4996e0`](https://github.com/fonttools/fontspector/commit/c4996e08b590d3710763c117b99d9df61b631e3e))
    - Rearrange run result struct, add subresult codenames/severity ([`2d99a2b`](https://github.com/fonttools/fontspector/commit/2d99a2b760b43d7cdf4630800d25493e0d7485a1))
    - Add configuration and check context ([`caeb4b7`](https://github.com/fonttools/fontspector/commit/caeb4b7478a4a51bd5130fe85eb7043758e2236d))
    - Improve display ([`27c29fd`](https://github.com/fonttools/fontspector/commit/27c29fdfe1ee02e8dc337e9542c288ca93efc0cb))
    - Merge pull request #5 from felipesanches/rationales_not_optional ([`ee113d9`](https://github.com/fonttools/fontspector/commit/ee113d98a0cb146a764163c6afeacae05f0ece9f))
    - Merge branch 'main' into rationales_not_optional ([`37122c3`](https://github.com/fonttools/fontspector/commit/37122c334183fa689fbe4f5617b1ca24e6abb95c))
    - Be (slightly) more grown-up about error handling ([`2818a76`](https://github.com/fonttools/fontspector/commit/2818a764da76b9acc2c33127cb156238dca970c1))
    - Rationale and proposal fields are not optional ([`752d559`](https://github.com/fonttools/fontspector/commit/752d5593f3c5a345a781f8b76e5907607bda7dbd))
    - Built-in profiles shouldn't pluginate ([`71cea65`](https://github.com/fonttools/fontspector/commit/71cea651e8556fa0ab1e119b25c39c6b52f0d1bd))
    - Add has_table utility ([`b7f43d1`](https://github.com/fonttools/fontspector/commit/b7f43d1021693e7f87c273271df00c9e7941c14e))
    - Allow included profiles, make registering profile a Result ([`4d7a296`](https://github.com/fonttools/fontspector/commit/4d7a296a76c2717c895784d8d1e795a1740a3859))
    - Improve log messages ([`774a638`](https://github.com/fonttools/fontspector/commit/774a638bb974b087f48dbfbc624a0ea91b6ede6d))
    - Add fixes ([`248f457`](https://github.com/fonttools/fontspector/commit/248f457d99f5352940f287d2c75e2d8b540f7048))
    - Update fontread/write dependencies ([`83a2abc`](https://github.com/fonttools/fontspector/commit/83a2abcf0ce9c4a3a2fe6d3fd4fc5c28862a3824))
    - Make check registry a map ([`44aae7b`](https://github.com/fonttools/fontspector/commit/44aae7bdc987e6a01587fcfd38dabb5fdfdeadd8))
    - Use a prelude ([`fb66913`](https://github.com/fonttools/fontspector/commit/fb669139300ca7e671ee2af8b47ba8f9e6ccfdd3))
    - Tidy lots of things up, allow pluggable file types ([`1651816`](https://github.com/fonttools/fontspector/commit/1651816d634137e319925acb9dc33da66ccf38e9))
    - Rename workspace members ([`f97a39a`](https://github.com/fonttools/fontspector/commit/f97a39a80faf667006de20741f14e7736c5a966c))
</details>

