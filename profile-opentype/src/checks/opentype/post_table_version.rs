use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "opentype/post_table_version",
    rationale = r#"
        Format 2.5 of the 'post' table was deprecated in OpenType 1.3 and
        should not be used.

        According to Thomas Phinney, the possible problem with post format 3
        is that under the right combination of circumstances, one can generate
        PDF from a font with a post format 3 table, and not have accurate backing
        store for any text that has non-default glyphs for a given codepoint.

        It will look fine but not be searchable. This can affect Latin text with
        high-end typography, and some complex script writing systems, especially
        with higher-quality fonts. Those circumstances generally involve creating
        a PDF by first printing a PostScript stream to disk, and then creating a
        PDF from that stream without reference to the original source document.
        There are some workflows where this applies,but these are not common
        use cases.

        Apple recommends against use of post format version 4 as "no longer
        necessary and should be avoided". Please see the Apple TrueType reference
        documentation for additional details.

        https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6post.html

        Acceptable post format versions are 2 and 3 for TTF and OTF CFF2 builds,
        and post format 3 for CFF builds.
    "#,
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
    proposal = "https://github.com/google/fonts/issues/215",
    proposal = "https://github.com/fonttools/fontbakery/issues/263",
    proposal = "https://github.com/fonttools/fontbakery/issues/3635",
    title = "Font has correct post table version?"
)]
fn post_table_version(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let version = f
        .font()
        .post()
        .map(|post| post.version().to_major_minor())
        .unwrap_or((0, 0));
    let is_cff = f.has_table(b"CFF ");
    let mut problems = vec![];
    match (is_cff, version) {
        (true, (3, _)) => {
            // Pass
        }
        (true, _) => {
            let msg = "CFF fonts must contain post format 3 table.";
            let mut status = Status::fail("post-table-version", msg);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "post".to_string(),
                field_name: Some("version".to_string()),
                actual: Some(json!(format!("{}.{}", version.0, version.1))),
                expected: Some(json!("3.0 for CFF")),
                message: msg.to_string(),
            });
            problems.push(status);
        }
        (false, (3, _)) => {
            let msg = "Post table format 3 use has niche use case problems. Please review the check rationale for additional details.";
            let mut status = Status::warn("post-table-version", msg);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "post".to_string(),
                field_name: Some("version".to_string()),
                actual: Some(json!("3.0")),
                expected: Some(json!("2.0 (recommended)")),
                message: msg.to_string(),
            });
            problems.push(status);
        }
        (_, (2, 5)) => {
            let msg = "Post format 2.5 was deprecated in OpenType 1.3 and should not be used.";
            let mut status = Status::fail("post-table-version", msg);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "post".to_string(),
                field_name: Some("version".to_string()),
                actual: Some(json!("2.5")),
                expected: Some(json!("2.0 or 3.0")),
                message: msg.to_string(),
            });
            problems.push(status);
        }
        (_, (4, _)) => {
            let msg = "According to Apple documentation, post format 4 tables are no longer necessary and should not be used.";
            let mut status = Status::fail("post-table-version", msg);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "post".to_string(),
                field_name: Some("version".to_string()),
                actual: Some(json!("4.0")),
                expected: Some(json!("2.0 or 3.0")),
                message: msg.to_string(),
            });
            problems.push(status);
        }
        _ => {
            // Pass for version 2 or other valid versions
        }
    }
    return_result(problems)
}
