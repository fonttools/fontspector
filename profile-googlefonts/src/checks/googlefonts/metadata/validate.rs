use std::path::Path;

use crate::checks::googlefonts::metadata::family_proto;
use chrono::prelude::*;
use fontspector_checkapi::prelude::*;
use hashbrown::HashSet;

fn weight_acceptable_suffixes(w: i32) -> Vec<&'static str> {
    match w {
        100 => vec!["Thin", "ThinItalic"],
        200 => vec!["ExtraLight", "ExtraLightItalic"],
        300 => vec!["Light", "LightItalic"],
        400 => vec!["Regular", "Italic"],
        500 => vec!["Medium", "MediumItalic"],
        600 => vec!["SemiBold", "SemiBoldItalic"],
        700 => vec!["Bold", "BoldItalic"],
        800 => vec!["ExtraBold", "ExtraBoldItalic"],
        900 => vec!["Black", "BlackItalic"],
        _ => vec![],
    }
}

const CATEGORY_HINTS: [(&str, &str); 11] = [
    ("Sans", "SANS_SERIF"),
    ("Grotesk", "SANS_SERIF"),
    ("Grotesque", "SANS_SERIF"),
    ("Serif", "SERIF"),
    ("Transitional", "SERIF"),
    ("Slab", "SERIF"),
    ("Old Style", "SERIF"),
    ("Garamond", "SERIF"),
    ("Display", "DISPLAY"),
    ("Hand", "HANDWRITING"),
    ("Script", "HANDWRITING"),
];

fn category_hints(family_name: &str) -> Option<&'static str> {
    for (component, inferred_category) in CATEGORY_HINTS.iter() {
        if family_name.contains(component) {
            return Some(inferred_category);
        }
    }
    None
}
const VALID_CLASSIFICATIONS: &[&str] = &["DISPLAY", "HANDWRITING", "MONOSPACE", "SYMBOLS"];
const VALID_STROKES: &[&str] = &["SERIF", "SLAB_SERIF", "SANS_SERIF"];

fn clean_url(url: &str) -> String {
    let mut cleaned = url.trim().to_string();
    if cleaned.ends_with('/') {
        cleaned.pop();
    }
    if cleaned.ends_with("index.htm") {
        cleaned = cleaned.replace("index.htm", "");
    }
    if cleaned.ends_with("index.html") {
        cleaned = cleaned.replace("index.html", "");
    }
    cleaned
}

#[check(
    id = "googlefonts/metadata/validate",
    title = "Check METADATA.pb parses correctly",
    rationale = "
        The purpose of this check is to ensure that the METADATA.pb file is not
        malformed.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2248",
    applies_to = "MDPB"
)]
fn validate(c: &Testable, _context: &Context) -> CheckFnResult {
    let msg = match family_proto(c) {
        Ok(msg) => msg,
        Err(e) => {
            return Ok(Status::just_one_fatal(
                "parse-error",
                &format!("Failed to parse METADATA.pb: {e}"),
            ));
        }
    };
    let mut problems = vec![];
    if let Some(designer) = msg.designer.as_ref() {
        if designer.is_empty() {
            problems.push(Status::fail(
                "empty-designer",
                "Font designer field is empty.",
            ))
        }
        if designer.contains('/') {
            problems.push(Status::fail("slash",
                    &format!(
                    "Font designer field contains a forward slash '{designer}'. Please use commas to separate multiple names instead."
                )));
        }
    }

    // Check date added is YYYY-MM-DD
    if msg.date_added().is_empty() {
        problems.push(Status::error(Some("date-empty"), "Date added is empty"))
    }

    if msg
        .date_added
        .as_ref()
        .is_some_and(|da| NaiveDate::parse_from_str(da, "%Y-%m-%d").is_err())
    {
        problems.push(Status::error(
            Some("date-malformed"),
            "Date added is not in the format YYYY-MM-DD",
        ))
    }

    // Check category hints (googlefonts/metadata/category_hints)
    if let Some(inferred_category) = category_hints(msg.name()) {
        if !msg.category.contains(&inferred_category.to_string()) {
            problems.push(Status::warn(
                "inferred-category",
                &format!(
                    "Familyname seems to hint at \"{}\" category, but METADATA.pb declares it as \"{}\".",
                    inferred_category,
                    msg.category.join(", "),
                ),
            ));
        }
    }

    // Check minisite URL (googlefonts/metadata/minisite_url)
    if msg.minisite_url().is_empty() {
        problems.push(Status::info(
            "lacks-minisite-url",
            "Please consider adding a family.minisite_url entry.",
        ))
    } else {
        let expected_url = clean_url(msg.minisite_url());
        if msg.minisite_url() != expected_url {
            problems.push(Status::fail(
                "trailing-clutter",
                &format!(
                    "Please change minisite_url from {} to {}",
                    msg.minisite_url(),
                    expected_url
                ),
            ))
        }
    }

    let mut weight_style = HashSet::new();

    for font in msg.fonts.iter() {
        // Check weight values are canonical (googlefonts/metadata/canonical_weight_value)
        if ![100, 200, 300, 400, 500, 600, 700, 800, 900].contains(&font.weight()) {
            problems.push(Status::fail(
                    "bad-weight",
                    &format!("In METADATA.pb, the weight for {} is declared as {}, which is not a multiple of 100 between 100 and 900.",
                        font.full_name(), font.weight()),
                ))
        }
        // skip variable fonts
        if !font.filename().contains("[") {
            let post_script_name = font.post_script_name();
            // Check weight values match post_script_name (googlefonts/metadata/match_weight_postscript)
            let weight_suffixes = weight_acceptable_suffixes(font.weight());
            if !weight_suffixes
                .iter()
                .any(|suffix| post_script_name.ends_with(suffix))
            {
                problems.push(Status::fail(
                        "mismatch",
                        &format!(
                            "METADATA.pb: Mismatch between postScriptName {} and and weight value ({}). The name must end with {}",
                            font.weight(),
                            post_script_name,
                            weight_suffixes.join(" or ")
                        ),
                ));
            }

            // Check font.filename matches font.post_script_name (googlefonts/metadata/match_filename_postscript)
            if let Some(basename) = Path::new(font.filename()).file_stem() {
                if post_script_name != basename {
                    problems.push(Status::fail(
                            "mismatch",
                            &format!(
                                "METADATA.pb font filename = \"{}\" does not match post_script_name=\"{}\".",
                                font.filename(),
                                post_script_name,
                            ),
                    ));
                }
            }
        }
        // Check font.fullname matches font.post_script_name (with non-alphabetic removed) (googlefonts/metadata/match_fullname_postscript)
        if font.full_name().replace(|c| !char::is_alphanumeric(c), "")
            != font
                .post_script_name()
                .replace(|c| !char::is_alphanumeric(c), "")
        {
            problems.push(Status::fail(
                "mismatch",
                &format!(
                    "METADATA.pb font fullname = \"{}\" does not match post_script_name=\"{}\".",
                    font.full_name(),
                    font.post_script_name(),
                ),
            ));
        }

        // Check font name is same as family name (googlefonts/metadata/match_name_familyname)
        if font.name() != msg.name() {
            problems.push(Status::fail(
                "mismatch",
                &format!(
                    "METADATA.pb: {}: Family name \"{}\" does not match font name: \"{}\"",
                    font.filename(),
                    msg.name(),
                    font.name(),
                ),
            ));
        }

        // googlefonts/metadata/unique_weight_style_pairs
        if weight_style.contains(&(font.weight(), font.style())) {
            problems.push(Status::fail(
                "duplicated",
                &format!(
                    "METADATA.pb: {}: Found duplicated style:weight pair in METADATA.pb fonts field.",
                    font.filename(),
                ),
            ));
        } else {
            weight_style.insert((font.weight(), font.style()));
        }
    }

    // unique_full_name_values
    let full_names = msg
        .fonts
        .iter()
        .map(|f| f.full_name())
        .collect::<HashSet<_>>();
    if full_names.len() != msg.fonts.len() {
        problems.push(Status::fail(
            "duplicated",
            "Found duplicated \"full_name\" values in METADATA.pb fonts field.",
        ));
    }

    if !msg.languages.is_empty() && !msg.name().starts_with("Noto ") {
        problems.push(Status::fail(
            "language",
            "Non-Noto families should not have any language fields in METADATA.pb",
        ));
    }

    // The METADATA.pb file can only contain specific predefined values for the
    // 'stroke' and 'classifications' fields:

    // Valid stroke values: Serif, Slab Serif, Sans Serif
    // Valid classifications values: Display, Handwriting, Monospace, Symbols

    // Any other values are invalid and will cause issues with the Google Fonts API.

    if let Some(stroke) = msg.stroke.as_ref() {
        if !stroke.is_empty() && !VALID_STROKES.contains(&stroke.as_str()) {
            problems.push(Status::fail(
                "invalid-stroke",
                &format!(
                    "METADATA.pb stroke field contains invalid value '{}'. Valid values are: {}",
                    stroke,
                    VALID_STROKES.join(", ")
                ),
            ));
        }
    }

    for classification in &msg.classifications {
        if !VALID_CLASSIFICATIONS.contains(&classification.as_str()) {
            problems.push(Status::fail(
                "invalid-classification",
                &format!(
                    "METADATA.pb classifications field contains invalid value '{}'. Valid values are: {}",
                    classification,
                    VALID_CLASSIFICATIONS.join(", ")
                ),
            ));
        }
    }

    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        StatusCode, Testable,
    };

    fn replace_in_metadata(path: &str, old: &str, new: &str) -> Testable {
        let mdpb = test_able(path);
        let metadata = String::from_utf8(mdpb.contents.clone()).unwrap();
        assert!(
            metadata.contains(old),
            "Did not find expected snippet in METADATA: {old}"
        );
        let updated = metadata.replacen(old, new, 1);
        Testable::new_with_contents("METADATA.pb", updated.into_bytes())
    }

    fn replace_all_in_metadata(path: &str, old: &str, new: &str) -> Testable {
        let mdpb = test_able(path);
        let metadata = String::from_utf8(mdpb.contents.clone()).unwrap();
        assert!(
            metadata.contains(old),
            "Did not find expected snippet in METADATA: {old}"
        );
        let updated = metadata.replace(old, new);
        Testable::new_with_contents("METADATA.pb", updated.into_bytes())
    }

    fn assert_no_failures(results: &Option<fontspector_checkapi::CheckResult>) {
        let has_failure = results.as_ref().unwrap().subresults.iter().any(|r| {
            r.severity == StatusCode::Fail
                || r.severity == StatusCode::Error
                || r.severity == StatusCode::Fatal
        });
        assert!(
            !has_failure,
            "Expected no failing severities, got {results:#?}"
        );
    }

    #[test]
    fn test_noto_languages() {
        let mdpb = Testable::new_with_contents(
            "METADATA.pb",
            include_bytes!("../../../../../fontspector-py/data/test/notosanskhudawadi/METADATA.pb")
                .to_vec(),
        );
        assert!(String::from_utf8(mdpb.contents.clone())
            .unwrap()
            .contains("languages:"));
        let result = validate_impl(&mdpb, &Context::default())
            .unwrap()
            .collect::<Vec<_>>();
        assert!(
            result.iter().all(|r| r.severity < StatusCode::Warn),
            "Expected all checks to pass, but got: {result:#?}",
        );

        let league_gothic =
            include_bytes!("../../../../../fontspector-py/data/test/leaguegothic-vf/METADATA.pb");
        let good_mdpb = Testable::new_with_contents("METADATA.pb", league_gothic.to_vec());
        assert!(!String::from_utf8(good_mdpb.contents)
            .unwrap()
            .contains("languages:"));
        let result = validate_impl(&mdpb, &Context::default())
            .unwrap()
            .collect::<Vec<_>>();
        assert!(
            result.iter().all(|r| r.severity < StatusCode::Warn),
            "Expected all checks to pass, but got: {result:#?}",
        );
        let league_languages =
            (String::from_utf8_lossy(league_gothic) + "\n  languages: \"en_Latn\"\n").to_string();
        let bad_mdpb = Testable::new_with_contents("METADATA.pb", league_languages.into_bytes());
        assert!(String::from_utf8(bad_mdpb.contents.clone())
            .unwrap()
            .contains("languages:"));
        let result = validate_impl(&bad_mdpb, &Context::default())
            .unwrap()
            .collect::<Vec<_>>();
        assert!(result
            .iter()
            .any(|r| r.severity == StatusCode::Fail && r.code.as_deref() == Some("language")));
    }

    #[test]
    fn test_metadata_date_added() {
        let good = test_able("familysans/METADATA.pb");
        assert_no_failures(&run_check(validate, good));

        let empty = replace_in_metadata(
            "familysans/METADATA.pb",
            "date_added: \"2017-08-21\"",
            "date_added: \"\"",
        );
        assert_results_contain(
            &run_check(validate, empty),
            StatusCode::Error,
            Some("date-empty".to_string()),
        );

        let malformed = replace_in_metadata(
            "familysans/METADATA.pb",
            "date_added: \"2017-08-21\"",
            "date_added: \"2020, Oct 1st\"",
        );
        assert_results_contain(
            &run_check(validate, malformed),
            StatusCode::Error,
            Some("date-malformed".to_string()),
        );
    }

    #[test]
    fn test_metadata_unique_full_name_values() {
        let duplicated = replace_in_metadata(
            "familysans/METADATA.pb",
            "full_name: \"Family Sans Thin\"",
            "full_name: \"Family Sans Thin Italic\"",
        );
        assert_results_contain(
            &run_check(validate, duplicated),
            StatusCode::Fail,
            Some("duplicated".to_string()),
        );
    }

    #[test]
    fn test_metadata_unique_weight_style_pairs() {
        let duplicated = replace_in_metadata(
            "familysans/METADATA.pb",
            "style: \"normal\"",
            "style: \"italic\"",
        );
        assert_results_contain(
            &run_check(validate, duplicated),
            StatusCode::Fail,
            Some("duplicated".to_string()),
        );
    }

    #[test]
    fn test_metadata_match_fullname_postscript() {
        // Merriweather test metadata intentionally has one bad fullname/postscript pair.
        let bad = test_able("merriweather/METADATA.pb");
        assert_results_contain(
            &run_check(validate, bad),
            StatusCode::Fail,
            Some("mismatch".to_string()),
        );

        let fixed = replace_in_metadata(
            "merriweather/METADATA.pb",
            "full_name: \"Merriweather\"",
            "full_name: \"Merriweather Regular\"",
        );
        assert_no_failures(&run_check(validate, fixed));

        let mismatched = replace_in_metadata(
            "merriweather/METADATA.pb",
            "full_name: \"Merriweather\"",
            "full_name: \"MistakenFont Regular\"",
        );
        assert_results_contain(
            &run_check(validate, mismatched),
            StatusCode::Fail,
            Some("mismatch".to_string()),
        );
    }

    #[test]
    fn test_metadata_match_name_familyname() {
        let good = test_able("familysans/METADATA.pb");
        assert_no_failures(&run_check(validate, good));

        let mismatch = replace_in_metadata(
            "familysans/METADATA.pb",
            "fonts {\n  name: \"Family Sans\"",
            "fonts {\n  name: \"Something Else\"",
        );
        assert_results_contain(
            &run_check(validate, mismatch),
            StatusCode::Fail,
            Some("mismatch".to_string()),
        );
    }

    #[test]
    fn test_metadata_canonical_weight_value() {
        let good = test_able("familysans/METADATA.pb");
        assert_no_failures(&run_check(validate, good));

        let bad_weight =
            replace_in_metadata("familysans/METADATA.pb", "weight: 100", "weight: 150");
        assert_results_contain(
            &run_check(validate, bad_weight),
            StatusCode::Fail,
            Some("bad-weight".to_string()),
        );
    }

    #[test]
    fn test_metadata_category_hints() {
        let good = test_able("familysans/METADATA.pb");
        assert_no_failures(&run_check(validate, good));

        let warned =
            replace_all_in_metadata("familysans/METADATA.pb", "Family Sans", "Seaweed Script");
        let warned = {
            let metadata = String::from_utf8(warned.contents.clone()).unwrap();
            let updated = metadata.replacen("category: \"SANS_SERIF\"", "category: \"DISPLAY\"", 1);
            Testable::new_with_contents("METADATA.pb", updated.into_bytes())
        };
        assert_results_contain(
            &run_check(validate, warned),
            StatusCode::Warn,
            Some("inferred-category".to_string()),
        );

        let fixed =
            replace_all_in_metadata("familysans/METADATA.pb", "Family Sans", "Seaweed Script");
        let fixed = {
            let metadata = String::from_utf8(fixed.contents.clone()).unwrap();
            let updated =
                metadata.replacen("category: \"SANS_SERIF\"", "category: \"HANDWRITING\"", 1);
            Testable::new_with_contents("METADATA.pb", updated.into_bytes())
        };
        let results = run_check(validate, fixed);
        let has_inferred_category = results
            .as_ref()
            .unwrap()
            .subresults
            .iter()
            .any(|r| r.code.as_deref() == Some("inferred-category"));
        assert!(
            !has_inferred_category,
            "Unexpected inferred-category warning"
        );
    }

    #[test]
    fn test_metadata_minisite_url() {
        let no_minisite = test_able("familysans/METADATA.pb");
        assert_results_contain(
            &run_check(validate, no_minisite),
            StatusCode::Info,
            Some("lacks-minisite-url".to_string()),
        );

        let good = replace_in_metadata(
            "familysans/METADATA.pb",
            "date_added: \"2017-08-21\"",
            "date_added: \"2017-08-21\"\nminisite_url: \"a_good_one.com\"",
        );
        assert_pass(&run_check(validate, good));

        let trailing_slash = replace_in_metadata(
            "familysans/METADATA.pb",
            "date_added: \"2017-08-21\"",
            "date_added: \"2017-08-21\"\nminisite_url: \"some_url/\"",
        );
        assert_results_contain(
            &run_check(validate, trailing_slash),
            StatusCode::Fail,
            Some("trailing-clutter".to_string()),
        );

        let trailing_index = replace_in_metadata(
            "familysans/METADATA.pb",
            "date_added: \"2017-08-21\"",
            "date_added: \"2017-08-21\"\nminisite_url: \"some_url/index.html\"",
        );
        assert_results_contain(
            &run_check(validate, trailing_index),
            StatusCode::Fail,
            Some("trailing-clutter".to_string()),
        );
    }
}
