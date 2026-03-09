use fontations::write::FontBuilder;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

const UNWANTED_TABLES: [&[u8; 4]; 23] = [
    b"acnt", b"ankr", b"bdat", b"bhed", b"bloc", b"bmap", b"bsln", b"EBSC", b"fdsc", b"feat",
    b"fond", b"gcid", b"just", b"kerx", b"lcar", b"ltag", b"mort", b"morx", b"opbd", b"prop",
    b"trak", b"xref", b"Zaph",
];

#[check(
    id = "unwanted_aat_tables",
    title = "Are there unwanted Apple tables?",
    rationale = "
        Apple's TrueType reference manual [1] describes SFNT tables not in the
        Microsoft OpenType specification [2] and these can sometimes sneak into final
        release files.

        This check ensures fonts only have OpenType tables.

        [1] https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6.html
        [2] https://docs.microsoft.com/en-us/typography/opentype/spec/
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/2190",
    hotfix = delete_unwanted_aat_tables
)]
fn unwanted_aat_tables(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);

    let mut found = vec![];
    for tag in UNWANTED_TABLES.iter() {
        if f.has_table(tag) {
            found.push(String::from_utf8(tag.to_vec()).map_err(|_| {
                FontspectorError::General(format!("Font tag '{:?}' wasn't UTF8?", tag.to_vec()))
            })?);
        }
    }
    let mut problems = vec![];
    if !found.is_empty() {
        let message = format!(
            "Unwanted AAT tables were found in the font and should be removed,\
                       either by fonttools/ttx or by editing them using the tool\
                       they're built with:\n\n{}",
            found.join("\n")
        );
        let mut status = Status::fail("has-unwanted-tables", &message);
        status.add_metadata(Metadata::FontProblem {
            message: message.clone(),
            context: Some(json!({
                "unwanted_tables": found,
                "table_count": found.len(),
            })),
        });
        problems.push(status);
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::{
        codetesting::{add_table, assert_pass, assert_results_contain, run_check, test_able},
        StatusCode,
    };

    #[test]
    fn test_unwanted_aat_tables_pass() {
        // Mada Regular has no unwanted AAT tables, should PASS
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(super::unwanted_aat_tables, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_unwanted_aat_tables_fail_each() {
        // Adding each unwanted AAT table one-by-one should trigger FAIL
        let unwanted_tables: Vec<&[u8; 4]> = vec![
            b"EBSC", b"Zaph", b"acnt", b"ankr", b"bdat", b"bhed", b"bloc", b"bmap", b"bsln",
            b"fdsc", b"feat", b"fond", b"gcid", b"just", b"kerx", b"lcar", b"ltag", b"mort",
            b"morx", b"opbd", b"prop", b"trak", b"xref",
        ];
        for unwanted in unwanted_tables {
            let mut testable = test_able("mada/Mada-Regular.ttf");
            add_table(&mut testable, unwanted);
            let results = run_check(super::unwanted_aat_tables, testable);
            assert_results_contain(
                &results,
                StatusCode::Fail,
                Some("has-unwanted-tables".to_string()),
            );
        }
    }
}

fn delete_unwanted_aat_tables(t: &mut Testable) -> FixFnResult {
    let f = testfont!(t);
    let mut new_font = FontBuilder::new();
    for table in f.font().table_directory.table_records() {
        let tag = table.tag.get();
        if !UNWANTED_TABLES.contains(&&tag.into_bytes()) {
            if let Some(table) = f.font().table_data(tag) {
                new_font.add_raw(tag, table);
            }
        }
    }
    let new_bytes = new_font.build();
    t.set(new_bytes);
    Ok(true)
}
