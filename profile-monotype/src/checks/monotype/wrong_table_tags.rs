use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

/// Canonical OpenType / common registered sfnt table tags.
const STANDARD_TAGS: &[&[u8; 4]] = &[
    b"avar", b"BASE", b"CBDT", b"CBLC", b"CFF ", b"CFF2", b"cmap", b"COLR", b"CPAL", b"cvar",
    b"cvt ", b"DSIG", b"EBDT", b"EBLC", b"EBSC", b"fpgm", b"fvar", b"gasp", b"GDEF", b"glyf",
    b"GPOS", b"GSUB", b"gvar", b"hdmx", b"head", b"hhea", b"HVAR", b"hmtx", b"JSTF", b"kern",
    b"loca", b"LTSH", b"MATH", b"maxp", b"MERG", b"meta", b"MVAR", b"name", b"OS/2", b"PCLT",
    b"post", b"prep", b"sbix", b"STAT", b"SVG ", b"VDMX", b"vhea", b"VORG", b"vmtx", b"VVAR",
];

/// Explicit misspellings / near-canonical aliases of standard tags.
/// Underscore standing in for a trailing space is the common corruption of
/// space-padded tags such as `cvt ` and `CFF `.
const BAD_ALIASES: &[(&[u8; 4], &[u8; 4])] =
    &[(b"cvt_", b"cvt "), (b"CFF_", b"CFF "), (b"SVG_", b"SVG ")];

fn ascii_lower(tag: &[u8; 4]) -> [u8; 4] {
    [
        tag[0].to_ascii_lowercase(),
        tag[1].to_ascii_lowercase(),
        tag[2].to_ascii_lowercase(),
        tag[3].to_ascii_lowercase(),
    ]
}

fn tag_string(tag: &[u8; 4]) -> String {
    String::from_utf8_lossy(tag).into_owned()
}

#[check(
    id = "monotype/wrong_table_tags",
    rationale = "
        Misspelled or near-canonical sfnt table tags (for example `cvt_` instead
        of `cvt `, or `Cmap` instead of `cmap`) are corrupted directory entries.
        Rendering engines will not recognise them, so required data can be
        silently missing.

        This check uses an explicit list of known OpenType tags plus a small
        bad-alias list (no fuzzy / Levenshtein matching). Legitimate private or
        vendor tags that are not near-matches of known tags are ignored.
    ",
    proposal = "https://github.com/Monotype/fontspector/issues/39",
    title = "Is the font free of misspelled standard tags (e.g. cvt_ instead of cvt)?"
)]
fn wrong_table_tags(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];

    for record in f.font().table_directory.table_records() {
        let bytes = record.tag().into_bytes();

        if STANDARD_TAGS.iter().any(|t| **t == bytes) {
            continue;
        }

        if let Some((_, correct)) = BAD_ALIASES.iter().find(|(bad, _)| **bad == bytes) {
            problems.push(Status::fail(
                "incorrect-table-tags",
                &format!(
                    "Table tag '{}' looks like a misspelling of standard tag '{}'. \
                     These corrupted entries cannot be used by rendering engines.",
                    tag_string(&bytes),
                    tag_string(correct)
                ),
            ));
            continue;
        }

        let lower = ascii_lower(&bytes);
        if let Some(canonical) = STANDARD_TAGS
            .iter()
            .find(|t| ascii_lower(t) == lower)
        {
            problems.push(Status::fail(
                "incorrect-table-tags",
                &format!(
                    "Table tag '{}' is a case mismatch of standard tag '{}'. \
                     These corrupted entries cannot be used by rendering engines.",
                    tag_string(&bytes),
                    tag_string(canonical)
                ),
            ));
        }
    }

    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::wrong_table_tags;
    use fontspector_checkapi::codetesting::{
        add_table, assert_pass, assert_results_contain, run_check, test_able,
    };
    use fontspector_checkapi::StatusCode;

    #[test]
    fn test_wrong_table_tags_pass() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(wrong_table_tags, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_wrong_table_tags_fail_cvt_underscore() {
        let mut testable = test_able("mada/Mada-Regular.ttf");
        add_table(&mut testable, b"cvt_");
        let results = run_check(wrong_table_tags, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("incorrect-table-tags".to_string()),
        );
    }

    #[test]
    fn test_wrong_table_tags_fail_case_mismatch() {
        let mut testable = test_able("mada/Mada-Regular.ttf");
        add_table(&mut testable, b"Cmap");
        let results = run_check(wrong_table_tags, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("incorrect-table-tags".to_string()),
        );
    }

    #[test]
    fn test_wrong_table_tags_pass_private_vendor_tag() {
        let mut testable = test_able("mada/Mada-Regular.ttf");
        // Legitimate private tag, not a near-match of a known standard tag.
        add_table(&mut testable, b"LINO");
        let results = run_check(wrong_table_tags, testable);
        assert_pass(&results);
    }
}
