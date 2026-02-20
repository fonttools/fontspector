use fontations::{
    read::{tables::os2::SelectionFlags, TableProvider},
    write::from_obj::ToOwnedTable,
};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::json;

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
    let mut problems = vec![];
    if !f.use_typo_metrics()? {
        let msg = "OS/2.fsSelection bit 7 (USE_TYPO_METRICS) was NOT set.";
        let mut status = Status::fail("missing-os2-fsselection-bit7", msg);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "OS/2".to_string(),
            field_name: Some("fsSelection".to_string()),
            actual: Some(json!("bit 7 not set")),
            expected: Some(json!("bit 7 set (USE_TYPO_METRICS)")),
            message: msg.to_string(),
        });
        problems.push(status);
    }
    return_result(problems)
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
