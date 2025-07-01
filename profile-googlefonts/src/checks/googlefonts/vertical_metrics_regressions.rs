use crate::network_conditions::{is_listed_on_google_fonts, remote_styles};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

#[check(
    id = "googlefonts/vertical_metrics_regressions",
    rationale = "
        
        If the family already exists on Google Fonts, we need to ensure that the
        checked family's vertical metrics are similar. This check will test the
        following schema which was outlined in Font Bakery issue #1162 [1]:

        - The family should visually have the same vertical metrics as the Regular
          style hosted on Google Fonts.

        - If the family on Google Fonts has differing hhea and typo metrics, the family
          being checked should use the typo metrics for both the hhea and typo entries.

        - If the family on Google Fonts has use typo metrics not enabled and the family
          being checked has it enabled, the hhea and typo metrics should use the family
          on Google Fonts winAscent and winDescent values.

        - If the upms differ, the values must be scaled so the visual appearance is
          the same.

        [1] https://github.com/fonttools/fontbakery/issues/1162
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/1162",
    title = "Check if the vertical metrics of a family are similar to the same
family hosted on Google Fonts."
)]
fn vertical_metrics_regressions(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    skip!(
        f.style() != Some("Regular"),
        "not-regular",
        "Skipping non-Regular style"
    );
    skip!(
        f.is_cjk_font(Some(context)),
        "cjk-font",
        "This check does not apply to CJK fonts."
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

    let gf_has_typo_metrics = remote_font.use_typo_metrics()?;
    let local_has_typo_metrics = f.use_typo_metrics()?;
    let remote_metrics = remote_font.vertical_metrics()?;
    let local_metrics = f.vertical_metrics()?;
    let remote_scaled_to_local = remote_metrics.scale_to_upm(local_metrics.upm);

    let (expected_ascender, expected_descender) = if gf_has_typo_metrics {
        if !local_has_typo_metrics {
            problems.push(Status::fail("bad-fsselection-bit7",
          "fsSelection bit 7 needs to be enabled because the family on Google Fonts has it enabled."));
        }
        (
            remote_scaled_to_local.os2_typo_ascender,
            remote_scaled_to_local.os2_typo_descender,
        )
    } else {
        if (remote_scaled_to_local.os2_win_ascent != local_metrics.os2_win_ascent
            || remote_scaled_to_local.os2_win_descent != local_metrics.os2_win_descent)
            && !local_has_typo_metrics
        {
            problems.push(Status::fail("bad-fsselection-bit7",
        "fsSelection bit 7 needs to be enabled because the win metrics differ from the family on Google Fonts."));
        }
        (
            (remote_scaled_to_local.os2_win_ascent as i16),
            -(remote_scaled_to_local.os2_win_descent as i16),
        )
    };

    if local_metrics.os2_typo_ascender != expected_ascender {
        problems.push(Status::fail(
            "bad-typo-ascender",
            &format!(
                "OS/2 sTypoAscender is {} when it should be {}",
                local_metrics.os2_typo_ascender, expected_ascender
            ),
        ));
    }
    if local_metrics.os2_typo_descender != expected_descender {
        problems.push(Status::fail(
            "bad-typo-descender",
            &format!(
                "OS/2 sTypoDescender is {} when it should be {}",
                local_metrics.os2_typo_descender, expected_descender
            ),
        ));
    }
    if local_metrics.hhea_ascent != expected_ascender {
        problems.push(Status::fail(
            "bad-hhea-ascender",
            &format!(
                "hhea Ascender is {} when it should be {}",
                local_metrics.hhea_ascent, expected_ascender
            ),
        ));
    }
    if local_metrics.hhea_descent != expected_descender {
        problems.push(Status::fail(
            "bad-hhea-descender",
            &format!(
                "hhea Descender is {} when it should be {}",
                local_metrics.hhea_descent, expected_descender
            ),
        ));
    }

    return_result(problems)
}
