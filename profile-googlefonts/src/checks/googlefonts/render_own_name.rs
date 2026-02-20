use fontations::skrifa::{raw::tables::name::NameId, MetadataProvider};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "googlefonts/render_own_name",
    title = "Ensure font can render its own name.",
    rationale = "
        A base expectation is that a font family's regular/default (400 roman) style
        can render its 'menu name' (nameID 1) in itself.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/3159"
)]
fn render_own_name(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let name = f
        .font()
        .localized_strings(NameId::FAMILY_NAME)
        .english_or_first()
        .ok_or(FontspectorError::General(
            "Family name not found".to_string(),
        ))?;
    let name_string = name.chars().collect::<String>();
    let codepoints = f.codepoints(Some(context));
    let mut problems = vec![];
    let missing_chars: Vec<char> = name
        .chars()
        .filter(|c| !codepoints.contains(&(*c as u32)))
        .collect();
    if !missing_chars.is_empty() {
        let msg = format!(
            ".notdef glyphs were found when attempting to render {}",
            name_string
        );
        let mut status = Status::fail("render-own-name", &msg);
        status.add_metadata(Metadata::FontProblem {
            message: msg.clone(),
            context: Some(json!({
                "family_name": name_string,
                "missing_characters": missing_chars.iter().map(|c| format!("U+{:04X} ({})", *c as u32, c)).collect::<Vec<_>>()
            })),
        });
        problems.push(status);
    }
    return_result(problems)
}
