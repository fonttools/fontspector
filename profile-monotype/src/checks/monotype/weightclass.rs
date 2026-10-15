use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use skrifa::raw::TableProvider;
use std::vec;

fn get_preferred_weight_name(weight_class: u16) -> Option<Vec<&'static str>> {
    match weight_class {
        50 => Some(vec!["Hairline", "Hair", "Hl"]),
        100 => Some(vec!["Thin", "Th"]),
        200 => Some(vec!["ExtraLight", "XLight", "XtraLt", "XLt"]),
        300 => Some(vec!["Light", "Lt"]),
        350 => Some(vec!["SemiLight", "SmLt"]),
        400 => Some(vec!["Regular", "Reg"]),
        500 => Some(vec!["Medium", "Md", "Med"]),
        600 => Some(vec!["SemiBold", "SmBd"]),
        700 => Some(vec!["Bold", "Bd"]),
        800 => Some(vec!["ExtraBold", "XBold", "XtraBold", "XBd"]),
        900 => Some(vec!["Black", "Blk", "Bl"]),
        950 => Some(vec!["ExtraBlack", "XBlack", "XtraBlack", "XBlk", "XBl"]),
        _ => None,
    }
}

fn get_other_weight_name(weight_class: u16) -> Option<Vec<&'static str>> {
    match weight_class {
        100 => Some(vec!["UltraLight", "Ultra-Light", "UltraLt", "ULt"]),
        350 => Some(vec!["Book"]),
        400 => Some(vec!["Book"]),
        900 => Some(vec!["Heavy", "Hv"]),
        950 => Some(vec!["UltraBlack", "Ultra-Black", "UltraBlk", "UBlk"]),
        _ => None,
    }
}

#[check(
    id = "monotype/weightclass",
    rationale = "
        We expect the following OS/2 usWeightClass values:

        Hairline 50
        Thin 100
        ExtraLight 200
        Light 300
        Regular 400
        Medium 500
        SemiBold 600
        Bold 700
        ExtraBold 800
        Black 900
        ExtraBlack 950
    ",
    proposal = "https://github.com/Monotype/fontspector/issues/7",
    title = "Check the OS/2 usWeightClass is appropriate for the font's best SubFamily name."
)]
fn weightclass(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let value = f.font().os2()?.us_weight_class();
    let style_name = f.best_subfamilyname().unwrap_or("Regular".to_string());
    let style_name_parts = style_name.split(' ').collect::<Vec<_>>();
    let expected_weight_names = get_preferred_weight_name(value);
    let expected_other_names = get_other_weight_name(value);

    if !(1..=1000).contains(&value) {
        return Ok(Status::just_one_fail(
            "bad-weight-class-value",
            &format!("OS/2 usWeightClass {value} is outside the expected range of 1-1000."),
        ));
    }

    if value == 400 && is_regular_weight(&style_name) {
        return Ok(Status::just_one_pass());
    }

    if let Some(expected_names) = expected_weight_names {
        for weight_name in &expected_names {
            if style_name_parts.contains(weight_name) {
                // if value is not multiple of 100, it's unusual but not necessarily wrong
                if value % 100 != 0 {
                    return Ok(Status::just_one_warn(
                        "bad-weight-class-value-for-css",
                        "CSS only supports one of the following OS/2 weight class values '100, 200, 300, 400, 500, 600, 700, 800, 900'. If the font family has more than 9 weights, this does not work for any weight."
                    ));
                } else {
                    return Ok(Status::just_one_pass());
                }
            }
        }
        let preferred_name = expected_names[0];
        if let Some(other_names) = expected_other_names {
            for other_weight_name in &other_names {
                if style_name_parts.contains(other_weight_name) {
                    return Ok(Status::just_one_warn(
                        "weight-class-name-value-mismatch-other", 
                        &format!(
                            "For OS/2 usWeightClass {value} we expect {preferred_name}, but got '{style_name}'."
                        )
                    ));
                }
            }
        }
        return Ok(Status::just_one_fail(
            "weight-class-name-value-mismatch", 
            &format!(
                "For OS/2 usWeightClass {value} we expect '{preferred_name}', but got '{style_name}'. Either usWeightClass is wrong or style name. Please investigate."
            )
        ));
    } else {
        return Ok(Status::just_one_fail(
            "bad-weight-class-value",
            &format!(
                "OS/2 usWeightClass {value} does not match specifications. We expect: Hairline 50, Thin 100, ExtraLight 200, Light 300, Regular 400, Medium 500, SemiBold 600, Bold 700, ExtraBold 800, Black 900, ExtraBlack 950."
            )
        ));
    }
}

fn is_regular_weight(style_name: &str) -> bool {
    let style_name_lower = style_name.to_lowercase();
    if style_name_lower.contains("regular") {
        return true;
    }

    // Collect all possible non-regular indicators from both weight-name helpers.
    for weight_class in 1..=1000 {
        if weight_class == 400 {
            continue;
        }
        let weight_names = get_preferred_weight_name(weight_class)
            .into_iter()
            .flatten()
            .chain(get_other_weight_name(weight_class).into_iter().flatten());

        for indicator in weight_names {
            if style_name_lower.contains(&indicator.to_lowercase()) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use fontspector_checkapi::{Context, Testable};
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
    fn test_weightclass() {
        let weight_tests = [
            (1100, "DarkBlack", Some("OS/2 usWeightClass 1100 is outside the expected range of 1-1000.".to_string())),
            (50, "Hairline", Some("CSS only supports one of the following OS/2 weight class values '100, 200, 300, 400, 500, 600, 700, 800, 900'. If the font family has more than 9 weights, this does not work for any weight.".to_string())),
            (400, "Hairline", Some("For OS/2 usWeightClass 400 we expect 'Regular', but got 'Hairline'. Either usWeightClass is wrong or style name. Please investigate.".to_string())),
            (950, "XBlack", Some("CSS only supports one of the following OS/2 weight class values '100, 200, 300, 400, 500, 600, 700, 800, 900'. If the font family has more than 9 weights, this does not work for any weight.".to_string())),
            (400, "XBlack", Some("For OS/2 usWeightClass 400 we expect 'Regular', but got 'XBlack'. Either usWeightClass is wrong or style name. Please investigate.".to_string())),
            (400, "Regular", None),
            (333, "Regular", Some("OS/2 usWeightClass 333 does not match specifications. We expect: Hairline 50, Thin 100, ExtraLight 200, Light 300, Regular 400, Medium 500, SemiBold 600, Bold 700, ExtraBold 800, Black 900, ExtraBlack 950.".to_string())),
            (900, "Condensed Black", None),
            (600, "XXCond SemiBold Italic", None),
            (500, "XXCond SemiBold Italic", Some("For OS/2 usWeightClass 500 we expect 'Medium', but got 'XXCond SemiBold Italic'. Either usWeightClass is wrong or style name. Please investigate.".to_string())),
            (100, "Whatever Thin", None),
            (200, "ExtraLight", None),
            (200, "XLight", None),
            (300, "Light", None),
            (300, "XLight", Some("For OS/2 usWeightClass 300 we expect 'Light', but got 'XLight'. Either usWeightClass is wrong or style name. Please investigate.".to_string())),
            (600, "SemiBold", None),
            (600, "DemiBold", Some("For OS/2 usWeightClass 600 we expect 'SemiBold', but got 'DemiBold'. Either usWeightClass is wrong or style name. Please investigate.".to_string())),
            (700, "Bold", None),
            (700, "XBold", Some("For OS/2 usWeightClass 700 we expect 'Bold', but got 'XBold'. Either usWeightClass is wrong or style name. Please investigate.".to_string())),
            (800, "XBold", None),
            (800, "Black", Some("For OS/2 usWeightClass 800 we expect 'ExtraBold', but got 'Black'. Either usWeightClass is wrong or style name. Please investigate.".to_string())),
            (900, "Black", None),
            (1000, "Black", Some("OS/2 usWeightClass 1000 does not match specifications. We expect: Hairline 50, Thin 100, ExtraLight 200, Light 300, Regular 400, Medium 500, SemiBold 600, Bold 700, ExtraBold 800, Black 900, ExtraBlack 950.".to_string())),
            (400, "Italic", None),
            (350, "SemiLight", Some("CSS only supports one of the following OS/2 weight class values '100, 200, 300, 400, 500, 600, 700, 800, 900'. If the font family has more than 9 weights, this does not work for any weight.".to_string())),
            (350, "SemiLight Italic", Some("CSS only supports one of the following OS/2 weight class values '100, 200, 300, 400, 500, 600, 700, 800, 900'. If the font family has more than 9 weights, this does not work for any weight.".to_string())),
            (400, "Cond Italic", None),
            (400, "Cond Regular Italic", None),
            (900, "Blk Italic", None),
            (900, "Heavy",  Some("For OS/2 usWeightClass 900 we expect Black, but got 'Heavy'.".to_string())),
            (400, "Blk Italic", Some("For OS/2 usWeightClass 400 we expect 'Regular', but got 'Blk Italic'. Either usWeightClass is wrong or style name. Please investigate.".to_string())),
            (800, "UltraBold", Some("For OS/2 usWeightClass 800 we expect 'ExtraBold', but got 'UltraBold'. Either usWeightClass is wrong or style name. Please investigate.".to_string()))
            ];
        for (weight_class_value, style_name, expected_result) in weight_tests {
            let mut font_builder = FontBuilder::new();
            let maxp = Maxp::default();
            font_builder.add_table(&maxp).unwrap();

            let os2: Os2 = Os2 {
                us_weight_class: weight_class_value,
                ..Default::default()
            };
            font_builder.add_table(&os2).unwrap();

            let mut name: Name = Name::default();
            let mut new_records = Vec::new();
            // english default 3/1/1033
            let name_rec_fam = NameRecord::new(
                3,
                1,
                1033,
                NameId::new(16),
                "A Family Name".to_string().into(),
            );
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
            let result = weightclass_impl(&testable, &context)
                .unwrap()
                .next()
                .unwrap();

            assert_eq!(result.message, expected_result);
        }
    }
}
