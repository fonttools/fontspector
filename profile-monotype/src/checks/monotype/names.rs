use fontspector_checkapi::{
    get_name_entry_string, get_name_platform_tuples, prelude::*, skip, testfont, FileTypeConvert,
    Metadata, PlatformSelector,
};
use serde_json::json;
use skrifa::raw::types::NameId;

#[check(
    id = "monotype/style_in_family_name",
    rationale = "
        Is name ID 1 free of the reserved keywords 'Regular','Bold' and 'Italic'?

        Breaks down into two checks:

            * Does name ID 1 contain a reserved keyword 'Bold' or 'Italic'? => WARN
        
            * If either keyword is in name ID 1, is it repeated in name ID 2 => FAIL

    ",
    title = "Checking that name ID 1 does not contain reserved style names."
)]
fn style_in_family_name(t: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    skip!(!font.has_table(b"name"), "no-name", "No name table.");

    let mut problems = vec![];
    let reserved_style_names = ["Regular", "Bold", "Italic"];

    for style_name in reserved_style_names.iter() {
        let platform_tuples = get_name_platform_tuples(font.font());
        for platform_tuple in platform_tuples {
            let selector = PlatformSelector {
                platform_id: platform_tuple.0,
                encoding_id: platform_tuple.1,
                language_id: platform_tuple.2,
            };
            let family_name = if let Some(name_string) =
                get_name_entry_string(&font.font(), selector.clone(), NameId::FAMILY_NAME)
            {
                name_string.to_string()
            } else {
                continue;
            };
            let subfamily_name = if let Some(name_string) =
                get_name_entry_string(&font.font(), selector, NameId::SUBFAMILY_NAME)
            {
                name_string.to_string()
            } else {
                continue;
            };
            let message = format!("Name ID 1 (Family Name, {platform_tuple:?}) should not contain '{}'. This breaks Style-linking and fonts may not work properly in MS applications.", style_name);
            let mut status;
            if family_name.contains(style_name) && subfamily_name.contains(style_name) {
                // If either keyword is in name ID 1, is it repeated in name ID 2 => FAIL
                status = Status::fail("style-in-family-name-and-subfamily", &message);
            } else if family_name.contains(style_name) && !subfamily_name.contains(style_name) {
                // Does name ID 1 contain a reserved keyword 'Bold' or 'Italic'? => WARN
                status = Status::warn("style-in-family-name", &message);
            } else {
                continue;
            }
            status.add_metadata(Metadata::TableProblem {
                table_tag: "name".to_string(),
                field_name: Some("nameID 1".to_string()),
                actual: Some(json!(family_name)),
                expected: Some(json!(family_name.replace(style_name, ""))),
                message: message.to_string(),
            });
            problems.push(status);
        }
    }

    return_result(problems)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fontspector_checkapi::{
        codetesting::{assert_results_contain, run_check},
        StatusCode, Testable,
    };
    use std::collections::HashMap;
    use write_fonts::{
        tables::{
            maxp::Maxp,
            name::{Name, NameRecord},
        },
        types::NameId,
        FontBuilder,
    };

    #[test]
    fn test_style_in_family_name() {
        let test_examples = [
            (
                "Family-Bold.ttf".to_string(),
                HashMap::from([
                    (NameId::FAMILY_NAME, "Family Bold"),
                    (NameId::SUBFAMILY_NAME, "Bold"),
                ]),
                StatusCode::Fail,
                Some("style-in-family-name-and-subfamily".to_string()),
            ),
            (
                "Family-Italic.ttf".to_string(),
                HashMap::from([
                    (NameId::FAMILY_NAME, "Family Italic"),
                    (NameId::SUBFAMILY_NAME, "Italic"),
                ]),
                StatusCode::Fail,
                Some("style-in-family-name-and-subfamily".to_string()),
            ),
            (
                "Family-Italic.ttf".to_string(),
                HashMap::from([
                    (NameId::FAMILY_NAME, "Family Italic"),
                    (NameId::SUBFAMILY_NAME, "Regular"),
                ]),
                StatusCode::Warn,
                Some("style-in-family-name".to_string()),
            ),
            (
                "Family-Italic.ttf".to_string(),
                HashMap::from([
                    (NameId::FAMILY_NAME, "Family"),
                    (NameId::SUBFAMILY_NAME, "Italic"),
                ]),
                StatusCode::Pass,
                None,
            ),
            (
                "Family-Italic.ttf".to_string(),
                HashMap::from([
                    (NameId::FAMILY_NAME, "Family"),
                    (NameId::SUBFAMILY_NAME, "Bold Italic"),
                ]),
                StatusCode::Pass,
                None,
            ),
            (
                "Family-Italic.ttf".to_string(),
                HashMap::from([
                    (NameId::FAMILY_NAME, "Family Bold"),
                    (NameId::SUBFAMILY_NAME, "Bold Italic"),
                ]),
                StatusCode::Fail,
                Some("style-in-family-name-and-subfamily".to_string()),
            ),
            (
                "Family-Regular.ttf".to_string(),
                HashMap::from([
                    (NameId::FAMILY_NAME, "Family Regular"),
                    (NameId::SUBFAMILY_NAME, "Regular"),
                ]),
                StatusCode::Fail,
                Some("style-in-family-name-and-subfamily".to_string()),
            ),
        ];
        for (filename, name_ids, expected_severity, expected_code) in test_examples {
            let mut builder = FontBuilder::new();
            builder.add_table(&Maxp::default()).unwrap();

            let mut name_table = Name::default();
            let mut new_records = Vec::new();

            for (nid, s) in name_ids.iter() {
                let name_rec = NameRecord::new(3, 1, 1033, *nid, (*s).to_string().into());
                new_records.push(name_rec);
            }

            new_records.sort();
            name_table.name_record = new_records;
            builder.add_table(&name_table).unwrap();

            let testable = Testable::new_with_contents(filename, builder.build().clone());

            let result = run_check(style_in_family_name, testable);
            assert_results_contain(&result, expected_severity, expected_code);
        }
    }
}
