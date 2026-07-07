# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.4.0 (2026-07-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 11 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release fontspector-profile-opentype v1.6.0, safety bump fontspector-hotfix v0.4.0 ([`dd380e5`](https://github.com/fonttools/fontspector/commit/dd380e5f7d89a57a7b13e4fac184b73c44c0e0b9))
</details>

## v0.3.1 (2026-06-26)

### Bug Fixes

 - <csr-id-607fa80d8b7c7aad5dbfd9d8cd4987d014f2d23e/> Don't ask for details if input is not a TTY

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 16 calendar days.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#817](https://github.com/fonttools/fontspector/issues/817)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#817](https://github.com/fonttools/fontspector/issues/817)**
    - Don't ask for details if input is not a TTY ([`607fa80`](https://github.com/fonttools/fontspector/commit/607fa80d8b7c7aad5dbfd9d8cd4987d014f2d23e))
 * **Uncategorized**
    - Release fontspector-checkapi v1.7.1, fontspector-profile-fontwerk v1.3.2, fontspector-profile-googlefonts v1.8.2, fontspector-profile-iso15008 v1.0.7, fontspector-profile-opentype v1.5.1, fontspector-profile-universal v1.8.2, fontspector-hotfix v0.3.1, fontspector v1.7.2 ([`accdd2a`](https://github.com/fonttools/fontspector/commit/accdd2a9c3ab285e71e5a047120cbe366cd80a84))
</details>

## v0.3.0 (2026-05-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release fontspector-checkapi v1.7.0, fontspector-profile-googlefonts v1.8.1, fontspector-profile-universal v1.8.1, fontspector-hotfix v0.3.0, safety bump fontspector-hotfix v0.3.0 ([`4d8f418`](https://github.com/fonttools/fontspector/commit/4d8f41871df584797c2a918d2614a84b583b689d))
</details>

## v0.2.0 (2026-05-18)

<csr-id-7b32eca0846e594655f304250928295ffbf6496a/>

### New Features

 - <csr-id-d7a8e964d234b7bf1693e85a7610d9a1f78c572f/> Interactive fixing
   * feat(googlefonts/canonical_filename): Add hotfix
* chore(web): Hotfix testables all at once
* chore(web): Improve display of fixables
* chore(opentype/GDEF_non_mark_chars): Add metadata, sort output
* chore: Thanks clippy
* chore(googlefonts): Move style map into constants
* feat(googlefonts/font_names): Detect bad styles based on typos and weightclass settings
* chore(web): Improve message grouping (again)
* chore: Adapt fix API to request and receive structured data
* feat: WIP dialogues in hotfixes
* feat(web): Fix but don't download
* chore: Slightly better dialogue Ux
* chore: Update web to new fix API
* feat(googlefonts/font_names): Add interactive fix function for statics
* chore: Update hotfix lib to new fix API
* chore: Update web to new fix API
* fix(googlefonts/font_names): Better problem reporting
* fix(web): Loading spinner
* chore: Thanks clippy
* chore(web): Drop the log file properly
* chore: Small web fixes
* test: Fix up Python test

### Refactor

 - <csr-id-7b32eca0846e594655f304250928295ffbf6496a/> New plugin architecture
   * refactor: Remove fontbakery-bridge
   
   * refactor: New plugin architecture
   
   * feat: Demonstrate Python-based plugins
   
   * docs: New plugin architecture

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 62 calendar days.
 - 84 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#710](https://github.com/fonttools/fontspector/issues/710), [#782](https://github.com/fonttools/fontspector/issues/782)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#710](https://github.com/fonttools/fontspector/issues/710)**
    - Interactive fixing ([`d7a8e96`](https://github.com/fonttools/fontspector/commit/d7a8e964d234b7bf1693e85a7610d9a1f78c572f))
 * **[#782](https://github.com/fonttools/fontspector/issues/782)**
    - New plugin architecture ([`7b32eca`](https://github.com/fonttools/fontspector/commit/7b32eca0846e594655f304250928295ffbf6496a))
 * **Uncategorized**
    - Release fontspector-checkapi v1.6.0, fontspector-profile-fontwerk v1.3.1, fontspector-profile-googlefonts v1.8.0, fontspector-profile-iso15008 v1.0.6, fontspector-profile-opentype v1.5.0, fontspector-profile-universal v1.8.0, fontspector-hotfix v0.2.0, fontspector v1.7.0, safety bump fontspector-hotfix v0.2.0 ([`b319e16`](https://github.com/fonttools/fontspector/commit/b319e16d70daabfed30fcb18d66b8400c00fd32f))
</details>

## v0.1.0 (2026-02-23)

### New Features

 - <csr-id-367b6753233622f65a13e58126d19dd268cdf582/> Add hotfix tool

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 13 calendar days.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#619](https://github.com/fonttools/fontspector/issues/619)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#619](https://github.com/fonttools/fontspector/issues/619)**
    - Add hotfix tool ([`367b675`](https://github.com/fonttools/fontspector/commit/367b6753233622f65a13e58126d19dd268cdf582))
 * **Uncategorized**
    - Release fontspector-checkapi v1.5.0, fontspector-fontbakery-bridge v1.3.0, fontspector-profile-fontwerk v1.3.0, fontspector-profile-googlefonts v1.7.0, fontspector-profile-opentype v1.4.0, fontspector-profile-universal v1.7.0, fontspector-hotfix v0.1.0, fontspector v1.6.0, safety bump fontspector-hotfix v0.1.0 ([`cb2a669`](https://github.com/fonttools/fontspector/commit/cb2a669f1f0963a68ba22bdc1e0cd56e602219ca))
</details>

