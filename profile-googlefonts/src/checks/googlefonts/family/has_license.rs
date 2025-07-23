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
