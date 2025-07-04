use fontations::{
    read::{tables::os2::SelectionFlags, TableProvider},
    write::from_obj::ToOwnedTable,
};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Source, SourceFile};

#[check(
    id = "googlefonts/use_typo_metrics",
    rationale = "
        All fonts on the Google Fonts collection should have OS/2.fsSelection bit 7
        (USE_TYPO_METRICS) set. This requirement is part of the vertical metrics scheme
        established as a Google Fonts policy aiming at a common ground supported by
        all major font rendering environments.

        For more details, read:
        https://github.com/googlefonts/gf-docs/blob/main/VerticalMetrics/README.md

        Below is the portion of that document that is most relevant to this check:

        Use_Typo_Metrics must be enabled. This will force MS Applications to use the
        OS/2 Typo values instead of the Win values. By doing this, we can freely set
        the Win values to avoid clipping and control the line height with the typo
        values. It has the added benefit of future line height compatibility. When
        a new script is added, we simply change the Win values to the new yMin
        and yMax, without needing to worry if the line height have changed.
    ",
    metadata = "{\"severity\": 10}",
    proposal = "https://github.com/fonttools/fontbakery/issues/3241",
    title = "OS/2.fsSelection bit 7 (USE_TYPO_METRICS) is set in all fonts.",
    hotfix = fix_use_typo_metrics,
    fix_source = sourcefix_use_typo_metrics,
)]
fn use_typo_metrics(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    skip!(
        f.is_cjk_font(Some(context)),
        "cjk",
        "This check does not apply to CJK fonts."
    );
    if !f.use_typo_metrics()? {
        Ok(Status::just_one_fail(
            "missing-os2-fsselection-bit7",
            "OS/2.fsSelection bit 7 (USE_TYPO_METRICS) was NOT set.",
        ))
    } else {
        Ok(Status::just_one_pass())
    }
}

fn fix_use_typo_metrics(t: &mut Testable) -> FixFnResult {
    let f = testfont!(t);
    if f.is_cjk_font(None) {
        return Ok(false);
    }
    let mut os2: fontations::write::tables::os2::Os2 = f.font().os2()?.to_owned_table();
    os2.fs_selection |= SelectionFlags::USE_TYPO_METRICS;
    t.set(f.rebuild_with_new_table(&os2)?);
    Ok(true)
}

fn sourcefix_use_typo_metrics(s: &mut SourceFile) -> FixFnResult {
    fn fix_a_ufo(font: &mut norad::Font) -> FixFnResult {
        if let Some(selection) = font.font_info.open_type_os2_selection.as_mut() {
            if !selection.contains(&7) {
                log::info!("Adding OS/2.fsSelection bit 7 (USE_TYPO_METRICS) to UFO font.");
                selection.push(7);
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            log::info!("Setting OS/2.fsSelection bit 7 (USE_TYPO_METRICS) in UFO font.");
            font.font_info.open_type_os2_selection = Some(vec![7]);
            Ok(true)
        }
    }
    fn fix_custom_parameters(cps: &mut Vec<glyphslib::common::CustomParameter>) -> FixFnResult {
        if let Some(cp) = cps.iter_mut().find(|cp| cp.name == "Use Typo Metrics") {
            if cp.value.as_i64() != Some(1) {
                log::info!("Setting 'Use Typo Metrics' custom parameter to 1 in Glyphs font.");
                cp.value = 1.into();
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            log::info!("Adding 'Use Typo Metrics' custom parameter with value 1 in Glyphs font.");
            cps.push(glyphslib::common::CustomParameter {
                name: "Use Typo Metrics".to_string(),
                value: 1.into(),
                disabled: false,
            });
            Ok(true)
        }
    }
    match s.source {
        Source::Ufo(ref mut font) => fix_a_ufo(font),
        Source::Designspace(ref mut ds) => ds.apply_fix(&fix_a_ufo),
        Source::Glyphs(ref mut font) => match &mut **font {
            glyphslib::Font::Glyphs2(glyphs2) => {
                fix_custom_parameters(&mut glyphs2.custom_parameters)
            }
            glyphslib::Font::Glyphs3(glyphs3) => {
                fix_custom_parameters(&mut glyphs3.custom_parameters)
            }
        },
    }
}
