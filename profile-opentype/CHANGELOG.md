# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.3.0 (2025-09-15)

### New Features

 - <csr-id-c6dfbb4ab45f78c2501baafa6abe603e9a3bd26e/> New check

### Bug Fixes

 - <csr-id-d58c9ef800041bb3ec8cd266907d4c03f1b548c9/> Improve formatting of bulleted lists, fixes #352

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

 - 4 commits contributed to the release over the course of 51 calendar days.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#357](https://github.com/fonttools/fontspector/issues/357), [#381](https://github.com/fonttools/fontspector/issues/381), [#418](https://github.com/fonttools/fontspector/issues/418)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#357](https://github.com/fonttools/fontspector/issues/357)**
    - Improve formatting of bulleted lists, fixes #352 ([`d58c9ef`](https://github.com/fonttools/fontspector/commit/d58c9ef800041bb3ec8cd266907d4c03f1b548c9))
 * **[#381](https://github.com/fonttools/fontspector/issues/381)**
    - New check ([`c6dfbb4`](https://github.com/fonttools/fontspector/commit/c6dfbb4ab45f78c2501baafa6abe603e9a3bd26e))
 * **[#418](https://github.com/fonttools/fontspector/issues/418)**
    - Move the tests from Python to Rust ([`abd4d4c`](https://github.com/fonttools/fontspector/commit/abd4d4cf426666d0dac52706a763fb01d0e85d2c))
 * **Uncategorized**
    - Fontspector-checkapi v1.1.2, fontspector-fontbakery-bridge v1.2.0, fontspector-profile-fontwerk v1.1.0, fontspector-profile-googlefonts v1.3.0, fontspector-profile-opentype v1.2.1, fontspector-profile-universal v1.1.2, fontspector v1.4.0 ([`90205a8`](https://github.com/fonttools/fontspector/commit/90205a8089d1394f957cdf13cdcc461b73824425))
</details>

## v1.2.1 (2025-08-11)

### Bug Fixes

 - <csr-id-d58c9ef800041bb3ec8cd266907d4c03f1b548c9/> Improve formatting of bulleted lists, fixes #352

## v1.2.0 (2025-07-02)

<csr-id-138edbbfba88008d71d9247eccbdfc017fef8b81/>
<csr-id-a6b7ffc4f39c6b1c1bd92cd9b07f4ba22d54ef2e/>

### Chore

 - <csr-id-138edbbfba88008d71d9247eccbdfc017fef8b81/> Update fontations dependencies
   * chore: Update fontations dependencies
   
   * test(italic_angle): Update bounds check

### New Features

 - <csr-id-06e1ff0b9234917d3040559465b70c4b3c44e61e/> fontwerk profile

### Bug Fixes

<csr-id-24e8aaeb0226552af2a94eb51b724afbf4f29cf3/>

 - <csr-id-46e90e51624979590af83272f96cbcfc521b7d0a/> Improve rationale rewrapping
   * fix(cli): Improve rationale rewrapping
* chore: Style fixes for new clippy
* Extend test for bold_italic_unique_for_nameid1 with condensed styles
* Add missing ttFonts to unittest
* Fix bold_italic_unique_for_nameid1
* Use: .first(), because we avoid indexing into vectors
* Fix: Lint issue

### Style

 - <csr-id-a6b7ffc4f39c6b1c1bd92cd9b07f4ba22d54ef2e/> deny indexing slicing
   * chore: More lints into Cargo.toml
   
   * style: Deny indexing slicing

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 11 calendar days.
 - 12 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 5 unique issues were worked on: [#161](https://github.com/fonttools/fontspector/issues/161), [#279](https://github.com/fonttools/fontspector/issues/279), [#287](https://github.com/fonttools/fontspector/issues/287), [#291](https://github.com/fonttools/fontspector/issues/291), [#299](https://github.com/fonttools/fontspector/issues/299)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#161](https://github.com/fonttools/fontspector/issues/161)**
    - Fontwerk profile ([`06e1ff0`](https://github.com/fonttools/fontspector/commit/06e1ff0b9234917d3040559465b70c4b3c44e61e))
 * **[#279](https://github.com/fonttools/fontspector/issues/279)**
    - Allow larger families with same bits ([`24e8aae`](https://github.com/fonttools/fontspector/commit/24e8aaeb0226552af2a94eb51b724afbf4f29cf3))
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

<csr-id-82b1cb17c491e78f6adc0811bb632cc1531dd7dc/>

 - <csr-id-ea5107c15304c96b035aab80551ce8ddb7e7e98b/> Add fix function
   * chore(api): add_table is not polymorphic so this idea just didn't work
* feat(cjk_vertical_metrics): Add fix function
* feat(cli): Allow TOML config, allow explicit_checks/exclude_checks keys
* fix(opentype/vendor_id): Support config file key property

### Bug Fixes

 - <csr-id-5382a410d33321beecee209b270ba4158fc1514b/> cff font* test: Extend unittest for monospace* feat(monospace): fix output message* test(monospace): make unittest fail with CFF font (as expected)* fix(monospace): remove 'glyf' from required tables

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 23 calendar days.
 - 30 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#125](https://github.com/fonttools/fontspector/issues/125), [#234](https://github.com/fonttools/fontspector/issues/234), [#275](https://github.com/fonttools/fontspector/issues/275), [#280](https://github.com/fonttools/fontspector/issues/280)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#125](https://github.com/fonttools/fontspector/issues/125)**
    - Allow TOML config, allow explicit_checks/exclude_checks keys ([`82b1cb1`](https://github.com/fonttools/fontspector/commit/82b1cb17c491e78f6adc0811bb632cc1531dd7dc))
 * **[#234](https://github.com/fonttools/fontspector/issues/234)**
    - Cff font* test: Extend unittest for monospace* feat(monospace): fix output message* test(monospace): make unittest fail with CFF font (as expected)* fix(monospace): remove 'glyf' from required tables ([`5382a41`](https://github.com/fonttools/fontspector/commit/5382a410d33321beecee209b270ba4158fc1514b))
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

 - 3 commits contributed to the release over the course of 7 calendar days.
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
    - Release fontspector-profile-opentype v1.0.2, fontspector-profile-googlefonts v1.0.2, fontspector-profile-universal v1.0.2 ([`fd2d019`](https://github.com/fonttools/fontspector/commit/fd2d0197d3918c1f74890b69fed4fe49ac8a3408))
</details>

## v1.0.1 (2025-05-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#105](https://github.com/fonttools/fontspector/issues/105), [#106](https://github.com/fonttools/fontspector/issues/106)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#105](https://github.com/fonttools/fontspector/issues/105)**
    - Fix font_version parsing ([`cbf7b4b`](https://github.com/fonttools/fontspector/commit/cbf7b4bdd0cc30ddda32c919cbbae9a5e0e09cd2))
 * **[#106](https://github.com/fonttools/fontspector/issues/106)**
    - Fix gpos/gsub typo ([`95867d8`](https://github.com/fonttools/fontspector/commit/95867d8e2048e160735ac7b7f5bb7e99b0fee50e))
 * **Uncategorized**
    - Release fontspector-checkhelper v1.0.1, fontspector-profile-opentype v1.0.1, fontspector-profile-googlefonts v1.0.1, fontspector-profile-universal v1.0.1 ([`6ee7aed`](https://github.com/fonttools/fontspector/commit/6ee7aeda28e6961710b748e346cc1cc8c3e26b82))
    - Add changelogs ([`8b511ed`](https://github.com/fonttools/fontspector/commit/8b511eda27d0f3c7bb9e1f21d9749585e35c2fce))
</details>

## v1.0.0 (2025-05-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 77 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #102 from fonttools/release-prep ([`e5435f4`](https://github.com/fonttools/fontspector/commit/e5435f4ab282338ccc818daca8dacf543de27022))
    - Read profile cargo files for release ([`5fe1c5a`](https://github.com/fonttools/fontspector/commit/5fe1c5aff636944c257ec25b19004426660db0c2))
    - Prep for 1.0.0 release ([`c1ef822`](https://github.com/fonttools/fontspector/commit/c1ef822c860b8dd53b363c9b69201981c75f757c))
    - Merge pull request #90 from fonttools/fix-87 ([`2319315`](https://github.com/fonttools/fontspector/commit/2319315f716210e99f25bc1e932be44e595322d4))
    - Downgrade fail to warn, fixes #87 ([`8e9b831`](https://github.com/fonttools/fontspector/commit/8e9b8311d2aca93ba65b6cb6cceecab00a1ebf76))
    - Merge pull request #80 from fonttools/dependency-hell ([`b8ec37d`](https://github.com/fonttools/fontspector/commit/b8ec37d7d52f440fc2d6a9470ee2d3056df2d94c))
    - Use skrifa::raw instead of read_fonts, pin deps ([`76eacb7`](https://github.com/fonttools/fontspector/commit/76eacb755b79772e761b832b8fe8983af81e07fa))
    - Merge pull request #63 from LuxxxLucy/lucy-multiple-proposal-br ([`2d675d5`](https://github.com/fonttools/fontspector/commit/2d675d5bfe5cdb3de99e1a2cf8c65964c144bc52))
    - A little more information ([`9ddede9`](https://github.com/fonttools/fontspector/commit/9ddede9bed035098a91ad07d6e8a70a873b2905c))
    - Update the checks ([`4110dcf`](https://github.com/fonttools/fontspector/commit/4110dcfd1c79131aea9893523b50e0b0bdfd2f95))
    - Latest read-fonts API ([`47a5310`](https://github.com/fonttools/fontspector/commit/47a531036503433ae38f78ef4fad98cf76536bf7))
    - Allow returning a code with an error ([`4b9f110`](https://github.com/fonttools/fontspector/commit/4b9f110c8d47f11401d49f533c06f93ed37ce7b1))
    - New clippy found new lints! ([`1933d0a`](https://github.com/fonttools/fontspector/commit/1933d0a7835610c4c59e2ca272696789320992e9))
    - Run cargo fmt ([`a97b2a9`](https://github.com/fonttools/fontspector/commit/a97b2a96d2ffbf6fab861b842096159d666a4dc9))
    - Neater repo organization (opentype profile) ([`f7c336d`](https://github.com/fonttools/fontspector/commit/f7c336db697bad52d7fa6cbd9fad9fd4a6392158))
    - Path direction check (with disclaimer) ([`9e1d13a`](https://github.com/fonttools/fontspector/commit/9e1d13a51a5ac1caa044b1e33a8deb4a31ed988a))
    - Move BezGlyph for re-use ([`853e720`](https://github.com/fonttools/fontspector/commit/853e720b160873e68dbb65ed52b7b6fc8fdf34a4))
    - Silence warnings ([`d26e978`](https://github.com/fonttools/fontspector/commit/d26e978be6dbedf3aea4a363c7aa4d06495e71d3))
    - Use ProfileBuilder to simplify OpenType profile ([`3f7d038`](https://github.com/fonttools/fontspector/commit/3f7d0387fd5112512b383261c4e59bb15e8efe93))
    - Move alt_caron to universal ([`d3f4035`](https://github.com/fonttools/fontspector/commit/d3f4035c1b67c7749600d124e5e97f7fd0f19d2c))
    - Stat_has_axis_value_tables / inconsistencies_between_fvar_STAT ([`ed2aa43`](https://github.com/fonttools/fontspector/commit/ed2aa43f514c6f340a8db6a5b9924f4ebd431c55))
    - Opentype/gpos_kerning_info => gpos_kerning_info (Universal profile) ([`854af14`](https://github.com/fonttools/fontspector/commit/854af14cacb2c785ac5f27e8fc64bd2b8c966743))
    - [opentype/stat/ital_axis] Escape the double quotes in rationale string ([`f73946b`](https://github.com/fonttools/fontspector/commit/f73946bd29f33aa2660ad820bd1dc7bcb6a7d2b1))
    - [opentype/stat/ital_axis] update rationale ([`0d8af42`](https://github.com/fonttools/fontspector/commit/0d8af42ffce929a8e54a90bdab84884c472c7c5f))
    - Use cache to determine codepoints in font ([`0514efc`](https://github.com/fonttools/fontspector/commit/0514efcf5e99d3c157fad5795816183d8f84e091))
    - Summarize super long title ([`8d4606f`](https://github.com/fonttools/fontspector/commit/8d4606f5da6ea8f66cc5e5adb2a04e1dd81f8aea))
    - Hide the blocked checks, we can't do anything about them ([`df298c1`](https://github.com/fonttools/fontspector/commit/df298c1a2a86b73521a4c208ce13d924d80ffe41))
    - NameID Display changed! ([`a12c7b0`](https://github.com/fonttools/fontspector/commit/a12c7b05939673a0f83b1d1e56ac6d9d227296db))
    - More micro-optimizations ([`7886854`](https://github.com/fonttools/fontspector/commit/788685487526a9d2d10a4b4466c59ebe307bd432))
    - Optimizations ([`dc71848`](https://github.com/fonttools/fontspector/commit/dc7184813e71e56c302d84bb18a06f9ae37747c8))
    - Export a DEFAULT_LOCATION setting ([`1ab59e9`](https://github.com/fonttools/fontspector/commit/1ab59e9064181e168765ea3f6cab9d8a28ddac5c))
    - Move all pens to a utility module in checkapi ([`8f86fd5`](https://github.com/fonttools/fontspector/commit/8f86fd56087c660943f39957d5471d865d2755fd))
    - Alt_caron ([`e878068`](https://github.com/fonttools/fontspector/commit/e8780680b042eb4800b069e2456222640ff00f75))
    - Fontations bug now fixed ([`f33b018`](https://github.com/fonttools/fontspector/commit/f33b0186cd55027ffbd564ba4422e102c524503b))
    - Silence warning ([`d2dad2a`](https://github.com/fonttools/fontspector/commit/d2dad2a5caacc8aab4de527787c7056a14d7bc94))
    - Italic angle check ([`73f7f01`](https://github.com/fonttools/fontspector/commit/73f7f01d9ee72a1807f4e1e6c06d8bb51fa4c605))
    - Gdef_non_mark_chars ([`73c02ec`](https://github.com/fonttools/fontspector/commit/73c02ec6de4d196ad7b3a5415e32c83462892f20))
    - Loca_maxp_num_glyphs ([`3f54264`](https://github.com/fonttools/fontspector/commit/3f542641bb55a8dd0089e1353912347eb464a63c))
    - Update legacy checks proposal field ([`ad3861e`](https://github.com/fonttools/fontspector/commit/ad3861e292ef2e1cbf118d5df8329c961123c90c))
    - Another check ([`9031c2a`](https://github.com/fonttools/fontspector/commit/9031c2adc5257bad10605d3962f78e08894aa92c))
    - More tests passing ([`43a758f`](https://github.com/fonttools/fontspector/commit/43a758f6a57ac82075e34775e2d8e21016a3c66a))
    - GDEF tests ([`a7c0d86`](https://github.com/fonttools/fontspector/commit/a7c0d86a9284a8293bdd8548754ec0b6e4d9dfcb))
    - Pass tests ([`e2ea2c8`](https://github.com/fonttools/fontspector/commit/e2ea2c820482310f8cfe83801701d03ac81b404b))
    - Expose FeatureRecord/Feature tables nicely ([`3a23051`](https://github.com/fonttools/fontspector/commit/3a230516002dbb17473a12c01c63b1e584dc0b1b))
    - Share itertools versions ([`71e6f81`](https://github.com/fonttools/fontspector/commit/71e6f81d35e3fbe8540a38ec532e382effa87459))
    - Debugging prints... ([`326b634`](https://github.com/fonttools/fontspector/commit/326b634e7eb36ddf2e445d722019e91befa0850f))
    - Everyone gets unicode-properties! ([`6218042`](https://github.com/fonttools/fontspector/commit/621804215cf361f7d515be71b71e8bd84bc481d7))
    - More passing tets ([`c9ae50f`](https://github.com/fonttools/fontspector/commit/c9ae50f2cf8727b11cf88681c00deda57ecf1825))
    - More! More! More! ([`c74c661`](https://github.com/fonttools/fontspector/commit/c74c66100b01403296c5843bb040cec58548b0a6))
    - Moah tests! ([`a4e1941`](https://github.com/fonttools/fontspector/commit/a4e1941d95944aefc0dc591b3a78e3e94a54e5f6))
    - Choose how we fail for assert_all_the_same ([`f219a34`](https://github.com/fonttools/fontspector/commit/f219a3494453e052b9da509edbb63ba1bf4f7dc4))
    - More tests ([`f4844a6`](https://github.com/fonttools/fontspector/commit/f4844a6723602657429abd1b3aa8fd2af5cf7737))
    - Bump read/write/skrifa versions, dump font-types, deal with fallout ([`d2fd7e4`](https://github.com/fonttools/fontspector/commit/d2fd7e4be7f70b014776c6a56ec035b5156692c0))
    - Make the tests pass ([`e80c95d`](https://github.com/fonttools/fontspector/commit/e80c95ddcaaea959e33e4664cdb423aa110904f6))
    - Add monospace check ([`6fa5520`](https://github.com/fonttools/fontspector/commit/6fa552004580b1ac1dd9822645790d08dfc5b6fe))
    - Improve glyph name API - move unwraps into API lib ([`2a094be`](https://github.com/fonttools/fontspector/commit/2a094bea6bbe22e15320c521aebbe493f3bb4c3c))
    - Use read-fonts' glyph class constants ([`3c41053`](https://github.com/fonttools/fontspector/commit/3c41053289a71d555710a66acc7cfc61cc2402ab))
    - Add --full-lists ([`8e1ae0b`](https://github.com/fonttools/fontspector/commit/8e1ae0b994b7b050c12245b32116d561554d9523))
    - Run code-tests in CI ([`ca20b6f`](https://github.com/fonttools/fontspector/commit/ca20b6fcaaaef95ad17d1224aa7f758757330ed2))
    - Remove unused leftover code ([`40af1bf`](https://github.com/fonttools/fontspector/commit/40af1bffbf22d17fcdbad5707bc45752b01ac277))
    - More alignment between checks and tests ([`523ad07`](https://github.com/fonttools/fontspector/commit/523ad0775d2f95306aa25bf96a14ef5e2acebc5a))
    - Align check with tests ([`338a433`](https://github.com/fonttools/fontspector/commit/338a43366003ef8928159cd5242a511a6e8daf12))
    - Pass a little more ([`b92af1d`](https://github.com/fonttools/fontspector/commit/b92af1d6d37648fa48d7506312bf70d6fc286a05))
    - Happening in pytest now ([`2a628d2`](https://github.com/fonttools/fontspector/commit/2a628d2f18d9a8abf19d3e872655f91bf04a1ef5))
    - Use fontbakery-bridge tests ([`1907f2b`](https://github.com/fonttools/fontspector/commit/1907f2b1a48cbe1f3978bc73554a127d5afd9a31))
    - Fix bugs found by Python test! ([`0955eec`](https://github.com/fonttools/fontspector/commit/0955eecad72a157be5b2c374a11ea08effcc8c42))
    - Missed ([`c5b9291`](https://github.com/fonttools/fontspector/commit/c5b929148096fafa19e0869e53021ae7c0e6f62f))
    - Postscript_name_consistency check ([`388e4bc`](https://github.com/fonttools/fontspector/commit/388e4bc0d08b6f14ad4dadbd57a0770cb34c8f59))
    - Name_postscript_vs_cff check ([`eb33242`](https://github.com/fonttools/fontspector/commit/eb33242909995d7ada66f9d57bf9998b00e3ab20))
    - Skip, don’t error, on CFF ([`e94a6c0`](https://github.com/fonttools/fontspector/commit/e94a6c0ad841ba15c7ef7ccdef9fdf71225ced29))
    - Move 'name/italic_names' to Universal profile. ([`0ce2aef`](https://github.com/fonttools/fontspector/commit/0ce2aef4e63fb53b278176bcb6516f86b6117e93))
    - Move 'name/no_copyright_on_description' to Universal profile. ([`0eaff9d`](https://github.com/fonttools/fontspector/commit/0eaff9d86043c79b29a3035722e1520fd67e6644))
    - Lack of STAT table is a skip, not an error ([`c664f31`](https://github.com/fonttools/fontspector/commit/c664f31758326c4f09b6373281ee2b76286271b9))
    - Some leftovers from universal/opentype split ([`c71f0da`](https://github.com/fonttools/fontspector/commit/c71f0da94f981a4bc69bda022ad1a2039a17f0d9))
    - Don’t overflow ([`95fd245`](https://github.com/fonttools/fontspector/commit/95fd2455cc140848e7f92f1ea3ff585e5a3c12a2))
    - Merge pull request #15 from felipesanches/issue_14 ([`57a2274`](https://github.com/fonttools/fontspector/commit/57a2274c13a2ac02292eaf60ec37f7cb63098304))
    - Split profiles Universal and OpenType ([`72550af`](https://github.com/fonttools/fontspector/commit/72550af9c9c8f9a9f4dad37a52f789290b4f6fb8))
</details>

