use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

#[check(
    id = "googlefonts/repo/dirname_matches_nameid_1",
    rationale = "
        
        For static fonts, we expect to name the directory in google/fonts
        according to the NameID 1 of the regular font, all lower case with
        no hyphens or spaces. This check verifies that the directory
        name matches our expectations.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2302",
    title = "Directory name in GFonts repo structure must
    match NameID 1 of the regular."
)]
fn dirname_matches_nameid_1(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    skip!(
        f.style() != Some("Regular"),
        "not-regular",
        "Skipping non-Regular style"
    );
    skip!(
        f.is_variable_font(),
        "variable-exempt",
        "Variable fonts are exempt from this check."
    );
    let family_name = f
        .best_familyname()
        .ok_or(FontspectorError::General(format!(
            "Could not determine a family name for {}",
            f.filename.to_string_lossy()
        )))?;
    let expected = family_name.to_lowercase().replace(" ", "").replace("-", "");
    let Some(parent) = f.filename.parent().and_then(|x| x.file_name()) else {
        skip!(
            "no-parent",
            "Could not determine the parent directory of the font file."
        );
    };
    if parent.to_string_lossy() != expected {
        Ok(Status::just_one_fail("mismatch", &format!(
            "Family name on the name table ('{}') does not match directory name in the repo structure ('{}'). Expected '{}'.",
            family_name,
            parent.to_string_lossy(),
            expected
        )))
    } else {
        Ok(Status::just_one_pass())
    }
}

#[cfg(test)]
mod tests {
    use super::dirname_matches_nameid_1;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_file},
        StatusCode, Testable,
    };

    #[test]
    fn test_check_repo_dirname_matches_nameid_1() {
        let src_family = test_file("rosarivo");
        let tmp_root = std::env::temp_dir().join("fontspector-dirname-nameid1");
        let _ = std::fs::remove_dir_all(&tmp_root);
        let tmp_gf_dir = tmp_root.join("ofl/rosarivo");
        let parent = tmp_gf_dir.parent().unwrap_or_else(|| {
            panic!(
                "Failed to determine temp parent directory for {:?}",
                tmp_gf_dir
            )
        });
        std::fs::create_dir_all(parent)
            .unwrap_or_else(|e| panic!("Failed creating temp dirs: {e}"));
        std::fs::create_dir_all(&tmp_gf_dir)
            .unwrap_or_else(|e| panic!("Failed creating family temp dir {:?}: {e}", tmp_gf_dir));
        let entries = std::fs::read_dir(&src_family)
            .unwrap_or_else(|e| panic!("Failed reading fixture dir {:?}: {e}", src_family));
        for entry in entries {
            let entry = entry.unwrap_or_else(|e| panic!("Failed reading fixture entry: {e}"));
            let from = entry.path();
            let to = tmp_gf_dir.join(entry.file_name());
            std::fs::copy(&from, &to)
                .unwrap_or_else(|e| panic!("Failed copying {:?} -> {:?}: {e}", from, to));
        }

        let regular = tmp_gf_dir.join("Rosarivo-Regular.ttf");
        let regular_testable = Testable::new(regular.clone())
            .unwrap_or_else(|e| panic!("Failed to load temp test font {:?}: {e}", regular));
        assert_pass(&run_check(dirname_matches_nameid_1, regular_testable));

        let renamed = tmp_root.join("ofl/not_rosarivo");
        std::fs::rename(&tmp_gf_dir, &renamed).unwrap_or_else(|e| {
            panic!(
                "Failed renaming temp dir {:?} -> {:?}: {e}",
                tmp_gf_dir, renamed
            )
        });
        let bad_regular = renamed.join("Rosarivo-Regular.ttf");
        assert_results_contain(
            &run_check(
                dirname_matches_nameid_1,
                Testable::new(bad_regular)
                    .unwrap_or_else(|e| panic!("Failed loading renamed test font: {e}")),
            ),
            StatusCode::Fail,
            Some("mismatch".to_string()),
        );

        let _ = std::fs::remove_dir_all(&tmp_root);
    }
}
