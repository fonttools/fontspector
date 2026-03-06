use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "no_vert_and_vrt2",
    title = "Ensure fonts don't have both vert and vrt2 GSUB features.",
    rationale = "
        The OpenType specification states that the 'vert' feature should
        never be used in conjunction with 'vrt2'. The 'vrt2' feature is a
        superset of 'vert' and having both present can cause issues on
        some platforms. For example, Kinto Sans fonts failed to install
        on Windows due to this problem.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/215"
)]
fn no_vert_and_vrt2(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let has_vert = f.has_feature(true, "vert");
    let has_vrt2 = f.has_feature(true, "vrt2");
    if has_vert && has_vrt2 {
        let msg = "This font has both 'vert' and 'vrt2' GSUB features. \
             The OpenType spec says 'vert' should never be used with \
             'vrt2', as 'vrt2' is a superset of 'vert'. Please remove \
             the 'vert' feature.";
        let mut status = Status::fail("has-vert-and-vrt2", msg);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "GSUB".to_string(),
            field_name: None,
            actual: Some(json!("both vert and vrt2 present")),
            expected: Some(json!("only one of vert or vrt2")),
            message: msg.to_string(),
        });
        Ok(Box::new(vec![status].into_iter()))
    } else {
        Ok(Status::just_one_pass())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::codetesting::{assert_pass, run_check, test_able};

    use super::no_vert_and_vrt2;

    #[test]
    fn test_pass_no_vert_features() {
        // A normal font without vert/vrt2 should pass
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(no_vert_and_vrt2, testable);
        assert_pass(&results);
    }
}
