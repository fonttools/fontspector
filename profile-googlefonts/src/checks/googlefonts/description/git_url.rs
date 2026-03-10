use fontspector_checkapi::prelude::*;
use scraper::{Html, Selector};

#[check(
    id = "googlefonts/description/git_url",
    rationale = "
        
        The contents of the DESCRIPTION.en-us.html file are displayed on the
        Google Fonts website in the about section of each font family specimen page.

        Since all of the Google Fonts collection is composed of libre-licensed fonts,
        this check enforces a policy that there must be a hypertext link in that page
        directing users to the repository where the font project files are
        made available.

        Such hosting is typically done on sites like Github, Gitlab, GNU Savannah or
        any other git-based version control service.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2523",
    title = "Does DESCRIPTION file contain a upstream Git repo URL?",
    applies_to = "DESC"
)]
fn git_url(desc: &Testable, _context: &Context) -> CheckFnResult {
    let mut problems = vec![];
    let fragment = Html::parse_fragment(std::str::from_utf8(&desc.contents)?);
    #[allow(clippy::unwrap_used)] // it's a constant
    let selector = Selector::parse("a[href]").unwrap();
    let git_urls = fragment
        .select(&selector)
        .flat_map(|element| element.value().attr("href"))
        .filter(|href| href.contains("://git"))
        .collect::<Vec<_>>();
    if git_urls.is_empty() {
        problems.push(Status::fail(
            "lacks-git-url",
            "Please host your font project on a public Git repo (such as GitHub or GitLab) and place a link in the DESCRIPTION.en_us.html file.",
        ));
    } else {
        for url in git_urls {
            problems.push(Status::info(
                "url-found",
                &format!("Found a git repo URL: {url}"),
            ));
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::{
        codetesting::{assert_results_contain, run_check},
        StatusCode, Testable,
    };
    use std::path::PathBuf;

    use super::git_url;

    fn make_desc(content: &str) -> Testable {
        Testable {
            filename: PathBuf::from("DESCRIPTION.en_us.html"),
            source: None,
            contents: content.as_bytes().to_vec(),
        }
    }

    #[test]
    fn test_fail_no_git_url() {
        let desc = make_desc("<p>A simple description with no links.</p>\n");
        let results = run_check(git_url, desc);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("lacks-git-url".to_string()),
        );
    }

    #[test]
    fn test_info_with_git_urls() {
        let desc = make_desc("<p>Description</p><a href='https://github.com/uswds/public-sans'>Good URL</a><a href='https://gitlab.com/smc/fonts/uroob'>Another Good One</a>\n");
        let results = run_check(git_url, desc);
        assert_results_contain(&results, StatusCode::Info, Some("url-found".to_string()));
    }

    #[test]
    fn test_fail_false_git_in_url() {
        let desc = make_desc("<a href='https://v2.designsystem.digital.gov'>Bad URL</a>\n");
        let results = run_check(git_url, desc);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("lacks-git-url".to_string()),
        );
    }
}
