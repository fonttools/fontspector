use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use google_fonts_axisregistry::build_filename;
use serde_json::json;

#[check(
    id = "googlefonts/canonical_filename",
    rationale = "
        
        A font's filename must be composed as \"<familyname>-<stylename>.ttf\":

        - Nunito-Regular.ttf

        - Oswald-BoldItalic.ttf


        Variable fonts must list the axis tags in alphabetical order in
        square brackets and separated by commas:

        - Roboto[wdth,wght].ttf

        - Familyname-Italic[wght].ttf
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Checking file is named canonically.",
    hotfix = fix_canonical_filename,
)]
fn canonical_filename(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let current_filename = t.basename().unwrap_or_default();
    let expected_filename = build_filename(f.font(), &t.extension().unwrap_or_default());
    let mut problems = vec![];
    if current_filename != expected_filename {
        let msg = "Font filename does not match canonical format";
        let mut status = Status::fail(
            "bad-filename",
            &format!("Expected \"{expected_filename}\". Got \"{current_filename}\"."),
        );
        status.add_metadata(Metadata::FontProblem {
            message: msg.to_string(),
            context: Some(json!({
                "expected_filename": expected_filename,
                "actual_filename": current_filename
            })),
        });
        problems.push(status);
    } else {
        problems.push(Status::pass());
    }
    return_result(problems)
}

fn fix_canonical_filename(t: &mut Testable) -> Result<bool, FontspectorError> {
    let f = testfont!(t);
    let expected_filename = build_filename(f.font(), &t.extension().unwrap_or_default());
    t.set_filename(&expected_filename);
    Ok(true)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::codetesting::{assert_pass, run_check, test_able};

    use super::canonical_filename;

    #[test]
    fn test_pass_good_filenames() {
        for font_file in [
            "montserrat/Montserrat-Thin.ttf",
            "montserrat/Montserrat-Regular.ttf",
            "montserrat/Montserrat-Bold.ttf",
            "montserrat/Montserrat-Black.ttf",
            "montserrat/Montserrat-ThinItalic.ttf",
            "montserrat/Montserrat-BoldItalic.ttf",
        ] {
            let testable = test_able(font_file);
            let results = run_check(canonical_filename, testable);
            assert_pass(&results);
        }
    }

    #[test]
    fn test_pass_vf_canonical() {
        let testable = test_able("ubuntusans/UbuntuSans[wdth,wght].ttf");
        let results = run_check(canonical_filename, testable);
        assert_pass(&results);
    }
}
