use fontspector_checkapi::{prelude::*, skip, FileTypeConvert, StatusCode};

#[check(
    id = "opentype/varfont/family_axis_ranges",
    title = "Check that family axis ranges are identical",
    rationale = "Between members of a family (such as Roman & Italic), the ranges of variable axes must be identical.",
    proposal = "https://github.com/fonttools/fontbakery/issues/4445",
    implementation = "all"
)]
fn family_axis_ranges(c: &TestableCollection, context: &Context) -> CheckFnResult {
    let mut fonts = TTF.from_collection(c);
    fonts.retain(|f| f.is_variable_font());
    skip!(
        fonts.len() < 2,
        "not-enough-fonts",
        "Not enough variable fonts to compare"
    );
    let values: Vec<_> = fonts
        .iter()
        .map(|f| {
            let label = f
                .filename
                .file_name()
                .map(|x| x.to_string_lossy())
                .map(|x| x.to_string())
                .unwrap_or("Unknown file".to_string());
            let comparable = f
                .axis_ranges()
                .map(|(ax, min, def, max)| format!("{ax}={min:.2}:{def:.2}:{max:.2}"))
                .collect::<Vec<String>>()
                .join(", ");
            (comparable.clone(), comparable, label)
        })
        .collect();
    assert_all_the_same(
        context,
        &values,
        "axis-range-mismatch",
        "Variable axis ranges not matching between font files",
        StatusCode::Fail,
    )
}
