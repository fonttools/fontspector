use fontspector_checkapi::prelude::*;
use scraper::{Html, Selector};

#[check(
    id = "googlefonts/description/urls",
    rationale = "
        
        The snippet of HTML in the DESCRIPTION.en_us.html file is added to the font
        family webpage on the Google Fonts website.

        Google Fonts has a content formatting policy for that snippet that expects the
        text content of anchors not to include the http:// or https:// prefixes.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/3497",
    proposal = "https://github.com/fonttools/fontbakery/issues/4283",
    title = "URLs on DESCRIPTION file must not display http(s) prefix.",
    applies_to = "DESC"
)]
fn urls(desc: &Testable, _context: &Context) -> CheckFnResult {
    let mut problems = vec![];
    let fragment = Html::parse_fragment(std::str::from_utf8(&desc.contents)?);
    #[allow(clippy::unwrap_used)] // it's a constant
    let selector = Selector::parse("a[href]").unwrap();
    for url in fragment.select(&selector) {
        if let Some(attr) = url.value().attr("href") {
            let text: String = url.text().collect();
            if text.is_empty() {
                problems.push(Status::fail(
                    "empty-link-text",
                    &format!(
                        "The following anchor in the DESCRIPTION file has empty text content:\n\n{attr}"
                    ),
                ));
            } else if text.starts_with("http://") || text.starts_with("https://") {
                problems.push(Status::fail(
                    "prefix-found",
                    &format!(
                        "Please remove the \"http(s)://\" prefix from the text content of the following anchor:\n\n{attr}"
                    ),
                ));
            }
        }
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

    use super::urls;

    fn make_desc(content: &str) -> Testable {
        Testable {
            filename: PathBuf::from("DESCRIPTION.en_us.html"),
            source: None,
            contents: content.as_bytes().to_vec(),
        }
    }

    #[test]
    fn test_pass_good_link_text() {
        let desc = make_desc("<p><a href='https://example.com'>example.com</a></p>\n");
        let results = run_check(urls, desc);
        assert_pass(&results);
    }

    #[test]
    fn test_fail_prefix_found() {
        let desc = make_desc("<p><a href='https://example.com'>https://example.com</a></p>\n");
        let results = run_check(urls, desc);
        assert_results_contain(&results, StatusCode::Fail, Some("prefix-found".to_string()));
    }

    #[test]
    fn test_fail_empty_link_text() {
        let desc = make_desc("<p><a href='https://example.com'></a></p>\n");
        let results = run_check(urls, desc);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("empty-link-text".to_string()),
        );
    }
}
