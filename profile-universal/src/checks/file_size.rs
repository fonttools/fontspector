use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use humansize::{format_size, DECIMAL};

#[check(
    id = "file_size",
    rationale = "
        Serving extremely large font files causes usability issues.
        This check ensures that file sizes are reasonable.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/3320",
    title = "Ensure files are not too large."
)]
pub fn file_size(t: &Testable, context: &Context) -> CheckFnResult {
    let _ = testfont!(t); // Using this for the skip return
    let size = t.contents.len();
    let config = context.local_config("file_size");
    let fatal_size = config.get("FATAL_SIZE").and_then(|v| v.as_u64());
    let fail_size = config.get("FAIL_SIZE").and_then(|v| v.as_u64());
    let warn_size = config.get("WARN_SIZE").and_then(|v| v.as_u64());
    skip!(
        fatal_size.is_none() && fail_size.is_none() && warn_size.is_none(),
        "no-size-limits",
        "No size limits configured"
    );

    if let Some(fatal_size) = fatal_size {
        if size as u64 > fatal_size {
            return Ok(Status::just_one_fatal(
                "enormous-font",
                &format!(
                    "Font file is {}, larger than limit {}",
                    format_size(size, DECIMAL),
                    format_size(fatal_size, DECIMAL),
                ),
            ));
        }
    }
    if let Some(fail_size) = fail_size {
        if size as u64 > fail_size {
            return Ok(Status::just_one_fail(
                "massive-font",
                &format!(
                    "Font file is {}, larger than limit {}",
                    format_size(size, DECIMAL),
                    format_size(fail_size, DECIMAL),
                ),
            ));
        }
    }
    if let Some(warn_size) = warn_size {
        if size as u64 > warn_size {
            return Ok(Status::just_one_warn(
                "large-font",
                &format!(
                    "Font file is {}; ideally it should be less than {}",
                    format_size(size, DECIMAL),
                    format_size(warn_size, DECIMAL),
                ),
            ));
        }
    }

    Ok(Status::just_one_pass())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check_with_config, test_able},
        StatusCode, TestableType,
    };
    use serde_json::json;

    fn get_config() -> HashMap<String, serde_json::Value> {
        HashMap::from([(
            "file_size".to_string(),
            json!({
                "WARN_SIZE": 1048576, // 1Mb
                "FAIL_SIZE": 9437184, // 9Mb
            }),
        )])
    }

    #[test]
    fn test_file_size_pass() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check_with_config(
            super::file_size,
            TestableType::Single(&testable),
            get_config(),
        );
        assert_pass(&results);
    }

    #[test]
    fn test_file_size_warn() {
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results = run_check_with_config(
            super::file_size,
            TestableType::Single(&testable),
            get_config(),
        );
        assert_results_contain(&results, StatusCode::Warn, Some("large-font".to_string()));
    }

    #[test]
    fn test_file_size_fail() {
        let testable = test_able("cjk/NotoSansJP[wght].ttf");
        let results = run_check_with_config(
            super::file_size,
            TestableType::Single(&testable),
            get_config(),
        );
        assert_results_contain(&results, StatusCode::Fail, Some("massive-font".to_string()));
    }

    #[test]
    fn test_file_size_fatal() {
        let testable = test_able("cjk/BpmfZihiKaiStd-Regular.ttf");
        let config = HashMap::from([(
            "file_size".to_string(),
            json!({
                "WARN_SIZE": 1048576,
                "FAIL_SIZE": 5242880,
                "FATAL_SIZE": 10485760,
            }),
        )]);
        let results = run_check_with_config(
            super::file_size,
            TestableType::Single(&testable),
            config,
        );
        assert_results_contain(&results, StatusCode::Fatal, Some("enormous-font".to_string()));
    }
}
