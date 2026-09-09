use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use skrifa::raw::TableProvider;

fn get_expected_width_name(width_class: u16) -> Option<&'static [&'static str]> {
    match width_class {
        1 => Some(&["XXCond", "Ultra-Condensed", "Ultra-Cond"]),
        2 => Some(&["XCond", "Extra-Condensed", "Extra-Cond"]),
        3 => Some(&["Cond", "Condensed"]),
        4 => Some(&["SemiCond", "Semi-Cond", "Semi-Condensed"]),
        5 => Some(&["Normal"]),
        6 => Some(&["SemiWide", "Semi-Wide", "Semi-Expanded"]),
        7 => Some(&["Wide", "Expanded"]),
        8 => Some(&["XWide", "Extra-Wide", "Extra-Expanded"]),
        9 => Some(&["XXWide", "Ultra-Wide", "Ultra-Expanded"]),
        _ => None,
    }
}

#[check(
    id = "monotype/widthclass",
    rationale = "
        We expect the following OS/2 usWidthClass values:

        XXCond 1
        XCond 2
        Cond 3
        SemiCond 4
        Normal 5
        SemiWide 6
        Wide 7
        XWide 8
        XXWide 9
    ",
    proposal = "https://github.com/Monotype/fontspector/issues/1",
    title = "Check the OS/2 usWidthClass is appropriate for the font's best SubFamily name."
)]
fn widthclass(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let value = f.font().os2()?.us_width_class();
    let style_name = f.best_subfamilyname().unwrap_or("Regular".to_string());
    let style_name_parts = style_name.split(' ').collect::<Vec<_>>();
    let expected_width_names = get_expected_width_name(value);

    if value == 5 && is_normal_width(&style_name) {
        return Ok(Status::just_one_pass());
    }

    if let Some(expected_names) = expected_width_names {
        for width_name in expected_names {
            if style_name_parts.contains(width_name) {
                return Ok(Status::just_one_pass());
            }
        }
        Ok(Status::just_one_fail(
            "bad-width-class-value", 
            &format!(
                "For OS/2 usWidthClass {value} we expect {expected_names:?}, but got '{style_name}'. Either usWidthClass is wrong or style name. Please investigate."
            )
        ))
    } else {
        Ok(Status::just_one_fail(
            "bad-width-class-value",
            &format!(
                "OS/2 usWidthClass {value} does not match specifications. We expect: XXCond 1, XCond 2, Cond 3, SemiCond 4, (Normal) 5, SemiWide 6, Wide 7, XWide 8, XXWide 9."
            )
        ))
    }
}

fn is_normal_width(style_name: &str) -> bool {
    let style_name_lower = style_name.to_lowercase();

    // if any width is in the style name, it's not regular
    let non_regular_indicators = [
        "cond",   // includes XXCond, XCond, Cond, SemiCond
        "wide",   // includes xwide, xxwide, extra-wide, ultra-wide
        "expand", // includes extra-expanded, ultra-expanded
    ];

    for indicator in non_regular_indicators.iter() {
        if style_name_lower.contains(indicator) {
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
    fn test_widthclass() {
        let width_tests = [
            (5, "Hairline", None),
            (5, "Cond Regular", Some("For OS/2 usWidthClass 5 we expect [\"Normal\"], but got 'Cond Regular'. Either usWidthClass is wrong or style name. Please investigate.".to_string())),
            (3, "Condensed Black", None),
            (2, "XCond SemiBold Italic", None),
            (5, "XCond SemiBold Italic", Some("For OS/2 usWidthClass 5 we expect [\"Normal\"], but got 'XCond SemiBold Italic'. Either usWidthClass is wrong or style name. Please investigate.".to_string())),
            (6, "Semi-Wide SemiBold Italic", None),
            (7, "Semi-Wide SemiBold Italic", Some("For OS/2 usWidthClass 7 we expect [\"Wide\", \"Expanded\"], but got 'Semi-Wide SemiBold Italic'. Either usWidthClass is wrong or style name. Please investigate.".to_string())),
            (9, "XXWide Hair Italic", None),
            (5, "Whatever Thin", None),
            (5, "ExtraLight", None),
            (5, "XLight", None),
            (5, "Light", None),
            (5, "XBlack", None),
            (5, "XBlack", None),
            (5, "Italic", None),
            (5, "SemiLight", None),
            (5, "SemiLight Italic", None),
            (3, "Cond Italic", None),
            (3, "Cond Regular Italic", None),
            (4, "Cond Regular Italic", Some("For OS/2 usWidthClass 4 we expect [\"SemiCond\", \"Semi-Cond\", \"Semi-Condensed\"], but got 'Cond Regular Italic'. Either usWidthClass is wrong or style name. Please investigate.".to_string())),
            (10, "XXWide", None),
            ];
        for (width_class_value, style_name, expected_result) in width_tests {
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
            let result = widthclass_impl(&testable, &context)
                .unwrap()
                .next()
                .unwrap();

            assert_eq!(result.message, expected_result);
        }
    }
}
