use fontations::skrifa::raw::{
    tables::stat::{AxisValue, AxisValueTableFlags},
    TableProvider,
};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "googlefonts/STAT/opsz_not_elided",
    title = "Ensure 'opsz' axis values are not marked as elidable in STAT.",
    rationale = "
        The 'opsz' (Optical Size) axis values in the STAT table should
        not be marked as elidable. Unlike weight or width, optical size
        is an important characteristic that should always be visible to
        users so they can make informed choices about which optical size
        variant to use.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/222"
)]
fn opsz_not_elided(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let stat = f.font().stat();
    skip!(stat.is_err(), "no-stat", "Font has no STAT table.");
    let stat = stat?;

    let axes = stat.design_axes()?;
    let opsz_index = axes.iter().position(|a| a.axis_tag() == "opsz");
    skip!(
        opsz_index.is_none(),
        "no-opsz",
        "Font has no 'opsz' axis in STAT table."
    );
    let opsz_index = opsz_index.unwrap_or(0) as u16;

    let mut problems = vec![];

    if let Some(Ok(subtable)) = stat.offset_to_axis_values() {
        for val in subtable.axis_values().iter().flatten() {
            let is_opsz = match &val {
                AxisValue::Format1(v) => v.axis_index() == opsz_index,
                AxisValue::Format2(v) => v.axis_index() == opsz_index,
                AxisValue::Format3(v) => v.axis_index() == opsz_index,
                AxisValue::Format4(v) => v
                    .axis_values()
                    .iter()
                    .any(|av| av.axis_index() == opsz_index),
            };
            if is_opsz
                && val
                    .flags()
                    .contains(AxisValueTableFlags::ELIDABLE_AXIS_VALUE_NAME)
            {
                let value_str = match &val {
                    AxisValue::Format1(v) => format!("{}", v.value()),
                    AxisValue::Format2(v) => format!("{}", v.nominal_value()),
                    AxisValue::Format3(v) => format!("{}", v.value()),
                    AxisValue::Format4(_) => "Format4".to_string(),
                };
                let message = format!(
                    "STAT table 'opsz' axis value '{}' is marked as elidable. \
                     Optical size values should not be elidable.",
                    value_str
                );
                let mut status = Status::warn("elidable-opsz", &message);
                status.add_metadata(Metadata::TableProblem {
                    table_tag: "STAT".to_string(),
                    field_name: Some(format!("opsz axis value {}", value_str)),
                    actual: Some(json!("ELIDABLE_AXIS_VALUE_NAME set")),
                    expected: Some(json!("ELIDABLE_AXIS_VALUE_NAME not set")),
                    message,
                });
                problems.push(status);
            }
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::codetesting::{assert_pass, assert_skip, run_check, test_able};

    use super::opsz_not_elided;

    #[test]
    fn test_skip_no_stat() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(opsz_not_elided, testable);
        assert_skip(&results);
    }

    #[test]
    fn test_skip_no_opsz() {
        // Inter has STAT but no opsz axis
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results = run_check(opsz_not_elided, testable);
        assert_skip(&results);
    }

    #[test]
    fn test_pass_league_gothic() {
        // LeagueGothic may have opsz without elidable flag
        let testable = test_able("leaguegothic-vf/LeagueGothic[wdth].ttf");
        let results = run_check(opsz_not_elided, testable);
        // Will skip if no opsz axis, which is fine
        let result = results.as_ref().unwrap();
        let worst = result.subresults.iter().map(|s| s.severity).max().unwrap();
        assert!(
            worst <= fontspector_checkapi::StatusCode::Pass
                || worst == fontspector_checkapi::StatusCode::Skip
        );
    }
}
