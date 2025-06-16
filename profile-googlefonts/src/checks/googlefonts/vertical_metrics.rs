use crate::network_conditions::is_listed_on_google_fonts;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

#[check(
    id = "googlefonts/vertical_metrics",
    rationale = "
        
        This check generally enforces Google Fonts’ vertical metrics specifications.
        In particular:
        * lineGap must be 0
        * Sum of hhea ascender + abs(descender) + linegap must be
          between 120% and 200% of UPM
        * Warning if sum is over 150% of UPM

        The threshold levels 150% (WARN) and 200% (FAIL) are somewhat arbitrarily chosen
        and may hint at a glaring mistake in the metrics calculations or UPM settings.

        Our documentation includes further information:
        https://github.com/googlefonts/gf-docs/tree/main/VerticalMetrics
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/3762 and https://github.com/fonttools/fontbakery/pull/3921",
    title = "Check font follows the Google Fonts vertical metric schema"
)]
fn vertical_metrics(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    let family_name = f.best_familyname().ok_or(FontspectorError::General(
        "Font lacks a family name".to_string(),
    ))?;
    if !context.skip_network {
        skip!(
            is_listed_on_google_fonts(&family_name, context)?,
            "already-onboarded",
            "Not checking vertical metrics for fonts already onboarded to Google Fonts"
        );
    }
    skip!(
        f.is_cjk_font(Some(context)),
        "cjk",
        "Not checking CJK fonts"
    );
    let metrics = f.vertical_metrics()?;

    if metrics.os2_typo_linegap != 0 {
        problems.push(Status::fail(
            "bad-OS/2.sTypoLineGap",
            &format!(
                "OS/2.sTypoLineGap is {}; it should be 0",
                metrics.os2_typo_linegap
            ),
        ));
    }
    if metrics.hhea_linegap != 0 {
        problems.push(Status::fail(
            "bad-hhea.lineGap",
            &format!("hhea.lineGap is {}; it should be 0", metrics.hhea_linegap),
        ));
    }

    // Convert them all to f32 when adding to avoid potential overflow
    let hhea_sum = (metrics.hhea_ascent as f32
        + metrics.hhea_descent.abs() as f32
        + metrics.hhea_linegap as f32)
        / metrics.upm as f32;
    if hhea_sum < 1.2 {
        problems.push(Status::fail(
            "bad-hhea-range",
            &format!(
                "The sum of hhea.ascender + abs(hhea.descender) + hhea.lineGap is {} when it should be at least {}",
                (hhea_sum * metrics.upm as f32) as i32,
                (metrics.upm as f32 * 1.2) as i32
            ),
        ));
    } else if hhea_sum > 2.0 {
        problems.push(Status::fail(
            "bad-hhea-range",
            &format!(
                "The sum of hhea.ascender + abs(hhea.descender) + hhea.lineGap is {} when it should be at most {}",
                (hhea_sum * metrics.upm as f32) as i32,
                (metrics.upm as f32 * 2.0) as i32
            ),
        ));
    } else if hhea_sum > 1.5 {
        problems.push(Status::warn(
            "bad-hhea-range",
            &format!(
                "We recommend the absolute sum of the hhea metrics should be between 1.2-1.5x of the font's upm. This font has {}x ({})",
                hhea_sum,
                (hhea_sum * metrics.upm as f32) as i32
            ),
        ));
    }

    if metrics.os2_typo_ascender < 0 {
        problems.push(Status::fail(
            "typo-ascender",
            &format!(
                "OS/2.sTypoAscender is {}; it must be strictly positive",
                metrics.os2_typo_ascender
            ),
        ));
    }
    if metrics.hhea_ascent <= 0 {
        problems.push(Status::fail(
            "hhea-ascent",
            &format!(
                "hhea.ascent is {}; it must be strictly positive",
                metrics.hhea_ascent
            ),
        ));
    }
    if metrics.os2_typo_descender > 0 {
        problems.push(Status::fail(
            "typo-descender",
            &format!(
                "OS/2.sTypoDescender is {}; it must be negative or zero",
                metrics.os2_typo_descender
            ),
        ));
    }
    if metrics.hhea_descent > 0 {
        problems.push(Status::fail(
            "hhea-descent",
            &format!(
                "hhea.descender is {}; it must be negative or zero",
                metrics.hhea_descent
            ),
        ));
    }
    return_result(problems)
}
