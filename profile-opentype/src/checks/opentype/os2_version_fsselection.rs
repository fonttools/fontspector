use fontations::{
    skrifa::raw::{tables::os2::SelectionFlags, TableProvider},
    write::from_obj::ToOwnedTable,
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

/// fsSelection flags that are only valid for OS/2 version >= 4.
const V4_FLAGS: &[(SelectionFlags, &str)] = &[
    (SelectionFlags::USE_TYPO_METRICS, "USE_TYPO_METRICS (bit 7)"),
    (SelectionFlags::WWS, "WWS (bit 8)"),
    (SelectionFlags::OBLIQUE, "OBLIQUE (bit 9)"),
];

#[check(
    id = "opentype/os2_version_fsselection",
    title = "OS/2 version must be >= 4 if certain fsSelection bits are set.",
    rationale = "
        The fsSelection bits USE_TYPO_METRICS (bit 7), WWS (bit 8), and
        OBLIQUE (bit 9) were introduced in OS/2 version 4. Setting these
        bits in an earlier version of the OS/2 table is invalid and may
        cause unpredictable behavior in some environments.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/178",
    hotfix = fix_os2_version_fsselection,
)]
fn os2_version_fsselection(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let os2 = f.font().os2()?;
    let version = os2.version();
    let flags = os2.fs_selection();
    let mut problems = vec![];

    if version < 4 {
        for (flag, name) in V4_FLAGS {
            if flags.contains(*flag) {
                let message = format!(
                    "OS/2 version is {}, but fsSelection {} is set. \
                     This bit requires OS/2 version >= 4.",
                    version, name
                );
                let mut status = Status::warn("v4-flag-in-old-os2", &message);
                status.add_metadata(Metadata::TableProblem {
                    table_tag: "OS/2".to_string(),
                    field_name: Some("version / fsSelection".to_string()),
                    actual: Some(json!({"version": version, "flag": name})),
                    expected: Some(json!("version >= 4 for this flag")),
                    message: message.clone(),
                });
                problems.push(status);
            }
        }
    }
    return_result(problems)
}

fn fix_os2_version_fsselection(t: &mut Testable) -> FixFnResult {
    let f = testfont!(t);
    let os2 = f.font().os2()?;
    let version = os2.version();

    if version >= 4 {
        return Ok(false);
    }

    let mut os2_write: fontations::write::tables::os2::Os2 = os2.to_owned_table();
    for (flag, _name) in V4_FLAGS {
        os2_write.fs_selection.remove(*flag);
    }

    t.set(f.rebuild_with_new_table(&os2_write)?);
    Ok(true)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::codetesting::{assert_pass, run_check, test_able};

    use super::os2_version_fsselection;

    #[test]
    fn test_pass_normal_font() {
        // Mada-Regular should pass (typical OS/2 version >= 4)
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(os2_version_fsselection, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_warn_v4_flag_in_old_os2() {
        // Load Mada-Regular (OS/2 version 4), then rebuild with
        // OS/2 version 3 and USE_TYPO_METRICS (bit 7) set in fsSelection.
        use fontations::skrifa::raw::types::Tag;
        use fontations::skrifa::FontRef;
        use fontations::write::FontBuilder;
        use fontspector_checkapi::codetesting::assert_results_contain;
        use fontspector_checkapi::StatusCode;

        let mut testable = test_able("mada/Mada-Regular.ttf");
        let f = FontRef::new(&testable.contents).unwrap();
        let os2_tag = Tag::new(b"OS/2");

        // Get original OS/2 table data and modify it
        let os2_data = f.table_data(os2_tag).unwrap();
        let mut os2_bytes = os2_data.as_ref().to_vec();
        // Set version to 3 (bytes 0-1, big-endian)
        os2_bytes[0] = 0x00;
        os2_bytes[1] = 0x03;
        // Set USE_TYPO_METRICS bit (bit 7 = 0x0080) in fsSelection (bytes 62-63)
        let fs_sel = u16::from_be_bytes([os2_bytes[62], os2_bytes[63]]);
        let new_fs_sel = fs_sel | 0x0080;
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

        let results = run_check(os2_version_fsselection, testable);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("v4-flag-in-old-os2".to_string()),
        );
    }
}
