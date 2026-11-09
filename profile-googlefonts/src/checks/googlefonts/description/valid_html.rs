use fontspector_checkapi::prelude::*;
use scraper::{Html, Selector};

#[check(
    id = "googlefonts/description/valid_html",
    rationale = "
        
        Sometimes people write malformed HTML markup. This check should ensure the
        file is good.

        Additionally, when packaging families for being pushed to the `google/fonts`
        git repo, if there is no DESCRIPTION.en_us.html file, some older versions of
        the `add_font.py` tool insert a placeholder description file which contains
        invalid html. This file needs to either be replaced with an existing
        description file or edited by hand.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2664",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Is this a proper HTML snippet?",
    applies_to = "DESC"
)]
fn valid_html(desc: &Testable, _context: &Context) -> CheckFnResult {
    let mut problems = vec![];
    let content = std::str::from_utf8(&desc.contents)?;
    if content.contains("<html>") || content.contains("</html>") {
        problems.push(Status::fail(
            "html-tag",
            "DESCRIPTION file should not have an <html> tag, since it should only be a snippet that will later be included in the Google Fonts font family specimen webpage.",
        ));
    }
    let fragment = Html::parse_fragment(content);
    if !fragment.errors.is_empty() {
        problems.push(Status::fail(
            "malformed-snippet",
            &format!(
                "{} does not look like a proper HTML snippet. Please look for syntax errors. Maybe the following parser error message can help you find what's wrong:\n----------------\n{}\n----------------\n",
                desc.filename.as_os_str().to_string_lossy(),
                fragment.errors.join("\n")
            ),
        ));
    }
    #[allow(clippy::unwrap_used)] // it's a constant
    let selector = Selector::parse("p").unwrap();
    if fragment.select(&selector).count() == 0 {
        problems.push(Status::fail(
            "lacks-paragraph",
            &format!(
                "{} does not include an HTML <p> tag.",
                desc.filename.as_os_str().to_string_lossy()
            ),
        ));
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check},
        StatusCode, Testable,
    };
    use std::path::PathBuf;

    use super::valid_html;

    fn make_desc(content: &str) -> Testable {
        Testable {
            filename: PathBuf::from("DESCRIPTION.en_us.html"),
            source: None,
            contents: content.as_bytes().to_vec(),
        }
    }

    #[test]
    fn test_pass_good_html() {
        let desc = make_desc("<p>This is a good HTML snippet with a paragraph tag.</p>\n");
        let results = run_check(valid_html, desc);
        assert_pass(&results);
    }

    #[test]
    fn test_fail_lacks_paragraph() {
        let desc = make_desc("This is plain text without any paragraph tags.\n");
        let results = run_check(valid_html, desc);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("lacks-paragraph".to_string()),
        );
    }

    #[test]
    fn test_fail_html_tag() {
        let desc = make_desc("<html>foo</html>");
        let results = run_check(valid_html, desc);
        assert_results_contain(&results, StatusCode::Fail, Some("html-tag".to_string()));
    }

    #[test]
    fn test_pass_ampersand_without_entity() {
        let desc = make_desc("<p>This example has the & caracter, and does not escape it with an HTML entity code.</p>");
        let results = run_check(valid_html, desc);
        assert_pass(&results);
    }
}
