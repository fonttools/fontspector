use crate::checks::googlefonts::metadata::family_proto;
use fontspector_checkapi::prelude::*;

#[check(
    id = "googlefonts/description/has_article",
    rationale = "
        
        Fonts may have a longer article about them, or a description, but
        not both - except for Noto fonts which should have both!
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/3841",
    proposal = "https://github.com/fonttools/fontbakery/issues/4318",
    proposal = "https://github.com/fonttools/fontbakery/issues/4702",
    title = "Check for presence of an ARTICLE.en_us.html file",
    implementation = "all"
)]
fn has_article(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let mut problems = vec![];
    let article = c.get_file("ARTICLE.en_us.html");
    let description = c.get_file("DESCRIPTION.en_us.html");
    let article_is_empty = article.map(|t| t.contents.is_empty()).unwrap_or(false);
    let description_is_empty = description.map(|t| t.contents.is_empty()).unwrap_or(false);
    let is_noto = c
        .get_file("METADATA.pb")
        .and_then(|t| family_proto(t).ok())
        .map(|msg| msg.name().starts_with("Noto "))
        .unwrap_or(false);
    if !is_noto {
        if article.is_none() {
            problems.push(Status::info(
                "missing-article",
                "This font doesn't have an ARTICLE.en_us.html file.",
            ));
        } else {
            if article_is_empty {
                problems.push(Status::fail(
                    "empty-article",
                    "The ARTICLE.en_us.html file is empty.",
                ));
            }
            if description.is_some() {
                problems.push(Status::fail(
                    "description-and-article",
                    "This font has both a DESCRIPTION.en_us.html file and an ARTICLE.en_us.html file. In this case the description must be deleted.",
                ));
            }
        }
    } else {
        if article.is_none() {
            problems.push(Status::fail(
                "missing-article",
                "This is a Noto font but it lacks an ARTICLE.en_us.html file.",
            ));
        }
        if article_is_empty {
            problems.push(Status::fail(
                "empty-article",
                "The ARTICLE.en_us.html file is empty.",
            ));
        }
        if description.is_none() || description_is_empty {
            problems.push(Status::fail(
                "missing-description",
                "This is a Noto font but it lacks a DESCRIPTION.en_us.html file.",
            ));
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use std::collections::HashMap;

    use super::has_article;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, test_able},
        StatusCode, Testable, TestableCollection, TestableType,
    };

    fn run(files: Vec<Testable>) -> Option<fontspector_checkapi::CheckResult> {
        let collection = TestableCollection::from_testables(files, None);
        fontspector_checkapi::codetesting::run_check_with_config(
            has_article,
            TestableType::Collection(&collection),
            HashMap::new(),
        )
    }

    #[test]
    fn test_check_description_has_article() {
        // Good: Noto font with both ARTICLE and DESCRIPTION
        assert_pass(&run(vec![
            test_able("notosanskhudawadi/NotoSansKhudawadi-Regular.ttf"),
            test_able("notosanskhudawadi/METADATA.pb"),
            test_able("notosanskhudawadi/DESCRIPTION.en_us.html"),
            test_able("notosanskhudawadi/article/ARTICLE.en_us.html"),
        ]));

        // Bad: Noto font missing ARTICLE
        assert_results_contain(
            &run(vec![
                test_able("noto_sans_tamil_supplement/NotoSansTamilSupplement-Regular.ttf"),
                test_able("noto_sans_tamil_supplement/METADATA.pb"),
            ]),
            StatusCode::Fail,
            Some("missing-article".to_string()),
        );

        // Bad: non-Noto font with both DESCRIPTION and ARTICLE
        assert_results_contain(
            &run(vec![
                test_able("tirodevanagarihindi/TiroDevanagariHindi-Regular.ttf"),
                test_able("tirodevanagarihindi/METADATA.pb"),
                test_able("tirodevanagarihindi/DESCRIPTION.en_us.html"),
                test_able("tirodevanagarihindi/article/ARTICLE.en_us.html"),
            ]),
            StatusCode::Fail,
            Some("description-and-article".to_string()),
        );
    }
}
