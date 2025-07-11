# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.1.0 (2025-07-11)

<csr-id-f44be5515dcaea17b96b1df7a4b11407561d0c17/>
<csr-id-d76cf586e4d8b509a6af4b2df724264afe125359/>
<csr-id-a6b7ffc4f39c6b1c1bd92cd9b07f4ba22d54ef2e/>

### Chore

 - <csr-id-f44be5515dcaea17b96b1df7a4b11407561d0c17/> Improve error handling
   * chore: Better error handling
   
   * chore: Better error handling for fix functions too
 - <csr-id-d76cf586e4d8b509a6af4b2df724264afe125359/> Roll dependencies
   * chore(deps): Add renovate config
   
   * chore(deps): Update pyo3 deps and fix
   
   * Update lock file
   
   * fix(fontbakery-bridge): Fix up fontbakery-bridge for new pyo3

### Chore

 - <csr-id-c69a556c43bd09c1368073faf6f3ac6c985e0dc0/> Add changelog

### New Features

 - <csr-id-06e1ff0b9234917d3040559465b70c4b3c44e61e/> fontwerk profile

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

 - 8 commits contributed to the release over the course of 56 calendar days.
 - 64 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#113](https://github.com/fonttools/fontspector/issues/113), [#133](https://github.com/fonttools/fontspector/issues/133), [#161](https://github.com/fonttools/fontspector/issues/161), [#275](https://github.com/fonttools/fontspector/issues/275), [#287](https://github.com/fonttools/fontspector/issues/287), [#299](https://github.com/fonttools/fontspector/issues/299)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#113](https://github.com/fonttools/fontspector/issues/113)**
    - Make Fontbakery Python bridge usable ([`7082188`](https://github.com/fonttools/fontspector/commit/7082188f3e6c2ecae5090eba82390835cc1e41ff))
 * **[#133](https://github.com/fonttools/fontspector/issues/133)**
    - Roll dependencies ([`d76cf58`](https://github.com/fonttools/fontspector/commit/d76cf586e4d8b509a6af4b2df724264afe125359))
 * **[#161](https://github.com/fonttools/fontspector/issues/161)**
    - Fontwerk profile ([`06e1ff0`](https://github.com/fonttools/fontspector/commit/06e1ff0b9234917d3040559465b70c4b3c44e61e))
 * **[#275](https://github.com/fonttools/fontspector/issues/275)**
    - Improve error handling ([`f44be55`](https://github.com/fonttools/fontspector/commit/f44be5515dcaea17b96b1df7a4b11407561d0c17))
 * **[#287](https://github.com/fonttools/fontspector/issues/287)**
    - Deny indexing slicing ([`a6b7ffc`](https://github.com/fonttools/fontspector/commit/a6b7ffc4f39c6b1c1bd92cd9b07f4ba22d54ef2e))
 * **[#299](https://github.com/fonttools/fontspector/issues/299)**
    - Improve rationale rewrapping ([`46e90e5`](https://github.com/fonttools/fontspector/commit/46e90e51624979590af83272f96cbcfc521b7d0a))
 * **Uncategorized**
    - Add changelog ([`c69a556`](https://github.com/fonttools/fontspector/commit/c69a556c43bd09c1368073faf6f3ac6c985e0dc0))
    - Release fontspector-fontbakery-bridge v1.1.0, fontspector-profile-googlefonts v1.2.0, fontspector v1.3.0 ([`05adc63`](https://github.com/fonttools/fontspector/commit/05adc636ddf710008076985ab8c2a03ea19a0f6c))
</details>

## v1.0.0 (2025-05-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 216 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #102 from fonttools/release-prep ([`e5435f4`](https://github.com/fonttools/fontspector/commit/e5435f4ab282338ccc818daca8dacf543de27022))
    - Just copy in bit of fontbakery we need ([`064a216`](https://github.com/fonttools/fontspector/commit/064a216cef77c611a763c47312bfec2035ea0062))
    - Update metadata ([`524e153`](https://github.com/fonttools/fontspector/commit/524e153eb9bfe62e5d0f1d6f12be446764791af4))
    - Prep for 1.0.0 release ([`c1ef822`](https://github.com/fonttools/fontspector/commit/c1ef822c860b8dd53b363c9b69201981c75f757c))
    - Add metadata here too ([`44c10a3`](https://github.com/fonttools/fontspector/commit/44c10a3963006bd6977444efa1d8f1dea9b2a8c8))
    - Merge pull request #99 from fonttools/rich-metadata ([`dfd2c49`](https://github.com/fonttools/fontspector/commit/dfd2c49e542a5c5def5929c6c5e5dbd30e5015bb))
    - Add metadata here too ([`1db9d4a`](https://github.com/fonttools/fontspector/commit/1db9d4ae66aadc013b06fa399516c145ef40fcef))
    - Merge pull request #83 from fonttools/adobe-profile ([`170ffde`](https://github.com/fonttools/fontspector/commit/170ffde473594377a590b95ffbfc7ea5d592d768))
    - Adobe profile ([`1e31635`](https://github.com/fonttools/fontspector/commit/1e31635eaa1d6c023c21f70c70e62dac5a583265))
    - Merge pull request #63 from LuxxxLucy/lucy-multiple-proposal-br ([`2d675d5`](https://github.com/fonttools/fontspector/commit/2d675d5bfe5cdb3de99e1a2cf8c65964c144bc52))
    - Fix the python interface ([`f9e8ecc`](https://github.com/fonttools/fontspector/commit/f9e8ecc380d350e172154a2880cf3c3749876b6b))
    - Fix warnings ([`a138d6b`](https://github.com/fonttools/fontspector/commit/a138d6bb66f9b9eb46e154df5f69dcf9033fcfb1))
    - Move Python test runner to separate crate ([`c56bc40`](https://github.com/fonttools/fontspector/commit/c56bc40a3f53e2295feda1e66ee601ad7f4722dd))
    - Allow the Python tests to be run inside of package tests ([`ee8a960`](https://github.com/fonttools/fontspector/commit/ee8a960eacce12b39580b0ea191f363c117164bd))
    - Improve error messages ([`7255ff8`](https://github.com/fonttools/fontspector/commit/7255ff8e0b6bf51e0cba6bf5f2c71fe3698a5a0f))
    - Allow running Python tests using fontspector checks ([`3b27b96`](https://github.com/fonttools/fontspector/commit/3b27b96b6b15f1538aa6866c060511324543292f))
    - Another check ([`172fea4`](https://github.com/fonttools/fontspector/commit/172fea494a2aef8530c9418c17f3a45d14ee6544))
    - Load more functions, support non-generator checks ([`b3397f8`](https://github.com/fonttools/fontspector/commit/b3397f8508d8876e3256a112025ce98d5a50d8b9))
    - Rework Python bridge ([`e357d73`](https://github.com/fonttools/fontspector/commit/e357d73000b82b71ee93f28f71c5b16c5ca819d1))
    - Add fontbakery bridge (proof of concept) ([`05b309e`](https://github.com/fonttools/fontspector/commit/05b309e0cf6b18d84102566548eb6a7c48065c9f))
</details>

