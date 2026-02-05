use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

const OPTIONAL_TABLE_TAGS: [&[u8; 4]; 20] = [
    b"cvt ", b"fpgm", b"loca", b"prep", b"VORG", b"EBDT", b"EBLC", b"EBSC", b"BASE", b"GPOS",
    b"GSUB", b"JSTF", b"gasp", b"hdmx", b"LTSH", b"PCLT", b"VDMX", b"vhea", b"vmtx", b"kern",
];

#[check(
    id = "required_tables",
    title = "Font contains all required tables?",
    rationale = "
        According to the OpenType spec
        https://docs.microsoft.com/en-us/typography/opentype/spec/otff#required-tables

        Whether TrueType or CFF outlines are used in an OpenType font, the following
        tables are required for the font to function correctly:

        - cmap (Character to glyph mapping)⏎
        - head (Font header)⏎
        - hhea (Horizontal header)⏎
        - hmtx (Horizontal metrics)⏎
        - maxp (Maximum profile)⏎
        - name (Naming table)⏎
        - OS/2 (OS/2 and Windows specific metrics)⏎
        - post (PostScript information)

        The spec also documents that variable fonts require the following table:

        - STAT (Style attributes)

        Depending on the typeface and coverage of a font, certain tables are
        recommended for optimum quality.

        For example:⏎
        - the performance of a non-linear font is improved if the VDMX, LTSH,
          and hdmx tables are present.⏎
        - Non-monospaced Latin fonts should have a kern table.⏎
        - A gasp table is necessary if a designer wants to influence the sizes
          at which grayscaling is used under Windows. Etc.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
    proposal = "https://github.com/fonttools/fontspector/issues/516"  // vmtx + VVAR
)]
fn required_tables(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut required_table_tags: Vec<_> = vec![
        b"cmap", b"head", b"hhea", b"hmtx", b"maxp", b"name", b"OS/2", b"post",
    ];

    if f.is_variable_font() {
        // According to https://github.com/fonttools/fontbakery/issues/1671
        // STAT table is required on WebKit on MacOS 10.12 for variable fonts.
        required_table_tags.push(b"STAT");
    }

    // See https://github.com/fonttools/fontbakery/issues/617
    //
    // We should collect the rationale behind the need for each of the
    // required tables above. Perhaps split it into individual checks
    // with the correspondent rationales for each subset of required tables.
    //
    // opentype/kern_table is a good example of a separate
    // check for a specific table providing a detailed description of
    // the rationale behind it.

    let mut problems: Vec<Status> = vec![];
    let mut optional: Vec<String> = vec![];

    for tag in OPTIONAL_TABLE_TAGS {
        if f.has_table(tag) {
            optional.push(String::from_utf8(tag.to_vec()).map_err(|_| {
                FontspectorError::General(format!("Font tag '{:?}' wasn't UTF8?", tag.to_vec()))
            })?)
        }
    }
    if !optional.is_empty() {
        problems.push(Status::info(
            "optional-tables",
            &format!(
                "This font contains the following optional tables:\n\n    {}",
                optional.join("\n    ")
            ),
        ))
    }

    let mut missing = vec![];
    for tag in required_table_tags {
        if !f.has_table(tag) {
            missing.push(String::from_utf8(tag.to_vec()).map_err(|_| {
                FontspectorError::General(format!("Font tag '{:?}' wasn't UTF8?", tag.to_vec()))
            })?);
        }
    }

    // Note (from the OpenType spec):
    // OpenType fonts that contain TrueType outlines should use the value of 0x00010000
    // for sfntVersion. OpenType fonts containing CFF data (version 1 or 2) should use
    // 0x4F54544F ('OTTO', when re-interpreted as a Tag) for sfntVersion.
    let version = f.font().table_directory.sfnt_version();
    if version == 0x4F54544F && (!f.has_table(b"CFF ") && !f.has_table(b"CFF2")) {
        if f.has_table(b"fvar") {
            missing.push("CFF2".to_string());
        } else {
            missing.push("CFF ".to_string());
        }
    } else if version == 0x00010000 && !f.has_table(b"glyf") {
        missing.push("glyf".to_string());
    }

    if !missing.is_empty() {
        problems.push(Status::fail(
            "required-tables",
            &format!(
                "This font is missing the following required tables:\n\n    {}",
                missing.join("\n    ")
            ),
        ))
    }

    // Variable fonts with vmtx should also have VVAR for performance.
    // https://github.com/fonttools/fontspector/issues/516
    if f.is_variable_font() && f.has_table(b"vmtx") && !f.has_table(b"VVAR") {
        problems.push(Status::warn(
            "missing-vvar",
            "Font has a vmtx table but no VVAR table. \
             Adding a VVAR table speeds up processing of vertical typesetting \
             significantly with only a minor file size increase.",
        ))
    }

    return_result(problems)
}

#[cfg(test)]
mod tests {
    use super::{required_tables, OPTIONAL_TABLE_TAGS};
    use fontspector_checkapi::{
        codetesting::{
            add_table, assert_messages_contain, assert_results_contain, remove_table, run_check,
            test_able,
        },
        StatusCode,
    };

    #[test]
    fn test_truetype_font_pass() {
        // TrueType font contains all required tables, so it must PASS (no FAIL)
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(required_tables, testable);
        // Check has INFO for optional tables, but should not have FAIL
        if let Some(result) = &results {
            assert!(
                result.worst_status() < StatusCode::Warn,
                "TrueType font should pass (worst status: {:?})",
                result.worst_status()
            );
        }
    }

    #[test]
    fn test_truetype_font_optional_tables() {
        // Verify INFO is reported with optional tables (loca, GPOS, GSUB)
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(required_tables, testable);
        assert_results_contain(
            &results,
            StatusCode::Info,
            Some("optional-tables".to_string()),
        );
        assert_messages_contain(&results, "loca");
        assert_messages_contain(&results, "GPOS");
        assert_messages_contain(&results, "GSUB");
    }

    #[test]
    fn test_cff_font_pass() {
        // OpenType-CFF font contains all required tables, so it must PASS (no FAIL)
        let testable = test_able("source-sans-pro/OTF/SourceSansPro-Black.otf");
        let results = run_check(required_tables, testable);
        if let Some(result) = &results {
            assert!(
                result.worst_status() < StatusCode::Warn,
                "CFF font should pass (worst status: {:?})",
                result.worst_status()
            );
        }
    }

    #[test]
    fn test_cff_font_optional_tables() {
        // Verify INFO is reported with optional tables (BASE, GPOS, GSUB)
        let testable = test_able("source-sans-pro/OTF/SourceSansPro-Black.otf");
        let results = run_check(required_tables, testable);
        assert_results_contain(
            &results,
            StatusCode::Info,
            Some("optional-tables".to_string()),
        );
        assert_messages_contain(&results, "BASE");
        assert_messages_contain(&results, "GPOS");
        assert_messages_contain(&results, "GSUB");
    }

    #[test]
    fn test_cff2_font_pass() {
        // OpenType-CFF2 variable font contains all required tables, so it must PASS (no FAIL)
        let testable = test_able("source-sans-pro/VAR/SourceSansVariable-Italic.otf");
        let results = run_check(required_tables, testable);
        if let Some(result) = &results {
            assert!(
                result.worst_status() < StatusCode::Warn,
                "CFF2 font should pass (worst status: {:?})",
                result.worst_status()
            );
        }
    }

    #[test]
    fn test_cff2_font_optional_tables() {
        // Verify INFO is reported with optional tables (BASE, GPOS, GSUB)
        let testable = test_able("source-sans-pro/VAR/SourceSansVariable-Italic.otf");
        let results = run_check(required_tables, testable);
        assert_results_contain(
            &results,
            StatusCode::Info,
            Some("optional-tables".to_string()),
        );
        assert_messages_contain(&results, "BASE");
        assert_messages_contain(&results, "GPOS");
        assert_messages_contain(&results, "GSUB");
    }

    #[test]
    fn test_missing_required_tables() {
        // Remove each required table one-by-one to validate the FAIL code-path
        // Also test glyf which is required for TrueType fonts
        //
        // Note: maxp is required for TestFont initialization, so removing it
        // causes an ERROR rather than FAIL. We skip it here since the behavior
        // is technically correct (font can't be processed without maxp).
        let tables_to_test: [&[u8; 4]; 8] = [
            b"cmap", b"head", b"hhea", b"hmtx", b"name", b"OS/2", b"post", b"glyf",
        ];

        for required in tables_to_test.iter() {
            let mut testable = test_able("mada/Mada-Regular.ttf");
            remove_table(&mut testable, required);
            let results = run_check(required_tables, testable);
            assert_results_contain(
                &results,
                StatusCode::Fail,
                Some("required-tables".to_string()),
            );
            let tag_str = std::str::from_utf8(required.as_slice()).unwrap();
            assert_messages_contain(&results, tag_str);
        }
    }

    #[test]
    fn test_optional_tables_detection() {
        // First remove all optional tables from the font
        let mut testable = test_able("mada/Mada-Regular.ttf");
        for optional in OPTIONAL_TABLE_TAGS.iter() {
            remove_table(&mut testable, optional);
        }

        // Then re-insert them one by one to validate the INFO code-path
        for optional in OPTIONAL_TABLE_TAGS.iter() {
            let mut test_font = testable.clone();
            add_table(&mut test_font, optional);
            let results = run_check(required_tables, test_font);
            assert_results_contain(
                &results,
                StatusCode::Info,
                Some("optional-tables".to_string()),
            );
            let tag_str = std::str::from_utf8(optional.as_slice()).unwrap();
            assert_messages_contain(&results, tag_str);
        }
    }

    #[test]
    fn test_vvar_missing() {
        // Variable font with vmtx but no VVAR should WARN
        // NotoSansJP has fvar and vmtx but no VVAR
        let testable = test_able("cjk/NotoSansJP[wght].ttf");
        let results = run_check(required_tables, testable);
        assert_results_contain(&results, StatusCode::Warn, Some("missing-vvar".to_string()));
        assert_messages_contain(&results, "vmtx");
    }

    #[test]
    fn test_vvar_present() {
        // Variable font with vmtx AND VVAR should not trigger warning
        // ShantellSans has fvar, vmtx, and VVAR
        let testable = test_able("shantell/ShantellSans[BNCE,INFM,SPAC,wght].ttf");
        let results = run_check(required_tables, testable);
        if let Some(result) = &results {
            for status in &result.subresults {
                assert!(
                    status.code.as_deref() != Some("missing-vvar"),
                    "Should not warn about missing VVAR when VVAR is present"
                );
            }
        }
    }
}
