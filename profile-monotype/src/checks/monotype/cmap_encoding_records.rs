use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use skrifa::raw::{tables::cmap::PlatformId, TableProvider};

#[check(
    id = "monotype/cmap_encoding_records",
    rationale = "
        Check if the font has at least one cmap encoding records matching one of the below:
            platform 0 (Unicode), encoding 3 (Unicode BMP only) or 4 (Unicode full repertoire)
            platform 3 (Windows), encoding 1 (Unicode BMP) or 10 (Unicode full repertoire)
    ",
    title = "Checking cmap encoding records."
)]
fn cmap_encoding_records(t: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    let cmap = font.font().cmap()?;

    let mut valid_encoding_records = vec![];

    for encoding_record in cmap.encoding_records().iter() {
        if encoding_record.platform_id() == PlatformId::Unicode
            && (encoding_record.encoding_id() == 3 || encoding_record.encoding_id() == 4)
        {
            valid_encoding_records.push(encoding_record);
        }
        if encoding_record.platform_id() == PlatformId::Windows
            && (encoding_record.encoding_id() == 1 || encoding_record.encoding_id() == 10)
        {
            valid_encoding_records.push(encoding_record);
        }
    }

    Ok(if valid_encoding_records.is_empty() {
        Status::just_one_fail("no-valid-cmap-subtable", "No valid cmap subtable found.")
    } else {
        Status::just_one_pass()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        StatusCode,
    };

    use write_fonts::{from_obj::ToOwnedTable, tables::cmap::Cmap};

    #[test]
    fn test_cmap_encoding_records_pass() {
        let testable = test_able("montserrat/Montserrat-Regular.ttf");
        let results = run_check(cmap_encoding_records, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_cmap_encoding_records_fail() {
        let mut testable = test_able("montserrat/Montserrat-Regular.ttf");
        let f = fontspector_checkapi::prelude::TTF
            .from_testable(&testable)
            .unwrap();
        let mut cmap: Cmap = f.font().cmap().unwrap().to_owned_table();
        cmap.encoding_records.clear();
        testable.set(f.rebuild_with_new_table(&cmap).unwrap());

        let results = run_check(cmap_encoding_records, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("no-valid-cmap-subtable".to_string()),
        );
    }
}
