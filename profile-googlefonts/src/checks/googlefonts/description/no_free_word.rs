use fontspector_checkapi::prelude::*;

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
        Ok(Status::just_one_warn(
            "free-in-description",
            "The description contains the word 'free'. \
             All Google Fonts are libre/free, so this is redundant.",
        ))
    } else {
        Ok(Status::just_one_pass())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
