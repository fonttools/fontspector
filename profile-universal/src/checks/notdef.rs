use fontspector_checkapi::{
    pens::ContourCountPen, prelude::*, testfont, FileTypeConvert, DEFAULT_LOCATION,
};

use skrifa::raw::TableProvider;
use write_fonts::types::GlyphId;

#[check(
    id = "notdef",
    rationale = "
        Specific requirements to the .notdef glyph:

        - Is glyph 0 named .notdef?
        - Does .notdef have a non-zero advance width?
        - Does glyph 0 (.notdef) have a non-empty outline?
    ",
    proposal = "https://github.com/Monotype/fontspector/issues/12",
    title = ".notdef sanity check"
)]
fn notdef(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];

    let glyphid = GlyphId::new(0);
    let glyphname = f.glyph_name_for_id_synthesise(glyphid);

    // Is glyph 0 named .notdef?
    if glyphname != ".notdef" {
        problems.push(Status::fail(
            "notdef-glyph-id-0",
            &format!("Glyph ID 0 is expected to be named .notdef, but is named {glyphname}"),
        ));
    } else {
        // Does .notdef have a non-zero advance width?
        let upem = f.font().head()?.units_per_em() as f32;
        let hmtx = f.font().hmtx()?;
        let advance_width = hmtx.advance(glyphid).map(|x| x as f32).unwrap_or(upem);
        if advance_width == 0.0 {
            problems.push(Status::fail(
                "notdef-advance-width-0",
                ".notdef is expected to have a width.",
            ));
        }

        // Does glyph 0 (.notdef) have a non-empty outline?
        let mut pen = ContourCountPen::new();
        f.draw_glyph(glyphid, &mut pen, DEFAULT_LOCATION)?;
        let contour_count = pen.contour_count();

        if contour_count == 0 {
            problems.push(Status::fail(
                "notdef-contours-0",
                ".notdef is expected to have outlines.",
            ));
        } else if contour_count == 2 {
            // .notdef has 2 contours. It looks like it is a TOFU.
            // passing information about the 2-contour TOFU
            problems.push(Status::pass());
        } else if contour_count == 3 {
            problems.push(Status::info(
                "notdef-contours-3",
                ".notdef has 3 contours. It might be a black box with a white question mark.",
            ));
        } else if contour_count == 4 {
            problems.push(Status::info(
                "notdef-contours-4",
                ".notdef has 4 contours. It might be a TOFU with a question mark.",
            ));
        } else if contour_count == 5 {
            problems.push(Status::info(
                "notdef-contours-5",
                ".notdef has 5 contours. It looks like it is a TOFU with a cross.",
            ));
        } else if contour_count >= 6 {
            // For example GlyphsApp adds a .notdef with 10 contours by default.
            // We don't want any fancy outlines.
            // We want a nice tofu or question mark in a box.
            problems.push(Status::warn(
                "notdef-too-many-contours",
                &format!(
                    ".notdef is expected to have a few contours, only, but this has {}.",
                    contour_count
                ),
            ));
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::notdef;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        FileTypeConvert, StatusCode,
    };
    use skrifa::raw::TableProvider;
    use write_fonts::{from_obj::ToOwnedTable, tables::hmtx::Hmtx, types::GlyphId};

    #[test]
    fn test_notdef_pass() {
        let testable = test_able("montserrat/Montserrat-Black.ttf");
        let results = run_check(notdef, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_notdef_fail_notdef_contours_too_many() {
        let testable =
            test_able("bad_character_set/control_chars/FontbakeryTesterCCGood-Regular.ttf");
        let results = run_check(notdef, testable);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("notdef-too-many-contours".to_string()),
        );
    }

    #[test]
    fn test_notdef_fail_notdef_advance_width() {
        let mut testable = test_able("montserrat/Montserrat-Black.ttf");
        let f = fontspector_checkapi::prelude::TTF
            .from_testable(&testable)
            .unwrap();

        let mut hmtx: Hmtx = f.font().hmtx().unwrap().to_owned_table();
        let glyphid = GlyphId::new(0);
        if let Some(metric) = hmtx.h_metrics.get_mut(glyphid.to_u32() as usize) {
            metric.advance = 0;
        }
        testable.set(f.rebuild_with_new_table(&hmtx).unwrap());

        let results = run_check(notdef, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("notdef-advance-width-0".to_string()),
        );
    }
}
