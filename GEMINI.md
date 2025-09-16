## Data structures and core concepts

Fontspector is a font testing framework written in Rust. It has a number of core concepts:

- A _check_ is a single test that can be run on a font. It is implemented as a `Check` structure,
  which brings together the check implementation (a function) and metadata about the check.
  The `#[check]` attribute macro is used to define checks. We use the term "check" rather than
  "test" to avoid confusion with the unit tests; we check a font, and we test the checks. The `Check` structure is defined in `fontspector-checkapi/src/check.rs`.
- Each check has a check ID. The mapping between check IDs and their implementation files can be found in `checks.txt`.
- A _profile_ is a collection of checks that can be run together. Individual font manufacturers
  define which checks they would like to run on their fonts by creating a profile.
  Profiles are implemented as Rust crates that depend on the `fontspector-checkapi` crate. The
  check implementations have a "home" within a specific profile, but can be reused in other
  profiles. The `Profile` structure is defined in `fontspector-checkapi/src/profile.rs`.
- A check returns a `CheckResult`. Fontspector will run multiple checks on multiple fonts, and  
  each check may discover one or more problems with the font. `CheckResult` is a structure which wraps up everything that you need to report the result of a check: which file it was run on,
  which check was run, metadata to be displayed about the check (such as the reasoning behind the check and a user-friendly title), together with all of the results of the check (which may be zero or more problems found). The `CheckResult` structure is defined in `fontspector-checkapi/src/checkresult.rs`.
- A `Status` is a single reported problem found by a check. A check may return zero or more
  `Status` items, each of which has a _severity_ (error, warning, info, or pass), optionally a _message_ (to be reported to the user) and a _code_ (a short string that identifies the specific problem found, for example for unit tests or for advanced users who know what to expect).
- The severity mentioned above is represented by the `StatusCode` enum. We should probably have
  called it `Severity`, and maybe one day we will. `Status` and `StatusCode` are defined in `fontspector-checkapi/src/status.rs`.
- We don't simply check fonts. Fontspector can check HTML files, metadata files, and so on. But obviously only some checks apply to certain file types. Checks declare which `FileType` they apply to.
- Files are wrapped up in `Testable` structs, which include their contents and file name. To determine if the file can be converted into a `FileType` to be run by a particular check, we call `.from_testable` on the `FileType` enum. If it returns `Some`, we can run the check; if it returns `None`, we skip the check for that file. The `FileTypeConvert` trait also tells us what kind of representation the file can be converted into. For example, `Testable`s which are TTF files can be converted into `TestFont`s.
- `TestFont` in `fontspector-checkapi/src/font.rs` contains a number of helper methods which tests can use to manipulate the font and extract data from it.
- Each check runs in a `Context`. (`fontspector-checkapi/src/context.rs`) The context contains a general-purpose cache that checks can use to avoid recomputing things, user-defined per-check configuration, some free-form metadata, and `Override`s which change the return values of a check.

## General practices

- When we implement a new check file, there's a bit of grunt work that
  needs doing: the Rust module needs to be added to the `mod.rs` in
  whichever directory it's added in, then that needs to be imported by the
  profile file, and the check added to the end of the list of checks in
  the profile (which is usually in lib.rs under the profile's crate). It's
  mostly boilerplate which can be infered from similar checks. Basically:
  make sure it's included in the Rust module tree and reachable from the
  `register` function, then add to the list of checks.
- We use `cargo fmt` and `cargo clippy` to ensure code quality. Please run
  these before committing changes, and clean up any warnings you introduce.
