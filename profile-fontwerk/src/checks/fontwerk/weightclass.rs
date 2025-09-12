use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

fn get_expected_weight_name(weight_class: u16) -> Option<&'static [&'static str]> {
    match weight_class {
        1..=99 => Some(&["Hairline", "Hair"]),
        100 => Some(&["Thin"]),
        200 => Some(&["XLight", "ExtraLight"]),
        300 => Some(&["Light"]),
        400 => Some(&["Regular"]),
        500 => Some(&["Medium"]),
        600 => Some(&["SemiBold"]),
        700 => Some(&["Bold"]),
        800 => Some(&["XBold", "ExtraBold"]),
        900 => Some(&["Black"]),
        901..=1000 => Some(&["XBlack", "ExtraBlack"]),
        _ => None,
    }
}

#[check(
    id = "fontwerk/weightclass",
    rationale = "
        Fontwerk expects the following OS/2 usWeightClass values:

        Hairline 1-99
        Thin 100
        XLight 200
        Light 300
        Regular 400
        Medium 500
        SemiBold 600
        Bold 700
        XBold 800
        Black 900
        XBlack 901-1000
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Check the OS/2 usWeightClass is appropriate for the font's best SubFamily name."
)]
fn weightclass(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let value = f.font().os2()?.us_weight_class();
    let style_name = f.best_subfamilyname().unwrap_or("Regular".to_string());
    let style_name_parts = style_name.split(' ').collect::<Vec<_>>();
    let expected_weight_names = get_expected_weight_name(value);

    if let Some(expected_names) = expected_weight_names {
        for weight_name in expected_names {
            if style_name_parts.contains(weight_name) {
                return Ok(Status::just_one_pass());
            }
        }
        Ok(Status::just_one_fail(
            "bad-weight-class-value", 
            &format!(
                "For OS/2 usWeightClass {value} we expect {expected_names:?}, but got '{style_name}'. Either usWeightClass is wrong or style name. Please investigate."
            )
        ))
    } else {
        Ok(Status::just_one_fail(
            "bad-weight-class-value",
            &format!(
                "OS/2 usWeightClass {value} does not match fontwerk specifications. We expect: Hairline 1..=99, Thin 100, XLight 200, Light 300, Regular 400, Medium 500, SemiBold 600, Bold 700, XBold 800, Black 900, XBlack 901..=1000."
            )
        ))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use fontations::write::{
        tables::maxp::Maxp,
        tables::name::{Name, NameRecord},
        tables::os2::Os2,
        types::NameId,
        FontBuilder,
    };
    use fontspector_checkapi::{Context, Testable};

    #[test]
    fn test_weightclass() {
        let weight_tests = [
            (50, "Hairline", None),
            (400, "Hairline", Some("For OS/2 usWeightClass 400 we expect [\"Regular\"], but got 'Hairline'. Either usWeightClass is wrong or style name. Please investigate.".to_string())),
            (1000, "XBlack", None),
            (400, "XBlack", Some("For OS/2 usWeightClass 400 we expect [\"Regular\"], but got 'XBlack'. Either usWeightClass is wrong or style name. Please investigate.".to_string())),
            (400, "Regular", None),
            (333, "Regular", Some("OS/2 usWeightClass 333 does not match fontwerk specifications. We expect: Hairline 1..=99, Thin 100, XLight 200, Light 300, Regular 400, Medium 500, SemiBold 600, Bold 700, XBold 800, Black 900, XBlack 901..=1000.".to_string())),
            (900, "Condensed Black", None),
            (600, "XXCond SemiBold Italic", None),
            (500, "XXCond SemiBold Italic", Some("For OS/2 usWeightClass 500 we expect [\"Medium\"], but got 'XXCond SemiBold Italic'. Either usWeightClass is wrong or style name. Please investigate.".to_string())),
            (50, "XXWide Hair Italic", None),
            (100, "Whatever Thin", None),
            (200, "ExtraLight", None),
            (200, "XLight", None),
            (300, "Light", None),
            (300, "XLight", Some("For OS/2 usWeightClass 300 we expect [\"Light\"], but got 'XLight'. Either usWeightClass is wrong or style name. Please investigate.".to_string())),
            (600, "SemiBold", None),
            (600, "DemiBold", Some("For OS/2 usWeightClass 600 we expect [\"SemiBold\"], but got 'DemiBold'. Either usWeightClass is wrong or style name. Please investigate.".to_string())),
            (700, "Bold", None),
            (700, "XBold", Some("For OS/2 usWeightClass 700 we expect [\"Bold\"], but got 'XBold'. Either usWeightClass is wrong or style name. Please investigate.".to_string())),
            (800, "XBold", None),
            (800, "Black", Some("For OS/2 usWeightClass 800 we expect [\"XBold\", \"ExtraBold\"], but got 'Black'. Either usWeightClass is wrong or style name. Please investigate.".to_string())),
            (900, "Black", None),
            (1000, "Black", Some("For OS/2 usWeightClass 1000 we expect [\"XBlack\", \"ExtraBlack\"], but got 'Black'. Either usWeightClass is wrong or style name. Please investigate.".to_string())),
            (950, "XBlack", None),
            (1000, "XBlack", None),
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
