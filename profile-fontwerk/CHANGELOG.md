# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.3.0 (2026-02-23)

### New Features

 - <csr-id-708d0735051541701cc46667192450bcfec307d5/> removing .exclude_check("googlefonts/gasp")

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 16 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#635](https://github.com/fonttools/fontspector/issues/635)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#635](https://github.com/fonttools/fontspector/issues/635)**
    - Removing .exclude_check("googlefonts/gasp") ([`708d073`](https://github.com/fonttools/fontspector/commit/708d0735051541701cc46667192450bcfec307d5))
</details>

## v1.2.3 (2026-02-06)

<csr-id-19b91cdf15a6d8ea7e60b616199d28e03c76ba4d/>

### Chore

 - <csr-id-19b91cdf15a6d8ea7e60b616199d28e03c76ba4d/> Reformat

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 2 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#611](https://github.com/fonttools/fontspector/issues/611)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#611](https://github.com/fonttools/fontspector/issues/611)**
    - Reformat ([`19b91cd`](https://github.com/fonttools/fontspector/commit/19b91cdf15a6d8ea7e60b616199d28e03c76ba4d))
 * **Uncategorized**
    - Release fontspector-checkapi v1.4.0, fontspector-profile-fontwerk v1.2.3, fontspector-profile-googlefonts v1.6.1, fontspector-profile-opentype v1.3.1, fontspector-profile-universal v1.6.0, fontspector v1.5.4 ([`b27d3e3`](https://github.com/fonttools/fontspector/commit/b27d3e3ab7e2cf650a02883d776808bde0611edd))
</details>

## v1.2.2 (2026-02-04)

<csr-id-a755d98f2dcb10c675c0084588ca16eee2a5f313/>
<csr-id-c69d124501d4251aee6a0d54233a3cc627d5f325/>

### Refactor

 - <csr-id-a755d98f2dcb10c675c0084588ca16eee2a5f313/> Regular weight fonts often don't have 'Regular' in its name, fix edge cases
 - <csr-id-c69d124501d4251aee6a0d54233a3cc627d5f325/> replace get_name_PEL_codes and get_name_entry_string
   + fix uniitest (because of different sorting of platform _tuples)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 25 calendar days.
 - 48 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#480](https://github.com/fonttools/fontspector/issues/480), [#583](https://github.com/fonttools/fontspector/issues/583)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#480](https://github.com/fonttools/fontspector/issues/480)**
    - Replace get_name_PEL_codes and get_name_entry_string ([`c69d124`](https://github.com/fonttools/fontspector/commit/c69d124501d4251aee6a0d54233a3cc627d5f325))
 * **[#583](https://github.com/fonttools/fontspector/issues/583)**
    - Regular weight fonts often don't have 'Regular' in its name, fix edge cases ([`a755d98`](https://github.com/fonttools/fontspector/commit/a755d98f2dcb10c675c0084588ca16eee2a5f313))
 * **Uncategorized**
    - Release fontspector-checkapi v1.3.2, fontspector-profile-fontwerk v1.2.2, fontspector-profile-googlefonts v1.6.0, fontspector-profile-universal v1.5.0, fontspector-profile-iso15008 v1.0.5, fontspector v1.5.3 ([`43eb125`](https://github.com/fonttools/fontspector/commit/43eb125ce74b3424af90f5f9aa8082fdb4e2e742))
</details>

## v1.2.1 (2025-12-17)

### Bug Fixes

 - <csr-id-e4722fef242bc3554263a87e2b67599312e4dc14/> fontwerk/weightclass
   * fix(fontwerk/weightclass): make unittest fail
* fix(fontwerk/weightclass): cover special case 'Italic' weight 400
* fix(fontwerk/weightclass): add SemiLight 350 to get_expected_weight_name

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 30 calendar days.
 - 93 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#520](https://github.com/fonttools/fontspector/issues/520), [#535](https://github.com/fonttools/fontspector/issues/535)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#520](https://github.com/fonttools/fontspector/issues/520)**
    - Fontwerk/weightclass ([`e4722fe`](https://github.com/fonttools/fontspector/commit/e4722fef242bc3554263a87e2b67599312e4dc14))
 * **[#535](https://github.com/fonttools/fontspector/issues/535)**
    - Fix(required_name_ids): make universal (fixes #534) ([`ad73d4c`](https://github.com/fonttools/fontspector/commit/ad73d4cf0a8e9627723b25e2861057e7d75ad49b))
 * **Uncategorized**
    - Release fontspector-checkapi v1.3.1, fontspector-profile-fontwerk v1.2.1, fontspector-profile-googlefonts v1.5.0, fontspector-profile-universal v1.4.0, fontspector v1.5.2 ([`2779526`](https://github.com/fonttools/fontspector/commit/2779526c00f235ea93e95882b4ebd2b41786c715))
</details>

## v1.2.0 (2025-09-15)

<csr-id-90205a8089d1394f957cdf13cdcc461b73824425/>
<csr-id-abd4d4cf426666d0dac52706a763fb01d0e85d2c/>

### New Features

 - <csr-id-6f7ee1248bf877b8a563cbbee7e8cc54d68b85a9/> Extend profile
   * feat(fontwerk/name_consistency): Check if names are consistent within name table
* feat(fontwerk/name_consistency): Refactor code
* feat(fontwerk/name_consistency): formatting issues
* feat(fontwerk/name_consistency): Remove unused imports
* feat(fontwerk/required_name_ids): New test required_name_ids
* feat(fontwerk/required_name_ids): Fix formatting issues
* feat: Refactoring code Fontwerk
* feat(register): extend exclude_check()
* feat(fontwerk/soft_hyphen): soft-hyphen not allowed + update lib.rs and mod.rs
* feat(fontwerk/embedding_bit): Add embedding test for Fontwerk fonts
* Fix formatting fstype check
* feat(fontwerk/glyph_coverage): add new check fontwerk/glyph_coverage
* fix(fontwerk/glyph_coverage): move null character to encoded glyph list
* fix(fontwerk/glyph_coverage): fix const lengths
* fix(fontwerk/glyph_coverage): fix unitest (moved NULL to encoded glyphs)
* feat(fontwerk/name_entries): allow to check against regex
* fix(name_entries): make lint happy
* feat: override valid_glyphnames found-invalid-names to StatusCode::Warn
* refactor: Undo version of Fontwerk-profile
* refactor: remove soft_hyphen from Fontwerk, use with_overrides instead.
* Update CONTRIBUTORS.txt

### Bug Fixes

 - <csr-id-b635bd4c1b503c03db57ea4d07c7555c1aa5c731/> Update Fontwerk profile
   * fix(fontwerk/glyph_coverage): removing .case glyphs from minimum character set.
* feat(fontwerk/weightclass): new test to meet fontwerk standards + .exclude_check("googlefonts/weightclass")

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

 - 5 commits contributed to the release over the course of 54 calendar days.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#356](https://github.com/fonttools/fontspector/issues/356), [#410](https://github.com/fonttools/fontspector/issues/410), [#418](https://github.com/fonttools/fontspector/issues/418)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#356](https://github.com/fonttools/fontspector/issues/356)**
    - Extend profile ([`6f7ee12`](https://github.com/fonttools/fontspector/commit/6f7ee1248bf877b8a563cbbee7e8cc54d68b85a9))
 * **[#410](https://github.com/fonttools/fontspector/issues/410)**
    - Update Fontwerk profile ([`b635bd4`](https://github.com/fonttools/fontspector/commit/b635bd4c1b503c03db57ea4d07c7555c1aa5c731))
 * **[#418](https://github.com/fonttools/fontspector/issues/418)**
    - Move the tests from Python to Rust ([`abd4d4c`](https://github.com/fonttools/fontspector/commit/abd4d4cf426666d0dac52706a763fb01d0e85d2c))
 * **Uncategorized**
    - Release fontspector-checkapi v1.2.0, fontspector-profile-fontwerk v1.2.0, fontspector-profile-googlefonts v1.4.0, fontspector-profile-opentype v1.3.0, fontspector-profile-universal v1.2.0, fontspector-profile-iso15008 v1.0.4, fontspector v1.5.0 ([`de4a966`](https://github.com/fonttools/fontspector/commit/de4a966105bea222ea98da69793ddbfbdd590f9d))
    - Fontspector-checkapi v1.1.2, fontspector-fontbakery-bridge v1.2.0, fontspector-profile-fontwerk v1.1.0, fontspector-profile-googlefonts v1.3.0, fontspector-profile-opentype v1.2.1, fontspector-profile-universal v1.1.2, fontspector v1.4.0 ([`90205a8`](https://github.com/fonttools/fontspector/commit/90205a8089d1394f957cdf13cdcc461b73824425))
</details>

## v1.1.0 (2025-08-11)

### New Features

 - <csr-id-6f7ee1248bf877b8a563cbbee7e8cc54d68b85a9/> Extend profile
   * feat(fontwerk/name_consistency): Check if names are consistent within name table
* feat(fontwerk/name_consistency): Refactor code
* feat(fontwerk/name_consistency): formatting issues
* feat(fontwerk/name_consistency): Remove unused imports
* feat(fontwerk/required_name_ids): New test required_name_ids
* feat(fontwerk/required_name_ids): Fix formatting issues
* feat: Refactoring code Fontwerk
* feat(register): extend exclude_check()
* feat(fontwerk/soft_hyphen): soft-hyphen not allowed + update lib.rs and mod.rs
* feat(fontwerk/embedding_bit): Add embedding test for Fontwerk fonts
* Fix formatting fstype check
* feat(fontwerk/glyph_coverage): add new check fontwerk/glyph_coverage
* fix(fontwerk/glyph_coverage): move null character to encoded glyph list
* fix(fontwerk/glyph_coverage): fix const lengths
* fix(fontwerk/glyph_coverage): fix unitest (moved NULL to encoded glyphs)
* feat(fontwerk/name_entries): allow to check against regex
* fix(name_entries): make lint happy
* feat: override valid_glyphnames found-invalid-names to StatusCode::Warn
* refactor: Undo version of Fontwerk-profile
* refactor: remove soft_hyphen from Fontwerk, use with_overrides instead.
* Update CONTRIBUTORS.txt

<csr-unknown>
<csr-unknown>
<csr-unknown>
<csr-unknown>
<csr-unknown>
Don’t bump the version number manually, this will happen automatically on release<csr-unknown/>
<csr-unknown/>
<csr-unknown/>
<csr-unknown/>
<csr-unknown/>

## v1.0.0 (2025-07-02)

<csr-id-367ab6a38fcae4d5053531becf969c697af1de66/>

### Chore

 - <csr-id-367ab6a38fcae4d5053531becf969c697af1de66/> Add CHANGELOG

### New Features

 - <csr-id-06e1ff0b9234917d3040559465b70c4b3c44e61e/> fontwerk profile

### Bug Fixes

<csr-id-46e90e51624979590af83272f96cbcfc521b7d0a/>

 - <csr-id-3a8cd3f220746bb67b33863ee3ec1125d1ad0f3b/> Correctly parse URL in OFL text
   * fix(googlefonts/metadata/consistent_repo_urls): Correctly parse URL in OFL text (#296)
* chore: Style fixes for new clippy
* chore: Style fixes for new clippy
* fix(cli): Improve rationale rewrapping
* chore: Style fixes for new clippy

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 9 calendar days.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#161](https://github.com/fonttools/fontspector/issues/161), [#296](https://github.com/fonttools/fontspector/issues/296), [#299](https://github.com/fonttools/fontspector/issues/299), [#302](https://github.com/fonttools/fontspector/issues/302)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#161](https://github.com/fonttools/fontspector/issues/161)**
    - Fontwerk profile ([`06e1ff0`](https://github.com/fonttools/fontspector/commit/06e1ff0b9234917d3040559465b70c4b3c44e61e))
 * **[#296](https://github.com/fonttools/fontspector/issues/296)**
    - Correctly parse URL in OFL text ([`3a8cd3f`](https://github.com/fonttools/fontspector/commit/3a8cd3f220746bb67b33863ee3ec1125d1ad0f3b))
 * **[#299](https://github.com/fonttools/fontspector/issues/299)**
    - Improve rationale rewrapping ([`46e90e5`](https://github.com/fonttools/fontspector/commit/46e90e51624979590af83272f96cbcfc521b7d0a))
 * **[#302](https://github.com/fonttools/fontspector/issues/302)**
    - Correctly parse URL in OFL text ([`3a8cd3f`](https://github.com/fonttools/fontspector/commit/3a8cd3f220746bb67b33863ee3ec1125d1ad0f3b))
 * **Uncategorized**
    - Release fontspector-profile-fontwerk v1.0.0, fontspector v1.2.0 ([`0efca53`](https://github.com/fonttools/fontspector/commit/0efca539ecee573a378018c93f9ae71b561715de))
    - Add CHANGELOG ([`367ab6a`](https://github.com/fonttools/fontspector/commit/367ab6a38fcae4d5053531becf969c697af1de66))
</details>

