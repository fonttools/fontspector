# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.1.0 (2026-11-11)

### Other

 - <csr-id-69e30ff7dab640c64c3708df6550dd4460d948a5/> Release fontspector-profile-microsoft v0.1.0, fontspector-profile-monotype v1.1.0
 - <csr-id-0ad68fae2f8d29f2d106ba7ed6e77173f690d7f5/> Release fontspector-profile-monotype v1.1.0

### Chore

 - <csr-id-72c96f7ba104074edfb9f5e52a682faaa13cd1bd/> Hide sourcefixes behind feature to slim deps
   * chore: Lint fixups
   
   * chore: Hide sourcefixes behind feature to slim deps
   
   * chore: Work under unification

### New Features

 - <csr-id-9f87c1473922a2d90ec04e989983c9a215b19d4e/> report unnecessary font tables
   * feat(monotype/unnecessary_tables): add first draft of new check for unnecessary tables
   
   * refactor(monotype/unnecessary_tables): find solution for TSI.|MT..|ASC.|edt.|ABA.
   
   * docs: design for configurable monotype/unnecessary_tables lists
 - <csr-id-ebccc3d4cdae3a7572d0a7b586db4fd1ac267027/> add first draft of a check
   * feat(monotype/vertical_metrics_sane): add first draft of a test
   
   * feat(monotype): use microsoft/vertical_metrics
   
   * fix(monotype/vertical_metrics_sane): win ascent and descent cannot be a negativ value –> check == 0
   
   ---------
 - <csr-id-3ae64e1f9bc0afc5d92d04f69c230b143cfb22b6/> add monotype specific check for weight classes
   * feat(monotype/weightclass): add monotype specific check for weight classes
   
   * test(monotype/weightclass): extend with example 'UltraBold' = 800 -> fail
   
   * refactor(monotype/weightclass): Update check texts
   
   ---------
 - <csr-id-1c19d5a1b938705400278788a1c9423faa035020/> new check for cmap encoding records
   * feat(): first (empty) implementation of new check 'monotype/cmap_subtables'
   
   * feat(monotype/cmap_encoding_records): implement first draft
 - <csr-id-643f0ac04d2cd63f0078963d87b84b46723946a7/> check if all fvar axes are present in STAT
   * feat(monotype): add microsoft::STAT_table_eliding_bit to monotype checks
   
   * feat(universal/fvar_stat_axes): new test to check if all var axes are present in STAT
   
   ---------
 - <csr-id-811b49ea88e5c972f64ed46dd9f78b5c970c5d44/> add microsoft/office_ribz_req
   * feat(monotype): add microsoft/office_ribz_req
   
   * fix(monotype): add missing profile-microsoft
   
   ---------
 - <csr-id-0a51f17d1a7edd2b6ddf6767b1d8c5ab107efba8/> specific name IDs must be unique in a font family
   * feat(opentype/family/unique_names): specific name IDs must be unique within font family
   
   * test(opentype/family/unique_names): extend unittest with variable fonts to check name ID 25
   
   * fix(family/unique_names): minor adjustments to comments
   
   * test(family/unique_names): add failing unittest for name ID combos
   
   for checking name IDs uniqueness of combos: 1+2, 16+17, 21+22
   
   * test(opentype/family/unique_names): code formatting
   
   * feat(opentype/family/unique_names): check name id combos to be unique in font family
   
   for checking name IDs uniqueness of combos: 1+2, 16+17, 21+22
   + remove obsolete test test_unique_names_cabin_fail
   
   * refactor(opentype/family/unique_names): rework check description based on latest updates
   
   * refactor(opentype/family/unique_names): extend unittests
   
   with examples from Monotype
   
   * refactor(opentype/family/unique_names): fix wrong name ID in check description
   
   * refactor(opentype/family/unique_names): extend unittest with expected message
   
   * refactor(opentype/family/unique_names): improve report message
   
   use file names instead of file paths
   
   * refactor(opentype/family/unique_names): fix 'panic' (make Lint happy)
   
   * test(opentype/family/unique_names): add failing unittest to check variable font without NID 25
   
   * refactor(opentype/family/unique_names): add config (for eg. name ID 25)
   
   * refactor(monotype): custom config for opentype/family/unique_names
   
   * refactor(opentype/family/unique_names): improve message text
   
   * refactor(opentype/family/unique_names): rework FontspectorError
   
   * refactor(opentype/family/unique_names): remove .to_string()
   
   ---------
 - <csr-id-065698a426c3e767ab4305b20e271b056da5fc5b/> add "monotype/widthclass"
   * feat(monotype): add "monotype/widthclass"
   
   * refactor(monotype/widthclass): formatting
   
   * fix(monotype/widthclass): fix failing unittest
   
   * test(test_widthclass): remove duplicate
   
   * refactor(monotype/widthclass): use two separate codes
   
   * test(monotype/widthclass): add family names
   
   * test(monotype/widthclass): extend (failing) test cases
   
   width name in family name (this happens)
   
   * refactor(monotype/widthclass): use full name instead of subfamily (only)
   
   * test(monotype/widthclass): add Monotype specific width names
   
   * refactor(monotype/widthclass): add Monotype specific width names
   
   * test(monotype/widthclass): add edge cases to unittests
   
   * test(monotype/widthclass): add more edge cases to unittests
   
   * refactor(monotype/widthclass): update report strings
   
   * refactor(monotype/widthclass): more consistent variable name
   
   * refactor(monotype/widthclass): formatting
   
   * feat(monotype/widthclass): add config for preferred width class names
   
   * feat(monotype/widthclass): add more width names
   
   ---------
 - <csr-id-74567865f188d27a10dd9fae239b112266d68437/> Is name ID 1 free of the reserved keywords?
   * fix(PlatformSelector): make it cloneable
   
   * feat: create new monotype check 'style_in_family_name'
   
   * test(monotype/style_in_family_name): extend unittest with keyword 'Regular' in name ID 1+2
   
   * fix(monotype/style_in_family_name): add 'Regular' to reserved style names
   
   * fix(monotype/style_in_family_name): title description
   
   * refactor(monotype/style_in_family_name): make Lint happy
   
   ---------

### Test

 - <csr-id-e63dd977862cac7b0b673f7e7ebbc04ab00ae272/> extend unittest
   * test(universal/valid_glyphnames): extend unittest
   
   * test(test_various_glyphnames): add "zero.tnum" as valid name
   
   * test(test_various_glyphnames): glyph name which exceeds the limit of 63 chars
   
   * test(valid_glyphnames): add failing test
   
   * test(valid_glyphnames): extend test_various_glyphnames_adobe
   
   * test(valid_glyphnames): add failing test for names including hyphens
   
   * refactor(valid_glyphnames): allow special characters via config
   
   * fix(valid_glyphnames): formatting
   
   * test(valid_glyphnames): add unittest with config for ALLOWED_SPECIAL_CHARS
   
   * refactor(monotype): use config ALLOWED_SPECIAL_CHARS for valid_glyphnames
   
   * refactor(monotype): remove unused imports
   
   ---------

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 22 calendar days.
 - 54 days passed between releases.
 - 13 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 11 unique issues were worked on: [#1000](https://github.com/fonttools/fontspector/issues/1000), [#904](https://github.com/fonttools/fontspector/issues/904), [#921](https://github.com/fonttools/fontspector/issues/921), [#923](https://github.com/fonttools/fontspector/issues/923), [#934](https://github.com/fonttools/fontspector/issues/934), [#948](https://github.com/fonttools/fontspector/issues/948), [#951](https://github.com/fonttools/fontspector/issues/951), [#961](https://github.com/fonttools/fontspector/issues/961), [#962](https://github.com/fonttools/fontspector/issues/962), [#975](https://github.com/fonttools/fontspector/issues/975), [#986](https://github.com/fonttools/fontspector/issues/986)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#1000](https://github.com/fonttools/fontspector/issues/1000)**
    - Hide sourcefixes behind feature to slim deps ([`72c96f7`](https://github.com/fonttools/fontspector/commit/72c96f7ba104074edfb9f5e52a682faaa13cd1bd))
 * **[#904](https://github.com/fonttools/fontspector/issues/904)**
    - Add "monotype/widthclass" ([`065698a`](https://github.com/fonttools/fontspector/commit/065698a426c3e767ab4305b20e271b056da5fc5b))
 * **[#921](https://github.com/fonttools/fontspector/issues/921)**
    - Add monotype specific check for weight classes ([`3ae64e1`](https://github.com/fonttools/fontspector/commit/3ae64e1f9bc0afc5d92d04f69c230b143cfb22b6))
 * **[#923](https://github.com/fonttools/fontspector/issues/923)**
    - Add first draft of a check ([`ebccc3d`](https://github.com/fonttools/fontspector/commit/ebccc3d4cdae3a7572d0a7b586db4fd1ac267027))
 * **[#934](https://github.com/fonttools/fontspector/issues/934)**
    - Specific name IDs must be unique in a font family ([`0a51f17`](https://github.com/fonttools/fontspector/commit/0a51f17d1a7edd2b6ddf6767b1d8c5ab107efba8))
 * **[#948](https://github.com/fonttools/fontspector/issues/948)**
    - Is name ID 1 free of the reserved keywords? ([`7456786`](https://github.com/fonttools/fontspector/commit/74567865f188d27a10dd9fae239b112266d68437))
 * **[#951](https://github.com/fonttools/fontspector/issues/951)**
    - New check for cmap encoding records ([`1c19d5a`](https://github.com/fonttools/fontspector/commit/1c19d5a1b938705400278788a1c9423faa035020))
 * **[#961](https://github.com/fonttools/fontspector/issues/961)**
    - Extend unittest ([`e63dd97`](https://github.com/fonttools/fontspector/commit/e63dd977862cac7b0b673f7e7ebbc04ab00ae272))
 * **[#962](https://github.com/fonttools/fontspector/issues/962)**
    - Report unnecessary font tables ([`9f87c14`](https://github.com/fonttools/fontspector/commit/9f87c1473922a2d90ec04e989983c9a215b19d4e))
 * **[#975](https://github.com/fonttools/fontspector/issues/975)**
    - Add microsoft/office_ribz_req ([`811b49e`](https://github.com/fonttools/fontspector/commit/811b49ea88e5c972f64ed46dd9f78b5c970c5d44))
 * **[#986](https://github.com/fonttools/fontspector/issues/986)**
    - Check if all fvar axes are present in STAT ([`643f0ac`](https://github.com/fonttools/fontspector/commit/643f0ac04d2cd63f0078963d87b84b46723946a7))
 * **Uncategorized**
    - Release fontspector-profile-monotype v1.1.0 ([`0ad68fa`](https://github.com/fonttools/fontspector/commit/0ad68fae2f8d29f2d106ba7ed6e77173f690d7f5))
    - Release fontspector-profile-microsoft v0.1.0, fontspector-profile-monotype v1.1.0 ([`69e30ff`](https://github.com/fonttools/fontspector/commit/69e30ff7dab640c64c3708df6550dd4460d948a5))
</details>

## v1.0.0 (2026-09-18)

### Chore

 - <csr-id-178f989dea5ccb0a2853fe9e32f7f6b270196b67/> Minimize the crates required by the hotfix crate
   * chore: Minimize the crates required by the hotfix crate
   
   * chore: Update fontc deps

### New Features

 - <csr-id-a1d6f34f36bfc98371c8c1b35a73f7f79ae55ccc/> new profile for 'Monotype'
   * feat: new profile for 'Monotype'
   
   * chore: fontations->skrifa
   
   ---------

### Bug Fixes

 - <csr-id-9f223e7b4504844293c9a16ef2fa613bbc255598/> extend excludes
   * fix(monotype): extend excludes
   
   * refactor(fontwerk): make 'checks' not private
   
   * refactor(monotype): rework registered checks
   
   ---------

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 9 calendar days.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#893](https://github.com/fonttools/fontspector/issues/893), [#905](https://github.com/fonttools/fontspector/issues/905), [#916](https://github.com/fonttools/fontspector/issues/916)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#893](https://github.com/fonttools/fontspector/issues/893)**
    - New profile for 'Monotype' ([`a1d6f34`](https://github.com/fonttools/fontspector/commit/a1d6f34f36bfc98371c8c1b35a73f7f79ae55ccc))
 * **[#905](https://github.com/fonttools/fontspector/issues/905)**
    - Minimize the crates required by the hotfix crate ([`178f989`](https://github.com/fonttools/fontspector/commit/178f989dea5ccb0a2853fe9e32f7f6b270196b67))
 * **[#916](https://github.com/fonttools/fontspector/issues/916)**
    - Extend excludes ([`9f223e7`](https://github.com/fonttools/fontspector/commit/9f223e7b4504844293c9a16ef2fa613bbc255598))
 * **Uncategorized**
    - Release fontspector-profile-monotype v1.0.0 ([`2ee1558`](https://github.com/fonttools/fontspector/commit/2ee1558e7a4d0bca79057f735ccdb1b5620caa2d))
</details>

