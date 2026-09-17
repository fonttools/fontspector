use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "monotype/vertical_metrics_sane",
    rationale = "
    This check ensures that the vertical metrics in the font are sane and follow expected conventions.
    Specifically, it checks that the OS/2 and hhea table values are within reasonable ranges:

    Is OS/2.sTypoAscender > 0?
    Is OS/2.sTypoDescender < 0?
    Is OS/2.sTypoLineGap ≥ 0?
    Is OS/2.usWinAscent > 0?
    Is OS/2.usWinDescent > 0?
    Is hhea.ascent > 0?
    Is hhea.descent < 0?
    Is hhea.lineGap ≥ 0?
    
    ",
    proposal = "https://github.com/Monotype/fontspector/issues/3",
    title = "This check only determines the sanity of the values e.g. if the usWinDescent is a positive value, and hhea.ascent is negative."
)]
fn vertical_metrics_sane(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    let metrics = f.vertical_metrics()?;

    // Is OS/2.sTypoAscender > 0?
    if metrics.os2_typo_ascender <= 0 {
        problems.push(Status::fail(
            "bad-os2-typo-ascender",
            &format!(
                "OS/2.sTypoAscender is {}; it should be strictly positive",
                metrics.os2_typo_ascender
            ),
        ));
    }

    // Is OS/2.sTypoDescender < 0?
    if metrics.os2_typo_descender >= 0 {
        problems.push(Status::fail(
            "bad-os2-typo-descender",
            &format!(
                "OS/2.sTypoDescender is {}; it should be strictly negative",
                metrics.os2_typo_descender
            ),
        ));
    }

    // Is OS/2.sTypoLineGap ≥ 0?
    if metrics.os2_typo_linegap < 0 {
        problems.push(Status::fail(
            "bad-os2-typo-linegap",
            &format!(
                "OS/2.sTypoLineGap is {}; it should be non-negative",
                metrics.os2_typo_linegap
            ),
        ));
    }

    // Is OS/2.usWinAscent > 0?
    if metrics.os2_win_ascent <= 0 {
        problems.push(Status::fail(
            "bad-os2-win-ascent",
            &format!(
                "OS/2.usWinAscent is {}; it should be strictly positive",
                metrics.os2_win_ascent
            ),
        ));
    }

    // Is OS/2.usWinDescent > 0?
    if metrics.os2_win_descent <= 0 {
        problems.push(Status::fail(
            "bad-os2-win-descent",
            &format!(
                "OS/2.usWinDescent is {}; it should be strictly positive",
                metrics.os2_win_descent
            ),
        ));
    }

    // Is hhea.ascent > 0?
    if metrics.hhea_ascent <= 0 {
        problems.push(Status::fail(
            "bad-hhea-ascent",
            &format!(
                "hhea.ascent is {}; it should be strictly positive",
                metrics.hhea_ascent
            ),
        ));
    }

    // Is hhea.descent < 0?
    if metrics.hhea_descent >= 0 {
        problems.push(Status::fail(
            "bad-hhea-descent",
            &format!(
                "hhea.descent is {}; it should be strictly negative",
                metrics.hhea_descent
            ),
        ));
    }

    // Is hhea.lineGap ≥ 0?
    if metrics.hhea_linegap < 0 {
        problems.push(Status::fail(
            "bad-hhea-linegap",
            &format!(
                "hhea.lineGap is {}; it should be non-negative",
                metrics.hhea_linegap
            ),
        ));
    }

    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::vertical_metrics_sane;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        StatusCode, Testable,
    };
    use write_fonts::{
        tables::{head::Head, hhea::Hhea, maxp::Maxp, os2::Os2},
        FontBuilder,
    };

    #[test]
    fn test_vertical_metrics_sane_pass() {
        let testable = test_able("montserrat/Montserrat-Black.ttf");
        let results = run_check(vertical_metrics_sane, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_vertical_metrics_sane_fail() {
        let new_hhea: Hhea = Hhea {
            ascender: 1000.into(),
            descender: (-200).into(),
            line_gap: 0.into(),
            ..Default::default()
        };

        let new_os2: Os2 = Os2 {
            us_win_ascent: 1000,
            us_win_descent: 200,
            s_typo_ascender: 1000,
            s_typo_descender: -200,
            s_typo_line_gap: 0,
            ..Default::default()
        };
        let vertical_metrics_tests = [
            (
                new_hhea.clone(),
                {
                    let mut os2 = new_os2.clone();
                    os2.us_win_descent = 0;
                    os2
                },
                Some("bad-os2-win-descent".to_string()),
            ),
            (
                new_hhea.clone(),
                {
                    let mut os2 = new_os2.clone();
                    os2.us_win_ascent = 0;
                    os2
                },
                Some("bad-os2-win-ascent".to_string()),
            ),
            (
                new_hhea.clone(),
                {
                    let mut os2 = new_os2.clone();
                    os2.s_typo_ascender = 0;
                    os2
                },
                Some("bad-os2-typo-ascender".to_string()),
            ),
            (
                new_hhea.clone(),
                {
                    let mut os2 = new_os2.clone();
                    os2.s_typo_descender = 0;
                    os2
                },
                Some("bad-os2-typo-descender".to_string()),
            ),
            (
                new_hhea.clone(),
                {
                    let mut os2 = new_os2.clone();
                    os2.s_typo_line_gap = -200;
                    os2
                },
                Some("bad-os2-typo-linegap".to_string()),
            ),
            (
                {
                    let mut hhea = new_hhea.clone();
                    hhea.line_gap = (-200).into();
                    hhea
                },
                new_os2.clone(),
                Some("bad-hhea-linegap".to_string()),
            ),
            (
                {
                    let mut hhea = new_hhea.clone();
                    hhea.ascender = 0.into();
                    hhea
                },
                new_os2.clone(),
                Some("bad-hhea-ascent".to_string()),
            ),
            (
                {
                    let mut hhea = new_hhea.clone();
                    hhea.descender = 0.into();
                    hhea
                },
                new_os2.clone(),
                Some("bad-hhea-descent".to_string()),
            ),
        ];
        for (hhea, os2, expected_code) in vertical_metrics_tests {
            let mut font_builder = FontBuilder::new();
            let maxp = Maxp::default();
            font_builder.add_table(&maxp).unwrap();

            let head: Head = Head {
                ..Default::default()
            };
            font_builder.add_table(&head).unwrap();
            font_builder.add_table(&hhea).unwrap();
            font_builder.add_table(&os2).unwrap();

            let font = font_builder.build();

            let testable = Testable::new_with_contents("demo.otf", font);
            let results = run_check(vertical_metrics_sane, testable);
            assert_results_contain(&results, StatusCode::Fail, expected_code);
        }
    }
}
