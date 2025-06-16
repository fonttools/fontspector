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
