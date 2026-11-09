use fontspector_checkapi::prelude::*;

#[check(
    id = "googlefonts/description/min_length",
    title = "DESCRIPTION.en_us.html must have more than 200 bytes.",
    rationale = "
        The DESCRIPTION.en_us.html file is intended to provide a brief overview of
        the font family. It should be long enough to be useful to users, but not so
        long that it becomes overwhelming.

        We chose 200 bytes as a minimum length because it suggests that someone has
        taken the time to write \"something sensible\" about the font.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
    applies_to = "DESC"
)]
fn min_length(desc: &Testable, _context: &Context) -> CheckFnResult {
    Ok(if desc.contents.len() <= 200 {
        Status::just_one_fail(
            "too-short",
            "DESCRIPTION.en_us.html must have size larger than 200 bytes.",
        )
    } else {
        Status::just_one_pass()
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check},
        StatusCode, Testable,
    };
    use std::path::PathBuf;

    use super::min_length;

    fn make_desc(content: &str) -> Testable {
        Testable {
            filename: PathBuf::from("DESCRIPTION.en_us.html"),
            source: None,
            contents: content.as_bytes().to_vec(),
        }
    }

    #[test]
    fn test_fail_199_bytes() {
        let desc = make_desc(&"a".repeat(199));
        let results = run_check(min_length, desc);
        assert_results_contain(&results, StatusCode::Fail, Some("too-short".to_string()));
    }

    #[test]
    fn test_fail_200_bytes() {
        let desc = make_desc(&"a".repeat(200));
        let results = run_check(min_length, desc);
        assert_results_contain(&results, StatusCode::Fail, Some("too-short".to_string()));
    }

    #[test]
    fn test_pass_201_bytes() {
        let desc = make_desc(&"a".repeat(201));
        let results = run_check(min_length, desc);
        assert_pass(&results);
    }
}
