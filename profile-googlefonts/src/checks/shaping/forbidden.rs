use super::{
    schema::{ShapingConfig, ShapingTest},
    ShapingCheck,
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use harfrust::{GlyphBuffer, Shaper};
use hashbrown::HashSet;
use itertools::Itertools;

#[check(
    id = "shaping/forbidden",
    rationale = "
        
        Fonts with complex layout rules can benefit from regression tests to ensure
        that the rules are behaving as designed. This checks runs a shaping test
        suite and reports if any glyphs are generated in the shaping which should
        not be produced. (For example, .notdef glyphs, visible viramas, etc.)

        Shaping test suites should be written by the font engineer and referenced
        in the FontBakery configuration file. For more information about write
        shaping test files and how to configure FontBakery to read the shaping
        test suites, see https://simoncozens.github.io/tdd-for-otl/
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/3223",
    title = "Check that no forbidden glyphs are found while shaping"
)]
fn forbidden(t: &Testable, context: &Context) -> CheckFnResult {
    let _f = testfont!(t); // We just use this to make sure it's a font
    let mut problems = vec![];
    for (filename, fails) in ForbiddenTest.run(t, context)? {
        let mut report = String::new();
        for fail in fails {
            report.push_str(&format!(
                "{} produced forbidden {}\n",
                fail.test.input, fail.detail
            ));
        }
        if !report.is_empty() {
            problems.push(Status::fail(
                "shaping-forbidden",
                &format!("{filename}: Forbidden glyphs found while shaping:\n\n{report}"),
            ))
        }
        // Add a diff table
        // draw as svg
    }
    return_result(problems)
}

fn serialize(buffer: &GlyphBuffer, shaper: &Shaper) -> String {
    let flags = harfrust::SerializeFlags::NO_POSITIONS
        | harfrust::SerializeFlags::NO_ADVANCES
        | harfrust::SerializeFlags::NO_CLUSTERS;
    buffer.serialize(shaper, flags)
}

struct ForbiddenTest;

impl ShapingCheck for ForbiddenTest {
    fn pass_fail(
        &self,
        _test: &ShapingTest,
        configuration: &ShapingConfig,
        buffer: &GlyphBuffer,
        shaper: &Shaper,
    ) -> Option<String> {
        let serialized = serialize(buffer, shaper);
        let serialized = serialized.trim_start_matches('[').trim_end_matches(']');
        let glyphs: HashSet<&str> = serialized.split('|').collect();
        let forbidden_glyphs: HashSet<&str> = configuration
            .forbidden_glyphs
            .iter()
            .map(|s| s.as_str())
            .collect();
        let found = glyphs.intersection(&forbidden_glyphs).collect_vec();
        if found.is_empty() {
            return None;
        }
        Some(found.into_iter().join(", "))
    }

    fn applies(&self, configuration: &ShapingConfig, _test: &ShapingTest) -> bool {
        !configuration.forbidden_glyphs.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use fontspector_checkapi::{
        codetesting::{
            assert_pass, assert_results_contain, run_check_with_config, test_able, test_file,
        },
        StatusCode,
    };
    use serde_json::json;

    use super::*;

    #[test]
    fn test_forbidden() {
        let testable = test_able("cjk/NotoSansJP[wght].ttf");
        let config = HashMap::from([(
            "shaping".to_string(),
            json!({
                "test_directory": test_file("shaping/forbidden")
            }),
        )]);
        let results =
            run_check_with_config(forbidden, TestableType::Single(&testable), config.clone());
        assert_pass(&results);

        let slabo = test_able("slabo/Slabo13px.ttf");
        let results = run_check_with_config(forbidden, TestableType::Single(&slabo), config);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("shaping-forbidden".to_string()),
        );
    }
}
