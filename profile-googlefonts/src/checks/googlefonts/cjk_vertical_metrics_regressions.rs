use crate::network_conditions::{is_listed_on_google_fonts, remote_styles};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

#[check(
    id = "googlefonts/cjk_vertical_metrics_regressions",
    rationale = "
        
        Check CJK family has the same vertical metrics as the same family
        hosted on Google Fonts.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/3244",
    title = "Check if the vertical metrics of a CJK family are similar to the same
family hosted on Google Fonts."
)]
fn cjk_vertical_metrics_regressions(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    skip!(
        f.style() != Some("Regular"),
        "not-regular",
        "Skipping non-Regular style"
    );
    skip!(
        !f.is_cjk_font(Some(context)),
        "not-cjk-font",
        "This check only applies to CJK fonts."
    );
    skip!(
        context.skip_network,
        "network-disabled",
        "Network access disabled"
    );
    skip!(
        !is_listed_on_google_fonts(&f.best_familyname().unwrap_or_default(), context)?,
        "not-listed-on-google-fonts",
        "Skipping check because font is not listed on Google Fonts"
    );
    let family_name = f.best_familyname().unwrap_or("New font".to_string());
    let remote = remote_styles(&family_name, context)
        .map_err(|e| FontspectorError::General(format!("Could not get remote style: {e}")))?;
    let remote_font = remote
        .iter()
        .flat_map(|r| TTF.from_testable(r))
        .find(|f| f.style() == Some("Regular"))
        .ok_or_else(|| {
            FontspectorError::General("Could not find remote Regular style".to_string())
        })?;

    let remote_metrics = remote_font.vertical_metrics()?;
    let local_metrics = f.vertical_metrics()?;
    let remote_scaled_to_local = remote_metrics.scale_to_upm(local_metrics.upm);

    if local_metrics.os2_typo_ascender != remote_scaled_to_local.os2_typo_ascender {
        problems.push(Status::fail(
            "cjk-metric-regression",
            &format!(
                "OS/2 sTypoAscender is {} when it should be {}",
                local_metrics.os2_typo_ascender, remote_scaled_to_local.os2_typo_ascender
            ),
        ));
    }
    if local_metrics.os2_typo_descender != remote_scaled_to_local.os2_typo_descender {
        problems.push(Status::fail(
            "cjk-metric-regression",
            &format!(
                "OS/2 sTypoDescender is {} when it should be {}",
                local_metrics.os2_typo_descender, remote_scaled_to_local.os2_typo_descender
            ),
        ));
    }
    if local_metrics.os2_typo_linegap != remote_scaled_to_local.os2_typo_linegap {
        problems.push(Status::fail(
            "cjk-metric-regression",
            &format!(
                "OS/2 sTypoLineGap is {} when it should be {}",
                local_metrics.os2_typo_linegap, remote_scaled_to_local.os2_typo_linegap
            ),
        ));
    }
    if local_metrics.os2_win_ascent != remote_scaled_to_local.os2_win_ascent {
        problems.push(Status::fail(
            "cjk-metric-regression",
            &format!(
                "OS/2 usWinAscent is {} when it should be {}",
                local_metrics.os2_win_ascent, remote_scaled_to_local.os2_win_ascent
            ),
        ));
    }
    if local_metrics.os2_win_descent != remote_scaled_to_local.os2_win_descent {
        problems.push(Status::fail(
            "cjk-metric-regression",
            &format!(
                "OS/2 usWinDescent is {} when it should be {}",
                local_metrics.os2_win_descent, remote_scaled_to_local.os2_win_descent
            ),
        ));
    }
    if local_metrics.hhea_ascent != remote_scaled_to_local.hhea_ascent {
        problems.push(Status::fail(
            "cjk-metric-regression",
            &format!(
                "hhea ascent is {} when it should be {}",
                local_metrics.hhea_ascent, remote_scaled_to_local.hhea_ascent
            ),
        ));
    }
    if local_metrics.hhea_descent != remote_scaled_to_local.hhea_descent {
        problems.push(Status::fail(
            "cjk-metric-regression",
            &format!(
                "hhea descent is {} when it should be {}",
                local_metrics.hhea_descent, remote_scaled_to_local.hhea_descent
            ),
        ));
    }
    if local_metrics.hhea_linegap != remote_scaled_to_local.hhea_linegap {
        problems.push(Status::fail(
            "cjk-metric-regression",
            &format!(
                "hhea lineGap is {} when it should be {}",
                local_metrics.hhea_linegap, remote_scaled_to_local.hhea_linegap
            ),
        ));
    }
    return_result(problems)
}
