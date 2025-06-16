use fontations::{
    skrifa::raw::{tables::head::MacStyle, TableProvider},
    write::from_obj::ToOwnedTable,
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "opentype/mac_style",
    title = "Checking head.macStyle value.",
    rationale = "
        The values of the flags on the macStyle entry on the 'head' OpenType table
        that describe whether a font is bold and/or italic must be coherent with the
        actual style of the font as inferred by its filename.
    ",
    hotfix = fix_mac_style,
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
)]
fn mac_style(f: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    let head = font.font().head()?;
    let style = font
        .style()
        .ok_or(FontspectorError::skip("no-style", "No style detected"))?;
    let bold = style == "Bold" || style == "BoldItalic";
    let italic = style.contains("Italic");
    let bits = head.mac_style();
    let bold_ok = bits.contains(MacStyle::BOLD) == bold;
    let italic_ok = bits.contains(MacStyle::ITALIC) == italic;
    let mut problems = vec![];
    if !bold_ok {
        problems.push(Status::fail(
            "bad-BOLD",
            &format!(
                "macStyle bold flag {} does not match font style {}",
                bits.contains(MacStyle::BOLD),
                style
            ),
        ));
    }
    if !italic_ok {
        problems.push(Status::fail(
            "bad-ITALIC",
            &format!(
                "macStyle italic flag {} does not match font style {}",
                bits.contains(MacStyle::ITALIC),
                italic
            ),
        ));
    }
    return_result(problems)
}

fn fix_mac_style(f: &mut Testable) -> FixFnResult {
    let font = testfont!(f);
    let mut head: fontations::write::tables::head::Head = font.font().head()?.to_owned_table();

    let Some(style) = font.style() else {
        return Ok(false);
    };
    let mut bits = head.mac_style;
    if style == "Bold" || style == "BoldItalic" {
        bits.insert(MacStyle::BOLD);
    } else {
        bits.remove(MacStyle::BOLD);
    }
    if style.contains("Italic") {
        bits.insert(MacStyle::ITALIC);
    } else {
        bits.remove(MacStyle::ITALIC);
    }
    head.mac_style = bits;
    f.set(font.rebuild_with_new_tables(&[head])?);
    Ok(true)
}
