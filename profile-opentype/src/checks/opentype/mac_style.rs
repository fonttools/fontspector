use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use skrifa::raw::{tables::head::MacStyle, TableProvider};
use write_fonts::from_obj::ToOwnedTable;

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

fn fix_mac_style(
    f: &mut Testable,
    _replies: Option<MoreInfoReplies>,
) -> Result<FixResult, FontspectorError> {
    let font = testfont!(f);
    let mut head: write_fonts::tables::head::Head = font.font().head()?.to_owned_table();

    let Some(style) = font.style() else {
        return Ok(FixResult::Unfixable);
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
    f.set(font.rebuild_with_new_table(&head)?);
    Ok(FixResult::Fixed)
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        StatusCode, Testable,
    };
    use write_fonts::{
        tables::{
            post::Post,
            head::Head,
            os2::Os2,
            maxp::Maxp,
            name::{Name, NameRecord},
        },
        types::NameId,
        FontBuilder,
    };

    fn test_mac_style_with(
        mac_style_value: MacStyle,
        style: &str,
    ) -> Option<fontspector_checkapi::CheckResult> {
        let mut testable = test_able("cabin/Cabin-Regular.ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut head: write_fonts::tables::head::Head = f.font().head().unwrap().to_owned_table();
        head.mac_style = mac_style_value;
        testable.set(f.rebuild_with_new_table(&head).unwrap());
        let new_testable = Testable::new_with_contents(
            format!("Test-{mac_style_value:?}-{style}.ttf"),
            testable.contents,
        );
        run_check(mac_style, new_testable)
    }

    #[test]
    fn test_mac_style_thin_pass() {
        let result = test_mac_style_with(MacStyle::empty(), "Thin");
        assert_pass(&result);
    }

    #[test]
    fn test_mac_style_italic_pass() {
        let result = test_mac_style_with(MacStyle::ITALIC, "Italic");
        assert_pass(&result);
    }

    #[test]
    fn test_mac_style_bold_pass() {
        let result = test_mac_style_with(MacStyle::BOLD, "Bold");
        assert_pass(&result);
    }

    #[test]
    fn test_mac_style_bold_italic_pass() {
        let result = test_mac_style_with(MacStyle::BOLD | MacStyle::ITALIC, "BoldItalic");
        assert_pass(&result);
    }

    #[test]
    fn test_mac_style_bad_bold() {
        let result = test_mac_style_with(MacStyle::empty(), "Bold");
        assert_results_contain(&result, StatusCode::Fail, Some("bad-BOLD".to_string()));
    }

    #[test]
    fn test_mac_style_bad_italic() {
        let result = test_mac_style_with(MacStyle::empty(), "Italic");
        assert_results_contain(&result, StatusCode::Fail, Some("bad-ITALIC".to_string()));
    }

    #[test]
    fn test_mac_style_italic_for_thin() {
        let result = test_mac_style_with(MacStyle::ITALIC, "Thin");
        assert_results_contain(&result, StatusCode::Fail, Some("bad-ITALIC".to_string()));
    }

    #[test]
    fn test_mac_style_bold_for_thin() {
        let result = test_mac_style_with(MacStyle::BOLD, "Thin");
        assert_results_contain(&result, StatusCode::Fail, Some("bad-BOLD".to_string()));
    }

    #[test]
    fn test_mac_style_no_style_in_filename_pass() {
        let mut testable = test_able("cabin/Cabin-Regular.ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut head: write_fonts::tables::head::Head = f.font().head().unwrap().to_owned_table();
        head.mac_style = MacStyle::empty();
        testable.set(f.rebuild_with_new_table(&head).unwrap());
        // Use a filename that doesn't encode a recognizable style
        let new_testable =
            Testable::new_with_contents("Test-0-None.ttf".to_string(), testable.contents);
        let result = run_check(mac_style, new_testable);
        assert_pass(&result);
    }

    #[test]
    fn test_mac_style_default_for_extrabold() {
        let mut builder = FontBuilder::new();
        builder.add_table(&Maxp::default()).unwrap();

        // post table is needed for is_italic within style() 
        // for checking the italic angle in the post table
        let post: Post = Post {
            ..Default::default()
        };
        builder.add_table(&post).unwrap();

        let os2: Os2 = Os2 {
            ..Default::default()
        };
        builder.add_table(&os2).unwrap();

        let head: Head = Head {
            ..Default::default()
        };
        builder.add_table(&head).unwrap();

        let mut name_table = Name::default();
        let mut new_records = Vec::new();

        let name_id1 = NameRecord::new(3, 1, 1033, NameId::new(1), "Family Extrabold".to_string().into());
        new_records.push(name_id1);

        let name_id2 = NameRecord::new(3, 1, 1033, NameId::new(2), "Regular".to_string().into());
        new_records.push(name_id2);

        let name_id4 = NameRecord::new(3, 1, 1033, NameId::new(4), "Family Extrabold".to_string().into());
        new_records.push(name_id4);

        let name_id6 = NameRecord::new(3, 1, 1033, NameId::new(6), "Family-Extrabold".to_string().into());
        new_records.push(name_id6);

        let name_id16 = NameRecord::new(3, 1, 1033, NameId::new(16), "Family".to_string().into());
        new_records.push(name_id16);

        let name_id17 = NameRecord::new(3, 1, 1033, NameId::new(17), "Extrabold".to_string().into());
        new_records.push(name_id17);

        new_records.sort();
        name_table.name_record = new_records;
        builder.add_table(&name_table).unwrap();

        let testable = Testable::new_with_contents("Family-Extrabold.ttf".to_string(), builder.build().clone());

        let font = TTF.from_testable(&testable).unwrap();
        let is_bold = font.is_bold().unwrap();
        assert_eq!(is_bold, false);

        let result = run_check(mac_style, testable);
        assert_results_contain(
            &result,
            StatusCode::Pass,
            None,
        );
    }
}
