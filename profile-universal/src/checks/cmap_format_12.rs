use std::collections::HashSet;

use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;
use skrifa::raw::{tables::cmap::CmapSubtable, TableProvider};

#[check(
    id = "cmap/format_12",
    rationale = "
        If a format 12 cmap table is used to address codepoints beyond the BMP,
        it should actually contain such codepoints. Additionally, it should also
        contain all characters mapped in the format 4 subtable.
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/3681",
    title = "Check that format 12 cmap subtables are correctly constituted."
)]
fn cmap_format_12(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let cmap = f.font().cmap()?;
    let format_4 = cmap
        .encoding_records()
        .iter()
        .flat_map(|x| x.subtable(cmap.offset_data()))
        .find(|x| x.format() == 4);
    let format_4_codepoints = if let Some(CmapSubtable::Format4(format_4)) = format_4 {
        format_4
            .iter()
            .map(|(cp, _glyph)| cp)
            .collect::<HashSet<_>>()
    } else {
        HashSet::new()
    };
    let mut skipped = true;
    let mut problems = vec![];
    for subtable in cmap
        .encoding_records()
        .iter()
        .flat_map(|x| x.subtable(cmap.offset_data()))
    {
        if let CmapSubtable::Format12(subtable) = subtable {
            skipped = false;
            if !subtable.iter().map(|(cp, _glyph)| cp).any(|cp| cp > 0x0FFF) {
                let message = "A format 12 subtable did not contain any codepoints beyond the Basic Multilingual Plane (BMP)";
                let mut status = Status::fail("pointless-format-12", message);
                status.add_metadata(Metadata::TableProblem {
                    table_tag: "cmap".to_string(),
                    field_name: Some("format12".to_string()),
                    actual: None,
                    expected: Some(json!({ "contains_bmp_plus": true })),
                    message: message.to_string(),
                });
                problems.push(status);
            }
            let cmap12_codepoints: HashSet<_> = subtable.iter().map(|(cp, _glyph)| cp).collect();
            let unmapped = format_4_codepoints
                .difference(&cmap12_codepoints)
                .collect::<Vec<_>>();
            if !unmapped.is_empty() {
                let unmapped_list: Vec<String> =
                    unmapped.iter().map(|cp| format!("U+{:04X}", cp)).collect();
                let message = format!(
                    "The format 12 subtable did not contain all codepoints from the format 4 subtable:\n\n{}",
                    bullet_list(context, &unmapped_list)
                );
                let mut status = Status::warn("missing-format-4", &message);
                status.add_metadata(Metadata::TableProblem {
                    table_tag: "cmap".to_string(),
                    field_name: Some("format12Coverage".to_string()),
                    actual: Some(json!(unmapped_list.clone())),
                    expected: Some(json!("All format 4 codepoints")),
                    message,
                });
                problems.push(status);
            }
        }
    }
    if skipped {
        Ok(Status::just_one_skip(
            "no-format-12",
            "No format 12 subtable was found",
        ))
    } else {
        Ok(Status::just_one_pass())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        StatusCode,
    };

    use skrifa::GlyphId;
    use write_fonts::{from_obj::ToOwnedTable, tables::cmap::Cmap};

    #[test]
    fn test_cmap_format_12_skip() {
        let testable = test_able("montserrat/Montserrat-Regular.ttf");
        let results = run_check(cmap_format_12, testable);
        assert_results_contain(&results, StatusCode::Skip, Some("no-format-12".to_string()));
    }

    #[test]
    fn test_cmap_format_12_pass() {
        fn non_bmp_cmap_mappings() -> Vec<(char, GlyphId)> {
            // contains four sequential map groups
            vec![
                // first group
                ('\u{1f12f}', GlyphId::new(481)),
                ('\u{1f130}', GlyphId::new(482)),
                // char 0x1f131 skipped, starts second group
                ('\u{1f132}', GlyphId::new(483)),
                ('\u{1f133}', GlyphId::new(484)),
                // gid 485 skipped, starts third group
                ('\u{1f134}', GlyphId::new(486)),
                // char 0x1f135 skipped, starts fourth group. identical duplicate bindings are fine
                ('\u{1f136}', GlyphId::new(488)),
                ('\u{1f136}', GlyphId::new(488)),
            ]
        }

        let mut testable = test_able("montserrat/Montserrat-Regular.ttf");
        let f = fontspector_checkapi::prelude::TTF
            .from_testable(&testable)
            .unwrap();

        let mappings = non_bmp_cmap_mappings();
        let cmap = Cmap::from_mappings(mappings).unwrap();

        testable.set(f.rebuild_with_new_table(&cmap).unwrap());

        let results = run_check(cmap_format_12, testable);
        assert_pass(&results);
    }

    // TODO: add tests to trigger warn and fail
    // #[test]
    // fn test_cmap_format_12_warn() {
    //     let mut testable = test_able("montserrat/Montserrat-Regular.ttf");
    //     let f = fontspector_checkapi::prelude::TTF
    //         .from_testable(&testable)
    //         .unwrap();
    //     let cmap: Cmap = f.font().cmap().unwrap().to_owned_table();
    //     // TODO: modify the cmap table to trigger a missing format 4 warning
    //     testable.set(f.rebuild_with_new_table(&cmap).unwrap());
    //     let results = run_check(cmap_format_12, testable);
    //     assert_results_contain(
    //         &results,
    //         StatusCode::Warn,
    //         Some("missing-format-4".to_string()),
    //     );
    // }
}
