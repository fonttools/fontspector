use fontations::{
    read::{tables::os2::SelectionFlags, TableProvider},
    write::from_obj::ToOwnedTable,
};
use fontspector_checkapi::{fixfont, prelude::*, skip, testfont, FileTypeConvert};

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
    let f = fixfont!(t);
    if f.is_cjk_font(None) {
        return Ok(false);
    }
    let mut os2: fontations::write::tables::os2::Os2 = f
        .font()
        .os2()
        .map_err(|e| format!("Couldn't read OS/2 table: {}", e))?
        .to_owned_table();
    os2.fs_selection |= SelectionFlags::USE_TYPO_METRICS;
    t.set(f.rebuild_with_new_tables(&[os2])?);
    Ok(true)
}
