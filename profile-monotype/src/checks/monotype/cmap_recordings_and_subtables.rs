use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use skrifa::raw::{tables::cmap::PlatformId, TableProvider};

#[check(
    id = "monotype/cmap_recordings_and_subtables",
    rationale = "
        Check if the font has at least one cmap encoding records matching one of the below:
            platform 0 (Unicode), encoding 3 (Unicode BMP only) or 4 (Unicode full repertoire)
            platform 3 (Windows), encoding 1 (Unicode BMP) or 10 (Unicode full repertoire)
        Check if each subtable with platform 3, encoding 1; or platform 0, encoding 3 has other than format 4 => 🔥 FAIL
        Check if each subtable with platform 3, encoding 10; or platform 0, encoding 4 has other than format 12 => 🔥 FAIL
    ",
    proposal = "https://github.com/Monotype/fontspector/issues/9",
    title = "Checking cmap recordings and subtables."
)]
fn cmap_recordings_and_subtables(t: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    let cmap = font.font().cmap()?;

    let mut valid_encoding_records = vec![];
    let mut problems = vec![];

    for encoding_record in cmap.encoding_records().iter() {
        // platform 0 == Unicode
        if encoding_record.platform_id() == PlatformId::Unicode
            && (encoding_record.encoding_id() == 3 || encoding_record.encoding_id() == 4)
        {
            valid_encoding_records.push(encoding_record);
        }
        // platform 3 == Windows
        if encoding_record.platform_id() == PlatformId::Windows
            && (encoding_record.encoding_id() == 1 || encoding_record.encoding_id() == 10)
        {
            valid_encoding_records.push(encoding_record);
        }
        // Check if each subtable with platform 3, encoding 1; or platform 0, encoding 3 has format 4 => 🔥 FAIL
        if (encoding_record.platform_id() == PlatformId::Windows
            && encoding_record.encoding_id() == 1)
            || (encoding_record.platform_id() == PlatformId::Unicode
                && encoding_record.encoding_id() == 3)
        {
            if let Ok(subtable) = encoding_record.subtable(cmap.offset_data()) {
                if subtable.format() != 4 {
                    problems.push(Status::fail(
                        "invalid-cmap-subtable-4",
                        &format!(
                            "For platform {} encoding {}: we expected cmap subtable to be in format 4 but got {}.",
                            encoding_record.platform_id() as u16,
                            encoding_record.encoding_id() as u16,
                            subtable.format(),
                        ),
                    ));
                }
            }
        }
        // Check if each subtable with platform 3, encoding 10; or platform 0, encoding 4 has format 12 => 🔥 FAIL
        if (encoding_record.platform_id() == PlatformId::Windows
            && encoding_record.encoding_id() == 10)
            || (encoding_record.platform_id() == PlatformId::Unicode
                && encoding_record.encoding_id() == 4)
        {
            if let Ok(subtable) = encoding_record.subtable(cmap.offset_data()) {
                if subtable.format() != 12 {
                    problems.push(Status::fail(
                        "invalid-cmap-subtable-12",
                        &format!(
                            "For platform {} encoding {}: we expected cmap subtable to be in format 12 but got {}.",
                            encoding_record.platform_id() as u16,
                            encoding_record.encoding_id() as u16,
                            subtable.format(),
                        ),
                    ));
                }
            }
        }
    }

    if valid_encoding_records.is_empty() {
        problems.push(Status::fail(
            "no-valid-cmap-records",
            "No valid cmap records found.",
        ));
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    use super::cmap_recordings_and_subtables;

    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        StatusCode,
    };

    use fontspector_checkapi::FileTypeConvert;
    use skrifa::raw::TableProvider;
    use write_fonts::{from_obj::ToOwnedTable, tables::cmap::Cmap};

    #[test]
    fn test_cmap_recordings_and_subtables_pass() {
        let testable = test_able("montserrat/Montserrat-Regular.ttf");
        let results = run_check(cmap_recordings_and_subtables, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_cmap_recordings_and_subtables_fail() {
        let mut testable = test_able("montserrat/Montserrat-Regular.ttf");
        let f = fontspector_checkapi::prelude::TTF
            .from_testable(&testable)
            .unwrap();
        let mut cmap: Cmap = f.font().cmap().unwrap().to_owned_table();
        cmap.encoding_records.clear();
        testable.set(f.rebuild_with_new_table(&cmap).unwrap());

        let results = run_check(cmap_recordings_and_subtables, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("no-valid-cmap-records".to_string()),
        );
    }
}
