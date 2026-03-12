use fontations::{
    skrifa::{raw::TableProvider, MetadataProvider},
    write::{from_obj::ToOwnedTable, tables::hmtx::Hmtx},
};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "whitespace_widths",
    rationale = "
        If the space and nbspace glyphs have different widths, then Google Workspace
        has problems with the font.

        The nbspace is used to replace the space character in multiple situations in
        documents; such as the space before punctuation in languages that do that. It
        avoids the punctuation to be separated from the last word and go to next line.

        This is automatic substitution by the text editors, not by fonts. It's also used
        by designers in text composition practice to create nicely shaped paragraphs.
        If the space and the nbspace are not the same width, it breaks the text
        composition of documents.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/3843",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Space and non-breaking space have the same width?",
    hotfix = fix_whitespace_widths,
)]
fn whitespace_widths(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    if let (Some(space), Some(nbspace)) = (
        f.font().charmap().map(0x0020u32),
        f.font().charmap().map(0x00A0u32),
    ) {
        let space_width = f.font().hmtx()?.advance(space).unwrap_or(0);
        let nbsp_width = f.font().hmtx()?.advance(nbspace).unwrap_or(0);
        if space_width != nbsp_width {
            let space_name = f.glyph_name_for_id_synthesise(space);
            let nbsp_name = f.glyph_name_for_id_synthesise(nbspace);
            let message = format!("The space glyph named {space_name} is {space_width} font units wide, non-breaking space named ({nbsp_name}) is {nbsp_width} font units wide, and both should be positive and the same. GlyphsApp has \"Sidebearing arithmetic\" (https://glyphsapp.com/tutorials/spacing) which allows you to set the non-breaking space width to always equal the space width.");
            let mut status = Status::fail("different-widths", &message);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "hmtx".to_string(),
                field_name: Some("advance width".to_string()),
                actual: Some(json!({
                    "space_width": space_width,
                    "nbsp_width": nbsp_width,
                })),
                expected: Some(json!({ "space_width": "equal to nbsp_width" })),
                message: message.clone(),
            });
            problems.push(status);
        }
        return_result(problems)
    } else {
        skip!("missing-glyphs", "Space and nbspace not found in font");
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::whitespace_widths;
    use fontations::read::TableProvider;
    use fontations::skrifa::MetadataProvider;
    use fontations::write::{from_obj::ToOwnedTable, tables::hmtx::Hmtx};
    use fontspector_checkapi::codetesting::{
        assert_pass, assert_results_contain, run_check, test_able,
    };
    use fontspector_checkapi::{FileTypeConvert, StatusCode};

    #[test]
    fn test_whitespace_widths_pass() {
        let testable = test_able("nunito/Nunito-Regular.ttf");
        let results = run_check(whitespace_widths, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_whitespace_widths_fail() {
        let mut testable = test_able("nunito/Nunito-Regular.ttf");
        let f = fontspector_checkapi::TTF.from_testable(&testable).unwrap();
        let mut hmtx: Hmtx = f.font().hmtx().unwrap().to_owned_table();
        let space_gid = f.font().charmap().map(0x0020u32).unwrap();
        if let Some(metric) = hmtx.h_metrics.get_mut(space_gid.to_u32() as usize) {
            metric.advance = 0;
        }
        testable.set(f.rebuild_with_new_table(&hmtx).unwrap());
        let results = run_check(whitespace_widths, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("different-widths".to_string()),
        );
    }
}

fn fix_whitespace_widths(t: &mut Testable, _replies: Option<MoreInfoReplies>) -> Result<FixResult, FontspectorError> {
    let f = testfont!(t);
    let mut hmtx: Hmtx = f.font().hmtx()?.to_owned_table();
    let charmap = f.font().charmap();
    if let (Some(space), Some(nbspace)) = (charmap.map(0x0020u32), charmap.map(0x00A0u32)) {
        let space_width = hmtx
            .h_metrics
            .get(space.to_u32() as usize)
            .map(|m| m.advance)
            .unwrap_or(0);
        if let Some(nbspace_metric) = hmtx.h_metrics.get_mut(nbspace.to_u32() as usize) {
            nbspace_metric.advance = space_width;
        }
        t.set(f.rebuild_with_new_table(&hmtx)?);
        return Ok(FixResult::Fixed);
    }
    Ok(FixResult::Unfixable)
}
