use edit_distance::edit_distance;
use fontations::{read::TableProvider, skrifa::string::StringId};
use fontspector_checkapi::{
    constants::STATIC_STYLE_NAMES, prelude::*, testfont, Choice, DialogField, DialogFieldType,
    FileTypeConvert, Metadata, TestFont,
};
use google_fonts_axisregistry::build_name_table;
use serde_json::json;
use tabled::builder::Builder;

use crate::{constants::gf_api_weight_name, utils::build_expected_font};

struct StaticChecker {
    family_name: String,
    subfamily_name: String,
    weight_class: u16,
}

impl StaticChecker {
    fn from_font(t: &TestFont) -> Result<Self, FontspectorError> {
        Ok(Self {
            family_name: t.best_familyname().unwrap_or("Unknown".to_string()),
            subfamily_name: t.best_subfamilyname().unwrap_or("Regular".to_string()),
            weight_class: t.font().os2()?.us_weight_class(),
        })
    }

    fn non_google_style(&self) -> bool {
        !STATIC_STYLE_NAMES.contains(&self.subfamily_name.as_str())
    }

    fn regular_italic(&self) -> bool {
        self.subfamily_name.to_lowercase() == "regular italic"
    }

    fn weightclass_suggests_style_name(&self) -> Option<&'static str> {
        if self.weight_class != 400 {
            Some(gf_api_weight_name(self.weight_class))
        } else {
            None
        }
    }

    fn maybe_typo(&self) -> Option<&'static str> {
        let mut closest = None;
        let mut min_distance = usize::MAX;
        for static_name in STATIC_STYLE_NAMES {
            let distance = edit_distance(&self.subfamily_name, static_name);
            if distance < min_distance {
                min_distance = distance;
                closest = Some(static_name);
            }
        }
        if min_distance <= 4 {
            closest
        } else {
            None
        }
    }

    fn more_info(&self) -> Option<MoreInfoRequest> {
        let mut choices = vec![];
        if !self.non_google_style() {
            return None;
        }

        choices.push(Choice {
            value: "promote".to_string(),
            description: format!(
                "Change family name to '{} {}' for this font",
                self.family_name, self.subfamily_name
            ),
        });
        if let Some(suggested_style_name) = self.weightclass_suggests_style_name() {
            choices.push(Choice {
                value: "change:".to_string() + suggested_style_name,
                description: format!(
                    "Change style name to '{}' based on OS/2 usWeightClass {}",
                    suggested_style_name, self.weight_class
                ),
            });
        }
        if let Some(closest) = self.maybe_typo() {
            choices.push(Choice {
                value: "change:".to_string() + closest,
                description: format!("Oops, it was a typo! Change style name to '{}'", closest),
            });
        }
        choices.push(Choice {
            value: "unsupported".to_string(),
            description: "Keep the unsupported style name".to_string(),
        });
        Some(MoreInfoRequest(vec![
            DialogField {
                key: "action".to_string(),
                prompt: format!("This is a static font with an unsupported style name {}.\n  What would you like to do?", self.subfamily_name),
                field_type: DialogFieldType::Choice(choices),
            }
        ]))
    }
}

const NAME_IDS: [(StringId, &str); 6] = [
    (StringId::FAMILY_NAME, "Family Name"),
    (StringId::SUBFAMILY_NAME, "Subfamily Name"),
    (StringId::FULL_NAME, "Full Name"),
    (StringId::POSTSCRIPT_NAME, "Postscript Name"),
    (StringId::TYPOGRAPHIC_FAMILY_NAME, "Typographic Family Name"),
    (
        StringId::TYPOGRAPHIC_SUBFAMILY_NAME,
        "Typographic Subfamily Name",
    ),
];
#[check(
    id = "googlefonts/font_names",
    rationale = "
        
        Google Fonts has several rules which need to be adhered to when
        setting a font's name table. Please read:
        https://googlefonts.github.io/gf-guide/statics.html#supported-styles
        https://googlefonts.github.io/gf-guide/statics.html#style-linking
        https://googlefonts.github.io/gf-guide/statics.html#unsupported-styles
        https://googlefonts.github.io/gf-guide/statics.html#single-weight-families
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/3800",
    title = "Check font names are correct",
    hotfix = fix_font_names,
)]
fn font_names(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    if f.has_axis("MORF") {
        let msg = "Font has a Morph axis";
        let mut status = Status::warn("morf-axis",
            "Font has a Morph axis. This check only works on fonts that have a wght axis. Since users can define their own stylenames for Morph families, please manually check that the family works on major platforms. You can use Agu Display as a reference."
        );
        status.add_metadata(Metadata::FontProblem {
            message: msg.to_string(),
            context: Some(json!({"axis": "MORF"})),
        });
        problems.push(status);
        return return_result(problems);
    }

    // If this is a static file, the subfamily name must be one of the
    // Google Fonts supported styles.
    if !f.is_variable_font() {
        let static_checker = StaticChecker::from_font(&f)?;
        if static_checker.non_google_style() {
            if static_checker.regular_italic() {
                // Common enough problem to special-case
                let msg = "Subfamily name should be 'Italic' not 'Regular Italic'";
                let mut status = Status::fail("regular-italic", msg);
                status.add_metadata(Metadata::TableProblem {
                    table_tag: "name".to_string(),
                    field_name: Some("subfamily name".to_string()),
                    message: "Subfamily name should be 'Italic' not 'Regular Italic'".to_string(),
                    actual: Some(json!(static_checker.subfamily_name.clone())),
                    expected: Some(json!("Italic")),
                });
                problems.push(status);
                return return_result(problems);
            }

            let Some(request) = static_checker.more_info() else {
                return return_result(problems);
            };
            let style_name = static_checker.subfamily_name.clone();

            // First check if we have a weight class that matches a supported style name, even if the style name itself doesn't match.
            // XBold might be a typo for Bold but it's more likely to be an ExtraBold if the weight class is 800, for example.
            if let Some(expected_style_name) = static_checker.weightclass_suggests_style_name() {
                let msg = format!(
                    "Unsupported style name '{}' for static font. OS/2 usWeightClass is {}, so suggested style name is '{}'.",
                    style_name,
                    static_checker.weight_class, expected_style_name
                );
                let mut status = Status::fail("unsupported-style", &msg);
                status.add_metadata(Metadata::TableProblem {
                    table_tag: "name".to_string(),
                    field_name: Some("subfamily name".to_string()),
                    message: "Style name does not match OS/2 usWeightClass".to_string(),
                    actual: Some(json!(static_checker.subfamily_name.clone())),
                    expected: Some(json!(expected_style_name)),
                });
                status.add_metadata(Metadata::FixNeedsMoreInformation(request));
                problems.push(status);
                return return_result(problems);
            }
            if let Some(closest) = static_checker.maybe_typo() {
                let msg =
                    format!("Unsupported style name '{style_name}' for static font. Did you mean '{closest}'?");
                let mut status = Status::fail("unsupported-style", &msg);
                status.add_metadata(Metadata::TableProblem {
                    table_tag: "name".to_string(),
                    field_name: Some("subfamily name".to_string()),
                    message: "Possibly mistyped style name".to_string(),
                    actual: Some(json!(static_checker.subfamily_name.clone())),
                    expected: Some(json!(closest)),
                });
                status.add_metadata(Metadata::FixNeedsMoreInformation(request));
                problems.push(status);
                return return_result(problems);
            } else {
                let msg = format!(
                    "Unsupported style name {} for static font. Expected one of {:?}, got '{}'.",
                    style_name,
                    STATIC_STYLE_NAMES,
                    static_checker.subfamily_name.clone()
                );
                let mut status = Status::fail("unsupported-style", &msg);
                status.add_metadata(Metadata::TableProblem {
                    table_tag: "name".to_string(),
                    field_name: Some("subfamily name".to_string()),
                    message: "Unsupported style name for static font".to_string(),
                    actual: Some(json!(static_checker.subfamily_name.clone())),
                    expected: Some(json!(STATIC_STYLE_NAMES)),
                });
                status.add_metadata(Metadata::FixNeedsMoreInformation(request));
                problems.push(status);
                return return_result(problems);
            }
        }
    }

    let expected_font_data = build_expected_font(&f, &[])?;
    let expected_font = TestFont::new_from_data(&t.filename, &expected_font_data).map_err(|e| {
        FontspectorError::General(format!("Couldn't build expected font from data: {e}"))
    })?;
    let mut ok = true;
    let mut md_table = Builder::new();
    md_table.push_record(vec!["Name", "Current", "Expected"]);
    let mut metadatas = vec![];

    for &(name_id, name) in NAME_IDS.iter() {
        let current = f.get_best_name(&[name_id]).unwrap_or("N/A".to_string());
        let expected = expected_font
            .get_best_name(&[name_id])
            .unwrap_or("N/A".to_string());

        let mut row = vec![name.to_string()];

        if name_id == StringId::FULL_NAME
            && expected.contains(" Regular")
            && current == expected.replace(" Regular", "")
        {
            let msg = "Regular missing from full name";
            let mut status = Status::warn("lacks-regular", msg);
            status.add_metadata(Metadata::FontProblem {
                message: msg.to_string(),
                context: Some(json!({
                    "current": current.clone(),
                    "expected": expected.clone()
                })),
            });
            problems.push(status);
        }
        if current != expected {
            row.push(format!("**{current}**"));
            row.push(format!("**{expected}**"));
            ok = false;
            metadatas.push(Metadata::TableProblem {
                table_tag: "name".to_string(),
                field_name: Some(name.to_string()),
                message: format!("Name table entry for {name} is incorrect"),
                actual: Some(json!(current.clone())),
                expected: Some(json!(expected.clone())),
            });
        } else {
            row.push(current);
            row.push(expected);
        }
        md_table.push_record(row);
    }

    if !ok {
        let mut status = Status::fail(
            "bad-names",
            &format!(
                "Font names are incorrect:\n\n{}",
                md_table.build().with(tabled::settings::Style::markdown())
            ),
        );
        status.metadata.extend(metadatas);
        problems.push(status);
    }
    return_result(problems)
}

fn fix_font_names(
    t: &mut Testable,
    replies: Option<MoreInfoReplies>,
) -> Result<FixResult, FontspectorError> {
    let f = testfont!(t);
    if f.has_axis("MORF") {
        return Ok(FixResult::Unfixable);
    }
    if f.is_variable_font() {
        let new_binary = build_name_table(f.font(), None, None, &[], None)
            .map_err(|e| FontspectorError::Fix(format!("Couldn't build name table: {e}")))?;
        t.set(new_binary);
        return Ok(FixResult::Fixed);
    }
    let static_checker = StaticChecker::from_font(&f)?;
    if let Some(replies) = replies {
        let action = replies
            .0
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if action == "unsupported" {
            return Ok(FixResult::Available);
        } else if action == "promote" {
            let family_name = f.best_familyname().unwrap_or("Unknown".to_string());
            let new_family_name = format!("{} {}", family_name, static_checker.subfamily_name);
            let new_font =
                build_name_table(f.font(), Some(&new_family_name), Some("Regular"), &[], None)
                    .map_err(|e| {
                        FontspectorError::Fix(format!(
                            "Couldn't build name table with promoted family name: {e}"
                        ))
                    })?;
            t.set(new_font);
            return Ok(FixResult::Fixed);
        } else if action.starts_with("change:") {
            let new_style_name = action.strip_prefix("change:").unwrap_or("");
            let new_font = build_name_table(f.font(), None, Some(new_style_name), &[], None)
                .map_err(|e| FontspectorError::Fix(format!("Couldn't build name table: {e}")))?;
            t.set(new_font);
            return Ok(FixResult::Fixed);
        }
    }
    if static_checker.regular_italic() {
        let new_font = build_name_table(f.font(), None, Some("Italic"), &[], None)
            .map_err(|e| FontspectorError::Fix(format!("Couldn't build name table: {e}")))?;
        t.set(new_font);
        return Ok(FixResult::Fixed);
    }
    if let Some(more_info) = static_checker.more_info() {
        Ok(FixResult::MoreInfoNeeded(more_info))
    } else {
        // Apparently we know how to fix it
        let new_binary = build_name_table(f.font(), None, None, &[], None)
            .map_err(|e| FontspectorError::Fix(format!("Couldn't build name table: {e}")))?;
        t.set(new_binary);
        Ok(FixResult::Fixed)
    }
}
