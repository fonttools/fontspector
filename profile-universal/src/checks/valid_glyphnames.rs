use std::collections::HashSet;

use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata, SourceFile};
use itertools::Itertools;
use serde_json::json;
use skrifa::{
    raw::{types::Version16Dot16, TableProvider},
    GlyphId as SkrifaGlyphId, MetadataProvider,
};

#[derive(Debug, PartialEq)]
enum NameValidity {
    OK,
    Naughty,
    Long,
}
fn test_glyph_name(s: &str, allowed_special_chars: Vec<char>) -> NameValidity {
    if s.starts_with(".null") || s.starts_with(".notdef") || s.starts_with(".ttfautohint") {
        return NameValidity::OK;
    }
    // A valid name starts with a-zA-Z_, and contains up to 63 characters from a-zA-Z0-9._.
    if !(s.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || allowed_special_chars.contains(&c))
        && s.len() < 63)
    {
        return NameValidity::Naughty;
    }
    if s.len() > 31 && s.len() <= 63 {
        return NameValidity::Long;
    }
    NameValidity::OK
}

#[check(
    id = "valid_glyphnames",
    title = "Glyph names are all valid?",
    rationale = "Microsoft's recommendations for OpenType Fonts states the following:

        'NOTE: The PostScript glyph name must be no longer than 31 characters,
        include only uppercase or lowercase English letters, European digits,
        the period or the underscore, i.e. from the set `[A-Za-z0-9_.]` and
        should start with a letter, except the special glyph name `.notdef`
        which starts with a period.'

        https://learn.microsoft.com/en-us/typography/opentype/otspec181/recom#-post--table


        In practice, though, particularly in modern environments, glyph names
        can be as long as 63 characters.

        According to the \"Adobe Glyph List Specification\" available at:

        https://github.com/adobe-type-tools/agl-specification
        
        Glyph names must also be unique, as duplicate glyph names prevent font installation on Mac OS X.",
    proposal = "https://github.com/fonttools/fontbakery/issues/2832",
    fix_source = sourcefix_valid_glyphnames,
)]
fn valid_glyphnames(f: &Testable, context: &Context) -> CheckFnResult {
    let config = context.local_config("valid_glyphnames");

    let default_special_chars: Vec<char> = vec!['.', '_']; // default space
    let allowed_special_chars = config
        .get("ALLOWED_SPECIAL_CHARS")
        .map_or(default_special_chars, convert_json_to_vec_chars);

    let font = testfont!(f);
    let mut problems: Vec<Status> = vec![];
    let post = font.font().post()?;
    skip!(
        post.version() == Version16Dot16::new(3, 0),
        "post-3",
        "TrueType fonts with a format 3 post table contain no glyph names."
    );
    let mut badnames = HashSet::new();
    let mut warnnames = HashSet::new();
    let mut allnames = HashSet::new();
    let mut duplicates = HashSet::new();

    for name in font.all_glyphs().map(|x| font.glyph_name_for_id(x)) {
        if let Some(name) = name {
            if allnames.contains(&name) {
                duplicates.insert(name.clone());
            }
            match test_glyph_name(&name, allowed_special_chars.clone()) {
                NameValidity::OK => {}
                NameValidity::Naughty => {
                    badnames.insert(name.clone());
                }
                NameValidity::Long => {
                    warnnames.insert(name.clone());
                }
            }
            allnames.insert(name);
        } else {
            // We have run out of names and are synthesising, stop here.
            break;
        }
    }
    if !badnames.is_empty() {
        problems.push(Status::fail(
            "found-invalid-names",
            &format!(
                "The following glyph names do not comply with naming conventions: {:}\n\n
                A glyph name must be entirely comprised of characters
                from the following set: A-Z a-z 0-9 .(period) _(underscore).
                A glyph name must not start with a digit or period.
                There are a few exceptions such as the special glyph '.notdef'.
                The glyph names \"twocents\", \"a1\", and \"_\" are all valid,
                while \"2cents\" and \".twocents\" are not.'",
                Itertools::intersperse(badnames.into_iter(), ", ".to_string()).collect::<String>()
            ),
        ));
    }
    if !warnnames.is_empty() {
        problems.push(Status::warn(
            "legacy-long-names",
            &format!(
                "The following glyph names are too long: {:?}",
                Itertools::intersperse(warnnames.into_iter(), ", ".to_string()).collect::<String>()
            ),
        ));
    }
    if !duplicates.is_empty() {
        problems.push(Status::fail(
            "duplicated-glyph-names",
            &format!(
                "These glyph names occur more than once: {:?}",
                Itertools::intersperse(duplicates.into_iter(), ", ".to_string())
                    .collect::<String>()
            ),
        ));
    }
    let spacename = font.glyph_name_for_unicode(0x20u32);
    let nbspname = font.glyph_name_for_unicode(0xa0u32);
    let space_gid: Option<SkrifaGlyphId> = font.font().charmap().map(0x20u32);
    let nbsp_gid: Option<SkrifaGlyphId> = font.font().charmap().map(0xa0u32);

    match nbspname.as_deref() {
        Some("space") | Some("uni00A0") | None => {}
        x if x == spacename.as_deref() => {}
        Some("nonbreakingspace")
        | Some("nbspace")
        | Some("u00A0")
        | Some("u000A0")
        | Some("u0000A0") => {
            #[allow(clippy::unwrap_used)]
            let name = nbspname.unwrap();
            let message = format!("Glyph 0x00A0 is called {}; must be named 'uni00A0'.", name);
            let mut status = Status::warn("not-recommended-00A0", &message);
            if let Some(gid) = nbsp_gid {
                status.add_metadata(Metadata::GlyphProblem {
                    glyph_name: font.glyph_name_for_id_synthesise(gid),
                    glyph_id: gid.to_u32(),
                    userspace_location: None,
                    position: None,
                    actual: Some(json!({ "glyph_name": name })),
                    expected: Some(json!({ "glyph_name": "uni00A0" })),
                    message,
                });
            }
            problems.push(status);
        }
        Some(other) => {
            let message = format!("Glyph 0x00A0 is called {other}; must be named 'uni00A0'.");
            let mut status = Status::fail("non-compliant-00A0", &message);
            if let Some(gid) = nbsp_gid {
                status.add_metadata(Metadata::GlyphProblem {
                    glyph_name: font.glyph_name_for_id_synthesise(gid),
                    glyph_id: gid.to_u32(),
                    userspace_location: None,
                    position: None,
                    actual: Some(json!({ "glyph_name": other })),
                    expected: Some(json!({ "glyph_name": "uni00A0" })),
                    message,
                });
            }
            problems.push(status);
        }
    }

    match spacename.as_deref() {
        Some("space") | None => {}
        Some("uni0020") | Some("u0020") | Some("u00020") | Some("u000020") => {
            #[allow(clippy::unwrap_used)]
            let name = spacename.unwrap();
            let message = format!("Glyph 0x0020 is called {}; must be named 'space'.", name);
            let mut status = Status::warn("not-recommended-0020", &message);
            if let Some(gid) = space_gid {
                status.add_metadata(Metadata::GlyphProblem {
                    glyph_name: font.glyph_name_for_id_synthesise(gid),
                    glyph_id: gid.to_u32(),
                    userspace_location: None,
                    position: None,
                    actual: Some(json!({ "glyph_name": name })),
                    expected: Some(json!({ "glyph_name": "space" })),
                    message,
                });
            }
            problems.push(status);
        }
        Some(other) => {
            let message = format!("Glyph 0x0020 is called {other}; must be named 'space'.");
            let mut status = Status::fail("non-compliant-0020", &message);
            if let Some(gid) = space_gid {
                status.add_metadata(Metadata::GlyphProblem {
                    glyph_name: font.glyph_name_for_id_synthesise(gid),
                    glyph_id: gid.to_u32(),
                    userspace_location: None,
                    position: None,
                    actual: Some(json!({ "glyph_name": other })),
                    expected: Some(json!({ "glyph_name": "space" })),
                    message,
                });
            }
            problems.push(status);
        }
    }

    return_result(problems)
}

fn sourcefix_valid_glyphnames(
    s: &mut SourceFile,
    _problems: &[Status],
    _replies: Option<MoreInfoReplies>,
) -> Result<FixResult, FontspectorError> {
    let font = &mut s.source;

    let mut changed = false;
    for glyph in font.glyphs.iter_mut() {
        if glyph.codepoints.contains(&0x20u32) && glyph.name != "space" {
            glyph.name = "space".into();
            changed = true;
        }
        if glyph.codepoints.contains(&0xa0u32) && glyph.name != "nbspace" {
            glyph.name = "nbspace".into();
            changed = true;
        }
    }
    if changed {
        Ok(FixResult::Fixed)
    } else {
        Ok(FixResult::NotBroken)
    }
}

fn convert_json_to_vec_chars(json_value: &serde_json::Value) -> Vec<char> {
    if let Some(array) = json_value.as_array() {
        array
            .iter()
            .filter_map(|v| v.as_str())
            .flat_map(|s| s.chars())
            .collect()
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{test_glyph_name, valid_glyphnames, NameValidity};
    use fontspector_checkapi::{
        codetesting::{
            assert_messages_contain, assert_pass, assert_results_contain, assert_skip, run_check,
            test_able,
        },
        FileTypeConvert, StatusCode,
    };

    #[test]
    fn test_valid_glyphnames_pass() {
        let testable = test_able("nunito/Nunito-Regular.ttf");
        let results = run_check(valid_glyphnames, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_valid_glyphnames_fail() {
        let mut testable = test_able("nunito/Nunito-Regular.ttf");

        let f = fontspector_checkapi::prelude::TTF
            .from_testable(&testable)
            .unwrap();

        let glyph_names: Vec<String> = [".notdef".to_string(), "invalid-name".to_string()].to_vec();

        let new_post =
            write_fonts::tables::post::Post::new_v2(glyph_names.iter().map(String::as_str));

        testable.set(f.rebuild_with_new_table(&new_post).unwrap());

        let results = run_check(valid_glyphnames, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("found-invalid-names".to_string()),
        );
    }

    #[test]
    fn test_valid_glyphnames_cff_skip() {
        // CFF fonts typically have post table version 3.0 which has no glyph names
        let testable = test_able("source-sans-pro/OTF/SourceSansPro-Regular.otf");
        let results = run_check(valid_glyphnames, testable);
        assert_skip(&results);
    }

    #[test]
    fn test_valid_glyphnames_cff2_skip() {
        let testable = test_able("source-sans-pro/VAR/SourceSansVariable-Roman.otf");
        let results = run_check(valid_glyphnames, testable);
        assert_skip(&results);
    }

    #[test]
    fn test_various_glyphnames_adobe() {
        // According to ADOBE: https://github.com/adobe-type-tools/agl-specification
        // must not start with a digit or period, and must be entirely comprised of characters from the following limited set:

        // A through Z
        // a through z
        // 0 through 9
        // . (period)
        // _ (underscore)
        // The only exception to these requirements is the special ‘.notdef’ glyph.

        // For example, ‘twocents’, ‘a1’, and ‘_’ are valid glyph names, but ‘2cents’ and ‘.twocents’ are not.
        let allowed_special_chars = vec!['.', '_'];
        let tests = vec![
            ("twocents", NameValidity::OK),
            ("a1", NameValidity::OK),
            ("_", NameValidity::OK),
            ("2cents", NameValidity::Naughty),
            (".twocents", NameValidity::Naughty),
            ("space", NameValidity::OK),
            // Valid glyph names according to Adobe
            ("a.sc", NameValidity::OK),
            ("T.swash", NameValidity::OK),
            ("T.begin", NameValidity::OK),
            ("T.end", NameValidity::OK),
            ("T.end1", NameValidity::OK),
            ("T.alt01", NameValidity::OK),
            ("T.alt02", NameValidity::OK),
            ("one.superior", NameValidity::OK),
            ("one.inferior", NameValidity::OK),
            ("one.numerator", NameValidity::OK),
            ("one.denominator", NameValidity::OK),
            ("one.fitted", NameValidity::OK),
            ("one.tab", NameValidity::OK),
            ("one.oldstyle", NameValidity::OK),
            ("one.taboldstyle", NameValidity::OK),
            // Other examples
            (
                "some_very_long_name_which_almost_exceeds_the_limit", // 50 characters
                NameValidity::Long,
            ),
            (
                "some_very_long_name_which_exceeds_the_limit_of_sixty_three_characters", // 69 characters
                NameValidity::Naughty,
            ),
            ("", NameValidity::Naughty),
            (" ", NameValidity::Naughty),
            ("zero.tnum", NameValidity::OK),
            (".notdef", NameValidity::OK),
            ("u0041", NameValidity::OK),
            ("u0041_0042", NameValidity::OK),
        ];
        for (name, expected_result) in tests {
            let result = test_glyph_name(&name, allowed_special_chars.clone());
            assert_eq!(result, expected_result);
        }
    }

    #[test]
    fn test_various_glyphnames_with_more_characters() {
        // Related to: https://github.com/Monotype/fontspector/issues/14
        let allowed_special_chars = vec!['.', '_', '-'];
        let tests = vec![
            // Valid glyph names according to Olli
            ("ie-cy", NameValidity::OK),
            ("Schwa-cy", NameValidity::OK),
            ("De-cy.loclBGR", NameValidity::OK),
            ("one-thai", NameValidity::OK),
            ("one-ar", NameValidity::OK),
        ];
        for (name, expected_result) in tests {
            let result = test_glyph_name(&name, allowed_special_chars.clone());
            assert_eq!(result, expected_result);
        }
    }
}
