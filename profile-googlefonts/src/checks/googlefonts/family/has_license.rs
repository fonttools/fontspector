use crate::{seems_like_gf_repo, LICENSE};
use fontspector_checkapi::{prelude::*, skip};

#[check(
    id = "googlefonts/family/has_license",
    rationale = "
        
        A license file is required for all fonts in the Google Fonts collection.
        This checks that the font's directory contains a file named OFL.txt or
        LICENSE.txt.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Check font has a license.",
    implementation = "all"
)]
fn has_license(c: &TestableCollection, context: &Context) -> CheckFnResult {
    let licenses = c.iter().filter(|x| LICENSE.applies(x)).collect::<Vec<_>>();
    Ok(if licenses.len() > 1 {
        Status::just_one_fail(
            "multiple",
            &format!(
                "More than a single license file found:\n\n{}",
                bullet_list(context, licenses.iter().flat_map(|x| x.basename())),
            ),
        )
    } else if licenses.is_empty() {
        skip!(
            !seems_like_gf_repo(c),
            "not-in-google-fonts-repo",
            "This check is only relevant for the google/fonts repository"
        );
        Status::just_one_fail(
            "no-license",
            "No license file was found. Please add an OFL.txt or a LICENSE.txt file.",
        )
    } else {
        Status::just_one_pass()
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::has_license;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, test_file},
        StatusCode, Testable, TestableCollection, TestableType,
    };

    fn run(files: Vec<Testable>) -> Option<fontspector_checkapi::CheckResult> {
        let collection = TestableCollection::from_testables(files, None);
        fontspector_checkapi::codetesting::run_check_with_config(
            has_license,
            TestableType::Collection(&collection),
            std::collections::HashMap::new(),
        )
    }

    fn make_fake_gf_repo(case_dir: &str) -> Vec<Testable> {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let repo_path = std::env::temp_dir()
            .join(format!("fontspector-has-license-{unique}"))
            .join("ofl")
            .join("some_family");
        fs::create_dir_all(&repo_path).unwrap();
        let source = test_file(case_dir);
        let mut testables = vec![];
        for entry in fs::read_dir(source).unwrap() {
            let entry = entry.unwrap();
            let destination = repo_path.join(entry.file_name());
            fs::copy(entry.path(), &destination).unwrap();
            testables.push(Testable::new(destination).unwrap());
        }
        testables
    }

    #[test]
    fn test_check_family_has_license() {
        assert_results_contain(
            &run(make_fake_gf_repo("028/multiple")),
            StatusCode::Fail,
            Some("multiple".to_string()),
        );

        assert_results_contain(
            &run(make_fake_gf_repo("028/none")),
            StatusCode::Fail,
            Some("no-license".to_string()),
        );

        assert_pass(&run(make_fake_gf_repo("028/pass_ofl")));
        assert_pass(&run(make_fake_gf_repo("028/pass_apache")));
    }
}
