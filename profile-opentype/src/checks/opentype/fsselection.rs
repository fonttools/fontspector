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

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        StatusCode, Testable,
    };

    fn test_fsselection_with_style(
        fs_selection: SelectionFlags,
        style: &str,
    ) -> Option<fontspector_checkapi::CheckResult> {
        use fontations::{
            skrifa::raw::{tables::head::MacStyle, TableProvider},
            write::from_obj::ToOwnedTable,
        };
        let mut testable = test_able("cabin/Cabin-Regular.ttf");
        // First update OS/2 fsSelection
        let new_bytes = {
            let f = TTF.from_testable(&testable).unwrap();
            let mut os2: fontations::write::tables::os2::Os2 =
                f.font().os2().unwrap().to_owned_table();
            os2.fs_selection = fs_selection;
            f.rebuild_with_new_table(&os2).unwrap()
        };
        testable.set(new_bytes);
        // Then update head macStyle to match
        let new_bytes = {
            let f = TTF.from_testable(&testable).unwrap();
            let mut head: fontations::write::tables::head::Head =
                f.font().head().unwrap().to_owned_table();
            let mut mac_style = MacStyle::empty();
            if fs_selection.contains(SelectionFlags::BOLD) {
                mac_style |= MacStyle::BOLD;
            }
            if fs_selection.contains(SelectionFlags::ITALIC) {
                mac_style |= MacStyle::ITALIC;
            }
            head.mac_style = mac_style;
            f.rebuild_with_new_table(&head).unwrap()
        };
        testable.set(new_bytes);
        // Create a new Testable with the desired filename for style detection
        let new_testable =
            Testable::new_with_contents(format!("Test-{style}.ttf"), testable.contents);
        run_check(fsselection, new_testable)
    }

    #[test]
    fn test_fsselection_regular_pass() {
        let result = test_fsselection_with_style(SelectionFlags::REGULAR, "Regular");
        assert_pass(&result);
    }

    #[test]
    fn test_fsselection_italic_pass() {
        let result = test_fsselection_with_style(SelectionFlags::ITALIC, "Italic");
        assert_pass(&result);
    }

    #[test]
    fn test_fsselection_bold_pass() {
        let result = test_fsselection_with_style(SelectionFlags::BOLD, "Bold");
        assert_pass(&result);
    }

    #[test]
    fn test_fsselection_bold_italic_pass() {
        let result = test_fsselection_with_style(
            SelectionFlags::BOLD | SelectionFlags::ITALIC,
            "BoldItalic",
        );
        assert_pass(&result);
    }

    #[test]
    fn test_fsselection_regular_marked_italic_fail() {
        let result = test_fsselection_with_style(SelectionFlags::REGULAR, "Italic");
        assert_results_contain(&result, StatusCode::Fail, Some("bad-REGULAR".to_string()));
        assert_results_contain(&result, StatusCode::Fail, Some("bad-ITALIC".to_string()));
    }

    #[test]
    fn test_fsselection_bold_for_regular_fail() {
        let result = test_fsselection_with_style(SelectionFlags::BOLD, "Regular");
        assert_results_contain(&result, StatusCode::Fail, Some("bad-REGULAR".to_string()));
        assert_results_contain(&result, StatusCode::Fail, Some("bad-BOLD".to_string()));
    }

    #[test]
    fn test_fsselection_macstyle_bold_mismatch() {
        // Set BOLD in fsSelection but not in macStyle (macStyle defaults to 0 for Cabin-Regular)
        let mut testable = test_able("cabin/Cabin-Regular.ttf");
        let new_bytes = {
            let f = TTF.from_testable(&testable).unwrap();
            let mut os2: fontations::write::tables::os2::Os2 =
                f.font().os2().unwrap().to_owned_table();
            os2.fs_selection |= SelectionFlags::BOLD;
            os2.fs_selection.remove(SelectionFlags::REGULAR);
            f.rebuild_with_new_table(&os2).unwrap()
        };
        testable.set(new_bytes);
        let new_testable =
            Testable::new_with_contents("Test-Bold.ttf".to_string(), testable.contents);
        let result = run_check(fsselection, new_testable);
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("fsselection-macstyle-bold".to_string()),
        );
    }

    #[test]
    fn test_fsselection_macstyle_italic_mismatch() {
        // Set ITALIC in fsSelection but not in macStyle
        let mut testable = test_able("cabin/Cabin-Regular.ttf");
        let new_bytes = {
            let f = TTF.from_testable(&testable).unwrap();
            let mut os2: fontations::write::tables::os2::Os2 =
                f.font().os2().unwrap().to_owned_table();
            os2.fs_selection |= SelectionFlags::ITALIC;
            os2.fs_selection.remove(SelectionFlags::REGULAR);
            f.rebuild_with_new_table(&os2).unwrap()
        };
        testable.set(new_bytes);
        let new_testable =
            Testable::new_with_contents("Test-Italic.ttf".to_string(), testable.contents);
        let result = run_check(fsselection, new_testable);
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("fsselection-macstyle-italic".to_string()),
        );
    }
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
