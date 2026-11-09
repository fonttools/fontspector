use fontspector_checkapi::prelude::*;

#[check(
    id = "googlefonts/description/eof_linebreak",
    title = "DESCRIPTION.en_us.html should end in a linebreak.",
    rationale = "
        Some older text-handling tools sometimes misbehave if the last line of data
        in a text file is not terminated with a newline character (also known as '\\n').

        We know that this is a very small detail, but for the sake of keeping all
        DESCRIPTION.en_us.html files uniformly formatted throughout the GFonts
        collection, we chose to adopt the practice of placing this final linebreak
        character on them.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2879",
    applies_to = "DESC"
)]
fn eof_linebreak(desc: &Testable, _context: &Context) -> CheckFnResult {
    Ok(if !desc.contents.ends_with(b"\n") {
        Status::just_one_warn(
            "missing-eof-linebreak",
            "The last characther on DESCRIPTION.en_us.html \
             is not a line-break. Please add it.",
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

    use super::eof_linebreak;

    fn make_desc(content: &str) -> Testable {
        Testable {
            filename: PathBuf::from("DESCRIPTION.en_us.html"),
            source: None,
            contents: content.as_bytes().to_vec(),
        }
    }

    #[test]
    fn test_warn_missing_eof_linebreak() {
        let desc = make_desc(
            "We want to avoid description files\nwithout an end-of-file linebreak\nlike this one.",
        );
        let results = run_check(eof_linebreak, desc);
        assert_results_contain(
            &results,
            StatusCode::Warn,
            Some("missing-eof-linebreak".to_string()),
        );
    }

    #[test]
    fn test_pass_with_eof_linebreak() {
        let desc = make_desc("On the other hand, this one\nis good enough.\n");
        let results = run_check(eof_linebreak, desc);
        assert_pass(&results);
    }
}
