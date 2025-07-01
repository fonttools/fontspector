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
