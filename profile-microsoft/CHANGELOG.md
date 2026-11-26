# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.0 (2026-11-10)

### Other

 - <csr-id-2cf0d1324efc6b7c1da7d01d221d2d5a2449c188/> Add package metadata
 - <csr-id-48e29ee72c0d32e4278e235056cbb24d8db9a207/> Various build improvements
   * build: Remove old silfonts plugin binary
   
   * build: Monotype plugin requires Microsoft plugin so it must be published
   
   * feat: Expose Microsoft profile
   
   * chore: Oops dependency
 - <csr-id-69e30ff7dab640c64c3708df6550dd4460d948a5/> Release fontspector-profile-microsoft v0.1.0, fontspector-profile-monotype v1.1.0

### Chore

 - <csr-id-72c96f7ba104074edfb9f5e52a682faaa13cd1bd/> Hide sourcefixes behind feature to slim deps
   * chore: Lint fixups
   
   * chore: Hide sourcefixes behind feature to slim deps
   
   * chore: Work under unification
 - <csr-id-cff578b07e45be364b13d3e23b1337938f8fcd6f/> Update harfrust to new API
 - <csr-id-178f989dea5ccb0a2853fe9e32f7f6b270196b67/> Minimize the crates required by the hotfix crate
   * chore: Minimize the crates required by the hotfix crate
   
   * chore: Update fontc deps
 - <csr-id-a193d56a4e4051b2f464bc59b9351a4439c023ce/> Move from dead fontations to skrifa/write_fonts
   * chore: Drop fontations dep, use skrifa/write_fonts
   
   * chore: Thanks clippy
 - <csr-id-ba4aa4052252bc103ba0742bea043c75d3f25265/> Thanks clippy
 - <csr-id-67faec64118db1aa7f5ef5399ea543a89494a872/> Move from rustybuzz to harfrust
 - <csr-id-efb68d5d5071abc4c45da0ea58715d87d39b360e/> Create list of publishable packages, send to smart-release
   * chore: Deny publishing these crates
   
   * chore(ci): Add a script to find the publishable crates
 - <csr-id-f44be5515dcaea17b96b1df7a4b11407561d0c17/> Improve error handling
   * chore: Better error handling
   
   * chore: Better error handling for fix functions too

### New Features

 - <csr-id-811b49ea88e5c972f64ed46dd9f78b5c970c5d44/> add microsoft/office_ribz_req
   * feat(monotype): add microsoft/office_ribz_req
   
   * fix(monotype): add missing profile-microsoft
   
   ---------
 - <csr-id-5ac424de337de94d8f09835b3dc2cf41223a7a5b/> Source fixing
   * feat: WIP source fixing infrastructure
   
   * chore: Better arg parsing
   
   * chore: CLI reporting fixes for source fixes
   
   * chore: typo
   
   * chore: More error handling, save a designspace file
   
   * feat(googlefonts/use_typo_metrics): Source-fix UFOs
   
   * chore: Don't try fixing directories
   
   * feat(soft_hyphen): Source fix
   
   * chore: Use git not local path
   
   * chore: Utility functions for source fixes
   
   * feat(linegaps): Source fix
   
   * chore(cli): Fontbakery users like uppercase names for -L
   
   * chore: Make attrs conditional
   
   * chore(cli): Add fix info to list checks output
   
   * feat(valid_glyphnames): Source fix
   
   * chore: Regenerate Cargo.lock
   
   * chore: This was never true
   
   * feat: Use interactive loop for source fixes too
   
   * chore: Thanks clippy
   
   * feat: Pipe results back into source fixes
   
   * chore: Make source map warning more prominent
   
   * feat(mandatory_glyphs): Sourcefix .notdef
   
   * feat(unreachable_glyphs): Sourcefix unreachable glyphs
   
   * chore: Thanks clippy
   
   * feat(line_breaks): Sourcefix line breaks in name table entries
   
   * feat(name/trailing_spaces): Sourcefix trailling spaces
   
   * chore: Source hygiene
   
   * feat(whitespace_widths): Sourcefix whitespace widths

### Bug Fixes

 - <csr-id-ba8ff42d7ebb12aed7ec90a608f7cc61da073394/> only validate name ID 2 for RBIZ
   * fix(microsoft/office_ribz_req): only validate name ID 2 for RBIZ
   
   best_subfamilyname() also considered name IDs 17 and 22, which made
   non-RIBBI styles fail incorrectly. Match FontBakery by checking
   SUBFAMILY_NAME only.
   
   * test(microsoft/office_ribz_req): add unit test
   
   * fix formatting
   
   ---------
 - <csr-id-46e90e51624979590af83272f96cbcfc521b7d0a/> Improve rationale rewrapping
   * fix(cli): Improve rationale rewrapping
   
   * chore: Style fixes for new clippy

### Refactor

 - <csr-id-7b32eca0846e594655f304250928295ffbf6496a/> New plugin architecture
   * refactor: Remove fontbakery-bridge
   
   * refactor: New plugin architecture
   
   * feat: Demonstrate Python-based plugins
   
   * docs: New plugin architecture
 - <csr-id-8b28d1aa1c7af4dacdbcfbd83af69dbf401ecf46/> Dynamically update script and language tags from MS website

### Style

 - <csr-id-a6b7ffc4f39c6b1c1bd92cd9b07f4ba22d54ef2e/> deny indexing slicing
   * chore: More lints into Cargo.toml
   
   * style: Deny indexing slicing

### Test

 - <csr-id-99aa644d87c1e6c8528f3faa66c9baf7beab6aa2/> Port more tests to Rust

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 27 commits contributed to the release.
 - 19 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 21 unique issues were worked on: [#1000](https://github.com/fonttools/fontspector/issues/1000), [#1013](https://github.com/fonttools/fontspector/issues/1013), [#1019](https://github.com/fonttools/fontspector/issues/1019), [#1022](https://github.com/fonttools/fontspector/issues/1022), [#107](https://github.com/fonttools/fontspector/issues/107), [#109](https://github.com/fonttools/fontspector/issues/109), [#111](https://github.com/fonttools/fontspector/issues/111), [#127](https://github.com/fonttools/fontspector/issues/127), [#275](https://github.com/fonttools/fontspector/issues/275), [#287](https://github.com/fonttools/fontspector/issues/287), [#299](https://github.com/fonttools/fontspector/issues/299), [#315](https://github.com/fonttools/fontspector/issues/315), [#336](https://github.com/fonttools/fontspector/issues/336), [#581](https://github.com/fonttools/fontspector/issues/581), [#782](https://github.com/fonttools/fontspector/issues/782), [#851](https://github.com/fonttools/fontspector/issues/851), [#875](https://github.com/fonttools/fontspector/issues/875), [#898](https://github.com/fonttools/fontspector/issues/898), [#905](https://github.com/fonttools/fontspector/issues/905), [#975](https://github.com/fonttools/fontspector/issues/975), [#997](https://github.com/fonttools/fontspector/issues/997)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#1000](https://github.com/fonttools/fontspector/issues/1000)**
    - Hide sourcefixes behind feature to slim deps ([`72c96f7`](https://github.com/fonttools/fontspector/commit/72c96f7ba104074edfb9f5e52a682faaa13cd1bd))
 * **[#1013](https://github.com/fonttools/fontspector/issues/1013)**
    - Port more tests to Rust ([`99aa644`](https://github.com/fonttools/fontspector/commit/99aa644d87c1e6c8528f3faa66c9baf7beab6aa2))
 * **[#1019](https://github.com/fonttools/fontspector/issues/1019)**
    - Various build improvements ([`48e29ee`](https://github.com/fonttools/fontspector/commit/48e29ee72c0d32e4278e235056cbb24d8db9a207))
 * **[#1022](https://github.com/fonttools/fontspector/issues/1022)**
    - Add package metadata ([`2cf0d13`](https://github.com/fonttools/fontspector/commit/2cf0d1324efc6b7c1da7d01d221d2d5a2449c188))
 * **[#107](https://github.com/fonttools/fontspector/issues/107)**
    - Move to fontations crate ([`da2830b`](https://github.com/fonttools/fontspector/commit/da2830ba694bf3379142a81dad043031e1c39f35))
 * **[#109](https://github.com/fonttools/fontspector/issues/109)**
    - Custom profiles in WASM ([`fd6fe61`](https://github.com/fonttools/fontspector/commit/fd6fe619381e3455fb0fb86dc35a59c502e68385))
 * **[#111](https://github.com/fonttools/fontspector/issues/111)**
    - Include most of the fixes from gftools-fix ([`2de6875`](https://github.com/fonttools/fontspector/commit/2de68751c8c4da8c29f9e46d444280cdf478c6b2))
 * **[#127](https://github.com/fonttools/fontspector/issues/127)**
    - Dynamically update script and language tags from MS website ([`8b28d1a`](https://github.com/fonttools/fontspector/commit/8b28d1aa1c7af4dacdbcfbd83af69dbf401ecf46))
 * **[#275](https://github.com/fonttools/fontspector/issues/275)**
    - Improve error handling ([`f44be55`](https://github.com/fonttools/fontspector/commit/f44be5515dcaea17b96b1df7a4b11407561d0c17))
 * **[#287](https://github.com/fonttools/fontspector/issues/287)**
    - Deny indexing slicing ([`a6b7ffc`](https://github.com/fonttools/fontspector/commit/a6b7ffc4f39c6b1c1bd92cd9b07f4ba22d54ef2e))
 * **[#299](https://github.com/fonttools/fontspector/issues/299)**
    - Improve rationale rewrapping ([`46e90e5`](https://github.com/fonttools/fontspector/commit/46e90e51624979590af83272f96cbcfc521b7d0a))
 * **[#315](https://github.com/fonttools/fontspector/issues/315)**
    - Source fixing ([`5ac424d`](https://github.com/fonttools/fontspector/commit/5ac424de337de94d8f09835b3dc2cf41223a7a5b))
 * **[#336](https://github.com/fonttools/fontspector/issues/336)**
    - Create list of publishable packages, send to smart-release ([`efb68d5`](https://github.com/fonttools/fontspector/commit/efb68d5d5071abc4c45da0ea58715d87d39b360e))
 * **[#581](https://github.com/fonttools/fontspector/issues/581)**
    - Move from rustybuzz to harfrust ([`67faec6`](https://github.com/fonttools/fontspector/commit/67faec64118db1aa7f5ef5399ea543a89494a872))
 * **[#782](https://github.com/fonttools/fontspector/issues/782)**
    - New plugin architecture ([`7b32eca`](https://github.com/fonttools/fontspector/commit/7b32eca0846e594655f304250928295ffbf6496a))
 * **[#851](https://github.com/fonttools/fontspector/issues/851)**
    - Update harfrust to new API ([`cff578b`](https://github.com/fonttools/fontspector/commit/cff578b07e45be364b13d3e23b1337938f8fcd6f))
 * **[#875](https://github.com/fonttools/fontspector/issues/875)**
    - Thanks clippy ([`ba4aa40`](https://github.com/fonttools/fontspector/commit/ba4aa4052252bc103ba0742bea043c75d3f25265))
 * **[#898](https://github.com/fonttools/fontspector/issues/898)**
    - Move from dead fontations to skrifa/write_fonts ([`a193d56`](https://github.com/fonttools/fontspector/commit/a193d56a4e4051b2f464bc59b9351a4439c023ce))
 * **[#905](https://github.com/fonttools/fontspector/issues/905)**
    - Minimize the crates required by the hotfix crate ([`178f989`](https://github.com/fonttools/fontspector/commit/178f989dea5ccb0a2853fe9e32f7f6b270196b67))
 * **[#975](https://github.com/fonttools/fontspector/issues/975)**
    - Add microsoft/office_ribz_req ([`811b49e`](https://github.com/fonttools/fontspector/commit/811b49ea88e5c972f64ed46dd9f78b5c970c5d44))
 * **[#997](https://github.com/fonttools/fontspector/issues/997)**
    - Only validate name ID 2 for RBIZ ([`ba8ff42`](https://github.com/fonttools/fontspector/commit/ba8ff42d7ebb12aed7ec90a608f7cc61da073394))
 * **Uncategorized**
    - Release fontspector-profile-microsoft v0.1.0, fontspector-profile-monotype v1.1.0 ([`69e30ff`](https://github.com/fonttools/fontspector/commit/69e30ff7dab640c64c3708df6550dd4460d948a5))
    - Merge branch 'main' into python-optional-feature ([`9bfbc25`](https://github.com/fonttools/fontspector/commit/9bfbc25c6c78417ce261fa5b4850c8eaba3ad37c))
    - Merge pull request #81 from fonttools/microsoft ([`7b7f817`](https://github.com/fonttools/fontspector/commit/7b7f817461c31ca0e4c44a4bbac5cee631db1e6c))
    - Finish the MS profile ([`37bbd7d`](https://github.com/fonttools/fontspector/commit/37bbd7dc56b13a12cf258639d131dec210ee9fe4))
    - Improve plugin build, fix some types, new check ([`4b73f4c`](https://github.com/fonttools/fontspector/commit/4b73f4c19cb17d99191e53285764a9e15d3fc10a))
    - Most of the microsoft profile ([`1b9ed7a`](https://github.com/fonttools/fontspector/commit/1b9ed7a67237009fc4bce30048cdfdeb1d7b4968))
</details>

