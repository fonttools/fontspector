# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.6.0 (2026-02-04)

### Chore

 - <csr-id-67faec64118db1aa7f5ef5399ea543a89494a872/> Move from rustybuzz to harfrust
 - <csr-id-3ec28c2864447022ed246cb04cfeb514889dc72d/> Add 'Allkin' to reserved font name exceptions

### New Features

 - <csr-id-09d3d5fe9eaa141aaf938cbc256fe6afc8ede513/> add check to validate fvar axis ranges against GF Axis Registry
 - <csr-id-4c3a73b87c64b01d7deea104960c36e3100c1f29/> add check to validate primary_script and primary_language
   * feat(googlefonts): add check to validate primary_script and primary_language
   
   Adds a new check `googlefonts/metadata/valid_primary_script_language` that
   validates METADATA.pb primary_script and primary_language fields against
   the google-fonts-languages data.
   
   - primary_script must be a valid ISO 15924 script code
   - primary_language must be a valid language ID in format 'lang_Script'
   
   * test(googlefonts): Tests for valid_primary_script_language
   
   ---------

### Bug Fixes

 - <csr-id-da27a2f7621a9974cbb5083005462c36576cda17/> Stroke and classifications should be uppercase

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 25 calendar days.
 - 47 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 5 unique issues were worked on: [#567](https://github.com/fonttools/fontspector/issues/567), [#570](https://github.com/fonttools/fontspector/issues/570), [#576](https://github.com/fonttools/fontspector/issues/576), [#581](https://github.com/fonttools/fontspector/issues/581), [#589](https://github.com/fonttools/fontspector/issues/589)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#567](https://github.com/fonttools/fontspector/issues/567)**
    - Add check to validate primary_script and primary_language ([`4c3a73b`](https://github.com/fonttools/fontspector/commit/4c3a73b87c64b01d7deea104960c36e3100c1f29))
 * **[#570](https://github.com/fonttools/fontspector/issues/570)**
    - Stroke and classifications should be uppercase ([`da27a2f`](https://github.com/fonttools/fontspector/commit/da27a2f7621a9974cbb5083005462c36576cda17))
 * **[#576](https://github.com/fonttools/fontspector/issues/576)**
    - Add check to validate fvar axis ranges against GF Axis Registry ([`09d3d5f`](https://github.com/fonttools/fontspector/commit/09d3d5fe9eaa141aaf938cbc256fe6afc8ede513))
 * **[#581](https://github.com/fonttools/fontspector/issues/581)**
    - Move from rustybuzz to harfrust ([`67faec6`](https://github.com/fonttools/fontspector/commit/67faec64118db1aa7f5ef5399ea543a89494a872))
 * **[#589](https://github.com/fonttools/fontspector/issues/589)**
    - Add 'Allkin' to reserved font name exceptions ([`3ec28c2`](https://github.com/fonttools/fontspector/commit/3ec28c2864447022ed246cb04cfeb514889dc72d))
</details>

## v1.5.0 (2025-12-17)

<csr-id-3aff895fb75d510fa826e19339424347b5d3ff61/>
<csr-id-4befd6c88900e2e06c363a8a6b1cdfc9518e9c91/>
<csr-id-36df1dbeb9feddc6cfde00db0ef69ad69f30da1b/>

### Chore

 - <csr-id-3aff895fb75d510fa826e19339424347b5d3ff61/> Refresh dependencies
   * chore: Refresh dependencies
   
   * chore: Fixup fontc API
 - <csr-id-4befd6c88900e2e06c363a8a6b1cdfc9518e9c91/> More from rustybuzz to harfrust
   * chore: Move from rustybuzz to harfrust
   
   * test(googlefonts/shaping/forbidden): Move tests to Rust
   
   * test: Pass full config to tests
   
   * chore: Missing docstrings
   
   * chore: not my fail
 - <csr-id-36df1dbeb9feddc6cfde00db0ef69ad69f30da1b/> thanks clippy

### New Features

 - <csr-id-55422bde203d9f312b5a4b9e625b4ec1f477f633/> Valid classifications and stroke
   * add check for stroke and classifications in metadata.pb
* rustfmt
* remove valid_stroke_and_classifications.rs
* add the check in validate.rs
* removed mod valid_stroke_and_classifications; in mod.rs
* rustfmt
* update link (PR URL)
* chore(metadata/validate): Merge into existing validate check

### Bug Fixes

 - <csr-id-adc935eb2c6ef03e239cff57af2b2adc9344295d/> update rust crate scraper to 0.25.0
 - <csr-id-2a4e75dfd9989f505e5b72adf51277d3d0605b4b/> Improve CJK vmetrics fixing

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 55 calendar days.
 - 64 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#316](https://github.com/fonttools/fontspector/issues/316), [#523](https://github.com/fonttools/fontspector/issues/523), [#531](https://github.com/fonttools/fontspector/issues/531), [#533](https://github.com/fonttools/fontspector/issues/533), [#551](https://github.com/fonttools/fontspector/issues/551), [#552](https://github.com/fonttools/fontspector/issues/552)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#316](https://github.com/fonttools/fontspector/issues/316)**
    - Improve CJK vmetrics fixing ([`2a4e75d`](https://github.com/fonttools/fontspector/commit/2a4e75dfd9989f505e5b72adf51277d3d0605b4b))
 * **[#523](https://github.com/fonttools/fontspector/issues/523)**
    - Thanks clippy ([`36df1db`](https://github.com/fonttools/fontspector/commit/36df1dbeb9feddc6cfde00db0ef69ad69f30da1b))
 * **[#531](https://github.com/fonttools/fontspector/issues/531)**
    - More from rustybuzz to harfrust ([`4befd6c`](https://github.com/fonttools/fontspector/commit/4befd6c88900e2e06c363a8a6b1cdfc9518e9c91))
 * **[#533](https://github.com/fonttools/fontspector/issues/533)**
    - Valid classifications and stroke ([`55422bd`](https://github.com/fonttools/fontspector/commit/55422bde203d9f312b5a4b9e625b4ec1f477f633))
 * **[#551](https://github.com/fonttools/fontspector/issues/551)**
    - Update rust crate scraper to 0.25.0 ([`adc935e`](https://github.com/fonttools/fontspector/commit/adc935eb2c6ef03e239cff57af2b2adc9344295d))
 * **[#552](https://github.com/fonttools/fontspector/issues/552)**
    - Refresh dependencies ([`3aff895`](https://github.com/fonttools/fontspector/commit/3aff895fb75d510fa826e19339424347b5d3ff61))
 * **Uncategorized**
    - Release fontspector-checkapi v1.3.1, fontspector-profile-fontwerk v1.2.1, fontspector-profile-googlefonts v1.5.0, fontspector-profile-universal v1.4.0, fontspector v1.5.2 ([`2779526`](https://github.com/fonttools/fontspector/commit/2779526c00f235ea93e95882b4ebd2b41786c715))
</details>

## v1.4.1 (2025-10-14)

<csr-id-244233ddd1a33b6279eec07324a77d3c4d14af6c/>
<csr-id-53fa4d31e4ac2c24d7f0a06ad18d2b8750776240/>

### Chore

 - <csr-id-244233ddd1a33b6279eec07324a77d3c4d14af6c/> Use gf-metadata crate for protobuf access
   * chore: Use gf-metadata crate instead of compiling our own protos
   
   * chore(ci): By rights we no longer need protoc, let's see if that's true
 - <csr-id-53fa4d31e4ac2c24d7f0a06ad18d2b8750776240/> Update reserved_font_name_exceptions.txt
   Added Zalando Sans

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 29 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#317](https://github.com/fonttools/fontspector/issues/317), [#489](https://github.com/fonttools/fontspector/issues/489)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#317](https://github.com/fonttools/fontspector/issues/317)**
    - Update reserved_font_name_exceptions.txt ([`53fa4d3`](https://github.com/fonttools/fontspector/commit/53fa4d31e4ac2c24d7f0a06ad18d2b8750776240))
 * **[#489](https://github.com/fonttools/fontspector/issues/489)**
    - Use gf-metadata crate for protobuf access ([`244233d`](https://github.com/fonttools/fontspector/commit/244233ddd1a33b6279eec07324a77d3c4d14af6c))
 * **Uncategorized**
    - Release fontspector-checkapi v1.3.0, fontspector-profile-googlefonts v1.4.1, fontspector-profile-universal v1.3.0, fontspector v1.5.1 ([`7b3d29e`](https://github.com/fonttools/fontspector/commit/7b3d29e9dab0c4bf11506345219e59b378291be2))
</details>

## v1.4.0 (2025-09-15)

<csr-id-90205a8089d1394f957cdf13cdcc461b73824425/>
<csr-id-abd4d4cf426666d0dac52706a763fb01d0e85d2c/>

### New Features

 - <csr-id-66839cbfb32380dd2b45184c82722f3fe4083341/> Display Unicode names of missing codepoints

### Bug Fixes

 - <csr-id-bcb694b8d657e273a0086c0ab504223bb741851a/> update rust crate scraper to 0.24.0
 - <csr-id-e6a7be64f557673eabf1c7e605fa443d1fea7aac/> Don't lowercase filenames before testing
 - <csr-id-d58c9ef800041bb3ec8cd266907d4c03f1b548c9/> Improve formatting of bulleted lists, fixes #352
 - <csr-id-ddcdb233646c320153e61319810b69a223e3952d/> Skip check if font is new
 - <csr-id-dc046deddb76c9aec8e0cfba84fb69d3e5cf397b/> Correctl deserialize remote_styles
 - <csr-id-525b27b4f24d91acad5ad0c0c101135592698b95/> Only check for regressions if we have the font already
   * fix(googlefonts/cjk_vertical_metrics_regressions): Only check for regressions if we have the font already
* chore(ci): apt-get update before installing

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

 - 11 commits contributed to the release over the course of 68 calendar days.
 - 9 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 8 unique issues were worked on: [#313](https://github.com/fonttools/fontspector/issues/313), [#325](https://github.com/fonttools/fontspector/issues/325), [#327](https://github.com/fonttools/fontspector/issues/327), [#339](https://github.com/fonttools/fontspector/issues/339), [#357](https://github.com/fonttools/fontspector/issues/357), [#394](https://github.com/fonttools/fontspector/issues/394), [#399](https://github.com/fonttools/fontspector/issues/399), [#418](https://github.com/fonttools/fontspector/issues/418)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#313](https://github.com/fonttools/fontspector/issues/313)**
    - Display Unicode names of missing codepoints ([`66839cb`](https://github.com/fonttools/fontspector/commit/66839cbfb32380dd2b45184c82722f3fe4083341))
 * **[#325](https://github.com/fonttools/fontspector/issues/325)**
    - Only check for regressions if we have the font already ([`525b27b`](https://github.com/fonttools/fontspector/commit/525b27b4f24d91acad5ad0c0c101135592698b95))
 * **[#327](https://github.com/fonttools/fontspector/issues/327)**
    - Correctl deserialize remote_styles ([`dc046de`](https://github.com/fonttools/fontspector/commit/dc046deddb76c9aec8e0cfba84fb69d3e5cf397b))
 * **[#339](https://github.com/fonttools/fontspector/issues/339)**
    - Skip check if font is new ([`ddcdb23`](https://github.com/fonttools/fontspector/commit/ddcdb233646c320153e61319810b69a223e3952d))
 * **[#357](https://github.com/fonttools/fontspector/issues/357)**
    - Improve formatting of bulleted lists, fixes #352 ([`d58c9ef`](https://github.com/fonttools/fontspector/commit/d58c9ef800041bb3ec8cd266907d4c03f1b548c9))
 * **[#394](https://github.com/fonttools/fontspector/issues/394)**
    - Don't lowercase filenames before testing ([`e6a7be6`](https://github.com/fonttools/fontspector/commit/e6a7be64f557673eabf1c7e605fa443d1fea7aac))
 * **[#399](https://github.com/fonttools/fontspector/issues/399)**
    - Update rust crate scraper to 0.24.0 ([`bcb694b`](https://github.com/fonttools/fontspector/commit/bcb694b8d657e273a0086c0ab504223bb741851a))
 * **[#418](https://github.com/fonttools/fontspector/issues/418)**
    - Move the tests from Python to Rust ([`abd4d4c`](https://github.com/fonttools/fontspector/commit/abd4d4cf426666d0dac52706a763fb01d0e85d2c))
 * **Uncategorized**
    - Release fontspector-checkapi v1.2.0, fontspector-profile-fontwerk v1.2.0, fontspector-profile-googlefonts v1.4.0, fontspector-profile-opentype v1.3.0, fontspector-profile-universal v1.2.0, fontspector-profile-iso15008 v1.0.4, fontspector v1.5.0 ([`de4a966`](https://github.com/fonttools/fontspector/commit/de4a966105bea222ea98da69793ddbfbdd590f9d))
    - Fontspector-checkapi v1.1.2, fontspector-fontbakery-bridge v1.2.0, fontspector-profile-fontwerk v1.1.0, fontspector-profile-googlefonts v1.3.0, fontspector-profile-opentype v1.2.1, fontspector-profile-universal v1.1.2, fontspector v1.4.0 ([`90205a8`](https://github.com/fonttools/fontspector/commit/90205a8089d1394f957cdf13cdcc461b73824425))
    - Release fontspector-fontbakery-bridge v1.1.0, fontspector-profile-googlefonts v1.2.0, fontspector v1.3.0 ([`05adc63`](https://github.com/fonttools/fontspector/commit/05adc636ddf710008076985ab8c2a03ea19a0f6c))
</details>

## v1.3.0 (2025-08-11)

### New Features

 - <csr-id-66839cbfb32380dd2b45184c82722f3fe4083341/> Display Unicode names of missing codepoints

### Bug Fixes

 - <csr-id-d58c9ef800041bb3ec8cd266907d4c03f1b548c9/> Improve formatting of bulleted lists, fixes #352
 - <csr-id-ddcdb233646c320153e61319810b69a223e3952d/> Skip check if font is new
 - <csr-id-dc046deddb76c9aec8e0cfba84fb69d3e5cf397b/> Correctl deserialize remote_styles
 - <csr-id-525b27b4f24d91acad5ad0c0c101135592698b95/> Only check for regressions if we have the font already
   * fix(googlefonts/cjk_vertical_metrics_regressions): Only check for regressions if we have the font already
* chore(ci): apt-get update before installing

## v1.2.0 (2025-07-11)

### New Features

 - <csr-id-66839cbfb32380dd2b45184c82722f3fe4083341/> Display Unicode names of missing codepoints

### Bug Fixes

 - <csr-id-dc046deddb76c9aec8e0cfba84fb69d3e5cf397b/> Correctl deserialize remote_styles
 - <csr-id-525b27b4f24d91acad5ad0c0c101135592698b95/> Only check for regressions if we have the font already
   * fix(googlefonts/cjk_vertical_metrics_regressions): Only check for regressions if we have the font already
* chore(ci): apt-get update before installing

## v1.1.1 (2025-07-02)

<csr-id-ffc7318ba547efe312e4d28508696b85f408d6a2/>
<csr-id-138edbbfba88008d71d9247eccbdfc017fef8b81/>
<csr-id-a6b7ffc4f39c6b1c1bd92cd9b07f4ba22d54ef2e/>

### Chore

 - <csr-id-ffc7318ba547efe312e4d28508696b85f408d6a2/> check version string name presence in mandatory_entries
   * chore: Remove unwanted package-lock
   
   * chore(googlefonts): Check version string presence in mandatory_entries, not version_format
   
   * test(googlefonts): Update test expectations for #44
 - <csr-id-138edbbfba88008d71d9247eccbdfc017fef8b81/> Update fontations dependencies
   * chore: Update fontations dependencies
   
   * test(italic_angle): Update bounds check

### Bug Fixes

<csr-id-7c02a3ba9acd19b519aa5e5a65bb8fc68bd7fe9f/>
<csr-id-3a8cd3f220746bb67b33863ee3ec1125d1ad0f3b/>
<csr-id-46e90e51624979590af83272f96cbcfc521b7d0a/>
<csr-id-faed004625c34224d986f191ce6f41356cea5d97/>

 - <csr-id-f9388545e3093cc20a8e549ecef9db5b618dd4fd/> Merge subsets_correct and includes_production_subsets
   * fix(googlefonts/metadata/subsets_correct): Merge subsets_correct and includes_production_subsets (#293)
* test(googlefonts/metadata/subsets_correct): Merge subsets_correct and includes_production_subsets (#293)
* fix(googlefonts/metadata/consistent_repo_urls): Correctly parse URL in OFL text (#296)
* chore: Style fixes for new clippy
* chore: Style fixes for new clippy
* fix(cli): Improve rationale rewrapping
* chore: Style fixes for new clippy
* Add failing unittest for googlefonts/gasp
* fix unittest
* Add skip to googlefonts/gasp
* Fix(Lint)

### Style

 - <csr-id-a6b7ffc4f39c6b1c1bd92cd9b07f4ba22d54ef2e/> deny indexing slicing
   * chore: More lints into Cargo.toml
   
   * style: Deny indexing slicing

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 11 calendar days.
 - 12 days passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 9 unique issues were worked on: [#286](https://github.com/fonttools/fontspector/issues/286), [#287](https://github.com/fonttools/fontspector/issues/287), [#291](https://github.com/fonttools/fontspector/issues/291), [#293](https://github.com/fonttools/fontspector/issues/293), [#296](https://github.com/fonttools/fontspector/issues/296), [#299](https://github.com/fonttools/fontspector/issues/299), [#302](https://github.com/fonttools/fontspector/issues/302), [#304](https://github.com/fonttools/fontspector/issues/304), [#306](https://github.com/fonttools/fontspector/issues/306)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#286](https://github.com/fonttools/fontspector/issues/286)**
    - Add skip for cff fonts ([`faed004`](https://github.com/fonttools/fontspector/commit/faed004625c34224d986f191ce6f41356cea5d97))
 * **[#287](https://github.com/fonttools/fontspector/issues/287)**
    - Deny indexing slicing ([`a6b7ffc`](https://github.com/fonttools/fontspector/commit/a6b7ffc4f39c6b1c1bd92cd9b07f4ba22d54ef2e))
 * **[#291](https://github.com/fonttools/fontspector/issues/291)**
    - Update fontations dependencies ([`138edbb`](https://github.com/fonttools/fontspector/commit/138edbbfba88008d71d9247eccbdfc017fef8b81))
 * **[#293](https://github.com/fonttools/fontspector/issues/293)**
    - Merge subsets_correct and includes_production_subsets ([`f938854`](https://github.com/fonttools/fontspector/commit/f9388545e3093cc20a8e549ecef9db5b618dd4fd))
    - Check version string name presence in mandatory_entries ([`ffc7318`](https://github.com/fonttools/fontspector/commit/ffc7318ba547efe312e4d28508696b85f408d6a2))
 * **[#296](https://github.com/fonttools/fontspector/issues/296)**
    - Use best family name for consistency with METADATA.pb ([`7c02a3b`](https://github.com/fonttools/fontspector/commit/7c02a3ba9acd19b519aa5e5a65bb8fc68bd7fe9f))
    - Correctly parse URL in OFL text ([`3a8cd3f`](https://github.com/fonttools/fontspector/commit/3a8cd3f220746bb67b33863ee3ec1125d1ad0f3b))
 * **[#299](https://github.com/fonttools/fontspector/issues/299)**
    - Improve rationale rewrapping ([`46e90e5`](https://github.com/fonttools/fontspector/commit/46e90e51624979590af83272f96cbcfc521b7d0a))
 * **[#302](https://github.com/fonttools/fontspector/issues/302)**
    - Correctly parse URL in OFL text ([`3a8cd3f`](https://github.com/fonttools/fontspector/commit/3a8cd3f220746bb67b33863ee3ec1125d1ad0f3b))
 * **[#304](https://github.com/fonttools/fontspector/issues/304)**
    - Use best family name for consistency with METADATA.pb ([`7c02a3b`](https://github.com/fonttools/fontspector/commit/7c02a3ba9acd19b519aa5e5a65bb8fc68bd7fe9f))
 * **[#306](https://github.com/fonttools/fontspector/issues/306)**
    - Merge subsets_correct and includes_production_subsets ([`f938854`](https://github.com/fonttools/fontspector/commit/f9388545e3093cc20a8e549ecef9db5b618dd4fd))
 * **Uncategorized**
    - Release fontspector-checkapi v1.1.1, fontspector-profile-opentype v1.2.0, fontspector-profile-googlefonts v1.1.1, fontspector-profile-universal v1.1.1, fontspector v1.2.0 ([`f407a9a`](https://github.com/fonttools/fontspector/commit/f407a9aaf0aae501443842311f1b5c27eab007b6))
</details>

## v1.1.0 (2025-06-19)

<csr-id-65132c0dd6dfae2e1ef30c994dee9135fe7dbf33/>
<csr-id-f44be5515dcaea17b96b1df7a4b11407561d0c17/>
<csr-id-8b28d1aa1c7af4dacdbcfbd83af69dbf401ecf46/>

### Chore

 - <csr-id-65132c0dd6dfae2e1ef30c994dee9135fe7dbf33/> Update font protos
 - <csr-id-f44be5515dcaea17b96b1df7a4b11407561d0c17/> Improve error handling
   * chore: Better error handling
   
   * chore: Better error handling for fix functions too

### New Features

<csr-id-5f06e2526b40e8a6852e5e1aad70fbfbba6753a1/>
<csr-id-725a78d0d14a2840e74dfa4014909bfac7b21d52/>
<csr-id-c26526c49bf0f30c842efafc4aa9414dece14699/>

 - <csr-id-ea5107c15304c96b035aab80551ce8ddb7e7e98b/> Add fix function
   * chore(api): add_table is not polymorphic so this idea just didn't work
* feat(cjk_vertical_metrics): Add fix function
* feat(googlefonts): New GF CJK vertical metrics schema
* test(googlefonts): New GF CJK vertical metrics schema
* feat(googlefonts/metadata/validate): Check no languages entries are present
* test(googlefonts/metadata/validate): Check no languages entries are present

### Bug Fixes

<csr-id-2e7c907463e093bb9e904ad783d63c0cd42628f8/>
<csr-id-c181f4647ba8c3750495046870144e03897904c3/>
<csr-id-5d058c99b38b636f6ec3130c10296ae664a3384d/>
<csr-id-f0420267c40ced0c8a5dcbb204cfc020c155969f/>
<csr-id-bc48fb1bc721e5b8b59780900e75c6e3ed177409/>

 - <csr-id-82398535287401e767098901b1da578809d28485/> Make list checks work, add permalinks
   * fix(web): Make list checks work, add permalinks (#162)
* feat(googlefonts): Allow soft_dotted check on wasm, why not
* fix(googlefonts): skip license/article checks when not in google/fonts (#142)
* fix(googlefonts/description/has_article): Change fail status to info
* test: Fixup tests
* test: Add missing file

### Refactor

 - <csr-id-8b28d1aa1c7af4dacdbcfbd83af69dbf401ecf46/> Dynamically update script and language tags from MS website

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 28 calendar days.
 - 30 days passed between releases.
 - 13 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 16 unique issues were worked on: [#120](https://github.com/fonttools/fontspector/issues/120), [#127](https://github.com/fonttools/fontspector/issues/127), [#137](https://github.com/fonttools/fontspector/issues/137), [#142](https://github.com/fonttools/fontspector/issues/142), [#144](https://github.com/fonttools/fontspector/issues/144), [#145](https://github.com/fonttools/fontspector/issues/145), [#150](https://github.com/fonttools/fontspector/issues/150), [#152](https://github.com/fonttools/fontspector/issues/152), [#162](https://github.com/fonttools/fontspector/issues/162), [#180](https://github.com/fonttools/fontspector/issues/180), [#200](https://github.com/fonttools/fontspector/issues/200), [#227](https://github.com/fonttools/fontspector/issues/227), [#230](https://github.com/fonttools/fontspector/issues/230), [#275](https://github.com/fonttools/fontspector/issues/275), [#280](https://github.com/fonttools/fontspector/issues/280), [#281](https://github.com/fonttools/fontspector/issues/281)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#120](https://github.com/fonttools/fontspector/issues/120)**
    - Check no languages entries are present ([`c26526c`](https://github.com/fonttools/fontspector/commit/c26526c49bf0f30c842efafc4aa9414dece14699))
 * **[#127](https://github.com/fonttools/fontspector/issues/127)**
    - Dynamically update script and language tags from MS website ([`8b28d1a`](https://github.com/fonttools/fontspector/commit/8b28d1aa1c7af4dacdbcfbd83af69dbf401ecf46))
 * **[#137](https://github.com/fonttools/fontspector/issues/137)**
    - Move to tabled to display tables correctly ([`bc48fb1`](https://github.com/fonttools/fontspector/commit/bc48fb1bc721e5b8b59780900e75c6e3ed177409))
 * **[#142](https://github.com/fonttools/fontspector/issues/142)**
    - Skip license/article checks when not in google/fonts ([`2e7c907`](https://github.com/fonttools/fontspector/commit/2e7c907463e093bb9e904ad783d63c0cd42628f8))
 * **[#144](https://github.com/fonttools/fontspector/issues/144)**
    - Update rust crate scraper to 0.23.0 ([`5d058c9`](https://github.com/fonttools/fontspector/commit/5d058c99b38b636f6ec3130c10296ae664a3384d))
 * **[#145](https://github.com/fonttools/fontspector/issues/145)**
    - Resolve path so there's always a directory ([`f042026`](https://github.com/fonttools/fontspector/commit/f0420267c40ced0c8a5dcbb204cfc020c155969f))
 * **[#150](https://github.com/fonttools/fontspector/issues/150)**
    - Skip checks on unlisted files ([`c181f46`](https://github.com/fonttools/fontspector/commit/c181f4647ba8c3750495046870144e03897904c3))
 * **[#152](https://github.com/fonttools/fontspector/issues/152)**
    - Skip license/article checks when not in google/fonts ([`2e7c907`](https://github.com/fonttools/fontspector/commit/2e7c907463e093bb9e904ad783d63c0cd42628f8))
 * **[#162](https://github.com/fonttools/fontspector/issues/162)**
    - Make list checks work, add permalinks ([`8239853`](https://github.com/fonttools/fontspector/commit/82398535287401e767098901b1da578809d28485))
 * **[#180](https://github.com/fonttools/fontspector/issues/180)**
    - New GF CJK vertical metrics schema ([`725a78d`](https://github.com/fonttools/fontspector/commit/725a78d0d14a2840e74dfa4014909bfac7b21d52))
 * **[#200](https://github.com/fonttools/fontspector/issues/200)**
    - Make list checks work, add permalinks ([`8239853`](https://github.com/fonttools/fontspector/commit/82398535287401e767098901b1da578809d28485))
 * **[#227](https://github.com/fonttools/fontspector/issues/227)**
    - Add designer name to mandatory name table entries ([`5f06e25`](https://github.com/fonttools/fontspector/commit/5f06e2526b40e8a6852e5e1aad70fbfbba6753a1))
 * **[#230](https://github.com/fonttools/fontspector/issues/230)**
    - Add designer name to mandatory name table entries ([`5f06e25`](https://github.com/fonttools/fontspector/commit/5f06e2526b40e8a6852e5e1aad70fbfbba6753a1))
 * **[#275](https://github.com/fonttools/fontspector/issues/275)**
    - Improve error handling ([`f44be55`](https://github.com/fonttools/fontspector/commit/f44be5515dcaea17b96b1df7a4b11407561d0c17))
 * **[#280](https://github.com/fonttools/fontspector/issues/280)**
    - Add fix function ([`ea5107c`](https://github.com/fonttools/fontspector/commit/ea5107c15304c96b035aab80551ce8ddb7e7e98b))
 * **[#281](https://github.com/fonttools/fontspector/issues/281)**
    - Update font protos ([`65132c0`](https://github.com/fonttools/fontspector/commit/65132c0dd6dfae2e1ef30c994dee9135fe7dbf33))
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
    - Release fontspector-profile-googlefonts v1.0.2, fontspector-profile-universal v1.0.2 ([`d26f105`](https://github.com/fonttools/fontspector/commit/d26f105058189e6baa1ccd726d2151851e4e7d85))
    - Bump axisregistry ([`a048609`](https://github.com/fonttools/fontspector/commit/a048609b7734f1b5f13d154118b2ffcc7d795b71))
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

<csr-id-5a95113ebe74c423d0ee31802f5a5cbb40621f87/>
<csr-id-7cc0e15f42ffbf1d512f2fa50d42fe12ba3aca44/>
<csr-id-d7968d62b6271d79869a3ebf34c1d20365482c6c/>
<csr-id-7fabefe24db197e83ac3eea33288c70664bb1679/>
<csr-id-73eab4b7168d41ea8b8d911a57d790de2d5fcf3d/>

### Other

 - <csr-id-5a95113ebe74c423d0ee31802f5a5cbb40621f87/> :axis_order
 - <csr-id-7cc0e15f42ffbf1d512f2fa50d42fe12ba3aca44/> move to Universal profile
   Still needs to be ported, though.
   
   (https://github.com/fonttools/fontbakery/pull/4937)
 - <csr-id-d7968d62b6271d79869a3ebf34c1d20365482c6c/> sync with latest fontbakery
 - <csr-id-7fabefe24db197e83ac3eea33288c70664bb1679/> further refactoring the code-testing helper functions
 - <csr-id-73eab4b7168d41ea8b8d911a57d790de2d5fcf3d/> refactoring codetesting functions

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 232 commits contributed to the release over the course of 301 calendar days.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #102 from fonttools/release-prep ([`e5435f4`](https://github.com/fonttools/fontspector/commit/e5435f4ab282338ccc818daca8dacf543de27022))
    - Use published versions ([`366f1dd`](https://github.com/fonttools/fontspector/commit/366f1dd71b32f2e78254d3b2a24cff4f0c2907cb))
    - Update Cargo.toml for release ([`f3d6b2a`](https://github.com/fonttools/fontspector/commit/f3d6b2a19102651508bcad4b2b38c2a399079149))
    - Prep for 1.0.0 release ([`c1ef822`](https://github.com/fonttools/fontspector/commit/c1ef822c860b8dd53b363c9b69201981c75f757c))
    - Merge pull request #100 from fonttools/iso15008 ([`c42f3f8`](https://github.com/fonttools/fontspector/commit/c42f3f8d0dfdbe97d9fa78342e135de0911d97fc))
    - Oh hey, this can be built on wasm too ([`7bfa741`](https://github.com/fonttools/fontspector/commit/7bfa74120579891bf2eb1b0f882914bd2781778d))
    - Restore warning format ([`dc8e610`](https://github.com/fonttools/fontspector/commit/dc8e610214a729a205d6950094db85d366af12a1))
    - Improve visual output ([`7ffb064`](https://github.com/fonttools/fontspector/commit/7ffb064209caadf9e0c308e6974726ee951edf40))
    - Merge pull request #99 from fonttools/rich-metadata ([`dfd2c49`](https://github.com/fonttools/fontspector/commit/dfd2c49e542a5c5def5929c6c5e5dbd30e5015bb))
    - Restore warning format ([`96ddee3`](https://github.com/fonttools/fontspector/commit/96ddee3be2d903b2eb53e17368d1bf39f456449e))
    - Improve visual output ([`ea6e3a9`](https://github.com/fonttools/fontspector/commit/ea6e3a9a0b280d7c27bf5aaaaa52613ed9b0fd8f))
    - Add some metadata to this check ([`96c7db0`](https://github.com/fonttools/fontspector/commit/96c7db0a11f412a0d8416ce56ed7847763f80e6a))
    - Merge pull request #96 from fonttools/non-ink-characters ([`1577008`](https://github.com/fonttools/fontspector/commit/15770084eaa140071658b5b6157ceb8174c8eb3a))
    - Add googlefonts/separator_glyphs ([`2db4e81`](https://github.com/fonttools/fontspector/commit/2db4e81a0cde8e3aeb851dd1402861c3ea689585))
    - Merge pull request #92 from fonttools/nixon-feedback ([`0b9a28b`](https://github.com/fonttools/fontspector/commit/0b9a28b9c647bfb7ec0f3ba8156d616fce82b37b))
    - Merge pull request #88 from fonttools/reduce-false-positives ([`dcf298d`](https://github.com/fonttools/fontspector/commit/dcf298d93ad3abe68d4f520f8e980914eb74c008))
    - Move skrifa dep to workspace ([`9475563`](https://github.com/fonttools/fontspector/commit/9475563d3da16cf982551f46dd50ec16e9264055))
    - Clarify which name table entry ([`13b9cb4`](https://github.com/fonttools/fontspector/commit/13b9cb456a22682febc0872bb52bd770461d7ebf))
    - Exclude pending_review checks included from other profiles ([`ac9259a`](https://github.com/fonttools/fontspector/commit/ac9259ad3f249700e03416b7c7fd79de239111ec))
    - Merge pull request #80 from fonttools/dependency-hell ([`b8ec37d`](https://github.com/fonttools/fontspector/commit/b8ec37d7d52f440fc2d6a9470ee2d3056df2d94c))
    - Reformat ([`ab0a4e4`](https://github.com/fonttools/fontspector/commit/ab0a4e4a5bbd316783438d0337782090a03e0a3f))
    - Use skrifa::raw instead of read_fonts, pin deps ([`76eacb7`](https://github.com/fonttools/fontspector/commit/76eacb755b79772e761b832b8fe8983af81e07fa))
    - Merge pull request #63 from LuxxxLucy/lucy-multiple-proposal-br ([`2d675d5`](https://github.com/fonttools/fontspector/commit/2d675d5bfe5cdb3de99e1a2cf8c65964c144bc52))
    - Merge pull request #78 from fonttools/dep-tidying ([`6633571`](https://github.com/fonttools/fontspector/commit/66335714c16c21c902d8459814a0b37ddfcddf5d))
    - Tidy dependencies ([`9a8c5fa`](https://github.com/fonttools/fontspector/commit/9a8c5face5eadbb2daffb606e4d42af052f73c7c))
    - Some small GF fixes ([`0c76c82`](https://github.com/fonttools/fontspector/commit/0c76c82841a4824cb0069f8a4a34f60c997e6a66))
    - Optimizations ([`84cd869`](https://github.com/fonttools/fontspector/commit/84cd869bc1d533fa9e6fa498fa61de65ff491290))
    - And we're done. ([`3dd1c08`](https://github.com/fonttools/fontspector/commit/3dd1c0899e787b8ab3729ebe8d83d5896da3df25))
    - Calm warning ([`338b726`](https://github.com/fonttools/fontspector/commit/338b726e262db12acaa9087f50b89ca64bad1fe9))
    - Vf_has_static_fonts ([`76b90e3`](https://github.com/fonttools/fontspector/commit/76b90e31219b21c07645e61ae58f85cb42f182f3))
    - Cjk_vertical_metrics_regressions ([`6ad4fd4`](https://github.com/fonttools/fontspector/commit/6ad4fd487c26b9e24b9d2432a7db5279c73129fa))
    - Vertical_metrics_regressions ([`37f214e`](https://github.com/fonttools/fontspector/commit/37f214e1ee60c169c5a96b842b80b4f811310c3e))
    - Article/images ([`c6ead4a`](https://github.com/fonttools/fontspector/commit/c6ead4ae106822a644046301ae27f4eae27116e2))
    - Metadata/weightclass ([`8a53ece`](https://github.com/fonttools/fontspector/commit/8a53ece204d94c40d41de1e3f384d75c714be804))
    - Unreachable_subsetting ([`3bd965f`](https://github.com/fonttools/fontspector/commit/3bd965f6874955e4a45fb782f50eaff3a6862997))
    - Shape_languages ([`78d7e92`](https://github.com/fonttools/fontspector/commit/78d7e92c9ddc71d79dc42a07eab3e98e2457480c))
    - Family_directory_name ([`160b57e`](https://github.com/fonttools/fontspector/commit/160b57e0d3bace10cff42ccd016da8e50ae7db98))
    - Tnum_horizontal_metrics ([`d150c59`](https://github.com/fonttools/fontspector/commit/d150c59d27183418229c5480862fe2747d80d959))
    - Italics_have_roman_counterparts ([`3ba53ab`](https://github.com/fonttools/fontspector/commit/3ba53ab9bcfb9ef888f1fd26afc4f63110f6bee1))
    - Includes_production_subsets ([`7bda88d`](https://github.com/fonttools/fontspector/commit/7bda88d78aa284cfa741bfd1739566b9f789f81f))
    - Cjk_vertical_metrics ([`f325a92`](https://github.com/fonttools/fontspector/commit/f325a927267da1f8f6c24dff89094d1311f53c75))
    - Get wasm building ([`ed79b11`](https://github.com/fonttools/fontspector/commit/ed79b11ad237e8c23146aeb402bf84f57d270739))
    - Designer_profiles ([`b2183b2`](https://github.com/fonttools/fontspector/commit/b2183b2f3f36daace67ffa367bc5196e1b88db13))
    - Double check we can use network ([`457c269`](https://github.com/fonttools/fontspector/commit/457c269a86b281fbeca3e2eb501981fadee4bca3))
    - Print statement begone ([`2c2320b`](https://github.com/fonttools/fontspector/commit/2c2320b8a019b7ad47fe1530c53fa52f6cb5aa6c))
    - Googlefonts/metadata/unique_weight_style_pairs ([`356559e`](https://github.com/fonttools/fontspector/commit/356559e47928bb1587812868b8ccd3a20458f2ff))
    - Googlefonts/metadata/primary_script ([`0c3a310`](https://github.com/fonttools/fontspector/commit/0c3a3104d64400325f639310afb0ee067a50721c))
    - Update the checks ([`4110dcf`](https://github.com/fonttools/fontspector/commit/4110dcfd1c79131aea9893523b50e0b0bdfd2f95))
    - Unique_full_name_values ([`17d7730`](https://github.com/fonttools/fontspector/commit/17d77309e2c0661a57ba097d81c693c175cf6420))
    - Single_cjk_subset ([`32b0bc6`](https://github.com/fonttools/fontspector/commit/32b0bc647aa112e474aa0359a706fba24cf67d77))
    - Vertical metrics ([`29d50df`](https://github.com/fonttools/fontspector/commit/29d50df2f50c2f6e6d414c9601e4995dee556cec))
    - OFL_copyright ([`cbf9994`](https://github.com/fonttools/fontspector/commit/cbf9994c74f7cc95c815a4301ae5dd0d78ad31b1))
    - Build on WASM again ([`601594f`](https://github.com/fonttools/fontspector/commit/601594fe2480fc3055df5d733940ab9e2417a423))
    - Only apply family name check to RIBBI fonts ([`9f125d9`](https://github.com/fonttools/fontspector/commit/9f125d93dc55826e042ba42f238b10e7ef1d038b))
    - License/OFL_body_text ([`64f2f2a`](https://github.com/fonttools/fontspector/commit/64f2f2a873cc7236047d14acefc66c3a253e2c08))
    - Consistent_repo_urls ([`a0b1e19`](https://github.com/fonttools/fontspector/commit/a0b1e19b28cec0130937d5ee9c917354700b5781))
    - Googlefonts/metadata/nameid/family_and_full_names plus some tidying ([`4d49537`](https://github.com/fonttools/fontspector/commit/4d49537725237b8b335369e995c9e897cf6d51be))
    - Metadata/broken_links ([`a5e7d90`](https://github.com/fonttools/fontspector/commit/a5e7d90d5063268f353f259c8b9a8f4fc22eeab1))
    - Valid_nameid25 ([`4c945fc`](https://github.com/fonttools/fontspector/commit/4c945fc8e6371a149ee847cc9a67565f9d31bf66))
    - Minisite_url, plus a few more tests ([`4e2fd9c`](https://github.com/fonttools/fontspector/commit/4e2fd9c8106858e83800665301f968f6cc50a92f))
    - Category_hints ([`ad6bd69`](https://github.com/fonttools/fontspector/commit/ad6bd691e6734e19add47cc2758e73b0e421bdbd))
    - Four more ([`ddaa9c5`](https://github.com/fonttools/fontspector/commit/ddaa9c5d004a895885d4c4ac1932af1f2600cd85))
    - Two more ([`12ae33e`](https://github.com/fonttools/fontspector/commit/12ae33ec6a2e838e6a6cc181527ff7cff686ab66))
    - Three more metadata checks ([`5786120`](https://github.com/fonttools/fontspector/commit/578612032b0af5639786cc3819962d2bfabdea36))
    - Four metadata checks ([`71de9c4`](https://github.com/fonttools/fontspector/commit/71de9c48f21df9e2e23f5daa25deffb8f05209a5))
    - Googlefonts/name/license_url check ([`0a0336f`](https://github.com/fonttools/fontspector/commit/0a0336fbb2eb225c99c3b3d1098384ecae89388b))
    - Font_copyright ([`e6b0213`](https://github.com/fonttools/fontspector/commit/e6b0213528ca9f5c51d0e96eaac60529c085946d))
    - Has_license ([`1f82a85`](https://github.com/fonttools/fontspector/commit/1f82a857479a4ecccea5e3444077f7a457fc25f3))
    - Fix wasm build ([`40f7588`](https://github.com/fonttools/fontspector/commit/40f7588a50e36c2515bb4c12067577b2ed4bf901))
    - Glyph_coverage ([`2442947`](https://github.com/fonttools/fontspector/commit/2442947c7c222da476294961ec392cec1ac31230))
    - Googlefonts/version_bump ([`3645af0`](https://github.com/fonttools/fontspector/commit/3645af0a2b57bf67cabb8ec5a6eb2182b88da63e))
    - Finish up axes_match check ([`517c0b2`](https://github.com/fonttools/fontspector/commit/517c0b2bc7a03b73c45b5cabea6668c861fdf7ab))
    - Remote_styles condition ([`b250614`](https://github.com/fonttools/fontspector/commit/b2506141e767e855d393d2e47d797ee3623dfe3b))
    - Canonical filename check ([`b64c5d3`](https://github.com/fonttools/fontspector/commit/b64c5d3250bb13b7c21434fbdd01dc121cc70372))
    - Improve googlefonts/metadata/has_regular ([`8038241`](https://github.com/fonttools/fontspector/commit/803824124dab4d1e3a15ad184aa2aa6dedf0ed90))
    - Googlefonts/metadata/regular_is_400 ([`51906e4`](https://github.com/fonttools/fontspector/commit/51906e422063fb51cf0dad86af5cffac95cc52bd))
    - Googlefonts/metadata/has_regular ([`1465354`](https://github.com/fonttools/fontspector/commit/146535494ca58195d657d62f3c52ccad886be3f9))
    - Don't stringify segments, it's slow ([`5eaefe5`](https://github.com/fonttools/fontspector/commit/5eaefe525a754a99a8f3f5a82b1ca96dabad4017))
    - Update vesions, minimize dependencies ([`8f43370`](https://github.com/fonttools/fontspector/commit/8f433709f66727148a18278383c3b519ce99e331))
    - Use assert_all_the_same helper method ([`b5995ae`](https://github.com/fonttools/fontspector/commit/b5995ae192c216758fe84e2630de44e78589ecf9))
    - Using protobuf methods is sometimes more convenient ;-) ([`f57a29c`](https://github.com/fonttools/fontspector/commit/f57a29cfdd95d6660530db3b594c0b7b01f50e74))
    - Followup to 07d49837a0182e539f0921e18aadec416b73c6b8 ([`3e96438`](https://github.com/fonttools/fontspector/commit/3e96438ee263e572b86a60c249c39d3ce7bffa78))
    - More WASM fixes ([`36d3108`](https://github.com/fonttools/fontspector/commit/36d310805605e8860bb2b197540756a0842b7b89))
    - Build on wasm ([`6efb22f`](https://github.com/fonttools/fontspector/commit/6efb22fa7e92a4ec611c28308893b25ca0889a6c))
    - Finish off and fix shaping checks ([`40b9027`](https://github.com/fonttools/fontspector/commit/40b902782a0bb0cd4558201b24bc6511fa0639db))
    - Soft_dotted check ([`d979af0`](https://github.com/fonttools/fontspector/commit/d979af07ef61d2f9821af117d8282c29544183fa))
    - Generalize, add shaping/forbidden ([`53d9ccb`](https://github.com/fonttools/fontspector/commit/53d9ccbeaa1c52a593a185c0180995b10d1523fe))
    - Shaping/regression ([`0adf783`](https://github.com/fonttools/fontspector/commit/0adf783b40ec1c07165fa4273900ec3a9ca8e5dd))
    - Googlefont/dotted_circle ([`2977b0e`](https://github.com/fonttools/fontspector/commit/2977b0edcb4af4ffd75b26da5fd5760d38c8da79))
    - Address  error: useless use of `vec!` ([`83eddf2`](https://github.com/fonttools/fontspector/commit/83eddf2d9464468b0d9ce39fdd6c051b16dc3f21))
    - Address error: used `unwrap()` on a `Result` value ([`b744a99`](https://github.com/fonttools/fontspector/commit/b744a9916a8ff24c904e41abf90f95634b54bdfb))
    - Address error: useless use of `vec!` ([`989c68c`](https://github.com/fonttools/fontspector/commit/989c68c1f56294a3e4077d0789fa77a1c1b0dc0e))
    - Address error: this creates an owned instance just for comparison ([`07d4983`](https://github.com/fonttools/fontspector/commit/07d49837a0182e539f0921e18aadec416b73c6b8))
    - Googlefonts/metadata/familyname ([`64f704c`](https://github.com/fonttools/fontspector/commit/64f704cb59a5f92ce2afe900fff361e9fbe2ce94))
    - Googlefonts/metadata/reserved_font_name ([`1433a7f`](https://github.com/fonttools/fontspector/commit/1433a7f3dee75412231e7347dbd42584ccacdbe1))
    - Googlefonts/metadata/escaped_strings ([`a4fdc2a`](https://github.com/fonttools/fontspector/commit/a4fdc2a6c6691ccdc79bce585e979413f3433e45))
    - Googlefonts/metadata/category ([`3d798bd`](https://github.com/fonttools/fontspector/commit/3d798bd52a91ac182108e70d3a3a538f874d8452))
    - Googlefonts/name/license (3rd attempt) ([`1a39394`](https://github.com/fonttools/fontspector/commit/1a39394ad1ede46d3befd3bc61bf0df88300bd31))
    - Googlefonts/name/license (2nd attempt) ([`b68111b`](https://github.com/fonttools/fontspector/commit/b68111bd9dd46e2303f912bc40db0c33690cc03f))
    - Googlefonts/name/license (1st attempt) ([`0ab6c10`](https://github.com/fonttools/fontspector/commit/0ab6c10ba6dbbedec440aa6f938b7c24b05d5763))
    - Minor tweaks to code-style ([`9f4a293`](https://github.com/fonttools/fontspector/commit/9f4a29343e6ef1410484702b3f08f612fe96a1e2))
    - Googlefonts/varfont/has_HVAR ([`5e668eb`](https://github.com/fonttools/fontspector/commit/5e668eb48963f514768652f8769ccb53aead7202))
    - Googlefonts/unitsperem ([`73b2935`](https://github.com/fonttools/fontspector/commit/73b2935a10740daf23c0c03a70ff3d2d91a5162e))
    - Googlefonts/old_ttfautohint ([`f05bf1a`](https://github.com/fonttools/fontspector/commit/f05bf1a0fda554e9bff1721e016edb295fb104fd))
    - Googlefonts/meta/script_lang_tags ([`f02a1c2`](https://github.com/fonttools/fontspector/commit/f02a1c246267445638a199173b3371003ed89f9b))
    - Googlefonts/has_ttfautohint_params ([`beda829`](https://github.com/fonttools/fontspector/commit/beda8292cfa79f3ea7338fd41a1d50d1045373f8))
    - When building wasm, disable checks that use the reqwest crate ([`73513e7`](https://github.com/fonttools/fontspector/commit/73513e7d02318140739a862a83fdbd594292491d))
    - STAT table checks ([`e803b52`](https://github.com/fonttools/fontspector/commit/e803b52791b7e108e3599ccef415891a34d9ab46))
    - Googlefonts/STAT/compulsory_axis_values ([`2e0c144`](https://github.com/fonttools/fontspector/commit/2e0c144ddddff3f518f137c2831898da6e9c6bbf))
    - Googlefonts/stat/axisregistry ([`4081d88`](https://github.com/fonttools/fontspector/commit/4081d882668bc816f990ca55e326d62dab4a3a45))
    - :axis_order ([`5a95113`](https://github.com/fonttools/fontspector/commit/5a95113ebe74c423d0ee31802f5a5cbb40621f87))
    - Sort checks/googlefonts/description/mod.rs ([`d95a744`](https://github.com/fonttools/fontspector/commit/d95a744c621be6d8dac5c5ab20c0d7a96f32f448))
    - Googlefonts/description/valid_html ([`7b2e47c`](https://github.com/fonttools/fontspector/commit/7b2e47c36fdddaae772643fe72298c73c7f906b2))
    - Googlefonts/description/urls ([`965cc72`](https://github.com/fonttools/fontspector/commit/965cc72f794297dd59c9d496beb5574d32e324a2))
    - Googlefonts/description/has_article ([`c107033`](https://github.com/fonttools/fontspector/commit/c1070335448dea91d46038d54f2ba05303bf7b97))
    - Googlefonts/description/has_unsupported_elements ([`a8f40ab`](https://github.com/fonttools/fontspector/commit/a8f40abffeec6572994204b717f072f84efda23c))
    - Test articles as well as descriptions ([`936c120`](https://github.com/fonttools/fontspector/commit/936c120780fd0ee3ec8db597e6a7edbe28dc0e1c))
    - Googlefonts/description/git_url ([`3424ade`](https://github.com/fonttools/fontspector/commit/3424ade16d00cb42438db3040d609f873f9b914a))
    - Googlefonts/description/broken_links ([`06d2fe5`](https://github.com/fonttools/fontspector/commit/06d2fe50523cd36953a89d04fed8c8ea5da24fe3))
    - Use get_name_entry_strings ([`37c44da`](https://github.com/fonttools/fontspector/commit/37c44daad3a82f0accf2876fce8ce508920b2324))
    - Googlefonts/name/version_format ([`c3d1ce5`](https://github.com/fonttools/fontspector/commit/c3d1ce5d3552fcbac55043a54acaf96035bcdc82))
    - Googlefonts/name/familyname_first_char ([`58269d7`](https://github.com/fonttools/fontspector/commit/58269d7658d3cce16cf168d9de3feddaa897bace))
    - Use main branch now ([`1031a47`](https://github.com/fonttools/fontspector/commit/1031a474bcba5be4670140dba47c3efd88a993fd))
    - Googlefonts/font_names check ([`ab37c89`](https://github.com/fonttools/fontspector/commit/ab37c8919058325c7f4e48e2e49e32fb9f6f30c8))
    - Googlefonts/vendor_id ([`d640012`](https://github.com/fonttools/fontspector/commit/d640012fbf13c8aa5485d989bb11553ccc20b234))
    - Googlefonts/name/mandatory_entries ([`a762aa7`](https://github.com/fonttools/fontspector/commit/a762aa781c6d284a7a7ba33db7d4b97f4a0c1223))
    - Fvar_axis_defaults ([`0fd2aeb`](https://github.com/fonttools/fontspector/commit/0fd2aeb7d879be9cd5d55fd8f4ec0d0c6b97840d))
    - Family_name_compliance ([`e87bcbe`](https://github.com/fonttools/fontspector/commit/e87bcbe578c20d58f9bd5a32cdbcaae3b8df5d08))
    - Add resources ([`7a587d3`](https://github.com/fonttools/fontspector/commit/7a587d3b920736923632ebfeaa4aa41dfc3c6c89))
    - Add gasp ([`650b67b`](https://github.com/fonttools/fontspector/commit/650b67bb090d528aad88d4d0efafd23803074aaa))
    - Googlefonts/name/line_breaks ([`f0110fa`](https://github.com/fonttools/fontspector/commit/f0110fa1f5b182784a771457bccbab5e932e136a))
    - More formatting ([`bc2d10e`](https://github.com/fonttools/fontspector/commit/bc2d10e6ed713a9fc9209db823d246b114066caa))
    - New clippy found new lints! ([`1933d0a`](https://github.com/fonttools/fontspector/commit/1933d0a7835610c4c59e2ca272696789320992e9))
    - Fix warnings ([`a138d6b`](https://github.com/fonttools/fontspector/commit/a138d6bb66f9b9eb46e154df5f69dcf9033fcfb1))
    - Horrible clippy magic to ignore lint in generated file ([`714eef1`](https://github.com/fonttools/fontspector/commit/714eef1aa9e5aa8ace41847e70fb9d7931741f95))
    - Run cargo fmt ([`a97b2a9`](https://github.com/fonttools/fontspector/commit/a97b2a96d2ffbf6fab861b842096159d666a4dc9))
    - Placing the checks inside a googlefonts directory, so that paths perfectly match check IDs ([`8a70e18`](https://github.com/fonttools/fontspector/commit/8a70e1899c2ae3892e2ec10bcd168a3140e12ee6))
    - Use profile builder on Google Fonts profile. ([`6c56608`](https://github.com/fonttools/fontspector/commit/6c56608bf92ffe2181b29b588b473be0ef0a40bf))
    - Split checks into individual checks per file. And make the file paths identical to the check-IDs ([`83fd74c`](https://github.com/fonttools/fontspector/commit/83fd74c6198abb259a138e63cc71bcff12b1c46c))
    - Overlapping path segments check ([`8742b36`](https://github.com/fonttools/fontspector/commit/8742b369dd9d5a839b3658aa718a1aabd51f1b09))
    - Colinear vectors (back again) ([`a3d44f2`](https://github.com/fonttools/fontspector/commit/a3d44f28390a858e1c13e197dc546a56fdf029e4))
    - Factor out common code ([`7667f05`](https://github.com/fonttools/fontspector/commit/7667f053f96e0af35319c03d3675ded4401d9dc6))
    - Short segments check ([`4c3c0dd`](https://github.com/fonttools/fontspector/commit/4c3c0dd75b57df40b08bbb0a56b1784d77b8fb7f))
    - Semivertical check ([`4e59f83`](https://github.com/fonttools/fontspector/commit/4e59f83527741790e6c80c92e6269d27b2f2d0b0))
    - Make a generic close_but_not_on ([`3a57f9b`](https://github.com/fonttools/fontspector/commit/3a57f9b313ff16cb08c22b3187c7673d929e455d))
    - Jaggy segments check ([`8404648`](https://github.com/fonttools/fontspector/commit/840464818ba1d83b2dd5d17558e787f48427d80c))
    - Adjust disclaimer ([`a042f13`](https://github.com/fonttools/fontspector/commit/a042f13f2f962b73fd715539129de7b51ce71a94))
    - Path direction check (with disclaimer) ([`9e1d13a`](https://github.com/fonttools/fontspector/commit/9e1d13a51a5ac1caa044b1e33a8deb4a31ed988a))
    - Alignment_miss ([`fc0511a`](https://github.com/fonttools/fontspector/commit/fc0511a389f2e2aed8c62f22acc19ce2c44ff343))
    - Debug print ([`0427377`](https://github.com/fonttools/fontspector/commit/0427377117ff2073a836cd1549de9b090161c843))
    - Parse all the languages once ([`de2ba8b`](https://github.com/fonttools/fontspector/commit/de2ba8b1de5910327a9474c99d5301b490761c2b))
    - These two checks now live in the Universal profile. ([`411c503`](https://github.com/fonttools/fontspector/commit/411c503d19d309d9ace414e5deb7d7bc605bb5d9))
    - General tidying ([`03947a7`](https://github.com/fonttools/fontspector/commit/03947a799ef9ca939277656457f437dfde282833))
    - Add fvar_instances check ([`47892b2`](https://github.com/fonttools/fontspector/commit/47892b2b3a15245c3b22251df38317ee799a04b6))
    - Share some crates, add axis registry ([`683ec0e`](https://github.com/fonttools/fontspector/commit/683ec0eeb3a0b1d34fc13c4935d448489be0fd58))
    - Rename check module ([`79bf024`](https://github.com/fonttools/fontspector/commit/79bf024800e14c0b4532258863469b6af89bd7f8))
    - Googlefonts/weightclass check ([`1761ffd`](https://github.com/fonttools/fontspector/commit/1761ffd839232f9ad652f095d373dea43169495a))
    - Move to Universal profile ([`7cc0e15`](https://github.com/fonttools/fontspector/commit/7cc0e15f42ffbf1d512f2fa50d42fe12ba3aca44))
    - Googlefonts/varfont/bold_wght_coord => varfont/bold_wght_coord ([`c040fe0`](https://github.com/fonttools/fontspector/commit/c040fe032d87f2db4f2546f2b9ae53fc524c4181))
    - Migrate render_own_name to GoogleFonts profile ([`220d710`](https://github.com/fonttools/fontspector/commit/220d71044c2ee91c7ff7b78b71231b04a4e3bdcb))
    - Sync with latest fontbakery ([`d7968d6`](https://github.com/fonttools/fontspector/commit/d7968d62b6271d79869a3ebf34c1d20365482c6c))
    - Add normalisation to tofu check ([`bf7494b`](https://github.com/fonttools/fontspector/commit/bf7494b38e455a32db02ed8a63a16a514e353c96))
    - Tofu detection check ([`0b52715`](https://github.com/fonttools/fontspector/commit/0b527153df71cb0297be0434c0c18c4feac32d68))
    - Use cache to determine codepoints in font ([`0514efc`](https://github.com/fonttools/fontspector/commit/0514efcf5e99d3c157fad5795816183d8f84e091))
    - Make is_listed_on_google_fonts a cached question ([`09f40bd`](https://github.com/fonttools/fontspector/commit/09f40bde7c8e3f7273738b5433da34fe42b4935f))
    - Partially implemented axes_match check ([`06ddd2f`](https://github.com/fonttools/fontspector/commit/06ddd2f4b3a3c1f9d776533feca6456ee2c920ba))
    - Missing file ([`a851a65`](https://github.com/fonttools/fontspector/commit/a851a65b5468a67fff2b845ce6c1731ed9c68595))
    - Two simple GF checks ([`f12fc30`](https://github.com/fonttools/fontspector/commit/f12fc30d4bed013f955af2a75756aeb8c586745a))
    - File_size ([`a74c5e4`](https://github.com/fonttools/fontspector/commit/a74c5e401c4f588dc27fa0a4cb8b839500c1b80d))
    - Use a real version ([`a5ff68f`](https://github.com/fonttools/fontspector/commit/a5ff68f92e8f5fad126fa9416ee36231e5b290ff))
    - Subsets checks ([`74e6194`](https://github.com/fonttools/fontspector/commit/74e6194c52811312e60a0f7f241a3efcdc6960dc))
    - Move more things to real crates ([`c4a173e`](https://github.com/fonttools/fontspector/commit/c4a173e40a7413ff221320b2d991e815ca9992b4))
    - Use typometrics check ([`1fb0b00`](https://github.com/fonttools/fontspector/commit/1fb0b00433aedf906fcd599dce3aee0dfe0590ea))
    - Oops there's quite a few of these ([`01f4cce`](https://github.com/fonttools/fontspector/commit/01f4cce0b480dc8250921f3e8b2308b55cec9b3b))
    - Googlefonts_metadata_license ([`c214e20`](https://github.com/fonttools/fontspector/commit/c214e20a69973a84219cc8e71be611c2cb78ba03))
    - Googlefonts_metadata_copyright ([`707ab3f`](https://github.com/fonttools/fontspector/commit/707ab3f4c45ee06359e55c0da38313decf04047b))
    - Add interpolation issues check ([`7671c6b`](https://github.com/fonttools/fontspector/commit/7671c6bc9c045ff6842356ba5437d48ae3f3d313))
    - This is now merged ([`34aa3a6`](https://github.com/fonttools/fontspector/commit/34aa3a6b7aad28088f49bf50a67a6dc21acb2283))
    - Now almost everything is tested ([`28ebc9a`](https://github.com/fonttools/fontspector/commit/28ebc9a643c754fc62368519fff265d06b5e8ff9))
    - More tests passing ([`43a758f`](https://github.com/fonttools/fontspector/commit/43a758f6a57ac82075e34775e2d8e21016a3c66a))
    - Share itertools versions ([`71e6f81`](https://github.com/fonttools/fontspector/commit/71e6f81d35e3fbe8540a38ec532e382effa87459))
    - More passes ([`ae2d088`](https://github.com/fonttools/fontspector/commit/ae2d088fb5a925108fc7e1441295d3efb4943279))
    - Bump read/write/skrifa versions, dump font-types, deal with fallout ([`d2fd7e4`](https://github.com/fonttools/fontspector/commit/d2fd7e4be7f70b014776c6a56ec035b5156692c0))
    - Build regex at most once ([`cfbd89b`](https://github.com/fonttools/fontspector/commit/cfbd89b06d21cafe8e94d78b83ee4841f90464e1))
    - Run code-tests in CI ([`ca20b6f`](https://github.com/fonttools/fontspector/commit/ca20b6fcaaaef95ad17d1224aa7f758757330ed2))
    - Merge pull request #10 from felipesanches/more_checks_2024_sep_20 ([`8cfb898`](https://github.com/fonttools/fontspector/commit/8cfb898458a69666f439676be4d02e7f115bf7a0))
    - Added code-tests for opentype/name/empty_records ([`432d0e3`](https://github.com/fonttools/fontspector/commit/432d0e3b9b47ab719499d7d13da28cf7976a6826))
    - Moving code-testing helper functions to a separate file ([`4e475b1`](https://github.com/fonttools/fontspector/commit/4e475b172c566573a85b793bba47cb6ce21b8268))
    - One more code-testing implementation, but still failing. Needs further investigation. ([`df3104e`](https://github.com/fonttools/fontspector/commit/df3104e51783fcfcfe2414dacf036724d9119303))
    - Further refactoring the code-testing helper functions ([`7fabefe`](https://github.com/fonttools/fontspector/commit/7fabefe24db197e83ac3eea33288c70664bb1679))
    - More idiomatic use of fontbuilder ([`5add5c1`](https://github.com/fonttools/fontspector/commit/5add5c1391c0e07ffabf8a9108a2afefe3b53ca6))
    - Prototype writing changed test fonts ([`6772208`](https://github.com/fonttools/fontspector/commit/67722089bbd8292071deb3bad694b3bc18ddcd39))
    - Prototyping assert_results_contin method for code-testing ([`b84850d`](https://github.com/fonttools/fontspector/commit/b84850dde53a53094c4a7244e98a5f37d3118cbe))
    - Used worst_status method on assert_pass ([`aae74f1`](https://github.com/fonttools/fontspector/commit/aae74f1455aba9c74a86e55572d5d478411e5419))
    - Refactoring codetesting functions ([`73eab4b`](https://github.com/fonttools/fontspector/commit/73eab4b7168d41ea8b8d911a57d790de2d5fcf3d))
    - Prototyping code-tests ([`74a0682`](https://github.com/fonttools/fontspector/commit/74a0682bfd8f8ee3d868732913e535a1a9790dff))
    - New check: googlefonts/name/rfn ([`d1e1a86`](https://github.com/fonttools/fontspector/commit/d1e1a8634993729ecb3e3ef155b4d5ee3175de8c))
    - List a few more missing desc checks on gfonts profile definition ([`e253697`](https://github.com/fonttools/fontspector/commit/e2536973cbc33673dee17880f1442ff70c841e96))
    - New check: googlefonts/description/eof_linebreak ([`1b43c4e`](https://github.com/fonttools/fontspector/commit/1b43c4e43374faf64c28b3ff07c93db023f60669))
    - New check: googlefonts/description/min_length ([`a538777`](https://github.com/fonttools/fontspector/commit/a538777746685683f048fb20621d7de0c10019bd))
    - Merge pull request #6 from felipesanches/new_check_ids ([`4fdc7c5`](https://github.com/fonttools/fontspector/commit/4fdc7c52a7582dbc984f89d8d0b35f6a58748cbd))
    - Update check-ID following FontBakery's new naming scheme ([`64e3e5d`](https://github.com/fonttools/fontspector/commit/64e3e5d452fec3f6c86cff9f34e33816951af3d5))
    - Add can_render_samples check ([`df5d4ec`](https://github.com/fonttools/fontspector/commit/df5d4ecdde296fb2b60caa277ff7cf76d70f0c38))
    - Use Colin’s utf8-aware protobuf ([`78bb21a`](https://github.com/fonttools/fontspector/commit/78bb21acb22f2aa3664f70894a9ee2963f91b500))
    - Move to the hellish procmacro ([`20d9a48`](https://github.com/fonttools/fontspector/commit/20d9a48838d57250cac9e84c8d7e00ac6359b4bd))
    - Regexes are slow, use optimised glyph name access ([`7ba0913`](https://github.com/fonttools/fontspector/commit/7ba09133812a73d425dd35b1536e1fbdd811bdd2))
    - Reporting improvements ([`7966f56`](https://github.com/fonttools/fontspector/commit/7966f565a8373ad79feefed46828e9169d2d1e0a))
    - Clippy lints ([`d46fdc3`](https://github.com/fonttools/fontspector/commit/d46fdc3ca2517e26a8d8fe5d91a6fded279b43ed))
    - Use contents, not filesystem, to read METADATA.pb ([`9cdffb7`](https://github.com/fonttools/fontspector/commit/9cdffb7a20c94f4129bbb33237b2bb0a6b8061cb))
    - WIP solve the sibling problem ([`10430e5`](https://github.com/fonttools/fontspector/commit/10430e572099e1185247ab78b083de43c154f1a6))
    - Make check implementation (one/all) an enum ([`d57b5c8`](https://github.com/fonttools/fontspector/commit/d57b5c8a08433ecb0ac60330c35df94a91461541))
    - Make TestableCollection the primary unit of testing ([`70da856`](https://github.com/fonttools/fontspector/commit/70da8567069c053415067598ffbe428901784b59))
    - Improve error/skip story, add fvar regular coords check ([`c23b8b0`](https://github.com/fonttools/fontspector/commit/c23b8b0eae9f7f97a15c2d70092196ab1175fe9b))
    - Make checks serializable, add check flags ([`c4996e0`](https://github.com/fonttools/fontspector/commit/c4996e08b590d3710763c117b99d9df61b631e3e))
    - Use error return ([`7a9dfbd`](https://github.com/fonttools/fontspector/commit/7a9dfbd6d208d35161006a30fd774337013d6bc9))
    - Rearrange run result struct, add subresult codenames/severity ([`2d99a2b`](https://github.com/fonttools/fontspector/commit/2d99a2b760b43d7cdf4630800d25493e0d7485a1))
    - Add configuration and check context ([`caeb4b7`](https://github.com/fonttools/fontspector/commit/caeb4b7478a4a51bd5130fe85eb7043758e2236d))
    - Merge pull request #5 from felipesanches/rationales_not_optional ([`ee113d9`](https://github.com/fonttools/fontspector/commit/ee113d98a0cb146a764163c6afeacae05f0ece9f))
    - Merge branch 'main' into rationales_not_optional ([`37122c3`](https://github.com/fonttools/fontspector/commit/37122c334183fa689fbe4f5617b1ca24e6abb95c))
    - Be (slightly) more grown-up about error handling ([`2818a76`](https://github.com/fonttools/fontspector/commit/2818a764da76b9acc2c33127cb156238dca970c1))
    - Rationale and proposal fields are not optional ([`752d559`](https://github.com/fonttools/fontspector/commit/752d5593f3c5a345a781f8b76e5907607bda7dbd))
    - Built-in profiles shouldn't pluginate ([`71cea65`](https://github.com/fonttools/fontspector/commit/71cea651e8556fa0ab1e119b25c39c6b52f0d1bd))
    - Add equal codepoint coverage check ([`e71c632`](https://github.com/fonttools/fontspector/commit/e71c63282fbdebdb4c3cd34def35e16e95992b47))
    - Lint nit ([`e5234ad`](https://github.com/fonttools/fontspector/commit/e5234ad0eb073a3d51fbc0cd4f43da96e112ff38))
    - Bake in GF profile ([`1628604`](https://github.com/fonttools/fontspector/commit/16286048b26e5a6fb7d07ab5ef69e05e9c592b09))
    - Split checks from profile ([`a74f666`](https://github.com/fonttools/fontspector/commit/a74f666c9632fb3ba216fff29b87cfd66799dbeb))
    - Date and designers checks ([`6cade80`](https://github.com/fonttools/fontspector/commit/6cade801371a74b6f0e23d96ae88be97fd977297))
    - Allow included profiles, make registering profile a Result ([`4d7a296`](https://github.com/fonttools/fontspector/commit/4d7a296a76c2717c895784d8d1e795a1740a3859))
    - Lint ([`53f4f72`](https://github.com/fonttools/fontspector/commit/53f4f72dab2ca4e803e209fffbef71167800df70))
    - Tidy up build script ([`5d65344`](https://github.com/fonttools/fontspector/commit/5d65344a79f5984c2880399ca0aaa5c0cdb13714))
    - Use Rust protoc ([`2ffb735`](https://github.com/fonttools/fontspector/commit/2ffb7355c51bba3d0e711fb90aa2b33c31674a02))
    - Test parsing METADATA.pb files ([`8f992e5`](https://github.com/fonttools/fontspector/commit/8f992e561819634d788870753ddf2b776095c308))
</details>

