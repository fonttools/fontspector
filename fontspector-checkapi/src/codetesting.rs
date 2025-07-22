#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

// No bad thing if we panic in tests
use crate::{prelude::*, Check, CheckResult, Context, FileTypeConvert, StatusCode};
use fontations::skrifa::raw::{types::NameId, TableProvider};
use fontations::write::{
    tables::name::{Name, NameRecord},
    FontBuilder,
};

/// The root of the workspace, used to locate test resources
// The usual thing to use here is env!("CARGO_MANIFEST_DIR"), but that's a pain
// when we're in a workspace - if you're running `cargo test` inside a package,
// the manifest dir is the package root; if you're running `cargo test -p foo`,
// the manifest dir is the workspace root. So ask Cargo for the workspace root
// and go from there.
static WORKSPACE_ROOT: LazyLock<PathBuf> = LazyLock::new(|| {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = std::path::Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
});

/// Return a pathname for a file in the test resources directory
pub fn test_file(fname: impl AsRef<Path>) -> PathBuf {
    let mut workspace_root = WORKSPACE_ROOT.clone();

    workspace_root.push("resources/test/");
    workspace_root.join(fname)
}

/// Return a Testable from a file in the test resources directory
pub fn test_able(fname: impl AsRef<Path>) -> Testable {
    let path = test_file(fname);
    Testable::new(path).expect("Failed to load test file")
}

/// Run a check on a font and return the result
pub fn run_check(check: Check<'_>, font: Testable) -> Option<CheckResult> {
    let ctx: Context = Context {
        skip_network: false,
        network_timeout: Some(10),
        configuration: HashMap::new(),
        check_metadata: check.metadata(),
        full_lists: false,
        cache: Default::default(),
        overrides: vec![],
    };
    check.run(&TestableType::Single(&font), &ctx, None)
}

/// Assert that a check passes
///
/// Takes a `CheckResult` and asserts that the worst status is `Pass`
pub fn assert_pass(check_result: Option<CheckResult>) {
    let status = check_result.unwrap().worst_status();
    assert_eq!(status, StatusCode::Pass);
}

/// Assert that a check result contains an expected status and code
pub fn assert_results_contain(
    check_result: Option<CheckResult>,
    severity: StatusCode,
    code: Option<String>,
) {
    let subresults = check_result.unwrap().subresults;
    assert!(subresults
        .iter()
        .any(|subresult| subresult.severity == severity && subresult.code == code));
}

/// Manipulate a font by changing a name table entry (for testing purposes only)
pub fn set_name_entry(
    font: &mut Testable,
    platform: u16,
    encoding: u16,
    language: u16,
    nameid: NameId,
    new_string: String,
) {
    let f = TTF.from_testable(font).unwrap();
    let name = f.font().name().unwrap();

    let new_record = NameRecord::new(
        platform,
        encoding,
        language,
        nameid,
        new_string.to_string().into(),
    );
    let mut new_records: Vec<NameRecord> = name
        .name_record()
        .iter()
        .filter(|record| record.name_id() != nameid)
        .map(|r| {
            #[allow(clippy::unwrap_used)]
            NameRecord::new(
                r.platform_id(),
                r.encoding_id(),
                r.language_id(),
                r.name_id(),
                r.string(name.string_data())
                    .unwrap()
                    .chars()
                    .collect::<String>()
                    .to_string()
                    .into(),
            )
        })
        .collect();
    new_records.push(new_record);
    let new_nametable = Name::new(new_records);
    let new_bytes = FontBuilder::new()
        .add_table(&new_nametable)
        .unwrap()
        .copy_missing_tables(f.font())
        .build();

    font.contents = new_bytes;
}
