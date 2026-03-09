use fontations::{
    read::{tables::os2::SelectionFlags, TableProvider},
    write::from_obj::ToOwnedTable,
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "googlefonts/no_oblique_bit",
    title = "Ensure the OS/2 OBLIQUE fsSelection bit is not set.",
    rationale = "
        Google Fonts does not want fonts with the OBLIQUE bit (bit 9) set
        in the OS/2 fsSelection field. Fonts that are oblique should use
        the Italic bit instead, or be served as a separate Italic file.

        Some fonts like Red Hat Text have been found with both the Italic
        and Oblique bits set, which is not desired for the Google Fonts
        collection.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/238",
    hotfix = fix_no_oblique_bit,
)]
fn no_oblique_bit(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let flags = f.get_os2_fsselection()?;
    if flags.contains(SelectionFlags::OBLIQUE) {
        let message = "The OS/2 fsSelection OBLIQUE bit (bit 9) is set. \
             Google Fonts does not want this bit enabled. \
             Oblique styles should use the Italic bit instead.";
        let mut status = Status::fail("oblique-bit-set", message);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "OS/2".to_string(),
            field_name: Some("fsSelection".to_string()),
            actual: Some(json!("OBLIQUE bit set")),
            expected: Some(json!("OBLIQUE bit not set")),
            message: message.to_string(),
        });
        return_result(vec![status])
    } else {
        Ok(Status::just_one_pass())
    }
}

fn fix_no_oblique_bit(t: &mut Testable) -> FixFnResult {
    let f = testfont!(t);
    let mut os2: fontations::write::tables::os2::Os2 = f.font().os2()?.to_owned_table();
    os2.fs_selection.remove(SelectionFlags::OBLIQUE);
    t.set(f.rebuild_with_new_table(&os2)?);
    Ok(true)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::codetesting::{assert_pass, run_check, test_able};

    use super::no_oblique_bit;

    #[test]
    fn test_pass_no_oblique() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(no_oblique_bit, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_fail_oblique_bit_set() {
        // Load Mada-Regular then set the OBLIQUE bit (bit 9 = 0x0200)
        // in the OS/2 fsSelection field.
        use fontations::skrifa::raw::types::Tag;
        use fontations::skrifa::FontRef;
        use fontations::write::FontBuilder;
        use fontspector_checkapi::codetesting::assert_results_contain;
        use fontspector_checkapi::StatusCode;

        let mut testable = test_able("mada/Mada-Regular.ttf");
        let f = FontRef::new(&testable.contents).unwrap();
        let os2_tag = Tag::new(b"OS/2");

        let os2_data = f.table_data(os2_tag).unwrap();
        let mut os2_bytes = os2_data.as_ref().to_vec();
        // Set OBLIQUE bit (bit 9 = 0x0200) in fsSelection (bytes 62-63, big-endian)
        let fs_sel = u16::from_be_bytes([os2_bytes[62], os2_bytes[63]]);
        let new_fs_sel = fs_sel | 0x0200;
        os2_bytes[62] = (new_fs_sel >> 8) as u8;
        os2_bytes[63] = (new_fs_sel & 0xFF) as u8;

        // Rebuild font with modified OS/2 table
        let mut builder = FontBuilder::new();
        builder.add_raw(os2_tag, &os2_bytes);
        for table_record in f.table_directory.table_records() {
            let tag = table_record.tag.get();
            if tag != os2_tag {
                if let Some(table_data) = f.table_data(tag) {
                    builder.add_raw(tag, table_data);
                }
            }
        }
        testable.contents = builder.build();

        let results = run_check(no_oblique_bit, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("oblique-bit-set".to_string()),
        );
    }

    #[test]
    fn test_hotfix_clears_oblique_bit() {
        use fontations::skrifa::raw::types::Tag;
        use fontations::skrifa::FontRef;
        use fontations::write::FontBuilder;

        // Create a font with the OBLIQUE bit set
        let mut testable = test_able("mada/Mada-Regular.ttf");
        let f = FontRef::new(&testable.contents).unwrap();
        let os2_tag = Tag::new(b"OS/2");

        let os2_data = f.table_data(os2_tag).unwrap();
        let mut os2_bytes = os2_data.as_ref().to_vec();
        let fs_sel = u16::from_be_bytes([os2_bytes[62], os2_bytes[63]]);
        let new_fs_sel = fs_sel | 0x0200;
        os2_bytes[62] = (new_fs_sel >> 8) as u8;
        os2_bytes[63] = (new_fs_sel & 0xFF) as u8;

        let mut builder = FontBuilder::new();
        builder.add_raw(os2_tag, &os2_bytes);
        for table_record in f.table_directory.table_records() {
            let tag = table_record.tag.get();
            if tag != os2_tag {
                if let Some(table_data) = f.table_data(tag) {
                    builder.add_raw(tag, table_data);
                }
            }
        }
        testable.contents = builder.build();

        // Apply the hotfix
        super::fix_no_oblique_bit(&mut testable).unwrap();

        // Verify the check now passes
        let results = run_check(no_oblique_bit, testable);
        assert_pass(&results);
    }
}
