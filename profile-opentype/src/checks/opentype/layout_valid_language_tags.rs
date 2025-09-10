use std::collections::HashSet;

use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{constants::VALID_LANG_TAGS, prelude::*, testfont, FileTypeConvert};

#[check(
    id = "opentype/layout_valid_language_tags",
    rationale = "
        Incorrect language tags can be indications of typos, leftover debugging code
        or questionable approaches, or user error in the font editor. Such typos can
        cause features and language support to fail to work as intended.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/3355",
    title = "Does the font have any invalid language tags?"
)]
pub fn layout_valid_language_tags(f: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    let mut bad_tag = HashSet::new();
    let gsub_script_list = font
        .font()
        .gsub()
        .ok()
        .and_then(|gsub| gsub.script_list().ok());
    let gpos_script_list = font
        .font()
        .gsub()
        .ok()
        .and_then(|gsub| gsub.script_list().ok());

    for scriptlist in [gsub_script_list, gpos_script_list].iter().flatten() {
        for script in scriptlist.script_records() {
            for lang in script.script(scriptlist.offset_data())?.lang_sys_records() {
                let tag = lang.lang_sys_tag().to_string();
                if !VALID_LANG_TAGS.contains(&tag.as_str()) {
                    bad_tag.insert(tag);
                }
            }
        }
    }

    Ok(if bad_tag.is_empty() {
        Status::just_one_pass()
    } else {
        Status::just_one_fail(
            "bad-language-tags",
            &format!(
                "The following invalid language tags were found in the font: {}",
                bad_tag.into_iter().collect::<Vec<_>>().join(", ")
            ),
        )
    })
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::codetesting::{
        assert_pass, assert_results_contain, run_check, test_able,
    };
    use fontspector_checkapi::StatusCode;

    #[test]
    fn test_layout_valid_language_tags_pass() {
        let testable = test_able("nunito/Nunito-Regular.ttf");
        let results = run_check(super::layout_valid_language_tags, testable);
        assert_pass(results);
    }

    #[test]
    fn test_layout_valid_language_tags_fail() {
        let testable = test_able("rosarivo/Rosarivo-Regular.ttf");
        let results = run_check(super::layout_valid_language_tags, testable);
        assert_results_contain(
            results,
            StatusCode::Fail,
            Some("bad-language-tags".to_string()),
        );
    }
}
