use fontations::skrifa::raw::types::NameId;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "name/italic_names",
    rationale = "
        This check ensures that several entries in the name table
        conform to the font's Upright or Italic style,
        namely IDs 1 & 2 as well as 16 & 17 if they're present.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/3666",
    title = "Check name table IDs 1, 2, 16, 17 to conform to Italic style."
)]
fn italic_names(t: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    let mut problems = vec![];
    let style = font.style();
    if let Some(style) = style {
        if !style.contains("Italic") {
            skip!("not-italic", "Font is not italic");
        }
    } else {
        skip!("not-italic", "Font is not italic");
    }
    if let Some(family_name) = font.get_name_entry_strings(NameId::FAMILY_NAME).next() {
        if family_name.contains("Italic") {
            let message = "Name ID 1 (Family Name) must not contain 'Italic'.";
            let mut status = Status::fail("bad-familyname", message);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "name".to_string(),
                field_name: Some("nameID 1".to_string()),
                actual: Some(json!(family_name)),
                expected: Some(json!(family_name.replace("Italic", ""))),
                message: message.to_string(),
            });
            problems.push(status);
        }
    }
    if let Some(subfamily_name) = font.get_name_entry_strings(NameId::SUBFAMILY_NAME).next() {
        if subfamily_name != "Italic" && subfamily_name != "Bold Italic" {
            let message = format!(
                "Name ID 2 (Subfamily Name) does not conform to specs. Only R/I/B/BI are allowed, found {subfamily_name}"
            );
            let mut status = Status::fail("bad-subfamilyname", &message);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "name".to_string(),
                field_name: Some("nameID 2".to_string()),
                actual: Some(json!(subfamily_name)),
                expected: Some(json!("Italic | Bold Italic")),
                message,
            });
            problems.push(status);
        }
    }
    if let Some(typo_family_name) = font
        .get_name_entry_strings(NameId::TYPOGRAPHIC_FAMILY_NAME)
        .next()
    {
        if typo_family_name.contains("Italic") {
            let message = "Name ID 16 (Typographic Family Name) must not contain 'Italic'.";
            let mut status = Status::fail("bad-typographicfamilyname", message);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "name".to_string(),
                field_name: Some("nameID 16".to_string()),
                actual: Some(json!(typo_family_name)),
                expected: Some(json!(typo_family_name.replace("Italic", ""))),
                message: message.to_string(),
            });
            problems.push(status);
        }
    }
    if let Some(typo_subfamily_name) = font
        .get_name_entry_strings(NameId::TYPOGRAPHIC_SUBFAMILY_NAME)
        .next()
    {
        if !typo_subfamily_name.ends_with("Italic") {
            let message = "Name ID 16 (Typographic Family Name) must contain 'Italic'.";
            let mut status = Status::fail("bad-typographicsubfamilyname", message);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "name".to_string(),
                field_name: Some("nameID 17".to_string()),
                actual: Some(json!(typo_subfamily_name)),
                expected: Some(json!(format!("{}Italic", typo_subfamily_name))),
                message: message.to_string(),
            });
            problems.push(status);
        }
    }
    return_result(problems)
}
