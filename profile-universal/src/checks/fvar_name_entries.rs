use fontations::skrifa::MetadataProvider;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "fvar_name_entries",
    rationale = "The purpose of this check is to make sure that all name entries referenced by variable font instances do exist in the name table.",
    proposal = "https://github.com/fonttools/fontbakery/issues/2069",
    title = "All name entries referenced by fvar instances exist on the name table?"
)]
fn fvar_name_entries(t: &Testable, _context: &Context) -> CheckFnResult {
    let mut problems: Vec<Status> = vec![];
    let f = testfont!(t);
    skip!(
        !f.is_variable_font(),
        "not-variable",
        "Font is not a variable font"
    );
    for instance in f.font().named_instances().iter() {
        let name_id = instance.subfamily_name_id();
        if f.font()
            .localized_strings(name_id)
            .english_or_first()
            .is_none()
        {
            let axes = f.font().axes();
            let loc = instance
                .user_coords()
                .zip(axes.iter())
                .map(|(c, a)| {
                    format!(
                        "{}={}",
                        f.font()
                            .localized_strings(a.name_id())
                            .english_or_first()
                            .map(|s| s.chars().collect::<String>())
                            .unwrap_or("????".to_string()),
                        c
                    )
                })
                .collect::<Vec<String>>();
            let message = format!(
                "Named instance with coordinates '{}' lacks an entry on the name table (nameID={}).",
                loc.join(" "),
                name_id
            );
            let mut status = Status::fail("missing-name", &message);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "fvar".to_string(),
                field_name: Some("instance nameID".to_string()),
                actual: Some(json!({ "nameID": name_id.to_u16(), "coordinates": loc })),
                expected: Some(json!("nameID should exist in name table")),
                message: message.clone(),
            });
            problems.push(status);
        }
    }
    return_result(problems)
}
