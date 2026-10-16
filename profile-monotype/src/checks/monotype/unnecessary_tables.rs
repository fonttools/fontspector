use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::json;

const UNNECESSARY_TABLE_TAGS: [&[u8; 4]; 10] = [
    b"LINO", b"SLST", b"FFTM", b"PfEd", b"TeX ", b"BDF ", b"bdat", b"bloc", b"ADFH", b"SPRK",
];

const UNNECESSARY_TABLE_TAGS_CFF: [&[u8; 4]; 6] =
    [b"gasp", b"cvt ", b"fpgm", b"glyf", b"loca", b"prep"];
const UNNECESSARY_TABLE_TAGS_TTF: [&[u8; 4]; 1] = [b"VORG"];

#[check(
    id = "monotype/unnecessary_tables",
    title = "Font contains unnecessary tables?",
    rationale = "
        Unnecessary tables (private tables and embedded bitmap tables).
    ",
    proposal = "https://github.com/Monotype/fontspector/issues/17"
)]
fn unnecessary_tables(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut unnecessary_table_tags = UNNECESSARY_TABLE_TAGS.to_vec();

    let mut problems: Vec<Status> = vec![];
    let mut found_unnecessary_tables: Vec<String> = vec![];

    if f.is_variable_font() {
        unnecessary_table_tags.push(b"kern");
    }

    let version = f.font().table_directory.sfnt_version();
    if version == 0x4F54544F {
        // CFF and CFF2
        unnecessary_table_tags.extend_from_slice(&UNNECESSARY_TABLE_TAGS_CFF);
    } else if version == 0x00010000 {
        // TrueType
        unnecessary_table_tags.extend_from_slice(&UNNECESSARY_TABLE_TAGS_TTF);
    } else {
        skip!("unknown-sfnt-version", "Unknown sfnt version encountered");
    }

    for tag in unnecessary_table_tags {
        if f.has_table(tag) {
            found_unnecessary_tables.push(String::from_utf8(tag.to_vec()).map_err(|_| {
                FontspectorError::General(format!("Font tag '{:?}' wasn't UTF8?", tag.to_vec()))
            })?)
        }
    }

    // Check if font contains tables starting with TSI.|MT..|ASC.|edt.|ABA.?
    let font_tables = f
        .font()
        .table_directory
        .table_records()
        .iter()
        .map(|record| record.tag().to_string())
        .collect::<Vec<_>>();
    for table in font_tables {
        if table.starts_with("TSI")
            || table.starts_with("MT")
            || table.starts_with("ASC")
            || table.starts_with("edt")
            || table.starts_with("ABA")
        {
            found_unnecessary_tables.push(table);
        }
    }

    if !found_unnecessary_tables.is_empty() {
        let message = format!(
            "This font contains the following unnecessary tables:\n\n    {}",
            found_unnecessary_tables
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("\n    ")
        );
        let mut status = Status::fail("unnecessary-tables", &message);
        status.add_metadata(Metadata::FontProblem {
            message: message.clone(),
            context: Some(json!({ "unnecessary_tables": found_unnecessary_tables.iter().map(|s| s.to_string()).collect::<Vec<_>>() })),
        });
        problems.push(status);
    }

    return_result(problems)
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::{
        unnecessary_tables, UNNECESSARY_TABLE_TAGS, UNNECESSARY_TABLE_TAGS_CFF,
        UNNECESSARY_TABLE_TAGS_TTF,
    };
    use fontspector_checkapi::{
        codetesting::{
            add_table, assert_messages_contain, assert_pass, assert_results_contain, remove_table,
            run_check, test_able,
        },
        StatusCode,
    };

    #[test]
    fn test_ttf_font_pass() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(unnecessary_tables, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_ttf_variable_font_pass() {
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results = run_check(unnecessary_tables, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_cff_font_pass() {
        let testable = test_able("source-sans-pro/OTF/SourceSansPro-Black.otf");
        let results = run_check(unnecessary_tables, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_cff2_font_pass() {
        let testable = test_able("source-sans-pro/VAR/SourceSansVariable-Italic.otf");
        let results = run_check(unnecessary_tables, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_unnecessary_tables_ttf() {
        let mut testable = test_able("mada/Mada-Regular.ttf");
        let mut unnecessary_table_tags = UNNECESSARY_TABLE_TAGS.to_vec();
        unnecessary_table_tags.extend_from_slice(&UNNECESSARY_TABLE_TAGS_TTF);
        let additional_tags: [&[u8; 4]; 5] = [b"MT01", b"TSI0", b"ASC0", b"edt0", b"ABA0"];
        unnecessary_table_tags.extend_from_slice(&additional_tags);

        for optional in unnecessary_table_tags.iter() {
            remove_table(&mut testable, optional);
        }

        for optional in unnecessary_table_tags.iter() {
            let mut test_font = testable.clone();
            add_table(&mut test_font, optional);
            let results = run_check(unnecessary_tables, test_font);
            assert_results_contain(
                &results,
                StatusCode::Fail,
                Some("unnecessary-tables".to_string()),
            );
            let tag_str = std::str::from_utf8(optional.as_slice()).unwrap();
            assert_messages_contain(&results, tag_str);
        }
    }

    #[test]
    fn test_unnecessary_tables_cff() {
        let mut testable = test_able("source-sans-pro/OTF/SourceSansPro-Black.otf");
        let mut unnecessary_table_tags = UNNECESSARY_TABLE_TAGS.to_vec();
        unnecessary_table_tags.extend_from_slice(&UNNECESSARY_TABLE_TAGS_CFF);

        for optional in unnecessary_table_tags.iter() {
            remove_table(&mut testable, optional);
        }

        for optional in unnecessary_table_tags.iter() {
            let mut test_font = testable.clone();
            add_table(&mut test_font, optional);
            let results = run_check(unnecessary_tables, test_font);
            assert_results_contain(
                &results,
                StatusCode::Fail,
                Some("unnecessary-tables".to_string()),
            );
            let tag_str = std::str::from_utf8(optional.as_slice()).unwrap();
            assert_messages_contain(&results, tag_str);
        }
    }
}
