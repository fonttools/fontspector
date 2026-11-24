use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, TestFont};
use skrifa::raw::{tables::os2::SelectionFlags, TableProvider};
use skrifa::string::StringId;

/// Style-link subfamily is name ID 2 (matches FontSniffer check_style_linking).
/// Name ID 17 is typographic and is not used for fsSelection italic/bold.
fn subfamily_name(f: &TestFont<'_>) -> Option<String> {
    f.get_best_name(&[StringId::SUBFAMILY_NAME])
        .filter(|s| !s.trim().is_empty())
}

/// Italic-like: style token Italic or Oblique (excel ID 147 vocabulary).
fn name_implies_italic(subfamily: &str) -> bool {
    subfamily
        .split(|c: char| !(c.is_ascii_alphanumeric()))
        .any(|tok| {
            let t = tok.to_ascii_lowercase();
            matches!(t.as_str(), "italic" | "oblique")
        })
}

/// Style-link Bold: token Bold / BoldItalic / BoldOblique — not SemiBold/ExtraBold.
fn name_implies_bold(subfamily: &str) -> bool {
    subfamily
        .split(|c: char| !(c.is_ascii_alphanumeric()))
        .any(|tok| {
            let t = tok.to_ascii_lowercase();
            matches!(t.as_str(), "bold" | "bolditalic" | "boldoblique")
        })
}

#[check(
    id = "monotype/fsselection_subfamily",
    rationale = "
        Clearance asks whether OS/2.fsSelection italic and bold bits are consistent
        with the name-table subfamily (name ID 2). Name ID 17 is not used: RIBBI
        style-link values live in ID 2; ID 17 is the typographic subfamily.

        Unlike opentype/fsselection, this check does not use filename-driven
        TestFont::style(). macStyle cross-checks remain in opentype/fsselection.

        Reference behaviour: FontSniffer check_style_linking (italic/bold vs name ID 2).
    ",
    proposal = "https://github.com/Monotype/fontspector/issues/41",
    title = "Is OS/2.fsSelection italic/bold consistent with the subfamily name?"
)]
fn fsselection_subfamily(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let Some(subfamily) = subfamily_name(&f) else {
        return Ok(Status::just_one_fail(
            "missing-subfamily-name",
            "Could not determine the subfamily name (name ID 2).",
        ));
    };

    let fs_flags = f.font().os2()?.fs_selection();
    let italic_seen = fs_flags.contains(SelectionFlags::ITALIC);
    let bold_seen = fs_flags.contains(SelectionFlags::BOLD);
    let italic_expected = name_implies_italic(&subfamily);
    let bold_expected = name_implies_bold(&subfamily);

    let mut problems = vec![];
    if italic_seen != italic_expected {
        problems.push(Status::fail(
            "fsselection-italic-mismatch",
            &format!(
                "The italic flag in the 'OS/2' table is {} but the subfamily name \
                 '{}' does not agree (expected italic-like names such as 'Italic', \
                 'Bold Italic', 'Oblique', or 'Bold Oblique'). Mismatches cause Word \
                 and other applications to mis-label the font in style menus.",
                if italic_seen { "set" } else { "not set" },
                subfamily
            ),
        ));
    }
    if bold_seen != bold_expected {
        problems.push(Status::fail(
            "fsselection-bold-mismatch",
            &format!(
                "The bold flag in the 'OS/2' table is {} but the subfamily name \
                 '{}' does not agree (expected 'Bold' or 'Bold Italic'). Mismatches \
                 cause Word and other applications to mis-label the font in style menus.",
                if bold_seen { "set" } else { "not set" },
                subfamily
            ),
        ));
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::{fsselection_subfamily, name_implies_bold, name_implies_italic};
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, set_name_entry, test_able},
        FileTypeConvert, StatusCode, Testable, TTF,
    };
    use skrifa::raw::{tables::os2::SelectionFlags, types::NameId, TableProvider};
    use write_fonts::from_obj::ToOwnedTable;

    fn set_fs_selection(t: &mut Testable, flags: SelectionFlags) {
        let f = TTF.from_testable(t).unwrap();
        let mut os2: write_fonts::tables::os2::Os2 = f.font().os2().unwrap().to_owned_table();
        os2.fs_selection = flags;
        t.set(f.rebuild_with_new_table(&os2).unwrap());
    }

    #[test]
    fn test_token_helpers() {
        assert!(name_implies_italic("Italic"));
        assert!(name_implies_italic("Bold Italic"));
        assert!(name_implies_italic("Light Italic"));
        assert!(name_implies_italic("Oblique"));
        assert!(!name_implies_italic("Regular"));
        assert!(!name_implies_italic("Bold"));

        assert!(name_implies_bold("Bold"));
        assert!(name_implies_bold("Bold Italic"));
        assert!(!name_implies_bold("SemiBold"));
        assert!(!name_implies_bold("ExtraBold"));
        assert!(!name_implies_bold("Regular"));
        assert!(!name_implies_bold("Italic"));
    }

    #[test]
    fn test_regular_pass() {
        let mut t = test_able("cabin/Cabin-Regular.ttf");
        set_fs_selection(&mut t, SelectionFlags::REGULAR);
        assert_pass(&run_check(fsselection_subfamily, t));
    }

    #[test]
    fn test_italic_pass() {
        let mut t = test_able("cabin/Cabin-Italic.ttf");
        set_fs_selection(&mut t, SelectionFlags::ITALIC);
        assert_pass(&run_check(fsselection_subfamily, t));
    }

    #[test]
    fn test_bold_pass() {
        let mut t = test_able("cabin/Cabin-Bold.ttf");
        set_fs_selection(&mut t, SelectionFlags::BOLD);
        assert_pass(&run_check(fsselection_subfamily, t));
    }

    #[test]
    fn test_italic_mismatch_filename_looks_correct() {
        // Filename says Italic, but name ID 2 is Regular and ITALIC bit is set.
        let mut t = test_able("cabin/Cabin-Regular.ttf");
        set_fs_selection(&mut t, SelectionFlags::ITALIC);
        set_name_entry(
            &mut t,
            3,
            1,
            0x0409,
            NameId::SUBFAMILY_NAME,
            "Regular".into(),
        );
        let t = Testable::new_with_contents("Cabin-Italic.ttf".to_string(), t.contents);
        assert_results_contain(
            &run_check(fsselection_subfamily, t),
            StatusCode::Fail,
            Some("fsselection-italic-mismatch".into()),
        );
    }

    #[test]
    fn test_bold_mismatch() {
        let mut t = test_able("cabin/Cabin-Regular.ttf");
        set_fs_selection(&mut t, SelectionFlags::BOLD);
        set_name_entry(
            &mut t,
            3,
            1,
            0x0409,
            NameId::SUBFAMILY_NAME,
            "Regular".into(),
        );
        assert_results_contain(
            &run_check(fsselection_subfamily, t),
            StatusCode::Fail,
            Some("fsselection-bold-mismatch".into()),
        );
    }

    #[test]
    fn test_name_id_17_ignored() {
        // ID 2 = Regular, ID 17 = Italic, REGULAR bit set → still PASS (ID 17 ignored).
        let mut t = test_able("cabin/Cabin-Regular.ttf");
        set_name_entry(
            &mut t,
            3,
            1,
            0x0409,
            NameId::SUBFAMILY_NAME,
            "Regular".into(),
        );
        set_name_entry(
            &mut t,
            3,
            1,
            0x0409,
            NameId::TYPOGRAPHIC_SUBFAMILY_NAME,
            "Italic".into(),
        );
        set_fs_selection(&mut t, SelectionFlags::REGULAR);
        assert_pass(&run_check(fsselection_subfamily, t));
    }
}
