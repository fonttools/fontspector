use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "opentype/name/empty_records",
    title = "Check name table for empty records.",
    rationale = "Check the name table for empty records, as this can cause problems in Adobe apps.",
    proposal = "https://github.com/fonttools/fontbakery/pull/2369"
)]
fn empty_records(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let name = f.font().name()?;
    let mut problems: Vec<Status> = vec![];
    for record in name.name_record() {
        if record
            .string(name.string_data())?
            .to_string()
            .trim()
            .is_empty()
        {
            problems.push(Status::fail(
                "empty-record",
                &format!(
                    "Empty name record found for name ID={} platform ID={} encoding ID={}",
                    record.name_id(),
                    record.platform_id(),
                    record.encoding_id(),
                ),
            ));
        }
    }
    return_result(problems)
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use fontations::skrifa::raw::types::NameId;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, set_name_entry, test_able},
        StatusCode,
    };

    #[test]
    fn test_empty_records_pass() {
        let testable = test_able("source-sans-pro/OTF/SourceSansPro-Regular.otf");
        let result = run_check(empty_records, testable);
        assert_pass(&result);
    }

    #[test]
    fn test_empty_records_fail() {
        let mut testable = test_able("source-sans-pro/OTF/SourceSansPro-Regular.otf");
        // Set a name entry to an empty string
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::UNIQUE_ID,
            "".to_string(),
        );
        let result = run_check(empty_records, testable);
        assert_results_contain(&result, StatusCode::Fail, Some("empty-record".to_string()));
    }
}
