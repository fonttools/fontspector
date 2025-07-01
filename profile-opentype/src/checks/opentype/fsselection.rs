use fontations::{
    skrifa::raw::{
        tables::{head::MacStyle, os2::SelectionFlags},
        TableProvider,
    },
    write::from_obj::ToOwnedTable,
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "opentype/fsselection",
    title = "Checking OS/2 fsSelection value.",
    rationale = "
        The OS/2.fsSelection field is a bit field used to specify the stylistic
        qualities of the font - in particular, it specifies to some operating
        systems whether the font is italic (bit 0), bold (bit 5) or regular
        (bit 6).

        This check verifies that the fsSelection field is set correctly for the
        font style. For a family of static fonts created in GlyphsApp, this is
        set by using the style linking checkboxes in the exports settings.

        Additionally, the bold and italic bits in OS/2.fsSelection must match
        the bold and italic bits in head.macStyle per the OpenType spec.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
    hotfix = fix_fsselection,
)]
fn fsselection(f: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    let fs_flags = font.font().os2()?.fs_selection();
    let style = font
        .style()
        .ok_or(FontspectorError::skip("no-style", "No style detected"))?;
    let bold_expected = style == "Bold" || style == "BoldItalic";
    let italic_expected = style.contains("Italic");
    let regular_expected = !bold_expected && !italic_expected;
    let mut problems = vec![];
    let bold_seen = fs_flags.contains(SelectionFlags::BOLD);
    let italic_seen = fs_flags.contains(SelectionFlags::ITALIC);
    let regular_seen = fs_flags.contains(SelectionFlags::REGULAR);
    for (flag, expected, label) in &[
        (bold_seen, bold_expected, "Bold"),
        (italic_seen, italic_expected, "Italic"),
        (regular_seen, regular_expected, "Regular"),
    ] {
        if flag != expected {
            problems.push(Status::fail(
                &format!("bad-{}", label.to_uppercase()),
                &format!("fsSelection {label} flag {flag} does not match font style {style}"),
            ));
        }
    }

    let mac_style_bits = font.font().head()?.mac_style();
    let mac_bold = mac_style_bits.contains(MacStyle::BOLD);
    let mac_italic = mac_style_bits.contains(MacStyle::ITALIC);
    for (flag, expected, label) in &[
        (bold_seen, mac_bold, "Bold"),
        (italic_seen, mac_italic, "Italic"),
    ] {
        if flag != expected {
            problems.push(Status::fail(
                &format!("fsselection-macstyle-{}", label.to_lowercase()),
                &format!("fsSelection {label} flag {flag} does not match macStyle {expected} flag"),
            ));
        }
    }
    return_result(problems)
}

fn fix_fsselection(t: &mut Testable) -> FixFnResult {
    let f = testfont!(t);
    let Some(style) = f.style() else {
        return Ok(false);
    };
    let mut os2: fontations::write::tables::os2::Os2 = f.font().os2()?.to_owned_table();
    os2.fs_selection &= SelectionFlags::USE_TYPO_METRICS;
    let bold_expected = style == "Bold" || style == "BoldItalic";
    let italic_expected = style.contains("Italic");
    let regular_expected = !bold_expected && !italic_expected;
    if bold_expected {
        os2.fs_selection |= SelectionFlags::BOLD;
    }
    if italic_expected {
        os2.fs_selection |= SelectionFlags::ITALIC;
    }
    if regular_expected {
        os2.fs_selection |= SelectionFlags::REGULAR;
    }

    t.set(f.rebuild_with_new_table(&os2)?);
    Ok(true)
}
