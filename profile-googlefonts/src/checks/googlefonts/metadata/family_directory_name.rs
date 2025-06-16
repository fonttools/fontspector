use fontspector_checkapi::{prelude::*, skip};

use crate::checks::googlefonts::metadata::family_proto;
#[check(
    id = "googlefonts/metadata/family_directory_name",
    rationale = "
        We want the directory name of a font family to be predictable and directly
        derived from the family name, all lowercased and removing spaces.
    ",
    applies_to = "MDPB",
    proposal = "https://github.com/fonttools/fontbakery/issues/3421",
    title = "Check font family directory name."
)]
fn family_directory_name(c: &Testable, _context: &Context) -> CheckFnResult {
    // Assume we actually have directories, we might not in a WASM context
    let Ok(fullpath) = std::fs::canonicalize(&c.filename) else {
        skip!("no-directory", "No directory information")
    };
    let msg = family_proto(c)?;
    let last_component = fullpath
        .parent()
        .and_then(|p| p.file_name())
        .ok_or(FontspectorError::General("No directory name".to_string()))?
        .to_string_lossy();
    let expected = msg.name().replace(" ", "").to_lowercase();
    if expected != last_component {
        Ok(Status::just_one_fail(
            "bad-directory-name",
            &format!(
                "Family name on METADATA.pb is \"{}\"\n\
            Directory name is \"{}\"\n\
            Expected \"{}\"",
                msg.name(),
                last_component,
                expected
            ),
        ))
    } else {
        Ok(Status::just_one_pass())
    }
}
