use std::collections::HashSet;

use fontations::skrifa::raw::{types::Version16Dot16, TableProvider};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Source, SourceFile};
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
    proposal = "https://github.com/fonttools/fontbakery/issues/2832",
    fix_source = sourcefix_valid_glyphnames,
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
                &format!("Glyph 0x00A0 is called {other}; must be named 'uni00A0'."),
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
                &format!("Glyph 0x0020 is called {other}; must be named 'space'."),
            ));
        }
    }

    return_result(problems)
}

fn sourcefix_valid_glyphnames(s: &mut SourceFile) -> FixFnResult {
    fn fix_a_ufo(font: &mut norad::Font) -> FixFnResult {
        let mut changed = false;
        let mut renames = vec![];
        let layer = font.default_layer_mut();
        if let Some(space_glyph) = layer
            .iter()
            .find(|g| g.codepoints.contains(' '))
            .map(|x| x.name().as_str())
        {
            if space_glyph != "space" {
                renames.push((space_glyph.to_string(), "space"));
            }
        }
        if let Some(nbspace_glyph) = layer
            .iter()
            .find(|g| g.codepoints.contains(0xa0 as char))
            .map(|x| x.name().as_str())
        {
            if nbspace_glyph != "nbspace" {
                renames.push((nbspace_glyph.to_string(), "nbspace"));
            }
        }
        for (old_name, new_name) in renames {
            layer.rename_glyph(&old_name, new_name, true).map_err(|e| {
                FontspectorError::Fix(format!(
                    "Failed to rename glyph {old_name} to {new_name}: {e}"
                ))
            })?;
            changed = true;
        }

        Ok(changed)
    }
    match s.source {
        Source::Ufo(ref mut font) => fix_a_ufo(font),
        Source::Designspace(ref mut ds) => ds.apply_fix(&fix_a_ufo),
        Source::Glyphs(ref mut font) => {
            let font = font.font_mut();
            let mut changed = false;
            for glyph in font.glyphs_mut().iter_mut() {
                if glyph.unicode().contains(&0x20u32) && glyph.name() != "space" {
                    glyph.set_name("space".to_string());
                    changed = true;
                }
                if glyph.unicode().contains(&0xa0u32) && glyph.name() != "nbspace" {
                    glyph.set_name("nbspace".to_string());
                    changed = true;
                }
            }
            Ok(changed)
        }
    }
}
