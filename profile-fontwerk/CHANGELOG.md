# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
   
   Don't bump the version number manually, this will happen automatically on release
   
   * refactor: remove soft_hyphen from Fontwerk, use with_overrides instead.
   
   * Update CONTRIBUTORS.txt

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 40 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#356](https://github.com/fonttools/fontspector/issues/356)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#356](https://github.com/fonttools/fontspector/issues/356)**
    - Extend profile ([`6f7ee12`](https://github.com/fonttools/fontspector/commit/6f7ee1248bf877b8a563cbbee7e8cc54d68b85a9))
</details>

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

