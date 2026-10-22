use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use skrifa::raw::TableProvider;
use std::vec;

fn get_preferred_width_name(width_class: u16) -> Option<Vec<&'static str>> {
    match width_class {
        1 => Some(vec!["ExtraCompressed", "ExtraComp", "XComp", "XCm"]),
        2 => Some(vec!["Compressed", "Comp", "Cm"]),
        3 => Some(vec!["Condensed", "Cond", "Cn"]),
        4 => Some(vec!["SemiCondensed", "SemiCond", "SmCond", "SmCn"]),
        5 => Some(vec!["Normal"]),
        6 => Some(vec!["SemiExtended", "SemiExt", "SmExt"]),
        7 => Some(vec!["Extended", "Ext"]),
        8 => Some(vec!["Wide", "Wd"]),
        9 => Some(vec!["ExtraWide", "XWide", "XtraWd", "XWd"]),
        _ => None,
    }
}

fn get_other_width_name(width_class: u16) -> Option<Vec<&'static str>> {
    match width_class {
        1 => Some(vec![
            "ExtraCompressed",
            "XXCond",
            "Ultra-Condensed",
            "Ultra-Cond",
            "XCm",
            "XComp",
            "ExtraComp",
        ]),
        2 => Some(vec![
            "Compressed",
            "XCond",
            "Extra-Condensed",
            "Extra-Cond",
            "Cm",
            "Comp",
        ]),
        3 => Some(vec!["Condensed", "Cond", "Cn"]),
        4 => Some(vec![
            "SemiCondensed",
            "SemiCond",
            "Semi-Cond",
            "Semi-Condensed",
            "SmCond",
            "SmCn",
            "Narrow",
            "Nar",
        ]),
        5 => Some(vec!["Normal"]),
        6 => Some(vec![
            "SemiExtended",
            "SemiWide",
            "Semi-Wide",
            "Semi-Expanded",
            "SemiExt",
            "SmExt",
        ]),
        7 => Some(vec!["Extended", "Wide", "Expanded", "Ext"]),
        8 => Some(vec![
            "Wide",
            "XWide",
            "Extra-Wide",
            "Extra-Expanded",
            "Wd",
            "ExtraExpanded",
            "Extra-expanded",
            "XExpanded",
            "ExtraExp",
            "XExp",
            "ExtraExtended",
            "XExtended",
            "ExtraExt",
            "XExt",
        ]),
        9 => Some(vec![
            "ExtraWide",
            "XtraWd",
            "XWd",
            "XXWide",
            "Ultra-Wide",
            "Ultra-Expanded",
            "UltraExpanded",
            "Ultra-expanded",
            "UExpanded",
            "UltraExp",
            "UExp",
        ]),
        _ => None,
    }
}

fn get_custom_width_name(
    width_class: u16,
    config: &serde_json::Map<String, serde_json::Value>,
) -> Option<Vec<&str>> {
    // config looks like:
    // {
    //     "1": ["ExtraCompressed", "XXCond", "Ultra-Condensed", ...],
    //     "2": ["Compressed", "XCond", "Extra-Condensed", ...],
    //     ...
    // }
    if !(1..=9).contains(&width_class) {
        return None;
    }

    config
        .get(&width_class.to_string())?
        .as_array()?
        .iter()
        .map(serde_json::Value::as_str)
        .collect()
}

#[check(
    id = "monotype/widthclass",
    rationale = "
        We expect the following OS/2 usWidthClass values:

        ExtraCompressed 1
        Compressed 2
        Condensed 3
        SemiCondensed 4
        (Normal) 5
        SemiExtended 6
        Extended 7
        Wide 8
        ExtraWide 9
    ",
    proposal = "https://github.com/Monotype/fontspector/issues/1",
    title = "Check the OS/2 usWidthClass is appropriate for the font's best SubFamily name."
)]
fn widthclass(t: &Testable, context: &Context) -> CheckFnResult {
    let local_config = context.local_config("monotype/widthclass");
    let config = local_config.as_object();

    if let Some(config) = config {
        // If there is a local configuration, we could potentially validate it here.
        // the config must be a valid JSON object with keys "1" through "9" mapping to arrays of width names.
        for i in 1..=9 {
            if !config.contains_key(&i.to_string()) {
                return Ok(Status::just_one_fail(
                    "invalid-config",
                    &format!("Missing key '{i}' in local configuration for monotype/widthclass.",),
                ));
            }
        }
    }

    let f = testfont!(t);
    let value = f.font().os2()?.us_width_class();
    let best_family_name = if let Some(fam_name) = f.best_familyname() {
        fam_name
    } else {
        return Ok(Status::just_one_fail(
            "missing-family-name",
            "Could not determine the family name of the font.",
        ));
    };
    let best_subfamily_name = if let Some(sub_name) = f.best_subfamilyname() {
        sub_name
    } else {
        return Ok(Status::just_one_fail(
            "missing-subfamily-name",
            "Could not determine the subfamily name of the font.",
        ));
    };
    let best_full_name = format!("{} {}", best_family_name, best_subfamily_name);
    let style_name_parts = best_full_name.split(' ').collect::<Vec<_>>();
    let expected_width_names = match config {
        Some(config) => get_custom_width_name(value, config),
        None => get_preferred_width_name(value),
    };
    let expected_other_names = get_other_width_name(value);

    if value == 5 && is_normal_width(&best_full_name) {
        return Ok(Status::just_one_pass());
    }

    if let Some(expected_names) = expected_width_names {
        for width_name in &expected_names {
            if style_name_parts.contains(width_name) {
                return Ok(Status::just_one_pass());
            }
        }
        if let Some(other_names) = expected_other_names {
            for other_width_name in &other_names {
                if style_name_parts.contains(other_width_name) {
                    return Ok(Status::just_one_warn(
                        "width-class-name-value-mismatch-other", 
                        &format!(
                            "For OS/2 usWidthClass {value} we expect {expected_names:?}, but got '{best_full_name}'."
                        )
                    ));
                }
            }
        }
        Ok(Status::just_one_fail(
            "width-class-name-value-mismatch", 
            &format!(
                "For OS/2 usWidthClass {value} we expect {expected_names:?}, but got '{best_full_name}'. Either usWidthClass is wrong or style name. Please investigate."
            )
        ))
    } else {
        Ok(Status::just_one_fail(
            "bad-width-class-value",
            &format!(
                "OS/2 usWidthClass {value} is not in the range allowed by the OpenType spec (1-9)."
            ),
        ))
    }
}

fn is_normal_width(full_name: &str) -> bool {
    let full_name_lower = full_name.to_lowercase();

    // if any width is in the style name, it's not regular
    let non_normal_indicators = [
        "cond",   // includes XXCond, XCond, Cond, SemiCond
        "wide",   // includes xwide, xxwide, extra-wide, ultra-wide
        "expand", // includes extra-expanded, ultra-expanded
        "extend", // includes extra-extended, ultra-extended
        "comp",   // includes Compressed, UltraCompressed, ExtraCompressed, XComp, SemiComp, ...
        "cm",     // includes XCm, Cm
        "cn",     // includes XCn, Cn
        "wd",     // includes Wide, XWide, XXWide, ExtraWide, UltraWide
                  // "ex",  // This does not work, beacuae it matches with Extra like in ExtraLight
                  // TODO: Monotype name_collector
    ];

    for indicator in non_normal_indicators.iter() {
        if full_name_lower.contains(indicator) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use fontspector_checkapi::{Context, Testable};
    use std::collections::HashMap;
    use write_fonts::{
        tables::{
            maxp::Maxp,
            name::{Name, NameRecord},
            os2::Os2,
        },
        types::NameId,
        FontBuilder,
    };

    #[test]
    fn test_widthclass_invalid_config() {
        let width_tests = [(
            1,
            "A Family Name",
            "CustomWidthXXCondensed Hairline",
            Some("Missing key '3' in local configuration for monotype/widthclass.".to_string()),
        )];
        for (width_class_value, family_name, style_name, expected_result) in width_tests {
            let mut font_builder = FontBuilder::new();
            let maxp = Maxp::default();
            font_builder.add_table(&maxp).unwrap();

            let os2: Os2 = Os2 {
                us_width_class: width_class_value,
                ..Default::default()
            };
            font_builder.add_table(&os2).unwrap();

            let mut name: Name = Name::default();
            let mut new_records = Vec::new();
            // english default 3/1/1033
            let name_rec_fam =
                NameRecord::new(3, 1, 1033, NameId::new(16), family_name.to_string().into());
            new_records.push(name_rec_fam);
            let name_rec_sub =
                NameRecord::new(3, 1, 1033, NameId::new(17), style_name.to_string().into());
            new_records.push(name_rec_sub);
            new_records.sort();
            name.name_record = new_records;
            font_builder.add_table(&name).unwrap();

            let font = font_builder.build();

            let testable = Testable::new_with_contents("demo.otf", font);
            let conf = HashMap::from([(
                "monotype/widthclass".to_string(),
                serde_json::json!({
                    "1": ["CustomWidthXXCondensed"],
                    "2": ["XCond", "XCondensed"],
                    // missing entries for width classes 3 through 9
                }),
            )]);
            let context = Context {
                configuration: conf.clone(),
                ..Default::default()
            };
            let result = widthclass_impl(&testable, &context)
                .unwrap()
                .next()
                .unwrap();

            assert_eq!(result.message, expected_result);
        }
    }

    #[test]
    fn test_widthclass_config() {
        // possible config for the "monotype/widthclass" check might look like this:
        // .with_configuration_defaults(
        //     "monotype/widthclass",
        //     HashMap::from([
        //         ("1".to_string(), json!(["XXCond", "XXCondensed"])),
        //         ("2".to_string(), json!(["XCond", "XCondensed"])),
        //         ("3".to_string(), json!(["Cond", "Condensed"])),
        //         ("4".to_string(), json!(["SemiCond", "SemiCondensed"])),
        //         ("5".to_string(), json!([])),
        //         ("6".to_string(), json!(["SemiWide"])),
        //         ("7".to_string(), json!(["Wide"])),
        //         ("8".to_string(), json!(["XWide"])),
        //         ("9".to_string(), json!(["XXWide"])),
        //         ]),
        // )

        let width_tests = [
            (1, "A Family Name", "CustomWidthXXCondensed Hairline", None),
            (8, "A Family Name", "XWide Hairline", Some("For OS/2 usWidthClass 8 we expect [\"CustomWide\"], but got 'A Family Name XWide Hairline'.".to_string())),
            (9, "A Family Name", "ExtraExtraWide Hairline", None),
            ];
        for (width_class_value, family_name, style_name, expected_result) in width_tests {
            let mut font_builder = FontBuilder::new();
            let maxp = Maxp::default();
            font_builder.add_table(&maxp).unwrap();

            let os2: Os2 = Os2 {
                us_width_class: width_class_value,
                ..Default::default()
            };
            font_builder.add_table(&os2).unwrap();

            let mut name: Name = Name::default();
            let mut new_records = Vec::new();
            // english default 3/1/1033
            let name_rec_fam =
                NameRecord::new(3, 1, 1033, NameId::new(16), family_name.to_string().into());
            new_records.push(name_rec_fam);
            let name_rec_sub =
                NameRecord::new(3, 1, 1033, NameId::new(17), style_name.to_string().into());
            new_records.push(name_rec_sub);
            new_records.sort();
            name.name_record = new_records;
            font_builder.add_table(&name).unwrap();

            let font = font_builder.build();

            let testable = Testable::new_with_contents("demo.otf", font);
            let conf = HashMap::from([(
                "monotype/widthclass".to_string(),
                serde_json::json!({
                    "1": ["CustomWidthXXCondensed"],
                    "2": ["XCond", "XCondensed"],
                    "3": ["Cond", "Condensed"],
                    "4": ["SemiCond", "SemiCondensed"],
                    "5": ["Normal"],
                    "6": ["SemiWide"],
                    "7": ["Wide"],
                    "8": ["CustomWide"],
                    "9": ["ExtraExtraWide"],
                }),
            )]);
            let context = Context {
                configuration: conf.clone(),
                ..Default::default()
            };
            let result = widthclass_impl(&testable, &context)
                .unwrap()
                .next()
                .unwrap();

            assert_eq!(result.message, expected_result);
        }
    }

    #[test]
    fn test_widthclass() {
        let width_tests = [
            (5, "A Family Name", "Hairline", None),
            (5, "A Family Name", "Cond Regular", Some("For OS/2 usWidthClass 5 we expect [\"Normal\"], but got 'A Family Name Cond Regular'. Either usWidthClass is wrong or style name. Please investigate.".to_string())),
            (3, "A Family Name", "Condensed Black", None),
            // (2, "A Family Name", "XCond SemiBold Italic", None),
            // (5, "A Family Name", "XCond SemiBold Italic", Some("For OS/2 usWidthClass 5 we expect [\"Normal\"], but got 'A Family Name XCond SemiBold Italic'. Either usWidthClass is wrong or style name. Please investigate.".to_string())),
            (6, "A Family Name", "Semi-Wide SemiBold Italic", Some("For OS/2 usWidthClass 6 we expect [\"SemiExtended\", \"SemiExt\", \"SmExt\"], but got 'A Family Name Semi-Wide SemiBold Italic'.".to_string())),
            (7, "A Family Name", "Semi-Wide SemiBold Italic", Some("For OS/2 usWidthClass 7 we expect [\"Extended\", \"Ext\"], but got 'A Family Name Semi-Wide SemiBold Italic'. Either usWidthClass is wrong or style name. Please investigate.".to_string())),
            (5, "A Family Name", "Whatever Thin", None),
            (5, "A Family Name", "ExtraLight", None),
            (5, "A Family Name", "XLight", None),
            (5, "A Family Name", "Light", None),
            (5, "A Family Name", "XBlack", None),
            (5, "A Family Name", "Italic", None),
            (5, "A Family Name", "SemiLight", None),
            (5, "A Family Name", "SemiLight Italic", None),
            (3, "A Family Name", "Cond Italic", None),
            (3, "A Family Name", "Cond Regular Italic", None),
            (4, "A Family Name", "Cond Regular Italic", Some("For OS/2 usWidthClass 4 we expect [\"SemiCondensed\", \"SemiCond\", \"SmCond\", \"SmCn\"], but got 'A Family Name Cond Regular Italic'. Either usWidthClass is wrong or style name. Please investigate.".to_string())),
            (10, "A Family Name", "XXWide", Some("OS/2 usWidthClass 10 is not in the range allowed by the OpenType spec (1-9).".to_string())),
            (3, "A Family Name Cond", "Bold", None),
            (7, "A Family Name Wide", "Bold", Some("For OS/2 usWidthClass 7 we expect [\"Extended\", \"Ext\"], but got 'A Family Name Wide Bold'.".to_string())),
            // Monotype specific width names
            (1, "A Family Name", "ExtraCompressed Bold", None),
            (2, "A Family Name", "Compressed Bold", None),
            (3, "A Family Name", "Condensed Bold", None),
            (4, "A Family Name", "SemiCondensed Bold", None),
            (5, "A Family Name", "Bold", None),
            (6, "A Family Name", "SemiExtended Bold", None),
            (7, "A Family Name", "Extended Bold", None),
            (8, "A Family Name", "Wide Bold", None),
            (9, "A Family Name", "ExtraWide Bold", None),
            // add edge cases for width classes 1
            (1, "A Family Name", "XCm Bold", None),
            (1, "A Family Name", "XComp Bold", None),
            (1, "A Family Name", "ExtraComp Bold", None),
            // add edge cases for width classes 2
            (2, "A Family Name", "Comp Bold", None),
            (2, "A Family Name", "Cm Bold", None),
            // add edge cases for width classes 3
            (3, "A Family Name", "Cn Bold", None),
            // add edge cases for width classes 4
            (4, "A Family Name", "SmCond Bold", None),
            (4, "A Family Name", "SmCn Bold", None),
            // (5, "A Family Name", "SmCn Bold", Some("For OS/2 usWidthClass 5 we expect [\"Normal\"], but got 'A Family Name SmCn Bold'. Either usWidthClass is wrong or style name. Please investigate.".to_string())),
            // add edge cases for width classes 6
            (6, "A Family Name", "SemiExt Bold", None),
            (6, "A Family Name", "SmExt Bold", None),
            // add edge cases for width classes 7
            (7, "A Family Name", "Ext Bold", None),
            // add edge cases for width classes 8
            (8, "A Family Name", "Wd Bold", None),
            // add edge cases for width classes 9
            (9, "A Family Name", "XtraWd Bold", None),
            (9, "A Family Name", "XWd Bold", None),
            ];
        for (width_class_value, family_name, style_name, expected_result) in width_tests {
            let mut font_builder = FontBuilder::new();
            let maxp = Maxp::default();
            font_builder.add_table(&maxp).unwrap();

            let os2: Os2 = Os2 {
                us_width_class: width_class_value,
                ..Default::default()
            };
            font_builder.add_table(&os2).unwrap();

            let mut name: Name = Name::default();
            let mut new_records = Vec::new();
            // english default 3/1/1033
            let name_rec_fam =
                NameRecord::new(3, 1, 1033, NameId::new(16), family_name.to_string().into());
            new_records.push(name_rec_fam);
            let name_rec_sub =
                NameRecord::new(3, 1, 1033, NameId::new(17), style_name.to_string().into());
            new_records.push(name_rec_sub);
            new_records.sort();
            name.name_record = new_records;
            font_builder.add_table(&name).unwrap();

            let font = font_builder.build();

            let testable = Testable::new_with_contents("demo.otf", font);
            let context = Context {
                ..Default::default()
            };
            let result = widthclass_impl(&testable, &context)
                .unwrap()
                .next()
                .unwrap();

            assert_eq!(result.message, expected_result);
        }
    }
}
