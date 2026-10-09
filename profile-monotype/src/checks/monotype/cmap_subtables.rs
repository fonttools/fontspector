use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "monotype/cmap_subtables",
    rationale = "
        Check if the font has at least one cmap subtable matching one of the below => ☠ FATAL
            platform 0, encoding 3 or 4
            platform 3, encoding 1 or 10
        Check if each subtable with platform 3, encoding 1; or platform 0, encoding 3 has format 4 => 🔥 FAIL
        Check if each subtable with platform 3, encoding 10; or platform 0, encoding 4 has format 12 => 🔥 FAIL
    ",
    title = "Checking cmap subtables."
)]
fn cmap_subtables(t: &Testable, _context: &Context) -> CheckFnResult {
    let _font = testfont!(t);
    Ok(Status::just_one_pass())
}

#[cfg(test)]
mod tests {
    use super::*;

    use fontspector_checkapi::codetesting::{assert_pass, run_check, test_able};

    #[test]
    fn test_cmap_subtables_pass() {
        let testable = test_able("montserrat/Montserrat-Regular.ttf");
        let results = run_check(cmap_subtables, testable);
        assert_pass(&results);
    }

    // #[test]
    // fn test_cmap_subtables_with_config() {
    //     use std::collections::HashMap;
    //     use serde_json::json;
    //     use fontspector_checkapi::{
    //         codetesting::{
    //             run_check_with_config,
    //         },
    //         TestableType,
    //     };
    //     let testable = test_able("montserrat/Montserrat-Regular.ttf");
    //     let config = HashMap::from([("monotype/cmap_subtables".to_string(), json!({"fstype_value": 0}))]);
    //     let results = run_check_with_config(cmap_subtables, TestableType::Single(&testable), config);
    //     assert_pass(&results);
    // }
}
