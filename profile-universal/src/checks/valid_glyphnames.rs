use std::collections::HashSet;

use fontations::skrifa::raw::{types::Version16Dot16, TableProvider};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use itertools::Itertools;

enum NameValidity {
    OK,
    Naughty,
    Long,
}
fn test_glyph_name(s: &str) -> NameValidity {
    if s.starts_with(".null") || s.starts_with(".notdef") || s.starts_with(".ttfautohint") {
        return NameValidity::OK;
    }
    // A valid name starts with a-zA-Z_, and contains up to 63 characters from a-zA-Z0-9._.
    if !(s.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_')
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
    proposal = "https://github.com/fonttools/fontbakery/issues/2832"
)]
fn valid_glyphnames(f: &Testable, _context: &Context) -> CheckFnResult {
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
            match test_glyph_name(&name) {
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

    match nbspname.as_deref() {
        Some("space") | Some("uni00A0") | None => {}
        x if x == spacename.as_deref() => {}
        Some("nonbreakingspace")
        | Some("nbspace")
        | Some("u00A0")
        | Some("u000A0")
        | Some("u0000A0") => {
            #[allow(clippy::unwrap_used)]
            problems.push(Status::warn(
                "not-recommended-00A0",
                &format!(
                    "Glyph 0x00A0 is called {}; must be named 'uni00A0'.",
                    nbspname.unwrap()
                ),
            ));
        }
        Some(other) => {
            problems.push(Status::fail(
                "non-compliant-00A0",
                &format!("Glyph 0x00A0 is called {}; must be named 'uni00A0'.", other),
            ));
        }
    }

    match spacename.as_deref() {
        Some("space") | None => {}
        Some("uni0020") | Some("u0020") | Some("u00020") | Some("u000020") => {
            #[allow(clippy::unwrap_used)]
            problems.push(Status::warn(
                "not-recommended-0020",
                &format!(
                    "Glyph 0x0020 is called {}; must be named 'space'.",
                    spacename.unwrap()
                ),
            ));
        }
        Some(other) => {
            problems.push(Status::fail(
                "non-compliant-0020",
                &format!("Glyph 0x0020 is called {}; must be named 'space'.", other),
            ));
        }
    }

    return_result(problems)
}
