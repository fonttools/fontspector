use fontspector_checkapi::{prelude::*, Metadata};

/// Check if text contains "free" as a standalone word (case-insensitive).
fn contains_word_free(text: &str) -> bool {
    text.split(|c: char| !c.is_alphanumeric())
        .any(|word| word.eq_ignore_ascii_case("free"))
}

/// Strip HTML tags from content to get plain text.
fn strip_html_tags(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for ch in html.chars() {
        if ch == '<' {
            in_tag = true;
        } else if ch == '>' {
            in_tag = false;
        } else if !in_tag {
            result.push(ch);
        }
    }
    result
}

#[check(
    id = "googlefonts/description/no_free_word",
    title = "Ensure font description does not use the word 'free'.",
    rationale = "
        All Google Fonts are libre/free, so mentioning that a font is
        'free' in its description is redundant and unhelpful. The word
        'free' should not appear in Google Fonts description files.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/219",
    applies_to = "DESC"
)]
fn no_free_word(desc: &Testable, _context: &Context) -> CheckFnResult {
    let html = String::from_utf8_lossy(&desc.contents);
    let text = strip_html_tags(&html);

    if contains_word_free(&text) {
        let message = "The description contains the word 'free'. \
             All Google Fonts are libre/free, so this is redundant."
            .to_string();
        let mut status = Status::warn("free-in-description", &message);
        status.add_metadata(Metadata::FontProblem {
            message,
            context: None,
        });
        Ok(Box::new(vec![status].into_iter()))
    } else {
        Ok(Status::just_one_pass())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::{contains_word_free, no_free_word, strip_html_tags};
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check},
        StatusCode, Testable,
    };
    use std::path::PathBuf;

    fn make_desc_testable(html_content: &str) -> Testable {
        Testable {
            filename: PathBuf::from("DESCRIPTION.en_us.html"),
            source: None,
            contents: html_content.as_bytes().to_vec(),
        }
    }

    #[test]
    fn test_no_false_positive_on_freedom() {
        assert!(!contains_word_free("This font offers typographic freedom"));
        assert!(!contains_word_free("A freestyle display font"));
        assert!(!contains_word_free("Carefree design"));
    }

    #[test]
    fn test_detects_free() {
        assert!(contains_word_free("This is a free font"));
        assert!(contains_word_free("Free to use"));
        assert!(contains_word_free("available for free"));
        assert!(contains_word_free("It is FREE"));
    }

    #[test]
    fn test_strip_html() {
        assert_eq!(strip_html_tags("<p>Hello <b>world</b></p>"), "Hello world");
        assert_eq!(strip_html_tags("<a href=\"free\">text</a>"), "text");
    }

    #[test]
    fn check_pass_no_free_word() {
        let desc =
            make_desc_testable("<p>A beautiful serif typeface designed for readability.</p>\n");
        let results = run_check(no_free_word, desc);
        assert_pass(&results);
    }

    #[test]
    fn check_pass_free_in_html_attribute_only() {
        let desc = make_desc_testable("<a href=\"https://example.com/free\">Download</a>\n");
        let results = run_check(no_free_word, desc);
        assert_pass(&results);
    }

    #[test]
    fn check_pass_free_as_substring() {
        let desc =
            make_desc_testable("<p>A carefree freestyle font offering typographic freedom.</p>\n");
        let results = run_check(no_free_word, desc);
        assert_pass(&results);
    }

    #[test]
    fn check_warns_free_in_description() {
        let desc = make_desc_testable("<p>This is a free font for everyone.</p>\n");
        let results = run_check(no_free_word, desc);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("free-in-description".to_string()),
        );
    }

    #[test]
    fn check_warns_free_uppercase() {
        let desc = make_desc_testable("<p>This font is FREE to use.</p>\n");
        let results = run_check(no_free_word, desc);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("free-in-description".to_string()),
        );
    }

    #[test]
    fn check_warns_free_after_html_stripping() {
        let desc = make_desc_testable("<p>A <b>free</b> typeface for display use.</p>\n");
        let results = run_check(no_free_word, desc);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("free-in-description".to_string()),
        );
    }
}
