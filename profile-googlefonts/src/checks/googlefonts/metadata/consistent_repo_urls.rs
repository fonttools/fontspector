use fontspector_checkapi::prelude::*;
use scraper::{Html, Selector};
use url::Url;

use crate::checks::googlefonts::metadata::family_proto;

fn clean_url(url: &str) -> String {
    let mut cleaned = url.trim().to_string();
    if let Some(split) = cleaned.split(")").next() {
        cleaned = split.to_string();
    }
    if cleaned.ends_with('/') {
        cleaned.pop();
    }
    if cleaned.ends_with(".git") {
        let _ = cleaned.split_off(cleaned.len() - 4);
    }
    cleaned
}

// GitHub account links may credit a designer. Only links with both an
// owner and a repository identify projects that can be compared.
fn github_repository(href: &str) -> Option<String> {
    let url = Url::parse(href).ok()?;
    if !matches!(url.scheme(), "http" | "https") || url.host_str()? != "github.com" {
        return None;
    }
    let mut path = url.path_segments()?;
    let owner = path.next()?;
    let repo = path.next()?;
    let repo = repo.strip_suffix(".git").unwrap_or(repo);
    if owner.is_empty() || repo.is_empty() {
        return None;
    }
    Some(format!("{owner}/{repo}").to_ascii_lowercase())
}

#[check(
    id = "googlefonts/metadata/consistent_repo_urls",
    rationale = "
        
        Sometimes, perhaps due to copy-pasting, projects may declare different URLs
        between the font.coyright and the family.sources.repository_url fields.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4056",
    title = "METADATA.pb: Check URL on copyright string is the same as in repository_url field.",
    implementation = "all"
)]
fn consistent_repo_urls(c: &TestableCollection, context: &Context) -> CheckFnResult {
    let mdpb = c
        .get_file("METADATA.pb")
        .ok_or_else(|| FontspectorError::skip("no-mdpb", "No METADATA.pb file found"))?;
    let msg = family_proto(mdpb)?;
    let repo_url = clean_url(msg.source.repository_url());
    if repo_url.is_empty() {
        return Ok(Status::just_one_fail(
            "lacks-repo-url",
            "Please add a family.source.repository_url entry.",
        ));
    }

    let mut bad_urls = vec![];

    for font in msg.fonts {
        if let Some(httpbit) = font.copyright().split("http").nth(1) {
            let link = clean_url(&format!("http{httpbit}"));
            if link != repo_url {
                bad_urls.push(("font copyright string", link));
            }
        }
    }

    if let Some(ofl) = c.get_file("OFL.txt") {
        let license_contents = String::from_utf8(ofl.contents.clone())?;
        let first_line = license_contents.lines().next().unwrap_or_default();
        if first_line.contains("http") {
            let link = clean_url(&format!(
                "http{}",
                first_line.split("http").nth(1).unwrap_or_default()
            ));
            if link != repo_url {
                bad_urls.push(("OFL text", link));
            }
        }
    }

    if let Some(description) = c.get_file("DESCRIPTION.en_us.html") {
        let fragment = Html::parse_fragment(std::str::from_utf8(&description.contents)?);
        #[allow(clippy::unwrap_used)] // The selector is a constant.
        let anchors = Selector::parse("a[href]").unwrap();
        let expected = github_repository(&repo_url);
        for anchor in fragment.select(&anchors) {
            if let Some(href) = anchor.value().attr("href") {
                if let Some(repo) = github_repository(href) {
                    if expected.as_ref() != Some(&repo) {
                        bad_urls.push(("HTML description", href.to_owned()));
                    }
                }
            }
        }
    }

    if !bad_urls.is_empty() {
        return Ok(Status::just_one_fail(
            "mismatch",
            &format!(
                "Repository URL is {}. But:\n\n{}",
                repo_url,
                bullet_list(
                    context,
                    bad_urls
                        .iter()
                        .map(|(location, url)| format!("{location} has '{url}'"))
                )
            ),
        ));
    }
    return Ok(Status::just_one_pass());
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::consistent_repo_urls;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, test_able},
        StatusCode, Testable, TestableCollection, TestableType,
    };

    fn run(files: Vec<Testable>) -> Option<fontspector_checkapi::CheckResult> {
        let collection = TestableCollection::from_testables(files, None);
        fontspector_checkapi::codetesting::run_check_with_config(
            consistent_repo_urls,
            TestableType::Collection(&collection),
            HashMap::new(),
        )
    }

    #[test]
    fn test_check_metadata_consistent_repo_urls() {
        // This fixture is intentionally mismatched.
        assert_results_contain(
            &run(vec![test_able("delicioushandrawn/METADATA.pb")]),
            StatusCode::Fail,
            Some("mismatch".to_string()),
        );

        // Fix copyright URL to match repository_url.
        let original = test_able("delicioushandrawn/METADATA.pb");
        let metadata = String::from_utf8(original.contents.clone())
            .unwrap_or_else(|e| panic!("Invalid UTF-8 in METADATA fixture: {e}"));
        let fixed = metadata.replace(
            "https://github.com/duartp/gloock",
            "https://github.com/alphArtype/Delicious-Handrawn",
        );
        assert_pass(&run(vec![Testable::new_with_contents(
            "METADATA.pb",
            fixed.as_bytes().to_vec(),
        )]));

        // Profile and studio credits may accompany the canonical repository.
        let with_description = |html: &str| {
            run(vec![
                Testable::new_with_contents("METADATA.pb", fixed.as_bytes().to_vec()),
                Testable::new_with_contents("DESCRIPTION.en_us.html", html.as_bytes().to_vec()),
            ])
        };
        assert_pass(&with_description(
            r#"<a href="https://github.com/Designer">Designer</a>
            <a href="https://example.com/studio">Studio</a>
            <a href="https://github.com/alphArtype/Delicious-Handrawn.git/">Project</a>"#,
        ));
        // A canonical-looking link label cannot disguise a different target.
        assert_results_contain(
            &with_description(
                r#"<a href="https://github.com/other/family">github.com/alphArtype/Delicious-Handrawn</a>"#,
            ),
            StatusCode::Fail,
            Some("mismatch".to_string()),
        );

        // Empty repository URL should fail.
        let original = test_able("delicioushandrawn/METADATA.pb");
        let metadata = String::from_utf8(original.contents.clone())
            .unwrap_or_else(|e| panic!("Invalid UTF-8 in METADATA fixture: {e}"));
        let no_repo = metadata.replacen(
            "repository_url: \"https://github.com/alphArtype/Delicious-Handrawn\"",
            "repository_url: \"\"",
            1,
        );
        assert_results_contain(
            &run(vec![Testable::new_with_contents(
                "METADATA.pb",
                no_repo.into_bytes(),
            )]),
            StatusCode::Fail,
            Some("lacks-repo-url".to_string()),
        );

        // League Gothic fixture has mismatch in DESCRIPTION.
        assert_results_contain(
            &run(vec![
                test_able("leaguegothic-vf/METADATA.pb"),
                test_able("leaguegothic-vf/DESCRIPTION.en_us.html"),
            ]),
            StatusCode::Fail,
            Some("mismatch".to_string()),
        );

        // CabinVF fixture has mismatch in OFL.
        assert_results_contain(
            &run(vec![
                test_able("cabinvf/METADATA.pb"),
                test_able("cabinvf/OFL.txt"),
            ]),
            StatusCode::Fail,
            Some("mismatch".to_string()),
        );
    }
}
