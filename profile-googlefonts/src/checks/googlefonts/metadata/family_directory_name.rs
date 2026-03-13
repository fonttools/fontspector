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

#[cfg(test)]
mod tests {
    use super::family_directory_name;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able, test_file},
        StatusCode, Testable,
    };

    #[test]
    fn test_check_metadata_family_directory_name() {
        let good = test_able("rosarivo/METADATA.pb");
        assert_pass(&run_check(family_directory_name, good));

        let original = test_file("rosarivo/METADATA.pb");
        let bytes = std::fs::read(&original)
            .unwrap_or_else(|e| panic!("Failed to read test fixture {:?}: {e}", original));

        let temp_root = std::env::temp_dir().join("fontspector-mdpb-dirname-test");
        let _ = std::fs::remove_dir_all(&temp_root);
        std::fs::create_dir_all(&temp_root)
            .unwrap_or_else(|e| panic!("Failed to create temp directory {:?}: {e}", temp_root));
        let fake_mdpb_path = temp_root.join("METADATA.pb");
        std::fs::write(&fake_mdpb_path, bytes).unwrap_or_else(|e| {
            panic!(
                "Failed to write temporary METADATA.pb {:?}: {e}",
                fake_mdpb_path
            )
        });

        let bad = Testable::new(fake_mdpb_path.clone()).unwrap_or_else(|e| {
            panic!(
                "Failed to load temporary METADATA.pb as Testable {:?}: {e}",
                fake_mdpb_path
            )
        });
        assert_results_contain(
            &run_check(family_directory_name, bad),
            StatusCode::Fail,
            Some("bad-directory-name".to_string()),
        );

        let _ = std::fs::remove_dir_all(&temp_root);
    }
}
