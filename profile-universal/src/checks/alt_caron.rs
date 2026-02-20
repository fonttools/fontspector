use std::collections::HashMap;

use fontations::skrifa::{raw::tables::glyf::Glyph, MetadataProvider};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

const CARON_CODEPOINTS: [u32; 4] = [
    0x013D, // LATIN CAPITAL LETTER L WITH CARON
    0x010F, // LATIN SMALL LETTER D WITH CARON
    0x013E, // LATIN SMALL LETTER L WITH CARON
    0x0165, // LATIN SMALL LETTER T WITH CARON
];

const WRONG_CARON_MARKS: [u32; 2] = [
    0x02C7, // CARON
    0x030C, // COMBINING CARON
];

const BAD_CARON_MARKS: [u32; 4] = [
    0x002C, // COMMA
    0x2019, // RIGHT SINGLE QUOTATION MARK
    0x201A, // SINGLE LOW-9 QUOTATION MARK
    0x0027, // APOSTROPHE
];

fn mangle_name(glyph: &str) -> String {
    glyph
        .replace(".case", "")
        .replace(".uc", "")
        .replace(".sc", "")
}

#[check(
    id = "alt_caron",
    title = "Check accent of Lcaron, dcaron, lcaron, tcaron",
    rationale = r#"
        Lcaron, dcaron, lcaron, tcaron should NOT be composed with quoteright
        or quotesingle or comma or caron(comb). It should be composed with a
        distinctive glyph which doesn't look like an apostrophe.

        Source:
        https://ilovetypography.com/2009/01/24/on-diacritics/
        http://diacritics.typo.cz/index.php?id=5
        https://www.typotheque.com/articles/lcaron
    "#,
    proposal = "https://github.com/fonttools/fontbakery/issues/3308"
)]
fn alt_caron(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    let charmap = f.font().charmap();
    let glyphname_to_codepoint: HashMap<String, u32> = f
        .codepoints(Some(context))
        .iter()
        .copied()
        .map(|codepoint| {
            (
                charmap
                    .map(codepoint)
                    .map(|gid| f.glyph_name_for_id_synthesise(gid))
                    .unwrap_or("".to_string()),
                codepoint,
            )
        })
        .collect();
    let charmap = f.font().charmap();
    for caron in CARON_CODEPOINTS {
        if let Some(gid) = charmap.map(caron) {
            if let Ok(Some(glyph)) = f.get_glyf_glyph(gid) {
                match glyph {
                    Glyph::Simple(_) => {
                        let name = f.glyph_name_for_id_synthesise(gid);
                        let message = format!(
                            "{name} is decomposed and therefore could not be checked. Please check manually."
                        );
                        let mut status = Status::warn("decomposed-outline", &message);
                        status.add_metadata(Metadata::GlyphProblem {
                            glyph_name: name,
                            glyph_id: gid.to_u32(),
                            userspace_location: None,
                            position: None,
                            actual: Some(json!({ "outline_type": "simple" })),
                            expected: None,
                            message,
                        });
                        problems.push(status);
                    }
                    Glyph::Composite(composite) => {
                        if composite.components().count() == 1 {
                            let message = format!(
                                "{} is composed of a single component and therefore could not be checked. Please check manually.",
                                f.glyph_name_for_id_synthesise(gid)
                            );
                            let mut status = Status::warn("single-compoents", &message);
                            status.add_metadata(Metadata::GlyphProblem {
                                glyph_name: f.glyph_name_for_id_synthesise(gid),
                                glyph_id: gid.to_u32(),
                                userspace_location: None,
                                position: None,
                                actual: Some(json!({ "component_count": 1 })),
                                expected: Some(json!({ "component_count_min": 2 })),
                                message,
                            });
                            problems.push(status);
                        } else {
                            for component in composite.components() {
                                let comp_name =
                                    mangle_name(&f.glyph_name_for_id_synthesise(component.glyph));
                                if let Some(codepoint) = glyphname_to_codepoint.get(&comp_name) {
                                    if BAD_CARON_MARKS.contains(codepoint) {
                                        let message = format!(
                                            "{} uses component: {}",
                                            f.glyph_name_for_id_synthesise(gid),
                                            comp_name
                                        );
                                        let mut status = Status::warn("bad-mark", &message);
                                        status.add_metadata(Metadata::GlyphProblem {
                                            glyph_name: f.glyph_name_for_id_synthesise(gid),
                                            glyph_id: gid.to_u32(),
                                            userspace_location: None,
                                            position: None,
                                            actual: Some(json!({ "component": comp_name })),
                                            expected: Some(
                                                json!({ "component_not_in": "bad_caron_marks" }),
                                            ),
                                            message,
                                        });
                                        problems.push(status);
                                    } else if WRONG_CARON_MARKS.contains(codepoint) {
                                        let message = format!(
                                            "{} uses component: {}",
                                            f.glyph_name_for_id_synthesise(gid),
                                            comp_name
                                        );
                                        let mut status = Status::fail("wrong-mark", &message);
                                        status.add_metadata(Metadata::GlyphProblem {
                                            glyph_name: f.glyph_name_for_id_synthesise(gid),
                                            glyph_id: gid.to_u32(),
                                            userspace_location: None,
                                            position: None,
                                            actual: Some(json!({ "component": comp_name })),
                                            expected: Some(
                                                json!({ "component_not_in": "wrong_caron_marks" }),
                                            ),
                                            message,
                                        });
                                        problems.push(status);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        StatusCode,
    };

    #[test]
    fn test_check_alt_caron_bad_and_wrong() {
        let testable = test_able("annie/AnnieUseYourTelescope-Regular.ttf");
        let results = run_check(super::alt_caron, testable);
        assert_results_contain(&results, StatusCode::Warn, Some("bad-mark".to_string()));
        assert_results_contain(&results, StatusCode::Fail, Some("wrong-mark".to_string()));
    }

    #[test]
    fn test_check_alt_caron_decomposed() {
        let testable = test_able("cousine/Cousine-Bold.ttf");
        let results = run_check(super::alt_caron, testable);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("decomposed-outline".to_string()),
        );
    }

    #[test]
    fn test_check_alt_caron_pass() {
        let testable = test_able("merriweather/Merriweather-Regular.ttf");
        let results = run_check(super::alt_caron, testable);
        assert_pass(&results);
    }
}
