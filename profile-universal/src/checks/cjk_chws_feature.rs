use std::collections::HashSet;

use fontations::skrifa::Tag;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "cjk_chws_feature",
    rationale = "
        The W3C recommends the addition of chws and vchw features to CJK fonts
        to enhance the spacing of glyphs in environments which do not fully support
        JLREQ layout rules.

        The chws_tool utility (https://github.com/googlefonts/chws_tool) can be used
        to add these features automatically.
    ",
    title = "Does the font contain chws and vchw features?",
    proposal = "https://github.com/fonttools/fontbakery/issues/3363"
)]
fn cjk_chws_feature(f: &Testable, context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    let mut problems = vec![];
    skip!(
        !font.is_cjk_font(Some(context)),
        "not-cjk",
        "Not a CJK font."
    );
    let message = "feature not found in font. Use chws_tool (https://github.com/googlefonts/chws_tool) to add it.";
    let tags: HashSet<Tag> = font
        .feature_records(false)
        .map(|(r, _)| r.feature_tag())
        .collect();

    let mut missing_features = vec![];
    if !tags.contains(&Tag::new(b"chws")) {
        let msg = format!("chws {message}");
        let mut status = Status::warn("missing-chws-feature", &msg);
        status.add_metadata(Metadata::FontProblem {
            message: msg.clone(),
            context: Some(json!({
                "missing_feature": "chws",
                "reason": "Enhances spacing of glyphs in CJK fonts for environments which do not fully support JLREQ layout rules"
            })),
        });
        problems.push(status);
        missing_features.push("chws");
    }
    if !tags.contains(&Tag::new(b"vchw")) {
        let msg = format!("vchw {message}");
        let mut status = Status::warn("missing-vchw-feature", &msg);
        status.add_metadata(Metadata::FontProblem {
            message: msg.clone(),
            context: Some(json!({
                "missing_feature": "vchw",
                "reason": "Vertical variant of chws feature for vertical text layout"
            })),
        });
        problems.push(status);
        missing_features.push("vchw");
    }
    return_result(problems)
}
