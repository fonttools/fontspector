use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "control_chars",
    rationale = "
        Use of some unacceptable control characters in the U+0000 - U+001F range can
        lead to rendering issues on some platforms.

        Acceptable control characters are defined as .null (U+0000) and
        CR (U+000D) for this check.
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/2430",
    title = "Does font file include unacceptable control character glyphs?"
)]
pub fn control_chars(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let codepoints = f.codepoints(Some(context));
    let bad_characters = (0x01..0x1F)
        .filter(|&c| c != 0x0D)
        .filter(|c| codepoints.contains(c))
        .map(|c| format!("U+{:04X} ({})", c, f.glyph_name_for_unicode_synthesise(c)))
        .collect::<Vec<String>>();
    if bad_characters.is_empty() {
        Ok(Status::just_one_pass())
    } else {
        Ok(Status::just_one_fail(
            "unacceptable",
            &format!(
                "The following unacceptable control characters were identified:\n\n{}",
                bullet_list(context, &bad_characters)
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::codetesting::{
        assert_pass, assert_results_contain, run_check, test_able,
    };
    use fontspector_checkapi::StatusCode;

    #[test]
    fn test_control_chars_good() {
        let testable =
            test_able("bad_character_set/control_chars/FontbakeryTesterCCGood-Regular.ttf");
        let results = run_check(super::control_chars, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_control_chars_one_bad() {
        let testable =
            test_able("bad_character_set/control_chars/FontbakeryTesterCCOneBad-Regular.ttf");
        let results = run_check(super::control_chars, testable);
        assert_results_contain(&results, StatusCode::Fail, Some("unacceptable".to_string()));
    }

    #[test]
    fn test_control_chars_multi_bad() {
        let testable =
            test_able("bad_character_set/control_chars/FontbakeryTesterCCMultiBad-Regular.ttf");
        let results = run_check(super::control_chars, testable);
        assert_results_contain(&results, StatusCode::Fail, Some("unacceptable".to_string()));
    }
}
