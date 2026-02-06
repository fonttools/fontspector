# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.4.0 (2026-02-06)

### Chore

 - <csr-id-19b91cdf15a6d8ea7e60b616199d28e03c76ba4d/> Reformat

### New Features

 - <csr-id-04a449bd24ff2519f5c0300cd013ffcdbf363bf7/> Add check (implements #557)
   * chore(checkapi): Add fontdrasil and axes function
   
   * feat(suspicious_sidebearings): Add check (implements #557)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 2 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#611](https://github.com/fonttools/fontspector/issues/611), [#618](https://github.com/fonttools/fontspector/issues/618)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#611](https://github.com/fonttools/fontspector/issues/611)**
    - Reformat ([`19b91cd`](https://github.com/fonttools/fontspector/commit/19b91cdf15a6d8ea7e60b616199d28e03c76ba4d))
 * **[#618](https://github.com/fonttools/fontspector/issues/618)**
    - Add check (implements #557) ([`04a449b`](https://github.com/fonttools/fontspector/commit/04a449bd24ff2519f5c0300cd013ffcdbf363bf7))
</details>

## v1.3.2 (2026-02-04)

<csr-id-c8bc460e7ee09451e8cb86ce9463fcbea3703c86/>

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

 - 2 commits contributed to the release.
 - 48 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#586](https://github.com/fonttools/fontspector/issues/586)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#586](https://github.com/fonttools/fontspector/issues/586)**
    - Port all remaining `required_table` tests to rust ([`c8bc460`](https://github.com/fonttools/fontspector/commit/c8bc460e7ee09451e8cb86ce9463fcbea3703c86))
 * **Uncategorized**
    - Release fontspector-checkapi v1.3.2, fontspector-profile-fontwerk v1.2.2, fontspector-profile-googlefonts v1.6.0, fontspector-profile-universal v1.5.0, fontspector-profile-iso15008 v1.0.5, fontspector v1.5.3 ([`43eb125`](https://github.com/fonttools/fontspector/commit/43eb125ce74b3424af90f5f9aa8082fdb4e2e742))
</details>

## v1.3.1 (2025-12-17)

<csr-id-4befd6c88900e2e06c363a8a6b1cdfc9518e9c91/>

### Chore

 - <csr-id-4befd6c88900e2e06c363a8a6b1cdfc9518e9c91/> More from rustybuzz to harfrust
   * chore: Move from rustybuzz to harfrust
   
   * test(googlefonts/shaping/forbidden): Move tests to Rust
   
   * test: Pass full config to tests
   
   * chore: Missing docstrings
   
   * chore: not my fail

### Bug Fixes

 - <csr-id-adc935eb2c6ef03e239cff57af2b2adc9344295d/> update rust crate scraper to 0.25.0

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 64 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#531](https://github.com/fonttools/fontspector/issues/531), [#551](https://github.com/fonttools/fontspector/issues/551)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#531](https://github.com/fonttools/fontspector/issues/531)**
    - More from rustybuzz to harfrust ([`4befd6c`](https://github.com/fonttools/fontspector/commit/4befd6c88900e2e06c363a8a6b1cdfc9518e9c91))
 * **[#551](https://github.com/fonttools/fontspector/issues/551)**
    - Update rust crate scraper to 0.25.0 ([`adc935e`](https://github.com/fonttools/fontspector/commit/adc935eb2c6ef03e239cff57af2b2adc9344295d))
 * **Uncategorized**
    - Release fontspector-checkapi v1.3.1, fontspector-profile-fontwerk v1.2.1, fontspector-profile-googlefonts v1.5.0, fontspector-profile-universal v1.4.0, fontspector v1.5.2 ([`2779526`](https://github.com/fonttools/fontspector/commit/2779526c00f235ea93e95882b4ebd2b41786c715))
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

 - <csr-id-7a7f2a6fe346597071b8099a481023b24afa094d/> Don't check for whitespace ink in format characters
   * fix(whitespace_ink): Don't check for whitespace ink in format characters
* chore: Update Cargo.lock

### Test

 - <csr-id-56e2f3f9167f15b2cb8cba8377403b8472514a7c/> Allow testing of collections

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 8 calendar days.
 - 29 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#472](https://github.com/fonttools/fontspector/issues/472), [#473](https://github.com/fonttools/fontspector/issues/473), [#478](https://github.com/fonttools/fontspector/issues/478), [#486](https://github.com/fonttools/fontspector/issues/486)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#472](https://github.com/fonttools/fontspector/issues/472)**
    - Adding new check ([`008b6fa`](https://github.com/fonttools/fontspector/commit/008b6fa8eab5ed5e9bb9cff5a6d4e51f822ce0be))
 * **[#473](https://github.com/fonttools/fontspector/issues/473)**
    - Adding new check ([`008b6fa`](https://github.com/fonttools/fontspector/commit/008b6fa8eab5ed5e9bb9cff5a6d4e51f822ce0be))
 * **[#478](https://github.com/fonttools/fontspector/issues/478)**
    - Allow testing of collections ([`56e2f3f`](https://github.com/fonttools/fontspector/commit/56e2f3f9167f15b2cb8cba8377403b8472514a7c))
 * **[#486](https://github.com/fonttools/fontspector/issues/486)**
    - Don't check for whitespace ink in format characters ([`7a7f2a6`](https://github.com/fonttools/fontspector/commit/7a7f2a6fe346597071b8099a481023b24afa094d))
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

 - <csr-id-dacd8d5d49e1c85774957133df73fa5112351bda/> Add per-font inclusions and exclusions
   * feat(cli): Add per-font inclusions and exclusions
* docs: Start a 'using' doc

### Bug Fixes

 - <csr-id-bcb694b8d657e273a0086c0ab504223bb741851a/> update rust crate scraper to 0.24.0

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

 - 6 commits contributed to the release over the course of 37 calendar days.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#371](https://github.com/fonttools/fontspector/issues/371), [#399](https://github.com/fonttools/fontspector/issues/399), [#403](https://github.com/fonttools/fontspector/issues/403), [#418](https://github.com/fonttools/fontspector/issues/418)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#371](https://github.com/fonttools/fontspector/issues/371)**
    - Get working again ([`0c6365a`](https://github.com/fonttools/fontspector/commit/0c6365a1d3383dad9d12de3984989ee5747b35cf))
 * **[#399](https://github.com/fonttools/fontspector/issues/399)**
    - Update rust crate scraper to 0.24.0 ([`bcb694b`](https://github.com/fonttools/fontspector/commit/bcb694b8d657e273a0086c0ab504223bb741851a))
 * **[#403](https://github.com/fonttools/fontspector/issues/403)**
    - Add per-font inclusions and exclusions ([`dacd8d5`](https://github.com/fonttools/fontspector/commit/dacd8d5d49e1c85774957133df73fa5112351bda))
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

## v1.1.1 (2025-07-02)

<csr-id-a6b7ffc4f39c6b1c1bd92cd9b07f4ba22d54ef2e/>

### Bug Fixes

<csr-id-46e90e51624979590af83272f96cbcfc521b7d0a/>

 - <csr-id-3a8cd3f220746bb67b33863ee3ec1125d1ad0f3b/> Correctly parse URL in OFL text
   * fix(googlefonts/metadata/consistent_repo_urls): Correctly parse URL in OFL text (#296)
* chore: Style fixes for new clippy
* chore: Style fixes for new clippy
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
 - 4 unique issues were worked on: [#287](https://github.com/fonttools/fontspector/issues/287), [#296](https://github.com/fonttools/fontspector/issues/296), [#299](https://github.com/fonttools/fontspector/issues/299), [#302](https://github.com/fonttools/fontspector/issues/302)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#287](https://github.com/fonttools/fontspector/issues/287)**
    - Deny indexing slicing ([`a6b7ffc`](https://github.com/fonttools/fontspector/commit/a6b7ffc4f39c6b1c1bd92cd9b07f4ba22d54ef2e))
 * **[#296](https://github.com/fonttools/fontspector/issues/296)**
    - Correctly parse URL in OFL text ([`3a8cd3f`](https://github.com/fonttools/fontspector/commit/3a8cd3f220746bb67b33863ee3ec1125d1ad0f3b))
 * **[#299](https://github.com/fonttools/fontspector/issues/299)**
    - Improve rationale rewrapping ([`46e90e5`](https://github.com/fonttools/fontspector/commit/46e90e51624979590af83272f96cbcfc521b7d0a))
 * **[#302](https://github.com/fonttools/fontspector/issues/302)**
    - Correctly parse URL in OFL text ([`3a8cd3f`](https://github.com/fonttools/fontspector/commit/3a8cd3f220746bb67b33863ee3ec1125d1ad0f3b))
 * **Uncategorized**
    - Release fontspector-checkapi v1.1.1, fontspector-profile-opentype v1.2.0, fontspector-profile-googlefonts v1.1.1, fontspector-profile-universal v1.1.1, fontspector v1.2.0 ([`f407a9a`](https://github.com/fonttools/fontspector/commit/f407a9aaf0aae501443842311f1b5c27eab007b6))
</details>

## v1.1.0 (2025-06-19)

<csr-id-f44be5515dcaea17b96b1df7a4b11407561d0c17/>
<csr-id-8b28d1aa1c7af4dacdbcfbd83af69dbf401ecf46/>

### Chore

 - <csr-id-f44be5515dcaea17b96b1df7a4b11407561d0c17/> Improve error handling
   * chore: Better error handling
   
   * chore: Better error handling for fix functions too

### New Features

<csr-id-82b1cb17c491e78f6adc0811bb632cc1531dd7dc/>

 - <csr-id-ea5107c15304c96b035aab80551ce8ddb7e7e98b/> Add fix function
   * chore(api): add_table is not polymorphic so this idea just didn't work
* feat(cjk_vertical_metrics): Add fix function
* feat(cli): Allow TOML config, allow explicit_checks/exclude_checks keys
* fix(opentype/vendor_id): Support config file key property

### Bug Fixes

<csr-id-5d058c99b38b636f6ec3130c10296ae664a3384d/>

 - <csr-id-82398535287401e767098901b1da578809d28485/> Make list checks work, add permalinks
   * fix(web): Make list checks work, add permalinks (#162)
* feat(googlefonts): Allow soft_dotted check on wasm, why not

### Refactor

 - <csr-id-8b28d1aa1c7af4dacdbcfbd83af69dbf401ecf46/> Dynamically update script and language tags from MS website

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 23 calendar days.
 - 30 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 7 unique issues were worked on: [#125](https://github.com/fonttools/fontspector/issues/125), [#127](https://github.com/fonttools/fontspector/issues/127), [#144](https://github.com/fonttools/fontspector/issues/144), [#162](https://github.com/fonttools/fontspector/issues/162), [#200](https://github.com/fonttools/fontspector/issues/200), [#275](https://github.com/fonttools/fontspector/issues/275), [#280](https://github.com/fonttools/fontspector/issues/280)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#125](https://github.com/fonttools/fontspector/issues/125)**
    - Allow TOML config, allow explicit_checks/exclude_checks keys ([`82b1cb1`](https://github.com/fonttools/fontspector/commit/82b1cb17c491e78f6adc0811bb632cc1531dd7dc))
 * **[#127](https://github.com/fonttools/fontspector/issues/127)**
    - Dynamically update script and language tags from MS website ([`8b28d1a`](https://github.com/fonttools/fontspector/commit/8b28d1aa1c7af4dacdbcfbd83af69dbf401ecf46))
 * **[#144](https://github.com/fonttools/fontspector/issues/144)**
    - Update rust crate scraper to 0.23.0 ([`5d058c9`](https://github.com/fonttools/fontspector/commit/5d058c99b38b636f6ec3130c10296ae664a3384d))
 * **[#162](https://github.com/fonttools/fontspector/issues/162)**
    - Make list checks work, add permalinks ([`8239853`](https://github.com/fonttools/fontspector/commit/82398535287401e767098901b1da578809d28485))
 * **[#200](https://github.com/fonttools/fontspector/issues/200)**
    - Make list checks work, add permalinks ([`8239853`](https://github.com/fonttools/fontspector/commit/82398535287401e767098901b1da578809d28485))
 * **[#275](https://github.com/fonttools/fontspector/issues/275)**
    - Improve error handling ([`f44be55`](https://github.com/fonttools/fontspector/commit/f44be5515dcaea17b96b1df7a4b11407561d0c17))
 * **[#280](https://github.com/fonttools/fontspector/issues/280)**
    - Add fix function ([`ea5107c`](https://github.com/fonttools/fontspector/commit/ea5107c15304c96b035aab80551ce8ddb7e7e98b))
 * **Uncategorized**
    - Release fontspector-checkapi v1.1.0, fontspector-profile-opentype v1.1.0, fontspector-profile-googlefonts v1.1.0, fontspector-profile-universal v1.1.0 ([`b126546`](https://github.com/fonttools/fontspector/commit/b12654669b361af01b98615c288f3bb816cfe0f6))
</details>

## v1.0.1 (2025-05-19)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 7 calendar days.
 - 11 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#107](https://github.com/fonttools/fontspector/issues/107), [#111](https://github.com/fonttools/fontspector/issues/111), [#113](https://github.com/fonttools/fontspector/issues/113)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#107](https://github.com/fonttools/fontspector/issues/107)**
    - Move to fontations crate ([`da2830b`](https://github.com/fonttools/fontspector/commit/da2830ba694bf3379142a81dad043031e1c39f35))
 * **[#111](https://github.com/fonttools/fontspector/issues/111)**
    - Include most of the fixes from gftools-fix ([`2de6875`](https://github.com/fonttools/fontspector/commit/2de68751c8c4da8c29f9e46d444280cdf478c6b2))
 * **[#113](https://github.com/fonttools/fontspector/issues/113)**
    - Make Fontbakery Python bridge usable ([`7082188`](https://github.com/fonttools/fontspector/commit/7082188f3e6c2ecae5090eba82390835cc1e41ff))
 * **Uncategorized**
    - Release fontspector-checkapi v1.0.1 ([`61aa270`](https://github.com/fonttools/fontspector/commit/61aa2705a95e7fb04d8b881931ee91cfe1af3893))
    - Commit changelog ([`4f4de59`](https://github.com/fonttools/fontspector/commit/4f4de5988e21c574a6d947a41c6b7e8656db4d62))
    - Adjusting changelogs prior to release of fontspector-checkapi v1.0.1 ([`69bf604`](https://github.com/fonttools/fontspector/commit/69bf6042bf8074cb216296d844867d99e63730b6))
    - Changelog ([`4ee3184`](https://github.com/fonttools/fontspector/commit/4ee3184cda649d31da7359ffe8e2e7a827ca3d34))
</details>

## v1.0.0 (2025-05-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 161 commits contributed to the release over the course of 301 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #102 from fonttools/release-prep ([`e5435f4`](https://github.com/fonttools/fontspector/commit/e5435f4ab282338ccc818daca8dacf543de27022))
    - Prep for 1.0.0 release ([`c1ef822`](https://github.com/fonttools/fontspector/commit/c1ef822c860b8dd53b363c9b69201981c75f757c))
    - Merge pull request #99 from fonttools/rich-metadata ([`dfd2c49`](https://github.com/fonttools/fontspector/commit/dfd2c49e542a5c5def5929c6c5e5dbd30e5015bb))
    - Add arbitrary metadata value to status ([`b3e2d6d`](https://github.com/fonttools/fontspector/commit/b3e2d6d7d741f74d211018a238d03b6cd4ffecc2))
    - Merge pull request #96 from fonttools/non-ink-characters ([`1577008`](https://github.com/fonttools/fontspector/commit/15770084eaa140071658b5b6157ceb8174c8eb3a))
    - Process overrides in profile and config file ([`b15fdd8`](https://github.com/fonttools/fontspector/commit/b15fdd8e297d58d3ce2938e2c38a6cc6568cbb86))
    - Move AnythingPen -> HasInkPen ([`3ca0531`](https://github.com/fonttools/fontspector/commit/3ca05318046d24109aa3404c9158c55ed9293159))
    - Merge pull request #92 from fonttools/nixon-feedback ([`0b9a28b`](https://github.com/fonttools/fontspector/commit/0b9a28b9c647bfb7ec0f3ba8156d616fce82b37b))
    - Merge pull request #88 from fonttools/reduce-false-positives ([`dcf298d`](https://github.com/fonttools/fontspector/commit/dcf298d93ad3abe68d4f520f8e980914eb74c008))
    - Use new skrifa native glyph name API ([`6629a99`](https://github.com/fonttools/fontspector/commit/6629a9948640d59dbf97cf6069b2e44b22c83209))
    - Exclude checks marked as excluded by profile ([`3e7a6df`](https://github.com/fonttools/fontspector/commit/3e7a6df5da311ac777e74e6e40d3935d366a31e4))
    - Merge pull request #83 from fonttools/adobe-profile ([`170ffde`](https://github.com/fonttools/fontspector/commit/170ffde473594377a590b95ffbfc7ea5d592d768))
    - Adobe profile ([`1e31635`](https://github.com/fonttools/fontspector/commit/1e31635eaa1d6c023c21f70c70e62dac5a583265))
    - Merge pull request #80 from fonttools/dependency-hell ([`b8ec37d`](https://github.com/fonttools/fontspector/commit/b8ec37d7d52f440fc2d6a9470ee2d3056df2d94c))
    - Reformat ([`ab0a4e4`](https://github.com/fonttools/fontspector/commit/ab0a4e4a5bbd316783438d0337782090a03e0a3f))
    - Use skrifa::raw instead of read_fonts, pin deps ([`76eacb7`](https://github.com/fonttools/fontspector/commit/76eacb755b79772e761b832b8fe8983af81e07fa))
    - Merge pull request #63 from LuxxxLucy/lucy-multiple-proposal-br ([`2d675d5`](https://github.com/fonttools/fontspector/commit/2d675d5bfe5cdb3de99e1a2cf8c65964c144bc52))
    - Merge pull request #78 from fonttools/dep-tidying ([`6633571`](https://github.com/fonttools/fontspector/commit/66335714c16c21c902d8459814a0b37ddfcddf5d))
    - Also some info debugging ([`b1a0f1d`](https://github.com/fonttools/fontspector/commit/b1a0f1db7999bbcf1039083fc2c4f1d733a2f8ea))
    - Merge pull request #77 from fonttools/duckdb ([`610bd5c`](https://github.com/fonttools/fontspector/commit/610bd5c0c6da2d6ab76427e594e2646edac2deac))
    - Merge branch 'main' into duckdb ([`ef0ebe8`](https://github.com/fonttools/fontspector/commit/ef0ebe87d43f220a56310c1a367e9486f2cdff7c))
    - Reorder INFO<->PASS ([`328ceb8`](https://github.com/fonttools/fontspector/commit/328ceb84062d2b4e44deba39c217eaf6feceba95))
    - Reorder status again ([`68c46cf`](https://github.com/fonttools/fontspector/commit/68c46cfacccefe139db875a13e63ce1876793446))
    - Cjk_vertical_metrics_regressions ([`6ad4fd4`](https://github.com/fonttools/fontspector/commit/6ad4fd487c26b9e24b9d2432a7db5279c73129fa))
    - Vertical_metrics_regressions ([`37f214e`](https://github.com/fonttools/fontspector/commit/37f214e1ee60c169c5a96b842b80b4f811310c3e))
    - Cjk_vertical_metrics ([`f325a92`](https://github.com/fonttools/fontspector/commit/f325a927267da1f8f6c24dff89094d1311f53c75))
    - Allow for chaining hotfixes ([`9543e3d`](https://github.com/fonttools/fontspector/commit/9543e3da857864027bc6e69d86b52b2d6fd4500b))
    - Update proposal to take multiple values ([`5f8efd8`](https://github.com/fonttools/fontspector/commit/5f8efd8cbcc6941f82aa68b1a546fd605ade5bb8))
    - Oops, check codepoints per file ([`f42f00f`](https://github.com/fonttools/fontspector/commit/f42f00feb225798ff9943c0c163cf01e9e1b7771))
    - Remote_styles condition ([`b250614`](https://github.com/fonttools/fontspector/commit/b2506141e767e855d393d2e47d797ee3623dfe3b))
    - Update vesions, minimize dependencies ([`8f43370`](https://github.com/fonttools/fontspector/commit/8f433709f66727148a18278383c3b519ce99e331))
    - Redo the way configuration files work to match fontbakery ([`bf2dac6`](https://github.com/fonttools/fontspector/commit/bf2dac6551472828c04519afe502440f870945f0))
    - Update script tags ([`bfd233d`](https://github.com/fonttools/fontspector/commit/bfd233df15409561b642ef2f2c6a9351b64d4721))
    - Allow returning a code with an error ([`4b9f110`](https://github.com/fonttools/fontspector/commit/4b9f110c8d47f11401d49f533c06f93ed37ce7b1))
    - Better explanation ([`f531a8a`](https://github.com/fonttools/fontspector/commit/f531a8af5cb064aaf375fd9b67612a7e533e5fc1))
    - New clippy found new lints! ([`1933d0a`](https://github.com/fonttools/fontspector/commit/1933d0a7835610c4c59e2ca272696789320992e9))
    - Is_italic condition ([`eb4a217`](https://github.com/fonttools/fontspector/commit/eb4a2172f2e1ee72d1098a2c79d05b62ae75845b))
    - Path direction check (with disclaimer) ([`9e1d13a`](https://github.com/fonttools/fontspector/commit/9e1d13a51a5ac1caa044b1e33a8deb4a31ed988a))
    - Move BezGlyph for re-use ([`853e720`](https://github.com/fonttools/fontspector/commit/853e720b160873e68dbb65ed52b7b6fc8fdf34a4))
    - Lifetime madness to support profile builder ([`3ebdcd4`](https://github.com/fonttools/fontspector/commit/3ebdcd4348e05accf010bdc69e16ec46a6082f21))
    - Add a ProfileBuilder structure ([`506255c`](https://github.com/fonttools/fontspector/commit/506255c036388e57b8de618f9dc563ccd2e989d7))
    - Share some crates, add axis registry ([`683ec0e`](https://github.com/fonttools/fontspector/commit/683ec0eeb3a0b1d34fc13c4935d448489be0fd58))
    - Outline type and “best name” APIs ([`5a2d9c7`](https://github.com/fonttools/fontspector/commit/5a2d9c785cde86eafac8d2ba96d6c410556ab84d))
    - Collections can have a name ([`5c202d7`](https://github.com/fonttools/fontspector/commit/5c202d75cd9623a2275d2a95fde91554014891ed))
    - Various cleanups ([`9bb92fc`](https://github.com/fonttools/fontspector/commit/9bb92fca9e86079c9d6422220742d995583d74a3))
    - Use cache to determine codepoints in font ([`0514efc`](https://github.com/fonttools/fontspector/commit/0514efcf5e99d3c157fad5795816183d8f84e091))
    - Cache a question about a testable ([`9b9578d`](https://github.com/fonttools/fontspector/commit/9b9578deba96122efc7847534d5e2add26601996))
    - HELLLOOO shared mutable state! ([`ec1bdfa`](https://github.com/fonttools/fontspector/commit/ec1bdfaacfae1a33fd0afc7246d0af398f7f3b9d))
    - Re-do test order ([`7c1e0ef`](https://github.com/fonttools/fontspector/commit/7c1e0ef261368a748a79deade32905c3289e6998))
    - Better docs ([`1741502`](https://github.com/fonttools/fontspector/commit/17415022e6040163e0549c088ea5824463b5f380))
    - Totally didn't let an AI make that URL up ([`a59e934`](https://github.com/fonttools/fontspector/commit/a59e934a89378cd07027f0ada35cbe04be4e4fb0))
    - Document all the things (again) ([`d8c46e7`](https://github.com/fonttools/fontspector/commit/d8c46e703c32581f26c167f43e065bbe268acb40))
    - Be more flexible ([`69916f4`](https://github.com/fonttools/fontspector/commit/69916f4bdbf9333cc6ad51755c23bfde92845e1b))
    - Isolate the *madness* behind an API function ([`8235ad1`](https://github.com/fonttools/fontspector/commit/8235ad10aa928c5d45e0bcc0f4b4b0ef09ee17ae))
    - Add a contour count pen ([`33622cc`](https://github.com/fonttools/fontspector/commit/33622ccc971a56e22ee198fcfde8c1cc4efe911e))
    - Improve gidXXX naming ([`a67f44f`](https://github.com/fonttools/fontspector/commit/a67f44f3504db46e0898f10403e63523e6fdfb59))
    - Fail better with horrible fonts ([`f571d5f`](https://github.com/fonttools/fontspector/commit/f571d5f690facb6931d30d8e2ca79509e93e1df0))
    - Has_axis helper function ([`33135ba`](https://github.com/fonttools/fontspector/commit/33135ba0a721bf83e7e79af2590a757fbfac2f5e))
    - Avoid “and 0 others” message ([`a315df9`](https://github.com/fonttools/fontspector/commit/a315df9c62c4f9847616ff465474e9046f6df86c))
    - A utility trait to make grovelling GSUB subtables less horrible ([`1eff4d4`](https://github.com/fonttools/fontspector/commit/1eff4d47b09c215bcd867a743783727bee764a75))
    - Properly implement --full-lists ([`d206b67`](https://github.com/fonttools/fontspector/commit/d206b67ec6c6d8db79ed792ece829f1ac8f2d994))
    - Only use clap in the CLI ([`a54b63f`](https://github.com/fonttools/fontspector/commit/a54b63fdd5eaedcfd56c22dd55b6df77d7ff3f32))
    - Upgrade to latest read-fonts API ([`00b3e8d`](https://github.com/fonttools/fontspector/commit/00b3e8d170d88ac44be5399c59657f259dcaf122))
    - Don't emit timing information on wasm ([`045eb5e`](https://github.com/fonttools/fontspector/commit/045eb5e16d1c422ad65d9ca2cc3a42a57d1405b4))
    - Empty_letters ([`51b0b33`](https://github.com/fonttools/fontspector/commit/51b0b337b08ddd4391df6dbea58aa59a9eaf8be5))
    - Store timing information for tests ([`0a3c032`](https://github.com/fonttools/fontspector/commit/0a3c0327b46451e751cee3a2d85c44190d1f699e))
    - Cjk_not_enough_glyphs ([`b40cc36`](https://github.com/fonttools/fontspector/commit/b40cc3684954fdc1c134f84f83a6b963c2900479))
    - Export a DEFAULT_LOCATION setting ([`1ab59e9`](https://github.com/fonttools/fontspector/commit/1ab59e9064181e168765ea3f6cab9d8a28ddac5c))
    - Move all pens to a utility module in checkapi ([`8f86fd5`](https://github.com/fonttools/fontspector/commit/8f86fd56087c660943f39957d5471d865d2755fd))
    - Alt_caron ([`e878068`](https://github.com/fonttools/fontspector/commit/e8780680b042eb4800b069e2456222640ff00f75))
    - Fix embarrassing typo ([`a9048ba`](https://github.com/fonttools/fontspector/commit/a9048baf376770a88c37c0388aec0c7ea436115e))
    - Add is_cjk_font condition ([`36e1a7c`](https://github.com/fonttools/fontspector/commit/36e1a7c6582929fc4fef7a5b706781f9113b75f3))
    - Two more checks ([`567d91a`](https://github.com/fonttools/fontspector/commit/567d91a87f6e410d7927c6b66c1f5aa21e5afaf0))
    - Expose FeatureRecord/Feature tables nicely ([`3a23051`](https://github.com/fonttools/fontspector/commit/3a230516002dbb17473a12c01c63b1e584dc0b1b))
    - Share itertools versions ([`71e6f81`](https://github.com/fonttools/fontspector/commit/71e6f81d35e3fbe8540a38ec532e382effa87459))
    - Impl Debug testfonts ([`7cd1cc3`](https://github.com/fonttools/fontspector/commit/7cd1cc345052baa4ae6902556c20c4e8d498950e))
    - Choose how we fail for assert_all_the_same ([`f219a34`](https://github.com/fonttools/fontspector/commit/f219a3494453e052b9da509edbb63ba1bf4f7dc4))
    - Bump read/write/skrifa versions, dump font-types, deal with fallout ([`d2fd7e4`](https://github.com/fonttools/fontspector/commit/d2fd7e4be7f70b014776c6a56ec035b5156692c0))
    - Improve glyph name API - move unwraps into API lib ([`2a094be`](https://github.com/fonttools/fontspector/commit/2a094bea6bbe22e15320c521aebbe493f3bb4c3c))
    - Use read-fonts' glyph class constants ([`3c41053`](https://github.com/fonttools/fontspector/commit/3c41053289a71d555710a66acc7cfc61cc2402ab))
    - Add --full-lists ([`8e1ae0b`](https://github.com/fonttools/fontspector/commit/8e1ae0b994b7b050c12245b32116d561554d9523))
    - More passes ([`d61590b`](https://github.com/fonttools/fontspector/commit/d61590b39cc724ef546ff66ee5753c2a3d6815e3))
    - Make tests work inside package and at workspace root ([`a34d6fc`](https://github.com/fonttools/fontspector/commit/a34d6fc772085f86bf46dd5ab9ba8d471bc54937))
    - Allow running Python tests using fontspector checks ([`3b27b96`](https://github.com/fonttools/fontspector/commit/3b27b96b6b15f1538aa6866c060511324543292f))
    - Skip! docs ([`a4dc6e6`](https://github.com/fonttools/fontspector/commit/a4dc6e6940b0b943f0e126ee529d9d1a8db66830))
    - Check OTFs too ([`827929e`](https://github.com/fonttools/fontspector/commit/827929e210ab1b7cf8ac2f3c73057ae0fb326665))
    - Improve check listing and ordering ([`1b9e239`](https://github.com/fonttools/fontspector/commit/1b9e239d675f40f6ca87d057352c2bc0ff47d952))
    - Three more checks ([`35db31f`](https://github.com/fonttools/fontspector/commit/35db31f26fdf3640a5be7397e97bce6b5dd48906))
    - Add just_one_info ([`45c434b`](https://github.com/fonttools/fontspector/commit/45c434b94b1a968f647077c5277514046d091a36))
    - A bunch more checks ([`c47194b`](https://github.com/fonttools/fontspector/commit/c47194b6132888d7a6e2372aff68c430dc909ffe))
    - Slant direction check ([`174c9a9`](https://github.com/fonttools/fontspector/commit/174c9a9831ae1476ee9ff89de1d9360a2aba0ab3))
    - Warn about unknown checks later, support profile configuration defaults ([`17bcf17`](https://github.com/fonttools/fontspector/commit/17bcf17496168d350d7ef1f3a20f557f4fd67b99))
    - Rework Python bridge ([`e357d73`](https://github.com/fonttools/fontspector/commit/e357d73000b82b71ee93f28f71c5b16c5ca819d1))
    - Clippy lint ([`bbe500f`](https://github.com/fonttools/fontspector/commit/bbe500f80300fe377115ce52d82138ed4a5ff2aa))
    - Rustfmt/lint ([`15a8be5`](https://github.com/fonttools/fontspector/commit/15a8be593d92863ad0a47ea03a9be70cd421c894))
    - Port another seven opentype checks ([`f11d58a`](https://github.com/fonttools/fontspector/commit/f11d58a7569cf32a15091880901923c49b62d534))
    - Merge pull request #10 from felipesanches/more_checks_2024_sep_20 ([`8cfb898`](https://github.com/fonttools/fontspector/commit/8cfb898458a69666f439676be4d02e7f115bf7a0))
    - Add fontbakery bridge (proof of concept) ([`05b309e`](https://github.com/fonttools/fontspector/commit/05b309e0cf6b18d84102566548eb6a7c48065c9f))
    - Added code-tests for opentype/name/empty_records ([`432d0e3`](https://github.com/fonttools/fontspector/commit/432d0e3b9b47ab719499d7d13da28cf7976a6826))
    - Moving code-testing helper functions to a separate file ([`4e475b1`](https://github.com/fonttools/fontspector/commit/4e475b172c566573a85b793bba47cb6ce21b8268))
    - Pass check metadata (a JSON string) into the check itself ([`f1013ab`](https://github.com/fonttools/fontspector/commit/f1013ab087b6c9aa16834b9e1ff371cb0cd541be))
    - Make section optional, fixes #11 ([`fc36a5c`](https://github.com/fonttools/fontspector/commit/fc36a5c506918139969d0bb60a8d924e017c2641))
    - Pass check metadata (a JSON string) into the check itself ([`b682152`](https://github.com/fonttools/fontspector/commit/b68215290bff6f1bd373e6c6ee2ab822d51eba4f))
    - Make section optional, fixes #11 ([`bcce8f9`](https://github.com/fonttools/fontspector/commit/bcce8f9009ce747f26d5cd4bfcfa4d83b0576ee6))
    - Comment out unfinished check ([`cf856a1`](https://github.com/fonttools/fontspector/commit/cf856a183aa29344ef67384068b6f894998fb819))
    - Some name checks ([`12a4163`](https://github.com/fonttools/fontspector/commit/12a4163175d185d20568a982d6045a96f8a187ee))
    - More general utilities ([`2b62944`](https://github.com/fonttools/fontspector/commit/2b6294460d4900a04ccbc106d3edc6261a839e37))
    - Implement three more checks ([`6264892`](https://github.com/fonttools/fontspector/commit/6264892c82030579f178ca5421f36811589b0a86))
    - Method to get a named file ([`94e1673`](https://github.com/fonttools/fontspector/commit/94e1673d1ffd591f218cb5b1dbf6cb541c7b349b))
    - Add skip constructor ([`825e975`](https://github.com/fonttools/fontspector/commit/825e9754095a548325c4fcf8427881b043f52f0f))
    - Terminal tweaks ([`e28e00f`](https://github.com/fonttools/fontspector/commit/e28e00f85dbc1454cd9f3ded9bf2ff3176b51983))
    - More helpful utilities ([`fbef0ac`](https://github.com/fonttools/fontspector/commit/fbef0ac70697f6c20998474683164fc3818a2a73))
    - Merge pull request #2 from felipesanches/new_check_arabic_spacing_symbols ([`e49cfed`](https://github.com/fonttools/fontspector/commit/e49cfed72bf775ee70d0abce5621a33c5a1cd299))
    - A nicer interface to getting a glyph class from a glyphID ([`22ea7a2`](https://github.com/fonttools/fontspector/commit/22ea7a21379b78fc3a3c7a22b4e696497c6b0cd7))
    - Returning CheckErrors here saves time ([`01e8619`](https://github.com/fonttools/fontspector/commit/01e861989032a733192badbd7f41f282ad616788))
    - Check that Arabic spacing symbols aren't classified as marks ([`dd4af2c`](https://github.com/fonttools/fontspector/commit/dd4af2c5e4631c1a1cba8815bb7368b346c23d8e))
    - Add hellish procmacro ([`4d04baf`](https://github.com/fonttools/fontspector/commit/4d04bafdac36c9d8ef32369d01f29c1e7c7a960b))
    - Optimize font access ([`5d998f5`](https://github.com/fonttools/fontspector/commit/5d998f55591f11b91254a69671416fdbdf8d11df))
    - Some useful functions ([`9912f84`](https://github.com/fonttools/fontspector/commit/9912f84764563610bd22c7091d477c038b06d064))
    - Allow missing checks with a warning, for now ([`1ca3a77`](https://github.com/fonttools/fontspector/commit/1ca3a7712e8aa3ad4821ce43bafdaaeb5ed18bde))
    - Clippy lints ([`d46fdc3`](https://github.com/fonttools/fontspector/commit/d46fdc3ca2517e26a8d8fe5d91a6fded279b43ed))
    - Tidy up checkorder madness, make siblings work in WASM ([`da1d142`](https://github.com/fonttools/fontspector/commit/da1d14229143dd009cf2a4987846e296eb305388))
    - WIP solve the sibling problem ([`10430e5`](https://github.com/fonttools/fontspector/commit/10430e572099e1185247ab78b083de43c154f1a6))
    - Make check implementation (one/all) an enum ([`d57b5c8`](https://github.com/fonttools/fontspector/commit/d57b5c8a08433ecb0ac60330c35df94a91461541))
    - Make TestableCollection the primary unit of testing ([`70da856`](https://github.com/fonttools/fontspector/commit/70da8567069c053415067598ffbe428901784b59))
    - Improve error/skip story, add fvar regular coords check ([`c23b8b0`](https://github.com/fonttools/fontspector/commit/c23b8b0eae9f7f97a15c2d70092196ab1175fe9b))
    - Fix warnings ([`6ffd9ed`](https://github.com/fonttools/fontspector/commit/6ffd9edba26946cb5f203fa310a3e0fe4a0db043))
    - Add WASM target ([`8390919`](https://github.com/fonttools/fontspector/commit/839091928587a43dce605c292c7a76e960082c49))
    - Clippy lints ([`9da264f`](https://github.com/fonttools/fontspector/commit/9da264f9eb177149c6212ed316fc28ef77761652))
    - Tidy up dependencies ([`395112f`](https://github.com/fonttools/fontspector/commit/395112f646b53d446dd082174026fa3ce381f095))
    - Split hotfixing from reporting ([`5ff0e39`](https://github.com/fonttools/fontspector/commit/5ff0e39aed5fc96c2f8ef77debb9099831d39f56))
    - Improve terminal reporting, add ghmarkdown ([`6480cf0`](https://github.com/fonttools/fontspector/commit/6480cf0c4ba14bfab6ce4ba035c1d3980f8414f9))
    - Put checkresult in its own file ([`81f9b6c`](https://github.com/fonttools/fontspector/commit/81f9b6cbe13255725b63d20ed685ae5f65ac0af7))
    - Make checks serializable, add check flags ([`c4996e0`](https://github.com/fonttools/fontspector/commit/c4996e08b590d3710763c117b99d9df61b631e3e))
    - Rearrange run result struct, add subresult codenames/severity ([`2d99a2b`](https://github.com/fonttools/fontspector/commit/2d99a2b760b43d7cdf4630800d25493e0d7485a1))
    - Fix bad merge ([`ad70d24`](https://github.com/fonttools/fontspector/commit/ad70d249e93c20c29b474adea4a77b2244ab58f3))
    - Tidy up results handling ([`9eab7d7`](https://github.com/fonttools/fontspector/commit/9eab7d786d92f77fa0c2c91a85b876e29af5e1f8))
    - Add configuration and check context ([`caeb4b7`](https://github.com/fonttools/fontspector/commit/caeb4b7478a4a51bd5130fe85eb7043758e2236d))
    - Improve display ([`27c29fd`](https://github.com/fonttools/fontspector/commit/27c29fdfe1ee02e8dc337e9542c288ca93efc0cb))
    - Fix sibling_fonts snafu ([`4050179`](https://github.com/fonttools/fontspector/commit/4050179f71aa52d0e413d86d453ead6da766c2d7))
    - Merge pull request #5 from felipesanches/rationales_not_optional ([`ee113d9`](https://github.com/fonttools/fontspector/commit/ee113d98a0cb146a764163c6afeacae05f0ece9f))
    - Merge branch 'main' into rationales_not_optional ([`37122c3`](https://github.com/fonttools/fontspector/commit/37122c334183fa689fbe4f5617b1ca24e6abb95c))
    - Be (slightly) more grown-up about error handling ([`2818a76`](https://github.com/fonttools/fontspector/commit/2818a764da76b9acc2c33127cb156238dca970c1))
    - Rationale and proposal fields are not optional ([`752d559`](https://github.com/fonttools/fontspector/commit/752d5593f3c5a345a781f8b76e5907607bda7dbd))
    - Find siblings and codepoints ([`deb9187`](https://github.com/fonttools/fontspector/commit/deb91873facba752bb0baae31b70e9d19997ef7b))
    - Allow easy interface to skipping ([`72f5f36`](https://github.com/fonttools/fontspector/commit/72f5f36f1be93665418fa3f94390c2e83fd4a0d4))
    - Add has_table utility ([`b7f43d1`](https://github.com/fonttools/fontspector/commit/b7f43d1021693e7f87c273271df00c9e7941c14e))
    - Explanation of weirdness ([`51aaddf`](https://github.com/fonttools/fontspector/commit/51aaddf872c7cdaa680775125edb7b5ffe1acfb5))
    - Allow included profiles, make registering profile a Result ([`4d7a296`](https://github.com/fonttools/fontspector/commit/4d7a296a76c2717c895784d8d1e795a1740a3859))
    - Provide a basename method on testable ([`4b5830f`](https://github.com/fonttools/fontspector/commit/4b5830f1769d7c081a224e49af604e9150e88f6b))
    - Allow simpler profile registration ([`7f7aeaa`](https://github.com/fonttools/fontspector/commit/7f7aeaab0de352f660f70760f9475d2f4544ee2f))
    - Add fixes ([`248f457`](https://github.com/fonttools/fontspector/commit/248f457d99f5352940f287d2c75e2d8b540f7048))
    - Update fontread/write dependencies ([`83a2abc`](https://github.com/fonttools/fontspector/commit/83a2abcf0ce9c4a3a2fe6d3fd4fc5c28862a3824))
    - Make check registry a map ([`44aae7b`](https://github.com/fonttools/fontspector/commit/44aae7bdc987e6a01587fcfd38dabb5fdfdeadd8))
    - Make it parallelable ([`a00b396`](https://github.com/fonttools/fontspector/commit/a00b3961e5461983bbc1b0b06baf367f4c357e2c))
    - Use a prelude ([`fb66913`](https://github.com/fonttools/fontspector/commit/fb669139300ca7e671ee2af8b47ba8f9e6ccfdd3))
    - Tidy lots of things up, allow pluggable file types ([`1651816`](https://github.com/fonttools/fontspector/commit/1651816d634137e319925acb9dc33da66ccf38e9))
    - Clean up warnings ([`b2a6b0b`](https://github.com/fonttools/fontspector/commit/b2a6b0b5b8316b78db740222ec2287f3d69bd366))
    - Add the concept of a profile ([`41a37dc`](https://github.com/fonttools/fontspector/commit/41a37dc02a6aa9f16b369af304c5c70861343439))
    - Rename some stuff ([`f174d56`](https://github.com/fonttools/fontspector/commit/f174d56325e86cd4ade690ab8e5ffaa9fcecca30))
    - Move to plugin architecture ([`5fdf975`](https://github.com/fonttools/fontspector/commit/5fdf9750991176c8e2776557ce6c17c642c24a73))
</details>

