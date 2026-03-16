use super::family_proto;
use fontations::skrifa::string::StringId;
use fontspector_checkapi::{prelude::*, FileTypeConvert, TestFont};
use gf_metadata::FontProto;

#[check(
    id = "googlefonts/metadata/consistent_with_fonts",
    title = "Check METADATA.pb parses correctly",
    rationale = "
        The purpose of this check is to ensure that the information in the METADATA.pb file
        is consistent with the font binaries in the font family.

        This subsumes the following fontbakery checks:

        - googlefonts/metadata/filenames
        - googlefonts/metadata/canonical_style_names
        - googlefonts/metadata/valid_full_name_values
        - googlefonts/metadata/nameid/post_script_name
        ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2597 and https://github.com/fonttools/fontbakery/issues/4829",
    implementation = "all"
)]
fn consistent_with_fonts(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let mut problems = vec![];
    let mdpb = c
        .get_file("METADATA.pb")
        .ok_or_else(|| FontspectorError::skip("no-mdpb", "No METADATA.pb file found"))?;
    let msg = family_proto(mdpb)?;
    let mut declared_files = msg
        .fonts
        .iter()
        .flat_map(|f| f.filename.as_ref())
        .cloned()
        .collect::<Vec<String>>();
    let provided_files = c
        .iter()
        .flat_map(|t| t.filename.file_name())
        .map(|t| t.to_string_lossy().to_string())
        .filter(|t| t.ends_with(".otf") || t.ends_with(".ttf"))
        .collect::<Vec<String>>();
    // Match up fonts in msg with fonts in fonts - googlefonts/metadata/filenames
    {
        for declared_not_present in declared_files
            .iter()
            .filter(|f| !provided_files.contains(f))
        {
            problems.push(Status::fail(
            "file-not-found",
            &format!("Filename \"{declared_not_present}\" is listed on METADATA.pb but an actual font file with that name was not found."),
        ));
        }
        for provided_not_declared in provided_files
            .iter()
            .filter(|f| !declared_files.contains(f))
        {
            problems.push(Status::fail(
                "file-not-declared",
                &format!(
                    "Filename \"{provided_not_declared}\" is not declared on METADATA.pb as a font.filename entry."
                ),
            ));
        }
        declared_files.retain(|f| provided_files.contains(f));
    }
    let md_font_pairs: Vec<(&FontProto, TestFont)> = msg
        .fonts
        .iter()
        .filter(|f| declared_files.contains(&f.filename().to_string()))
        .flat_map(|font_proto| {
            c.get_file(font_proto.filename())
                .map(|font| (font_proto, font))
        })
        .flat_map(|(font_proto, testable)| TTF.from_testable(testable).map(|ttf| (font_proto, ttf)))
        .collect();

    for (proto, font) in md_font_pairs.iter() {
        // canonical_style_names
        if proto.style() == "italic" || proto.style() == "normal" {
            if font.is_italic()? && proto.style() != "italic" {
                problems.push(Status::fail(
                    "italic",
                    &format!(
                        "The font style for {} is \"{}\" but it should be \"italic\".",
                        proto.filename(),
                        proto.style()
                    ),
                ));
            } else if !font.is_italic()? && proto.style() != "normal" {
                problems.push(Status::fail(
                    "normal",
                    &format!(
                        "The font style for {} is \"{}\" but it should be \"normal\".",
                        proto.filename(),
                        proto.style()
                    ),
                ));
            }
        }

        // googlefonts/metadata/valid_full_name_values
        let family_name = if font.style().is_some() {
            font.best_familyname()
        } else {
            font.get_best_name(&[StringId::TYPOGRAPHIC_FAMILY_NAME, StringId::FAMILY_NAME])
        }
        .ok_or_else(|| {
            FontspectorError::General(format!("No family name found for {}", proto.name()))
        })?;
        if !proto.full_name().contains(&family_name) {
            problems.push(Status::fail(
                "mismatch",
                &format!(
                    "METADATA.pb font.full_name field \"{}\"  does not match correct font name format \"{}\".",
                    proto.full_name(),
                    family_name
                ),
            ));
        }
        // googlefonts/metadata/nameid/family_and_full_names
        for full_name in font.get_name_entry_strings(StringId::FULL_NAME) {
            if proto.full_name() != full_name {
                problems.push(Status::fail(
                    "fullname-mismatch",
                    &format!(
                    "METADATA.pb full_name field \"{}\" does not match correct full name \"{}\".",
                    proto.full_name(),
                    full_name
                ),
                ));
            }
        }
        if font.is_ribbi() && !font.is_variable_font() {
            for family_name in font.get_name_entry_strings(StringId::FAMILY_NAME) {
                if proto.name() != family_name {
                    problems.push(Status::fail(
                    "familyname-mismatch",
                    &format!(
                    "METADATA.pb family name field \"{}\" does not match correct family name \"{}\".",
                    proto.name(),
                    family_name
                ),
                ));
                }
            }
        }

        // googlefonts/metadata/nameid/post_script_name (make sure postscript name is consistent)
        let post_script_name = font
            .get_best_name(&[StringId::POSTSCRIPT_NAME])
            .ok_or_else(|| {
                FontspectorError::General(format!("No post script name found for {}", proto.name()))
            })?;
        if proto.post_script_name() != post_script_name {
            problems.push(Status::fail(
                "mismatch",
                &format!(
                    "METADATA.pb post_script_name field \"{}\" does not match correct post script name \"{}\".",
                    proto.post_script_name(),
                    post_script_name
                ),
            ));
        }
        // googlefonts/metadata/valid_post_script_name_values (make sure postscript name is correct)
        let familyname = font
            .best_familyname()
            .ok_or_else(|| {
                FontspectorError::General(format!("No family name found for {}", proto.name()))
            })?
            .replace(" ", "");
        if !proto
            .post_script_name()
            .replace("-", "")
            .contains(&familyname)
        {
            problems.push(Status::fail(
                "mismatch",
                &format!(
                    "METADATA.pb post_script_name field \"{}\" does not match correct font name \"{}\".",
                    proto.post_script_name(),
                    familyname
                ),
            ));
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf};

    use super::consistent_with_fonts;
    use fontations::skrifa::raw::types::NameId;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, set_name_entry, test_able, test_file},
        StatusCode, Testable, TestableCollection, TestableType,
    };

    fn run(files: Vec<Testable>) -> Option<fontspector_checkapi::CheckResult> {
        let collection = TestableCollection::from_testables(files, None);
        fontspector_checkapi::codetesting::run_check_with_config(
            consistent_with_fonts,
            TestableType::Collection(&collection),
            HashMap::new(),
        )
    }

    fn rosarivo_collection() -> Vec<Testable> {
        vec![
            test_able("rosarivo/Rosarivo-Regular.ttf"),
            test_able("rosarivo/Rosarivo-Italic.ttf"),
            test_able("rosarivo/METADATA.pb"),
        ]
    }

    #[test]
    fn test_check_metadata_filenames() {
        assert_pass(&run(rosarivo_collection()));

        // Missing one declared file.
        assert_results_contain(
            &run(vec![
                test_able("rosarivo/Rosarivo-Regular.ttf"),
                test_able("rosarivo/METADATA.pb"),
            ]),
            StatusCode::Fail,
            Some("file-not-found".to_string()),
        );

        // Cabin has extra TTF files not declared in METADATA.pb.
        let mut files = vec![test_able("cabin/METADATA.pb")];
        let cabin_dir = test_file("cabin");
        let entries = std::fs::read_dir(&cabin_dir)
            .unwrap_or_else(|e| panic!("Failed to read cabin test directory {:?}: {e}", cabin_dir));
        for entry in entries {
            let entry =
                entry.unwrap_or_else(|e| panic!("Failed to iterate cabin test dir entry: {e}"));
            let path: PathBuf = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("ttf") {
                files.push(
                    Testable::new(path)
                        .unwrap_or_else(|e| panic!("Failed to load cabin font for test: {e}")),
                );
            }
        }
        assert_results_contain(
            &run(files),
            StatusCode::Fail,
            Some("file-not-declared".to_string()),
        );
    }

    #[test]
    fn test_check_metadata_nameid_family_and_full_names() {
        assert_pass(&run(rosarivo_collection()));

        let mut regular = test_able("rosarivo/Rosarivo-Regular.ttf");
        set_name_entry(
            &mut regular,
            3,
            1,
            0x409,
            NameId::FULL_NAME,
            "This is utterly wrong!".to_string(),
        );
        assert_results_contain(
            &run(vec![
                regular,
                test_able("rosarivo/Rosarivo-Italic.ttf"),
                test_able("rosarivo/METADATA.pb"),
            ]),
            StatusCode::Fail,
            Some("fullname-mismatch".to_string()),
        );

        let mut regular = test_able("rosarivo/Rosarivo-Regular.ttf");
        set_name_entry(
            &mut regular,
            3,
            1,
            0x409,
            NameId::FAMILY_NAME,
            "Completely Wrong Family".to_string(),
        );
        assert_results_contain(
            &run(vec![
                regular,
                test_able("rosarivo/Rosarivo-Italic.ttf"),
                test_able("rosarivo/METADATA.pb"),
            ]),
            StatusCode::Fail,
            Some("familyname-mismatch".to_string()),
        );
    }

    #[test]
    fn test_check_metadata_valid_post_script_name_values() {
        assert_pass(&run(rosarivo_collection()));

        let mdpb = test_able("rosarivo/METADATA.pb");
        let contents = String::from_utf8(mdpb.contents.clone())
            .unwrap_or_else(|e| panic!("Invalid UTF-8 in METADATA fixture: {e}"));
        let broken = contents.replacen(
            "post_script_name: \"Rosarivo-Regular\"",
            "post_script_name: \"WrongPSName\"",
            1,
        );
        assert_results_contain(
            &run(vec![
                test_able("rosarivo/Rosarivo-Regular.ttf"),
                test_able("rosarivo/Rosarivo-Italic.ttf"),
                Testable::new_with_contents("METADATA.pb", broken.into_bytes()),
            ]),
            StatusCode::Fail,
            Some("mismatch".to_string()),
        );
    }
}
