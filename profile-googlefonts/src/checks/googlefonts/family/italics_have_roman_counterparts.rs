use fontspector_checkapi::prelude::*;

#[check(
    id = "googlefonts/family/italics_have_roman_counterparts",
    rationale = "
        
        For each font family on Google Fonts, every Italic style must have
        a Roman sibling.

        This kind of problem was first observed at [1] where the Bold style was
        missing but BoldItalic was included.

        [1] https://github.com/google/fonts/pull/1482
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/1733",
    title = "Ensure Italic styles have Roman counterparts.",
    implementation = "all"
)]
fn italics_have_roman_counterparts(c: &TestableCollection, context: &Context) -> CheckFnResult {
    let filenames: Vec<String> = c
        .testables
        .iter()
        .map(|t| t.filename.to_string_lossy().to_string())
        .filter(|x| x.ends_with(".ttf"))
        .collect();
    let italics: Vec<&String> = filenames
        .iter()
        .filter(|x| x.contains("Italic") && x.find("-") < x.find("Italic"))
        .collect();
    let mut missing_roman = vec![];
    let mut problems = vec![];
    for italic in italics {
        if !italic.contains("-") {
            problems.push(Status::warn(
                "bad-filename",
                &format!("Filename seems to be incorrect: '{italic}'"),
            ));
            continue;
        }
        #[allow(clippy::unwrap_used)] // We just tested for one
        let after_hyphen = italic.split("-").last().unwrap();
        let Some(style_from_filename) = after_hyphen.split(".").next() else {
            problems.push(Status::warn(
                "bad-filename",
                &format!("Filename seems to be incorrect: '{italic}'"),
            ));
            continue;
        };
        let is_varfont = style_from_filename.contains("[");
        let roman_counterpart = if style_from_filename == "Italic" {
            if is_varfont {
                italic.replace("-Italic", "")
            } else {
                italic.replace("Italic", "Regular")
            }
        } else {
            italic.replace("-Italic", "")
        };
        if !filenames.contains(&roman_counterpart) {
            missing_roman.push(italic);
        }
    }
    if !missing_roman.is_empty() {
        problems.push(Status::fail(
            "missing-roman",
            &format!(
                "Italics missing a Roman counterpart:\n{}",
                bullet_list(context, missing_roman)
            ),
        ));
    }
    return_result(problems)
}
