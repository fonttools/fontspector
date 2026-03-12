use fontations::{
    skrifa::{raw::TableProvider, GlyphId, MetadataProvider},
    write::from_obj::ToOwnedTable,
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata, TestFont};
use serde_json::json;

const AVG_CHAR_WEIGHTS: [(char, u32); 27] = [
    ('a', 64),
    ('b', 14),
    ('c', 27),
    ('d', 35),
    ('e', 100),
    ('f', 20),
    ('g', 14),
    ('h', 42),
    ('i', 63),
    ('j', 3),
    ('k', 6),
    ('l', 35),
    ('m', 20),
    ('n', 56),
    ('o', 56),
    ('p', 17),
    ('q', 4),
    ('r', 49),
    ('s', 56),
    ('t', 71),
    ('u', 31),
    ('v', 10),
    ('w', 18),
    ('x', 3),
    ('y', 18),
    ('z', 2),
    (' ', 166),
];

#[check(
    id = "opentype/xavgcharwidth",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Checking OS/2 fsSelection value.",
    hotfix = fix_xavgcharwidth,
    rationale = "
        The OS/2.xAvgCharWidth field is used to calculate the width of a string of
        characters. It is the average width of all non-zero width glyphs in the font.

        This check ensures that the value is correct. A failure here may indicate
        a bug in the font compiler, rather than something that the designer can
        do anything about.
    "
)]
fn xavgcharwidth(f: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    let (rule, expected) = compute_expected_xavgcharwidth(&font)?;
    let actual = font.font().os2()?.x_avg_char_width();
    let difference = (expected as i16).abs_diff(actual);
    let mut problems = vec![];
    match difference {
        0 => {
            // Pass
        }
        1..=10 => {
            let msg = format!("OS/2 xAvgCharWidth is {actual} but it should be {expected} which corresponds to {rule}. These are similar values, which may be a symptom of the slightly different calculation of the xAvgCharWidth value in font editors. There's further discussion on this at https://github.com/fonttools/fontbakery/issues/1622");
            let mut status = Status::info("xAvgCharWidth-close", &msg);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "OS/2".to_string(),
                field_name: Some("xAvgCharWidth".to_string()),
                actual: Some(json!(actual)),
                expected: Some(json!(expected)),
                message: msg,
            });
            problems.push(status);
        }
        _ => {
            let msg = format!("OS/2 xAvgCharWidth is {actual} but it should be {expected} which corresponds to {rule}. This may indicate a problem with the font editor or the font compiler.");
            let mut status = Status::warn("xAvgCharWidth-wrong", &msg);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "OS/2".to_string(),
                field_name: Some("xAvgCharWidth".to_string()),
                actual: Some(json!(actual)),
                expected: Some(json!(expected)),
                message: msg,
            });
            problems.push(status);
        }
    }
    return_result(problems)
}

fn compute_expected_xavgcharwidth(
    font: &TestFont,
) -> Result<(&'static str, u32), FontspectorError> {
    let os2 = font.font().os2()?;
    let hmtx = font.font().hmtx()?;
    let charmap = font.font().charmap();
    if os2.version() >= 3 {
        let advances = hmtx
            .h_metrics()
            .iter()
            .map(|metric| metric.advance.get() as u32)
            .filter(|&w| w > 0)
            .collect::<Vec<_>>();
        if advances.is_empty() {
            return Err(FontspectorError::General(
                "No non-zero width glyphs in font for average character width calculation"
                    .to_string(),
            ));
        }
        Ok((
            "the average of the widths of all glyphs in the font",
            advances.iter().sum::<u32>() / advances.len() as u32,
        ))
    } else {
        let ids: Vec<Option<GlyphId>> = AVG_CHAR_WEIGHTS
            .iter()
            .map(|(c, _)| charmap.map(*c))
            .collect();
        if ids.iter().any(|id| id.is_none()) {
            return Err(FontspectorError::General(
                "Missing glyph in font for average character width calculation".to_string(),
            ));
        }
        #[allow(clippy::unwrap_used)] // We know all the characters are in the font
        let advances = ids
            .iter()
            .zip(AVG_CHAR_WEIGHTS.iter())
            .map(|(id, (_, w))| hmtx.advance(id.unwrap()).unwrap_or(0) as u32 * w)
            .collect::<Vec<_>>();
        Ok((
            "the weighted average of the widths of the latin lowercase glyphs in the font",
            advances.iter().sum::<u32>() / 1000u32,
        ))
    }
}

fn fix_xavgcharwidth(t: &mut Testable, _replies: Option<MoreInfoReplies>) -> Result<FixResult, FontspectorError> {
    let f = testfont!(t);
    let (_, expected) = compute_expected_xavgcharwidth(&f)?;
    let mut os2: fontations::write::tables::os2::Os2 = f.font().os2()?.to_owned_table();
    os2.x_avg_char_width = expected as i16;
    t.set(f.rebuild_with_new_table(&os2)?);
    Ok(FixResult::Fixed)
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use fontations::{skrifa::raw::TableProvider, write::from_obj::ToOwnedTable};
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        StatusCode,
    };

    #[test]
    fn test_xavgcharwidth_pass() {
        let testable = test_able("nunito/Nunito-Regular.ttf");
        let result = run_check(xavgcharwidth, testable);
        assert_pass(&result);
    }

    #[test]
    fn test_xavgcharwidth_close() {
        let mut testable = test_able("nunito/Nunito-Regular.ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut os2: fontations::write::tables::os2::Os2 = f.font().os2().unwrap().to_owned_table();
        os2.x_avg_char_width = 556;
        testable.set(f.rebuild_with_new_table(&os2).unwrap());
        let result = run_check(xavgcharwidth, testable);
        assert_results_contain(
            &result,
            StatusCode::Info,
            Some("xAvgCharWidth-close".to_string()),
        );
    }

    #[test]
    fn test_xavgcharwidth_wrong() {
        let mut testable = test_able("nunito/Nunito-Regular.ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut os2: fontations::write::tables::os2::Os2 = f.font().os2().unwrap().to_owned_table();
        os2.x_avg_char_width = 500;
        testable.set(f.rebuild_with_new_table(&os2).unwrap());
        let result = run_check(xavgcharwidth, testable);
        assert_results_contain(
            &result,
            StatusCode::Warn,
            Some("xAvgCharWidth-wrong".to_string()),
        );
    }
}
