use fontations::skrifa::raw::{tables::os2::SelectionFlags, types::NameId, TableProvider};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "opentype/fsselection_wws",
    rationale = "
        According to the OpenType specification, OS/2.fsSelection bit 8 (WWS)
        should be set if the font has name table strings consistent with a
        weight/width/slope family without requiring use of name IDs 21 and 22.

        Conversely, if name IDs 21 and 22 are present (indicating the font
        names are not WWS-conformant), the WWS bit should not be set.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/577",
    title = "Check that OS/2 fsSelection WWS bit is set correctly."
)]
fn fsselection_wws(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let fs_flags = f.font().os2()?.fs_selection();
    let wws_set = fs_flags.contains(SelectionFlags::WWS);
    let has_wws_family = f
        .get_name_entry_strings(NameId::WWS_FAMILY_NAME)
        .next()
        .is_some();
    let has_wws_subfamily = f
        .get_name_entry_strings(NameId::WWS_SUBFAMILY_NAME)
        .next()
        .is_some();
    let has_wws_names = has_wws_family || has_wws_subfamily;

    let mut problems = vec![];

    if wws_set && has_wws_names {
        let message = "OS/2 fsSelection WWS bit is set, but the font has name IDs 21/22 \
             (WWS Family/Subfamily). The WWS bit should only be set when the \
             font's naming is already WWS-conformant without needing IDs 21/22.";
        let mut status = Status::warn("wws-with-wws-names", message);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "OS/2".to_string(),
            field_name: Some("fsSelection".to_string()),
            actual: Some(json!({
                "wws_bit": true,
                "has_name_id_21": has_wws_family,
                "has_name_id_22": has_wws_subfamily,
            })),
            expected: Some(json!({
                "wws_bit": false,
            })),
            message: message.to_string(),
        });
        problems.push(status);
    }

    if !wws_set && !has_wws_names {
        let message = "OS/2 fsSelection WWS bit is not set, and the font does not have \
             name IDs 21/22 (WWS Family/Subfamily). If the font's naming is \
             WWS-conformant, the WWS bit should be set.";
        let mut status = Status::warn("no-wws-without-wws-names", message);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "OS/2".to_string(),
            field_name: Some("fsSelection".to_string()),
            actual: Some(json!({
                "wws_bit": false,
                "has_name_id_21": false,
                "has_name_id_22": false,
            })),
            expected: Some(json!({
                "wws_bit": true,
            })),
            message: message.to_string(),
        });
        problems.push(status);
    }

    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontations::skrifa::raw::tables::os2::SelectionFlags;
    use fontations::write::{
        tables::{
            maxp::Maxp,
            name::{Name, NameRecord},
            os2::Os2,
        },
        types::NameId,
        FontBuilder,
    };
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check},
        StatusCode, Testable,
    };

    use super::fsselection_wws;

    /// Build a minimal font with given fsSelection flags and optional WWS name IDs.
    fn build_test_font(fs_selection: SelectionFlags, add_wws_names: bool) -> Testable {
        let mut builder = FontBuilder::new();
        builder.add_table(&Maxp::default()).unwrap();

        let os2 = Os2 {
            fs_selection,
            ..Default::default()
        };
        builder.add_table(&os2).unwrap();

        let mut name_table = Name::default();
        let mut records = Vec::new();

        // Always add basic name records
        records.push(NameRecord::new(
            3,
            1,
            1033,
            NameId::new(1),
            "Test Family".to_string().into(),
        ));
        records.push(NameRecord::new(
            3,
            1,
            1033,
            NameId::new(2),
            "Regular".to_string().into(),
        ));

        if add_wws_names {
            // Name ID 21 = WWS Family Name
            records.push(NameRecord::new(
                3,
                1,
                1033,
                NameId::new(21),
                "Test Family WWS".to_string().into(),
            ));
            // Name ID 22 = WWS Subfamily Name
            records.push(NameRecord::new(
                3,
                1,
                1033,
                NameId::new(22),
                "Regular".to_string().into(),
            ));
        }

        records.sort();
        name_table.name_record = records;
        builder.add_table(&name_table).unwrap();

        Testable::new_with_contents("demo.ttf", builder.build().clone())
    }

    #[test]
    fn test_wws_set_no_wws_names_passes() {
        // WWS bit set, no name IDs 21/22 => correct, should pass
        let testable = build_test_font(SelectionFlags::WWS, false);
        let results = run_check(fsselection_wws, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_no_wws_with_wws_names_passes() {
        // WWS bit not set, name IDs 21/22 present => correct, should pass
        let testable = build_test_font(SelectionFlags::empty(), true);
        let results = run_check(fsselection_wws, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_wws_set_with_wws_names_warns() {
        // WWS bit set AND name IDs 21/22 present => inconsistent, should warn
        let testable = build_test_font(SelectionFlags::WWS, true);
        let results = run_check(fsselection_wws, testable);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("wws-with-wws-names".to_string()),
        );
    }

    #[test]
    fn test_no_wws_no_wws_names_warns() {
        // WWS bit not set AND no name IDs 21/22 => inconsistent, should warn
        let testable = build_test_font(SelectionFlags::empty(), false);
        let results = run_check(fsselection_wws, testable);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("no-wws-without-wws-names".to_string()),
        );
    }
}
