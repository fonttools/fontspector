use fontations::skrifa::MetadataProvider;
use fontspector_checkapi::{
    pens::HasInkPen, prelude::*, testfont, FileTypeConvert, Metadata, DEFAULT_LOCATION,
};
use serde_json::json;
use unicode_properties::{GeneralCategory, UnicodeGeneralCategory};

const EXTRA_NON_DRAWING: [u32; 6] = [0x180E, 0x200B, 0x2028, 0x2029, 0x2060, 0xFEFF];
const BUT_NOT: [u32; 2] = [0xAD, 0x1680];

#[check(
    id = "whitespace_ink",
    rationale = "
           This check ensures that certain whitespace glyphs are empty.
           Certain text layout engines will assume that these glyphs are empty,
           and will not draw them; if they were in fact not designed to be
           empty, the result will be text layout that is not as expected.
       ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    proposal = "https://github.com/fonttools/fontspector/issues/93",
    title = "Whitespace glyphs have ink?"
)]
fn whitespace_ink(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let inky = f
        .codepoints(Some(context))
        .into_iter()
        .filter(|cp| {
            (EXTRA_NON_DRAWING.contains(cp)
                || (char::from_u32(*cp)
                    .map(|c| matches!(c.general_category(), GeneralCategory::SpaceSeparator))
                    .unwrap_or(false)))
                && !BUT_NOT.contains(cp)
        })
        .map(|cp| {
            #[allow(clippy::unwrap_used)]
            let gid = f.font().charmap().map(cp).unwrap();
            (cp, gid)
        })
        .filter(|(_cp, gid)| {
            let mut has_ink_pen = HasInkPen::new();
            f.draw_glyph(*gid, &mut has_ink_pen, DEFAULT_LOCATION)
                .ok()
                .and(has_ink_pen.has_ink().then_some(()))
                .is_some()
        })
        .map(|(cp, gid)| (cp, gid, f.glyph_name_for_id_synthesise(gid)))
        .collect::<Vec<_>>();
    let mut problems = vec![];
    if !inky.is_empty() {
        let glyph_names: Vec<String> = inky.iter().map(|(_, _, name)| name.clone()).collect();
        let message = format!(
            "The following glyphs have ink; they should be replaced by an empty glyph:\n\n{}",
            bullet_list(context, glyph_names.clone())
        );
        let mut status = Status::fail("has-ink", &message);
        status.add_metadata(Metadata::FontProblem {
            message: message.clone(),
            context: Some(json!({
                "whitespace_glyphs_with_ink": inky.iter().map(|(cp, _, name)| format!("U+{:04X} ({})", cp, name)).collect::<Vec<_>>(),
                "total_inky_glyphs": inky.len(),
            })),
        });
        problems.push(status);
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::codetesting::{
        assert_pass, assert_results_contain, remap_glyph, run_check, test_able,
    };

    use fontspector_checkapi::StatusCode;

    use super::whitespace_ink;

    #[allow(clippy::expect_used)]
    #[test]
    fn test_check_whitespace_ink() {
        let testable = test_able("nunito/Nunito-Regular.ttf");
        let results = run_check(whitespace_ink, testable);
        assert_pass(&results);

        // "because Ogham space mark does have ink."
        let mut testable = test_able("nunito/Nunito-Regular.ttf");
        remap_glyph(&mut testable, 0x1680, "a").expect("remap failed");
        let results = run_check(whitespace_ink, testable);
        assert_pass(&results);

        let mut testable = test_able("nunito/Nunito-Regular.ttf");
        remap_glyph(&mut testable, 0x0020, "uni1E17").expect("remap failed");
        let results = run_check(whitespace_ink, testable);
        assert_results_contain(&results, StatusCode::Fail, Some("has-ink".to_string()));

        let mut testable = test_able("nunito/Nunito-Regular.ttf");
        remap_glyph(&mut testable, 0x0020, "scedilla").expect("remap failed");
        let results = run_check(whitespace_ink, testable);
        assert_results_contain(&results, StatusCode::Fail, Some("has-ink".to_string()));
    }
}
