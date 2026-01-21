use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "name/trailing_spaces",
    title = "Name table records must not have trailing spaces.",
    rationale = "This check ensures that no entries in the name table end in spaces;
                trailing spaces, particularly in font names, can be confusing to users.
                In most cases this can be fixed by removing trailing spaces from the
                metadata fields in the font editor.",
    proposal = "https://github.com/googlefonts/fontbakery/issues/2417",
    hotfix = fix_trailing_spaces
)]
fn trailing_spaces(f: &Testable, _context: &Context) -> CheckFnResult {
    let mut problems: Vec<Status> = vec![];

    if let Ok(name_table) = testfont!(f).font().name() {
        for name_record in name_table.name_record().iter() {
            let name_rec_info = format!("{:}/{:}/{:}/{:}",
                name_record.platform_id,
                name_record.encoding_id,
                name_record.language_id,
                name_record.name_id,
            );
            if name_record
                .string(name_table.string_data())
                .map(|s| s.to_string())
                .map(|s| s.trim_end() != s)
                .unwrap_or(false)
            {
                problems.push(Status::fail("trailing-space",&format!(
                    "Name table record {name_rec_info} has trailing spaces that must be removed:\n`{:}`",
                    name_record.string(name_table.string_data()).map_err(|_| FontspectorError::General("Error reading string".to_string()))?,
                )))
            }
            if name_record
                .string(name_table.string_data())
                .map(|s| s.to_string())
                .map(|s| s.contains("  "))
                .unwrap_or(false)
            {
                problems.push(Status::warn("double-spaces",&format!(
                    "Name table record {name_rec_info} has double spaces:\n`{:}`",
                    name_record.string(name_table.string_data()).map_err(|_| FontspectorError::General("Error reading string".to_string()))?,
                )))
            }
        }
    }
    return_result(problems)
}

fn fix_trailing_spaces(_f: &mut Testable) -> FixFnResult {
    Ok(false)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::{
        codetesting::{
            assert_messages_contain, assert_results_contain, run_check_with_config,
        },
        StatusCode, TestableType,
    };
    use fontspector_checkapi::{Testable};
    use fontations::skrifa::raw::types::NameId;
    use fontations::write::{
        tables::maxp::Maxp,
        tables::name::{Name, NameRecord},
        FontBuilder,
    };
    use std::collections::HashMap;

    #[test]
    fn test_trailing_spaces() {
        let mut builder = FontBuilder::new();
        builder.add_table(&Maxp::default()).unwrap();
        let mut name_table = Name::default();
        let mut new_records = Vec::new();
        let nids = [
            (0, "Copyright 2026 Foundry Name.  With double spaces.",),
        ];
        for (nid, s) in nids.iter() {
            let name_rec = NameRecord::new(3, 1, 1033, NameId::new(*nid), (*s).to_string().into());
            new_records.push(name_rec);
        }
        new_records.sort();
        name_table.name_record = new_records;
        builder.add_table(&name_table).unwrap();

        let testable = Testable::new_with_contents("demo.ttf", builder.build().clone());

        let results = run_check_with_config(
            super::trailing_spaces,
            TestableType::Single(&testable),
            HashMap::new(),
        );
        assert_messages_contain(&results, "has double spaces");
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("double-spaces".to_string()),
        );
    }
}
