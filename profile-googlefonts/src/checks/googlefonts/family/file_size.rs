use fontspector_checkapi::prelude::*;

const DEFAULT_FATAL_SIZE: u64 = 26_214_400; // 25MB

fn format_mb(bytes: u64) -> String {
    format!("{:.2} MB", bytes as f64 / 1_048_576.0)
}

#[check(
    id = "googlefonts/family/file_size",
    rationale = "
        The Google Fonts onboarding pipeline rejects font families whose
        total file size exceeds 25 MB. This family-level check sums all
        font file sizes and raises FATAL when the limit is exceeded.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4975",
    title = "Ensure total family file size is not too large.",
    implementation = "all"
)]
fn file_size(c: &TestableCollection, context: &Context) -> CheckFnResult {
    let total_size: u64 = c
        .iter()
        .filter(|t| TTF.applies(t))
        .map(|t| t.contents.len() as u64)
        .sum();
    let config = context.local_config("googlefonts/family/file_size");
    let fatal_size = config
        .get("FATAL_SIZE")
        .and_then(|v| v.as_u64())
        .unwrap_or(DEFAULT_FATAL_SIZE);

    if total_size > fatal_size {
        return Ok(Status::just_one_fatal(
            "total-size",
            &format!(
                "Total family file size is {}, exceeding the limit of {}",
                format_mb(total_size),
                format_mb(fatal_size),
            ),
        ));
    }

    Ok(Status::just_one_pass())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check_with_config, test_able},
        StatusCode, TestableCollection, TestableType,
    };
    use serde_json::json;

    #[test]
    fn test_family_file_size_pass() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let collection =
            TestableCollection::from_testables(vec![testable], Some("mada".to_string()));
        let results = run_check_with_config(
            super::file_size,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_pass(&results);
    }

    #[test]
    fn test_family_file_size_fatal() {
        let t1 = test_able("cjk/NotoSansJP[wght].ttf");
        let t2 = test_able("cjk/BpmfZihiKaiStd-Regular.ttf");
        let collection = TestableCollection::from_testables(vec![t1, t2], Some("cjk".to_string()));
        let results = run_check_with_config(
            super::file_size,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_results_contain(&results, StatusCode::Fatal, Some("total-size".to_string()));
    }

    #[test]
    fn test_family_file_size_custom_limit() {
        let testable = test_able("cjk/NotoSansJP[wght].ttf");
        let collection =
            TestableCollection::from_testables(vec![testable], Some("cjk".to_string()));
        let config = HashMap::from([(
            "googlefonts/family/file_size".to_string(),
            json!({
                "FATAL_SIZE": 1048576, // 1MB - force failure on single CJK font
            }),
        )]);
        let results = run_check_with_config(
            super::file_size,
            TestableType::Collection(&collection),
            config,
        );
        assert_results_contain(&results, StatusCode::Fatal, Some("total-size".to_string()));
    }
}
