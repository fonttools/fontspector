use fontations::{skrifa::raw::types::Tag, write::FontBuilder};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

const UNWANTED_TABLES: [(Tag, &str); 16] = [
    (Tag::new(b"FFTM"), "Table contains redundant FontForge timestamp info"),
    (Tag::new(b"TTFA"), "Redundant TTFAutohint table"),
    (Tag::new(b"TSI0"), "Table contains data only used in VTT"),
    (Tag::new(b"TSI1"), "Table contains data only used in VTT"),
    (Tag::new(b"TSI2"), "Table contains data only used in VTT"),
    (Tag::new(b"TSI3"), "Table contains data only used in VTT"),
    (Tag::new(b"TSI5"), "Table contains data only used in VTT"),
    (Tag::new(b"TSIC"), "Table contains data only used in VTT"),
    (Tag::new(b"TSIV"), "Table contains data only used in VOLT"),
    (Tag::new(b"TSIP"), "Table contains data only used in VOLT"),
    (Tag::new(b"TSIS"), "Table contains data only used in VOLT"),
    (Tag::new(b"TSID"), "Table contains data only used in VOLT"),
    (Tag::new(b"TSIJ"), "Table contains data only used in VOLT"),
    (Tag::new(b"TSIB"), "Table contains data only used in VOLT"),
    (Tag::new(b"prop"), "Table used on AAT, Apple's OS X specific technology. Although Harfbuzz now has optional AAT support, new fonts should not be using that."),
    (Tag::new(b"Debg"), "FontTools debugging table."),
];

#[check(
    id = "unwanted_tables",
    title = "Are there unwanted tables?",
    rationale = "Some font editors store source data in their own SFNT tables, and these can sometimes sneak into final release files, which should only have OpenType spec tables.",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
    hotfix = delete_unwanted_tables
)]
fn unwanted_tables(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);

    let mut reasons = vec![];
    for (table, reason) in UNWANTED_TABLES.iter() {
        if f.font().table_data(*table).is_some() {
            reasons.push(format!("Table: `{table}` Reason: {reason}\n"));
        }
    }
    Ok(if !reasons.is_empty() {
        Status::just_one_fail(
            "unwanted-tables",
            &format!("Unwanted tables found:\n {}", reasons.join("\n")),
        )
    } else {
        Status::just_one_pass()
    })
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::{
        codetesting::{add_table, assert_pass, assert_results_contain, run_check, test_able},
        StatusCode,
    };

    #[test]
    fn test_unwanted_tables_pass() {
        // Mada Regular has no unwanted tables, should PASS
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(super::unwanted_tables, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_unwanted_tables_fail_each() {
        // Adding each unwanted table one-by-one should trigger FAIL
        let unwanted_tables: Vec<&[u8; 4]> = vec![
            b"FFTM", b"TTFA", b"TSI0", b"TSI1", b"TSI2", b"TSI3", b"TSI5", b"TSIC", b"TSIV",
            b"TSIP", b"TSIS", b"TSID", b"TSIJ", b"TSIB", b"prop", b"Debg",
        ];
        for unwanted in unwanted_tables {
            let mut testable = test_able("mada/Mada-Regular.ttf");
            add_table(&mut testable, unwanted);
            let results = run_check(super::unwanted_tables, testable);
            assert_results_contain(
                &results,
                StatusCode::Fail,
                Some("unwanted-tables".to_string()),
            );
        }
    }
}

fn delete_unwanted_tables(t: &mut Testable) -> FixFnResult {
    let f = testfont!(t);
    let unwanted_tags = UNWANTED_TABLES
        .iter()
        .map(|(tag, _)| tag)
        .collect::<Vec<_>>();
    let mut new_font = FontBuilder::new();
    for table in f.font().table_directory.table_records() {
        let tag = table.tag.get();
        if !unwanted_tags.contains(&&tag) {
            if let Some(table) = f.font().table_data(tag) {
                new_font.add_raw(tag, table);
            }
        }
    }
    let new_bytes = new_font.build();
    t.set(new_bytes);
    Ok(true)
}
