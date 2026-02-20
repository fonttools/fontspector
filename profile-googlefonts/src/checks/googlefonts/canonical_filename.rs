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
    title = "Checking file is named canonically."
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
