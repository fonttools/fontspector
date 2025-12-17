use super::{
    schema::{ShapingConfig, ShapingTest},
    ShapingCheck,
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use harfrust::{GlyphBuffer, Shaper};
use itertools::Itertools;

#[check(
    id = "shaping/regression",
    rationale = "
        
        Fonts with complex layout rules can benefit from regression tests to ensure
        that the rules are behaving as designed. This checks runs a shaping test
        suite and compares expected shaping against actual shaping, reporting
        any differences.

        Shaping test suites should be written by the font engineer and referenced
        in the FontBakery configuration file. For more information about write
        shaping test files and how to configure FontBakery to read the shaping
        test suites, see https://simoncozens.github.io/tdd-for-otl/
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/3223",
    title = "Check that texts shape as per expectation"
)]
fn regression(t: &Testable, context: &Context) -> CheckFnResult {
    let _f = testfont!(t); // We just use this to make sure it's a font
    let message = "Shaping did not match";
    let mut problems = vec![];
    for (filename, fails) in RegressionTest.run(t, context)? {
        let mut report = String::new();
        for fail in fails {
            report.push_str(&format!(
                "{}: {}{}\n{}\n\n",
                message,
                fail.test.input,
                fail.test.note(),
                fail.detail
            ));
        }
        if !report.is_empty() {
            problems.push(Status::fail(
                "shaping-regression",
                &format!("{filename}: Expected and actual shaping not matching\n\n{report}"),
            ))
        }
        // draw as svg
    }
    return_result(problems)
}

fn serialize_appropriately(buffer: &GlyphBuffer, shaper: &Shaper, test: &ShapingTest) -> String {
    let mut flags = harfrust::SerializeFlags::default();
    #[allow(clippy::unwrap_used)] // the .applies filter ensures there's an expectation
    if !test.expectation.as_ref().unwrap().contains("=") {
        flags |= harfrust::SerializeFlags::NO_POSITIONS
            | harfrust::SerializeFlags::NO_ADVANCES
            | harfrust::SerializeFlags::NO_CLUSTERS;
    }
    let serialized = buffer.serialize(shaper, flags);
    // harfrust serializes as "[a|b|c]", but test expectations are written as "a|b|c"
    if serialized.starts_with('[') && serialized.ends_with(']') {
        serialized[1..serialized.len() - 1].to_string()
    } else {
        serialized
    }
}

struct RegressionTest;

impl ShapingCheck for RegressionTest {
    fn pass_fail(
        &self,
        test: &ShapingTest,
        _configuration: &ShapingConfig,
        buffer: &GlyphBuffer,
        shaper: &Shaper,
    ) -> Option<String> {
        let serialized = serialize_appropriately(buffer, shaper, test);
        #[allow(clippy::unwrap_used)] // the .applies filter ensures there's an expectation
        let expected = test.expectation.as_ref().unwrap();
        if &serialized == expected {
            return None;
        }
        let diff = similar::TextDiff::from_chars(expected, &serialized)
            .iter_all_changes()
            .map(|d| match d.tag() {
                similar::ChangeTag::Equal => " ",
                similar::ChangeTag::Delete => "-",
                similar::ChangeTag::Insert => "",
            })
            .join("");
        let report = format!("Expected: {expected}\nGot     : {serialized}\nDiff    : {diff}\n");
        Some(report)
    }

    fn applies(&self, _configuration: &ShapingConfig, test: &ShapingTest) -> bool {
        test.expectation.is_some()
    }
}
