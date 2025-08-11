# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.4.0 (2025-08-11)

### Chore

 - <csr-id-0c6365a1d3383dad9d12de3984989ee5747b35cf/> get working again
   * chore: Upgrade fontations deps
   
   * chore: API changed
   
   * chore: Comply with new elided lifetime rules

### New Features

 - <csr-id-7b324745f3a6b339a134be2f68f89fe726f4fc97/> Support on-the-fly compilation
 - <csr-id-3d7a965e900a6aed406ddf8a78f490ff3d42dd6a/> Support installing with cargo-binstall

### Bug Fixes

 - <csr-id-cefb5250c292752ece71d7fe3b2ccd30f682f416/> update rust crate indicatif to 0.18

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 40 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#314](https://github.com/fonttools/fontspector/issues/314), [#326](https://github.com/fonttools/fontspector/issues/326), [#340](https://github.com/fonttools/fontspector/issues/340), [#371](https://github.com/fonttools/fontspector/issues/371)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#314](https://github.com/fonttools/fontspector/issues/314)**
    - Update rust crate indicatif to 0.18 ([`cefb525`](https://github.com/fonttools/fontspector/commit/cefb5250c292752ece71d7fe3b2ccd30f682f416))
 * **[#326](https://github.com/fonttools/fontspector/issues/326)**
    - Support installing with cargo-binstall ([`3d7a965`](https://github.com/fonttools/fontspector/commit/3d7a965e900a6aed406ddf8a78f490ff3d42dd6a))
 * **[#340](https://github.com/fonttools/fontspector/issues/340)**
    - Support on-the-fly compilation ([`7b32474`](https://github.com/fonttools/fontspector/commit/7b324745f3a6b339a134be2f68f89fe726f4fc97))
 * **[#371](https://github.com/fonttools/fontspector/issues/371)**
    - Get working again ([`0c6365a`](https://github.com/fonttools/fontspector/commit/0c6365a1d3383dad9d12de3984989ee5747b35cf))
 * **Uncategorized**
    - Release fontspector-fontbakery-bridge v1.1.0, fontspector-profile-googlefonts v1.2.0, fontspector v1.3.0 ([`05adc63`](https://github.com/fonttools/fontspector/commit/05adc636ddf710008076985ab8c2a03ea19a0f6c))
</details>

## v1.3.0 (2025-07-11)

### New Features

 - <csr-id-3d7a965e900a6aed406ddf8a78f490ff3d42dd6a/> Support installing with cargo-binstall

### Bug Fixes

 - <csr-id-cefb5250c292752ece71d7fe3b2ccd30f682f416/> update rust crate indicatif to 0.18

## v1.2.0 (2025-07-02)

<csr-id-0c45bd3c0f7689c4afb5270881a9b374a433e00d/>
<csr-id-a6b7ffc4f39c6b1c1bd92cd9b07f4ba22d54ef2e/>

### Chore

 - <csr-id-0c45bd3c0f7689c4afb5270881a9b374a433e00d/> Order result summary bad to good
   * chore(reporters): Order result summary bad to good
   
   * chore(reporters): Remove empty sections

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

### Style

 - <csr-id-a6b7ffc4f39c6b1c1bd92cd9b07f4ba22d54ef2e/> deny indexing slicing
   * chore: More lints into Cargo.toml
   
   * style: Deny indexing slicing

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 11 calendar days.
 - 12 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#161](https://github.com/fonttools/fontspector/issues/161), [#287](https://github.com/fonttools/fontspector/issues/287), [#296](https://github.com/fonttools/fontspector/issues/296), [#299](https://github.com/fonttools/fontspector/issues/299), [#302](https://github.com/fonttools/fontspector/issues/302), [#308](https://github.com/fonttools/fontspector/issues/308)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#161](https://github.com/fonttools/fontspector/issues/161)**
    - Fontwerk profile ([`06e1ff0`](https://github.com/fonttools/fontspector/commit/06e1ff0b9234917d3040559465b70c4b3c44e61e))
 * **[#287](https://github.com/fonttools/fontspector/issues/287)**
    - Deny indexing slicing ([`a6b7ffc`](https://github.com/fonttools/fontspector/commit/a6b7ffc4f39c6b1c1bd92cd9b07f4ba22d54ef2e))
 * **[#296](https://github.com/fonttools/fontspector/issues/296)**
    - Correctly parse URL in OFL text ([`3a8cd3f`](https://github.com/fonttools/fontspector/commit/3a8cd3f220746bb67b33863ee3ec1125d1ad0f3b))
 * **[#299](https://github.com/fonttools/fontspector/issues/299)**
    - Improve rationale rewrapping ([`46e90e5`](https://github.com/fonttools/fontspector/commit/46e90e51624979590af83272f96cbcfc521b7d0a))
 * **[#302](https://github.com/fonttools/fontspector/issues/302)**
    - Correctly parse URL in OFL text ([`3a8cd3f`](https://github.com/fonttools/fontspector/commit/3a8cd3f220746bb67b33863ee3ec1125d1ad0f3b))
 * **[#308](https://github.com/fonttools/fontspector/issues/308)**
    - Order result summary bad to good ([`0c45bd3`](https://github.com/fonttools/fontspector/commit/0c45bd3c0f7689c4afb5270881a9b374a433e00d))
 * **Uncategorized**
    - Release fontspector-profile-iso15008 v1.0.2, fontspector v1.2.0 ([`3b75f05`](https://github.com/fonttools/fontspector/commit/3b75f0584028dde09ee048620017e95aaa9661af))
    - Release fontspector-profile-fontwerk v1.0.0, fontspector v1.2.0 ([`0efca53`](https://github.com/fonttools/fontspector/commit/0efca539ecee573a378018c93f9ae71b561715de))
    - Release fontspector-profile-fontwerk v1.0.0, fontspector v1.2.0 ([`a038c9b`](https://github.com/fonttools/fontspector/commit/a038c9bcbd8afc76610852b12e2de72b3dc944e7))
    - Release fontspector-checkapi v1.1.1, fontspector-profile-opentype v1.2.0, fontspector-profile-googlefonts v1.1.1, fontspector-profile-universal v1.1.1, fontspector v1.2.0 ([`f407a9a`](https://github.com/fonttools/fontspector/commit/f407a9aaf0aae501443842311f1b5c27eab007b6))
</details>

## v1.1.0 (2025-06-19)

<csr-id-f44be5515dcaea17b96b1df7a4b11407561d0c17/>
<csr-id-d2a69bed4d1046672bb328484a31ba0466b1b648/>

### Chore

 - <csr-id-f44be5515dcaea17b96b1df7a4b11407561d0c17/> Improve error handling
   * chore: Better error handling
   
   * chore: Better error handling for fix functions too
 - <csr-id-d2a69bed4d1046672bb328484a31ba0466b1b648/> fix typo

### New Features

<csr-id-0e4f7173896fcc94006d2a2bb15d966a3ccdaa6f/>

 - <csr-id-82b1cb17c491e78f6adc0811bb632cc1531dd7dc/> Allow TOML config, allow explicit_checks/exclude_checks keys
   * feat(cli): Allow TOML config, allow explicit_checks/exclude_checks keys
* fix(opentype/vendor_id): Support config file key property

### Bug Fixes

<csr-id-57f0fc7e0da55ec91a965c38a8c2518e3b4208a0/>
<csr-id-baec736820268d31100172fd70a324c47e6bc34a/>
<csr-id-7793cb485fbe3a45baa88ae5673d57c3be8b0f84/>

 - <csr-id-6be6c093abfc298ff9cb6b30118d0fd1107049c8/> Don't use terminal reporter if there are stdout reporters
 - <csr-id-82398535287401e767098901b1da578809d28485/> Make list checks work, add permalinks
   * fix(web): Make list checks work, add permalinks (#162)
* feat(googlefonts): Allow soft_dotted check on wasm, why not

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 28 calendar days.
 - 30 days passed between releases.
 - 9 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 10 unique issues were worked on: [#121](https://github.com/fonttools/fontspector/issues/121), [#122](https://github.com/fonttools/fontspector/issues/122), [#125](https://github.com/fonttools/fontspector/issues/125), [#126](https://github.com/fonttools/fontspector/issues/126), [#157](https://github.com/fonttools/fontspector/issues/157), [#162](https://github.com/fonttools/fontspector/issues/162), [#200](https://github.com/fonttools/fontspector/issues/200), [#228](https://github.com/fonttools/fontspector/issues/228), [#265](https://github.com/fonttools/fontspector/issues/265), [#275](https://github.com/fonttools/fontspector/issues/275)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#121](https://github.com/fonttools/fontspector/issues/121)**
    - Sort results in HTML/MD reports by severity ([`7793cb4`](https://github.com/fonttools/fontspector/commit/7793cb485fbe3a45baa88ae5673d57c3be8b0f84))
 * **[#122](https://github.com/fonttools/fontspector/issues/122)**
    - Use shadow-rs to provide detailed build information in --version and -V ([`0e4f717`](https://github.com/fonttools/fontspector/commit/0e4f7173896fcc94006d2a2bb15d966a3ccdaa6f))
 * **[#125](https://github.com/fonttools/fontspector/issues/125)**
    - Allow TOML config, allow explicit_checks/exclude_checks keys ([`82b1cb1`](https://github.com/fonttools/fontspector/commit/82b1cb17c491e78f6adc0811bb632cc1531dd7dc))
 * **[#126](https://github.com/fonttools/fontspector/issues/126)**
    - Reduce the use of printlns to avoid panic when piped ([`baec736`](https://github.com/fonttools/fontspector/commit/baec736820268d31100172fd70a324c47e6bc34a))
 * **[#157](https://github.com/fonttools/fontspector/issues/157)**
    - Update rust crate zip to v4 ([`57f0fc7`](https://github.com/fonttools/fontspector/commit/57f0fc7e0da55ec91a965c38a8c2518e3b4208a0))
 * **[#162](https://github.com/fonttools/fontspector/issues/162)**
    - Make list checks work, add permalinks ([`8239853`](https://github.com/fonttools/fontspector/commit/82398535287401e767098901b1da578809d28485))
 * **[#200](https://github.com/fonttools/fontspector/issues/200)**
    - Make list checks work, add permalinks ([`8239853`](https://github.com/fonttools/fontspector/commit/82398535287401e767098901b1da578809d28485))
 * **[#228](https://github.com/fonttools/fontspector/issues/228)**
    - Don't use terminal reporter if there are stdout reporters ([`6be6c09`](https://github.com/fonttools/fontspector/commit/6be6c093abfc298ff9cb6b30118d0fd1107049c8))
 * **[#265](https://github.com/fonttools/fontspector/issues/265)**
    - Fix typo ([`d2a69be`](https://github.com/fonttools/fontspector/commit/d2a69bed4d1046672bb328484a31ba0466b1b648))
 * **[#275](https://github.com/fonttools/fontspector/issues/275)**
    - Improve error handling ([`f44be55`](https://github.com/fonttools/fontspector/commit/f44be5515dcaea17b96b1df7a4b11407561d0c17))
 * **Uncategorized**
    - Release fontspector v1.1.0 ([`58ca147`](https://github.com/fonttools/fontspector/commit/58ca147b8caada787ad60defc165064f5d61263d))
</details>

## v1.0.2 (2025-05-20)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 4 calendar days.
 - 11 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#111](https://github.com/fonttools/fontspector/issues/111), [#112](https://github.com/fonttools/fontspector/issues/112), [#113](https://github.com/fonttools/fontspector/issues/113), [#114](https://github.com/fonttools/fontspector/issues/114)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#111](https://github.com/fonttools/fontspector/issues/111)**
    - Include most of the fixes from gftools-fix ([`2de6875`](https://github.com/fonttools/fontspector/commit/2de68751c8c4da8c29f9e46d444280cdf478c6b2))
 * **[#112](https://github.com/fonttools/fontspector/issues/112)**
    - Upgrade zip from yanked release ([`182edee`](https://github.com/fonttools/fontspector/commit/182edeedd7c4e885241433cba0bb1e246b352582))
 * **[#113](https://github.com/fonttools/fontspector/issues/113)**
    - Make Fontbakery Python bridge usable ([`7082188`](https://github.com/fonttools/fontspector/commit/7082188f3e6c2ecae5090eba82390835cc1e41ff))
 * **[#114](https://github.com/fonttools/fontspector/issues/114)**
    - Add badges reporter ([`8683fb8`](https://github.com/fonttools/fontspector/commit/8683fb89801fb1df5c07d71f45c07efa79b111e0))
 * **Uncategorized**
    - Release fontspector v1.0.2 ([`f2b10ca`](https://github.com/fonttools/fontspector/commit/f2b10ca543aa9fe3ccc34e85ebb0a7063795a9a0))
    - Release fontspector v1.0.2 ([`3ac581d`](https://github.com/fonttools/fontspector/commit/3ac581dc6067f3d7641a34c9cfa2ced698300326))
</details>

## v1.0.1 (2025-05-08)

* Small bug fixes in universal checks
* Improved release and CI workflow
* Add README to Cargo.toml manifest

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release fontspector v1.0.1 ([`b6ef630`](https://github.com/fonttools/fontspector/commit/b6ef630c8a3e151093343cdaa72b934a9070236b))
    - Update changelog ([`1f2767f`](https://github.com/fonttools/fontspector/commit/1f2767fb7287ef691e857eba0a4b74923d0f22ae))
    - Adjusting changelogs prior to release of fontspector v1.0.1 ([`c2e98f9`](https://github.com/fonttools/fontspector/commit/c2e98f9f3bd79fb20650ce872d098e723d5ded27))
    - Add changelog ([`cfa49cd`](https://github.com/fonttools/fontspector/commit/cfa49cd37952a26afd540700c3fe82b970125b57))
    - Bump fontspector version ([`59460d9`](https://github.com/fonttools/fontspector/commit/59460d97c94e5292b76133fd2b2e787d7242d529))
</details>

## v1.0.0 (2025-05-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 109 commits contributed to the release over the course of 301 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #102 from fonttools/release-prep ([`e5435f4`](https://github.com/fonttools/fontspector/commit/e5435f4ab282338ccc818daca8dacf543de27022))
    - Don't forget readme... ([`6fb2f74`](https://github.com/fonttools/fontspector/commit/6fb2f74b8a341fbc7c427efd88f9d32509398e4a))
    - Read profile cargo files for release ([`5fe1c5a`](https://github.com/fonttools/fontspector/commit/5fe1c5aff636944c257ec25b19004426660db0c2))
    - Prep for 1.0.0 release ([`c1ef822`](https://github.com/fonttools/fontspector/commit/c1ef822c860b8dd53b363c9b69201981c75f757c))
    - Merge pull request #100 from fonttools/iso15008 ([`c42f3f8`](https://github.com/fonttools/fontspector/commit/c42f3f8d0dfdbe97d9fa78342e135de0911d97fc))
    - Bake it in ([`8372ea1`](https://github.com/fonttools/fontspector/commit/8372ea194eb3a21fb074e415e2eab0e8f0d721b0))
    - Merge pull request #96 from fonttools/non-ink-characters ([`1577008`](https://github.com/fonttools/fontspector/commit/15770084eaa140071658b5b6157ceb8174c8eb3a))
    - Process overrides in profile and config file ([`b15fdd8`](https://github.com/fonttools/fontspector/commit/b15fdd8e297d58d3ce2938e2c38a6cc6568cbb86))
    - Merge pull request #92 from fonttools/nixon-feedback ([`0b9a28b`](https://github.com/fonttools/fontspector/commit/0b9a28b9c647bfb7ec0f3ba8156d616fce82b37b))
    - Move skrifa dep to workspace ([`9475563`](https://github.com/fonttools/fontspector/commit/9475563d3da16cf982551f46dd50ec16e9264055))
    - Merge pull request #86 from fonttools/fix-md-filename ([`ef4b330`](https://github.com/fonttools/fontspector/commit/ef4b330bfc81ddff2572b6cf9b979fc0748ec5c2))
    - Wrong name for markdown report template ([`154631b`](https://github.com/fonttools/fontspector/commit/154631bbbf225659843a2b8b3724999385f73b20))
    - Merge pull request #85 from fonttools/html-reporter ([`f7509d1`](https://github.com/fonttools/fontspector/commit/f7509d1b6418bb21feb931153bcbf8f8429af452))
    - Add HTML reporter ([`f090297`](https://github.com/fonttools/fontspector/commit/f0902978433839308e386b0ddd04245247d520bd))
    - Merge pull request #82 from fonttools/python-optional-feature ([`8ca6c47`](https://github.com/fonttools/fontspector/commit/8ca6c471cd917e3a9baa5a711f811de3aabdbd84))
    - Don’t build Python bridge by default ([`17aecb7`](https://github.com/fonttools/fontspector/commit/17aecb70472506aface01fa12661ca531a7a0d78))
    - Merge pull request #79 from fonttools/stdout-reporter ([`622b442`](https://github.com/fonttools/fontspector/commit/622b44224a071d77a9ee0fe49ce6c33de359a3b7))
    - Make clippy happier ([`11d1a12`](https://github.com/fonttools/fontspector/commit/11d1a12b3d30b87cebf09e5f8c4bdba38b70a6ce))
    - Allow for sending reports to stdout, fixes #28 ([`5c4d58e`](https://github.com/fonttools/fontspector/commit/5c4d58ecac42af87393b1f3c39465abec63fd495))
    - Merge pull request #63 from LuxxxLucy/lucy-multiple-proposal-br ([`2d675d5`](https://github.com/fonttools/fontspector/commit/2d675d5bfe5cdb3de99e1a2cf8c65964c144bc52))
    - Fix typo ([`0dee5f2`](https://github.com/fonttools/fontspector/commit/0dee5f29e8c842c1b802d2cfb4ef4ed91b44cd1c))
    - Fix report with multiple proposals ([`12aae41`](https://github.com/fonttools/fontspector/commit/12aae4183be24a71f796f2f86c8adbb2b99caaa4))
    - Merge pull request #78 from fonttools/dep-tidying ([`6633571`](https://github.com/fonttools/fontspector/commit/66335714c16c21c902d8459814a0b37ddfcddf5d))
    - Tidy dependencies ([`9a8c5fa`](https://github.com/fonttools/fontspector/commit/9a8c5face5eadbb2daffb606e4d42af052f73c7c))
    - Merge pull request #77 from fonttools/duckdb ([`610bd5c`](https://github.com/fonttools/fontspector/commit/610bd5c0c6da2d6ab76427e594e2646edac2deac))
    - Merge branch 'main' into duckdb ([`ef0ebe8`](https://github.com/fonttools/fontspector/commit/ef0ebe87d43f220a56310c1a367e9486f2cdff7c))
    - Exception handling ([`f574c71`](https://github.com/fonttools/fontspector/commit/f574c717ae0ff46a149ff7e58d51c24cea32eeb8))
    - Bundle templates and extract, similar to diffenator3 ([`42271d8`](https://github.com/fonttools/fontspector/commit/42271d8090f715c54e4c4a79d586a69bd543f0dc))
    - Update duckdb to suppress warnings ([`85cb392`](https://github.com/fonttools/fontspector/commit/85cb39222a65b9f09d3f4f253d66ceb64fc7f3e8))
    - Add duckdb reporter ([`fce59b3`](https://github.com/fonttools/fontspector/commit/fce59b377a8eca93db4f51c40bd61870b245033c))
    - CSV fixup ([`a4c4c3b`](https://github.com/fonttools/fontspector/commit/a4c4c3b2243eca63d19e18074eff1fa4341037df))
    - CSV reporter ([`0532314`](https://github.com/fonttools/fontspector/commit/053231460e089381d37879858b3b3c0fbb10c9a5))
    - Support loading TOML profiles, convert from Python ([`bc64195`](https://github.com/fonttools/fontspector/commit/bc641955ab91a9926e8cebeb231cf947da1637bc))
    - Optimizations ([`84cd869`](https://github.com/fonttools/fontspector/commit/84cd869bc1d533fa9e6fa498fa61de65ff491290))
    - "He says to follow the crossed out instructions." "Then why were they crossed out?" ([`74fcb5e`](https://github.com/fonttools/fontspector/commit/74fcb5ef9c8590d81da45c253fe20feddaba4221))
    - Warnings fix ([`c90d6c0`](https://github.com/fonttools/fontspector/commit/c90d6c086ae41c982fe2f4512158741979acca97))
    - Allow for chaining hotfixes ([`9543e3d`](https://github.com/fonttools/fontspector/commit/9543e3da857864027bc6e69d86b52b2d6fd4500b))
    - Update vesions, minimize dependencies ([`8f43370`](https://github.com/fonttools/fontspector/commit/8f433709f66727148a18278383c3b519ce99e331))
    - Fold article/ into parent directory's group ([`9188c5a`](https://github.com/fonttools/fontspector/commit/9188c5aa87767418e2cc2ddebf2110b9d4551704))
    - Collections can have a name ([`5c202d7`](https://github.com/fonttools/fontspector/commit/5c202d75cd9623a2275d2a95fde91554014891ed))
    - Alternative implementation of --list-checks-json ([`3258ea4`](https://github.com/fonttools/fontspector/commit/3258ea42a31ffddb65581875bf8c7058207c63e4))
    - Various cleanups ([`9bb92fc`](https://github.com/fonttools/fontspector/commit/9bb92fca9e86079c9d6422220742d995583d74a3))
    - HELLLOOO shared mutable state! ([`ec1bdfa`](https://github.com/fonttools/fontspector/commit/ec1bdfaacfae1a33fd0afc7246d0af398f7f3b9d))
    - Minimize tera dependencies ([`54451d4`](https://github.com/fonttools/fontspector/commit/54451d4bf3e2ca31ea2e987687b9fa32ed49c0c2))
    - Only use clap in the CLI ([`a54b63f`](https://github.com/fonttools/fontspector/commit/a54b63fdd5eaedcfd56c22dd55b6df77d7ff3f32))
    - Store timing information for tests ([`0a3c032`](https://github.com/fonttools/fontspector/commit/0a3c0327b46451e751cee3a2d85c44190d1f699e))
    - Add interpolation issues check ([`7671c6b`](https://github.com/fonttools/fontspector/commit/7671c6bc9c045ff6842356ba5437d48ae3f3d313))
    - Share itertools versions ([`71e6f81`](https://github.com/fonttools/fontspector/commit/71e6f81d35e3fbe8540a38ec532e382effa87459))
    - Add --full-lists ([`8e1ae0b`](https://github.com/fonttools/fontspector/commit/8e1ae0b994b7b050c12245b32116d561554d9523))
    - Improve check listing and ordering ([`1b9e239`](https://github.com/fonttools/fontspector/commit/1b9e239d675f40f6ca87d057352c2bc0ff47d952))
    - Don’t report things below worst-status loglevel ([`8c4994b`](https://github.com/fonttools/fontspector/commit/8c4994b70ab046d049365e5aed9f891f5fa1f55e))
    - Limit input files to files, not directories ([`2a00817`](https://github.com/fonttools/fontspector/commit/2a0081783478e7a0ecff7fbbc198793ef0a48c17))
    - Good to show off sometimes ([`e6b1e8e`](https://github.com/fonttools/fontspector/commit/e6b1e8e141c10806a8f39ad1dae132d567005fb4))
    - Tidy up release build warning ([`f972432`](https://github.com/fonttools/fontspector/commit/f972432d282db72d5d8bd313776464f6dc2c4dc1))
    - Some leftovers from universal/opentype split ([`c71f0da`](https://github.com/fonttools/fontspector/commit/c71f0da94f981a4bc69bda022ad1a2039a17f0d9))
    - Everybody loves progress bars ([`a417c60`](https://github.com/fonttools/fontspector/commit/a417c60c12008d0143fa8dbcd38e9a069eea3dc3))
    - Merge pull request #15 from felipesanches/issue_14 ([`57a2274`](https://github.com/fonttools/fontspector/commit/57a2274c13a2ac02292eaf60ec37f7cb63098304))
    - Split profiles Universal and OpenType ([`72550af`](https://github.com/fonttools/fontspector/commit/72550af9c9c8f9a9f4dad37a52f789290b4f6fb8))
    - Make Python fallback optional ([`4139866`](https://github.com/fonttools/fontspector/commit/413986622af03909f3eadcbbe5b525cad22b98db))
    - Rework Python bridge ([`e357d73`](https://github.com/fonttools/fontspector/commit/e357d73000b82b71ee93f28f71c5b16c5ca819d1))
    - Pass check metadata (a JSON string) into the check itself ([`f1013ab`](https://github.com/fonttools/fontspector/commit/f1013ab087b6c9aa16834b9e1ff371cb0cd541be))
    - Make section optional, fixes #11 ([`fc36a5c`](https://github.com/fonttools/fontspector/commit/fc36a5c506918139969d0bb60a8d924e017c2641))
    - Pass check metadata (a JSON string) into the check itself ([`b682152`](https://github.com/fonttools/fontspector/commit/b68215290bff6f1bd373e6c6ee2ab822d51eba4f))
    - Make section optional, fixes #11 ([`bcce8f9`](https://github.com/fonttools/fontspector/commit/bcce8f9009ce747f26d5cd4bfcfa4d83b0576ee6))
    - List checks ([`332ecc3`](https://github.com/fonttools/fontspector/commit/332ecc3dd7bcfd390dc9ee90bb2ea5e86b6c4d88))
    - Format ([`1cc1629`](https://github.com/fonttools/fontspector/commit/1cc162946dec0f3931aae9071995914f43224c58))
    - Improvements to parallelism ([`fd654ac`](https://github.com/fonttools/fontspector/commit/fd654ac5db60d7c356e460be8762d81b3e4ebaac))
    - Terminal tweaks ([`e28e00f`](https://github.com/fonttools/fontspector/commit/e28e00f85dbc1454cd9f3ded9bf2ff3176b51983))
    - Show rationale once per result, fixes #3 ([`ec67778`](https://github.com/fonttools/fontspector/commit/ec67778c5ba3bd9e869399b47068a14287d75d74))
    - More universal/opentype checks ([`f5750bd`](https://github.com/fonttools/fontspector/commit/f5750bdf9cdfcf5b1e5fefb76bc34a600046b488))
    - Clippy lints ([`d46fdc3`](https://github.com/fonttools/fontspector/commit/d46fdc3ca2517e26a8d8fe5d91a6fded279b43ed))
    - Tidy up checkorder madness, make siblings work in WASM ([`da1d142`](https://github.com/fonttools/fontspector/commit/da1d14229143dd009cf2a4987846e296eb305388))
    - WIP solve the sibling problem ([`10430e5`](https://github.com/fonttools/fontspector/commit/10430e572099e1185247ab78b083de43c154f1a6))
    - Make TestableCollection the primary unit of testing ([`70da856`](https://github.com/fonttools/fontspector/commit/70da8567069c053415067598ffbe428901784b59))
    - Clippy lints ([`9da264f`](https://github.com/fonttools/fontspector/commit/9da264f9eb177149c6212ed316fc28ef77761652))
    - Tidy up dependencies ([`395112f`](https://github.com/fonttools/fontspector/commit/395112f646b53d446dd082174026fa3ce381f095))
    - Split hotfixing from reporting ([`5ff0e39`](https://github.com/fonttools/fontspector/commit/5ff0e39aed5fc96c2f8ef77debb9099831d39f56))
    - Improve terminal reporting, add ghmarkdown ([`6480cf0`](https://github.com/fonttools/fontspector/commit/6480cf0c4ba14bfab6ce4ba035c1d3980f8414f9))
    - Checks are now serializable, JSON reporter is easy! ([`fa0fbbc`](https://github.com/fonttools/fontspector/commit/fa0fbbced9545e523c29ee76b9514d4fce45c13b))
    - Rearrange run result struct, add subresult codenames/severity ([`2d99a2b`](https://github.com/fonttools/fontspector/commit/2d99a2b760b43d7cdf4630800d25493e0d7485a1))
    - Tidy up deps ([`423b231`](https://github.com/fonttools/fontspector/commit/423b23133690a46092819600aff82adffaf7fdfa))
    - Simple, not very good, JSON reporter ([`ce5decf`](https://github.com/fonttools/fontspector/commit/ce5decf64f19bf5322b56955a2ce94ec90aeb4c8))
    - Support --succinct ([`85c5f81`](https://github.com/fonttools/fontspector/commit/85c5f81a09b60cc116335cfcb233adb6084327b6))
    - Fix bad merge ([`ad70d24`](https://github.com/fonttools/fontspector/commit/ad70d249e93c20c29b474adea4a77b2244ab58f3))
    - Begin to separate reporters ([`22a7aef`](https://github.com/fonttools/fontspector/commit/22a7aefce4080033412ad22c5112b7375115a23b))
    - Tidy up results handling ([`9eab7d7`](https://github.com/fonttools/fontspector/commit/9eab7d786d92f77fa0c2c91a85b876e29af5e1f8))
    - Split reporters out ([`b4bae8e`](https://github.com/fonttools/fontspector/commit/b4bae8ea05fdd62c07522aa651e356710e865827))
    - Hook up network args ([`aed3345`](https://github.com/fonttools/fontspector/commit/aed3345a681a20d4a4d74c637fbc17158fba5953))
    - Tidy up ([`1ba78b8`](https://github.com/fonttools/fontspector/commit/1ba78b8d19b60f35883a233362d07475427e8fd5))
    - Add configuration and check context ([`caeb4b7`](https://github.com/fonttools/fontspector/commit/caeb4b7478a4a51bd5130fe85eb7043758e2236d))
    - Support -q ([`edc0697`](https://github.com/fonttools/fontspector/commit/edc0697ebff8d520e3db363849658ad61f6e9fba))
    - Port some fontbakery options ([`ef810af`](https://github.com/fonttools/fontspector/commit/ef810af2ef6ddabbdd63b9f4b8c8d5d5fd64aae5))
    - Improve display ([`27c29fd`](https://github.com/fonttools/fontspector/commit/27c29fdfe1ee02e8dc337e9542c288ca93efc0cb))
    - Merge pull request #5 from felipesanches/rationales_not_optional ([`ee113d9`](https://github.com/fonttools/fontspector/commit/ee113d98a0cb146a764163c6afeacae05f0ece9f))
    - Merge branch 'main' into rationales_not_optional ([`37122c3`](https://github.com/fonttools/fontspector/commit/37122c334183fa689fbe4f5617b1ca24e6abb95c))
    - Be (slightly) more grown-up about error handling ([`2818a76`](https://github.com/fonttools/fontspector/commit/2818a764da76b9acc2c33127cb156238dca970c1))
    - Rationale and proposal fields are not optional ([`752d559`](https://github.com/fonttools/fontspector/commit/752d5593f3c5a345a781f8b76e5907607bda7dbd))
    - Fix check order, slightly improve reporting ([`cc98245`](https://github.com/fonttools/fontspector/commit/cc98245bd4e712cfc71d789e28a763da1e13eb2d))
    - Bake in GF profile ([`1628604`](https://github.com/fonttools/fontspector/commit/16286048b26e5a6fb7d07ab5ef69e05e9c592b09))
    - Allow included profiles, make registering profile a Result ([`4d7a296`](https://github.com/fonttools/fontspector/commit/4d7a296a76c2717c895784d8d1e795a1740a3859))
    - Apply hotfixes ([`9dfab85`](https://github.com/fonttools/fontspector/commit/9dfab855d6e2e4591b71c5f5ec3fbcbadaeaa7e7))
    - Make check registry a map ([`44aae7b`](https://github.com/fonttools/fontspector/commit/44aae7bdc987e6a01587fcfd38dabb5fdfdeadd8))
    - Make it parallelable ([`a00b396`](https://github.com/fonttools/fontspector/commit/a00b3961e5461983bbc1b0b06baf367f4c357e2c))
    - Tidy lots of things up, allow pluggable file types ([`1651816`](https://github.com/fonttools/fontspector/commit/1651816d634137e319925acb9dc33da66ccf38e9))
    - Rename workspace members ([`f97a39a`](https://github.com/fonttools/fontspector/commit/f97a39a80faf667006de20741f14e7736c5a966c))
    - Clean up warnings ([`b2a6b0b`](https://github.com/fonttools/fontspector/commit/b2a6b0b5b8316b78db740222ec2287f3d69bd366))
    - Add the concept of a profile ([`41a37dc`](https://github.com/fonttools/fontspector/commit/41a37dc02a6aa9f16b369af304c5c70861343439))
    - Rename some stuff ([`f174d56`](https://github.com/fonttools/fontspector/commit/f174d56325e86cd4ade690ab8e5ffaa9fcecca30))
    - Move to plugin architecture ([`5fdf975`](https://github.com/fonttools/fontspector/commit/5fdf9750991176c8e2776557ce6c17c642c24a73))
</details>

