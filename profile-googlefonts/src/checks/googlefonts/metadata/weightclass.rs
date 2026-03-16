use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, FileTypeConvert, TestFont};

use crate::checks::googlefonts::metadata::family_proto;

fn gf_api_weight_name(weight: u16) -> &'static str {
    match weight {
        100 => "Thin",
        200 => "ExtraLight",
        250 => "Thin",
        275 => "ExtraLight",
        300 => "Light",
        400 => "Regular",
        500 => "Medium",
        600 => "SemiBold",
        700 => "Bold",
        800 => "ExtraBold",
        900 => "Black",
        _ => "bad value",
    }
}

fn css_weight_name(weight: u16) -> &'static str {
    match weight {
        100 => "Thin",
        200 => "ExtraLight",
        300 => "Light",
        400 => "Regular",
        500 => "Medium",
        600 => "SemiBold",
        700 => "Bold",
        800 => "ExtraBold",
        900 => "Black",
        _ => "bad value",
    }
}
fn vf_weight_expectation(font: &TestFont) -> Result<(u16, String), FontspectorError> {
    if let Some((_, min, _, max)) = font.axis_ranges().find(|(r, _, _, _)| r == "wght") {
        if min <= 400.0 && max >= 400.0 {
            // if the wght range includes 400, use 400
            Ok((
                400,
                "400 because it is a varfont which includes this coordinate in its 'wght' axis"
                    .to_string(),
            ))
        } else {
            // if 400 isn't in the wght axis range, use the value closest to 400
            let font_weight = if (min - 400.0).abs() < (max - 400.0).abs() {
                min as u16
            } else {
                max as u16
            };
            let should_be = format!(
            "{font_weight} because it is the closest value to 400 on the 'wght' axis of this variable font"
        );
            Ok((font_weight, should_be))
        }
    } else {
        Ok((font.font().os2()?.us_weight_class(), "the same".to_string()))
    }
}

fn static_weight_expectation(font: &TestFont) -> Result<(u16, String), FontspectorError> {
    let weight = font.font().os2()?.us_weight_class();
    match weight {
        250 | 275 => {
            let expected_value = if weight == 250 { 100 } else { 200 };
            let should_be = format!(
                "{}, corresponding to CSS weight name '{}'",
                expected_value,
                css_weight_name(expected_value)
            );
            Ok((expected_value, should_be))
        }
        _ => Ok((weight, "the same".to_string())),
    }
}

#[check(
    id = "googlefonts/metadata/weightclass",
    rationale = "
        
        Check METADATA.pb font weights are correct.

        For static fonts, the metadata weight should be the same as the static font's
        OS/2 usWeightClass.

        For variable fonts, the weight value should be 400 if the font's wght axis range
        includes 400, otherwise it should be the value closest to 400.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2683 and https://github.com/fonttools/fontbakery/issues/4829",
    title = "Check METADATA.pb font weights are correct.",
    implementation = "all"
)]
fn weightclass(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let mut problems = vec![];
    let mdpb = c
        .get_file("METADATA.pb")
        .ok_or_else(|| FontspectorError::skip("no-mdpb", "No METADATA.pb file found"))?;
    let msg = family_proto(mdpb)?;
    let fonts_and_mdpb_weights = msg
        .fonts
        .iter()
        .map(|f| (f.weight(), c.get_file(f.filename())))
        .flat_map(|(weight, t)| t.map(|t| (weight, TTF.from_testable(t))))
        .collect::<Vec<_>>();
    for (mdpb_weight, font) in fonts_and_mdpb_weights {
        if let Some(font) = font {
            let (font_weight, should_be) = if font.is_variable_font() {
                vf_weight_expectation(&font)?
            } else {
                static_weight_expectation(&font)?
            };
            let gf_name = gf_api_weight_name(font_weight);
            let css_name = css_weight_name(mdpb_weight as u16);
            if gf_name != css_name {
                problems.push(Status::fail(
                "mismatch",
                &format!(
                "OS/2 table has usWeightClass={}, meaning '{}'.\n\nOn METADATA.pb it should be {}, but instead got {}.",
                font.font().os2()?.us_weight_class(),
                gf_name,
                should_be,
                font_weight
            )
            ));
            }
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::weightclass;
    use fontspector_checkapi::{
        codetesting::{
            assert_pass, assert_results_contain, run_check_with_config, set_weight_class, test_able,
        },
        StatusCode, Testable, TestableCollection, TestableType,
    };

    fn run(files: Vec<Testable>) -> Option<fontspector_checkapi::CheckResult> {
        let collection = TestableCollection::from_testables(files, None);
        run_check_with_config(
            weightclass,
            TestableType::Collection(&collection),
            HashMap::new(),
        )
    }

    fn cabinvf_with_weight(weight: u16) -> Testable {
        let mdpb = test_able("cabinvf/METADATA.pb");
        let text = String::from_utf8(mdpb.contents.clone())
            .unwrap_or_else(|e| panic!("Invalid UTF-8 in cabinvf METADATA fixture: {e}"));
        let updated = text.replacen(
            "weight: 400\n  filename: \"Cabin[wdth,wght].ttf\"",
            &format!("weight: {weight}\n  filename: \"Cabin[wdth,wght].ttf\""),
            1,
        );
        Testable::new_with_contents("METADATA.pb", updated.into_bytes())
    }

    fn cabin_static_with_weight(weight: u16) -> Testable {
        let mdpb = test_able("cabin/METADATA.pb");
        let text = String::from_utf8(mdpb.contents.clone())
            .unwrap_or_else(|e| panic!("Invalid UTF-8 in cabin METADATA fixture: {e}"));
        let updated = text.replacen(
            "weight: 400\n  filename: \"Cabin-Regular.ttf\"",
            &format!("weight: {weight}\n  filename: \"Cabin-Regular.ttf\""),
            1,
        );
        Testable::new_with_contents("METADATA.pb", updated.into_bytes())
    }

    #[test]
    fn test_check_metadata_weightclass() {
        assert_pass(&run(vec![
            test_able("cabinvf/Cabin[wdth,wght].ttf"),
            test_able("cabinvf/METADATA.pb"),
        ]));

        assert_results_contain(
            &run(vec![
                test_able("cabinvf/Cabin[wdth,wght].ttf"),
                cabinvf_with_weight(500),
            ]),
            StatusCode::Fail,
            Some("mismatch".to_string()),
        );

        assert_pass(&run(vec![
            test_able("leaguegothic-vf/LeagueGothic[wdth].ttf"),
            test_able("leaguegothic-vf/METADATA.pb"),
        ]));

        assert_pass(&run(vec![
            test_able("cabin/Cabin-Regular.ttf"),
            test_able("cabin/METADATA.pb"),
        ]));

        assert_results_contain(
            &run(vec![
                test_able("cabin/Cabin-Regular.ttf"),
                cabin_static_with_weight(500),
            ]),
            StatusCode::Fail,
            Some("mismatch".to_string()),
        );
    }

    #[test]
    fn test_check_metadata_weightclass_static_weight_aliases() {
        let mut thin = test_able("montserrat/Montserrat-Thin.ttf");
        set_weight_class(&mut thin, 100)
            .unwrap_or_else(|e| panic!("failed to set thin usWeightClass to 100: {e}"));
        assert_pass(&run(vec![
            thin.clone(),
            test_able("montserrat/METADATA.pb"),
        ]));

        set_weight_class(&mut thin, 250)
            .unwrap_or_else(|e| panic!("failed to set thin usWeightClass to 250: {e}"));
        assert_pass(&run(vec![thin, test_able("montserrat/METADATA.pb")]));

        let mut extra_light = test_able("montserrat/Montserrat-ExtraLight.ttf");
        set_weight_class(&mut extra_light, 200)
            .unwrap_or_else(|e| panic!("failed to set extralight usWeightClass to 200: {e}"));
        assert_pass(&run(vec![
            extra_light.clone(),
            test_able("montserrat/METADATA.pb"),
        ]));

        set_weight_class(&mut extra_light, 275)
            .unwrap_or_else(|e| panic!("failed to set extralight usWeightClass to 275: {e}"));
        assert_pass(&run(vec![extra_light, test_able("montserrat/METADATA.pb")]));
    }
}
