use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::{json, Value};

fn parse_table_tags(config: &Value, key: &str) -> Result<Vec<[u8; 4]>, Status> {
    let Some(arr) = config.get(key).and_then(|v| v.as_array()) else {
        return Err(Status::fail(
            "invalid-config",
            &format!("Missing or non-array config key '{key}' for monotype/unnecessary_tables."),
        ));
    };
    let mut tags = Vec::with_capacity(arr.len());
    for entry in arr {
        let Some(s) = entry.as_str() else {
            return Err(Status::fail(
                "invalid-config",
                &format!("Config key '{key}' must contain only strings."),
            ));
        };
        if s.len() != 4 || !s.is_ascii() {
            return Err(Status::fail(
                "invalid-config",
                &format!("Invalid table tag '{s}' in '{key}': must be exactly 4 ASCII bytes."),
            ));
        }
        let bytes = s.as_bytes();
        tags.push([bytes[0], bytes[1], bytes[2], bytes[3]]);
    }
    Ok(tags)
}

fn parse_prefixes(config: &Value, key: &str) -> Result<Vec<String>, Status> {
    let Some(arr) = config.get(key).and_then(|v| v.as_array()) else {
        return Err(Status::fail(
            "invalid-config",
            &format!("Missing or non-array config key '{key}' for monotype/unnecessary_tables."),
        ));
    };
    let mut prefixes = Vec::with_capacity(arr.len());
    for entry in arr {
        let Some(s) = entry.as_str() else {
            return Err(Status::fail(
                "invalid-config",
                &format!("Config key '{key}' must contain only strings."),
            ));
        };
        if s.is_empty() || s.len() >= 4 || !s.is_ascii() {
            return Err(Status::fail(
                "invalid-config",
                &format!(
                    "Invalid table prefix '{s}' in '{key}': must be ASCII and 1–3 characters."
                ),
            ));
        }
        prefixes.push(s.to_string());
    }
    Ok(prefixes)
}

#[check(
    id = "monotype/unnecessary_tables",
    title = "Font contains unnecessary tables?",
    rationale = "
        Unnecessary tables (private tables and embedded bitmap tables).

        This check loads tag and prefix lists from configuration. Profile defaults
        apply when keys are omitted. Example:

        ```
        [monotype/unnecessary_tables]
        unnecessary_table_tags = [ \"LINO\", \"FFTM\" ]
        unnecessary_table_tags_cff = [ \"gasp\", \"cvt \" ]
        unnecessary_table_tags_ttf = [ \"VORG\" ]
        unnecessary_table_tags_variable = [ \"kern\" ]
        unnecessary_table_prefixes = [ \"TSI\", \"MT\", \"ASC\", \"edt\", \"ABA\" ]
        ```
    ",
    proposal = "https://github.com/Monotype/fontspector/issues/17"
)]
fn unnecessary_tables(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let config = context.local_config("monotype/unnecessary_tables");

    let base_tags = match parse_table_tags(&config, "unnecessary_table_tags") {
        Ok(v) => v,
        Err(status) => return return_result(vec![status]),
    };
    let cff_tags = match parse_table_tags(&config, "unnecessary_table_tags_cff") {
        Ok(v) => v,
        Err(status) => return return_result(vec![status]),
    };
    let ttf_tags = match parse_table_tags(&config, "unnecessary_table_tags_ttf") {
        Ok(v) => v,
        Err(status) => return return_result(vec![status]),
    };
    let variable_tags = match parse_table_tags(&config, "unnecessary_table_tags_variable") {
        Ok(v) => v,
        Err(status) => return return_result(vec![status]),
    };
    let prefixes = match parse_prefixes(&config, "unnecessary_table_prefixes") {
        Ok(v) => v,
        Err(status) => return return_result(vec![status]),
    };

    let mut unnecessary_table_tags = base_tags;
    if f.is_variable_font() {
        unnecessary_table_tags.extend(variable_tags);
    }

    let version = f.font().table_directory.sfnt_version();
    if version == 0x4F54544F {
        // CFF and CFF2
        unnecessary_table_tags.extend(cff_tags);
    } else if version == 0x00010000 {
        // TrueType
        unnecessary_table_tags.extend(ttf_tags);
    } else {
        skip!("unknown-sfnt-version", "Unknown sfnt version encountered");
    }

    let mut problems: Vec<Status> = vec![];
    let mut found_unnecessary_tables: Vec<String> = vec![];

    for tag in &unnecessary_table_tags {
        if f.has_table(tag) {
            found_unnecessary_tables.push(String::from_utf8(tag.to_vec()).map_err(|_| {
                FontspectorError::General(format!("Font tag '{:?}' wasn't UTF8?", tag.to_vec()))
            })?);
        }
    }

    let font_tables = f
        .font()
        .table_directory
        .table_records()
        .iter()
        .map(|record| record.tag().to_string())
        .collect::<Vec<_>>();
    for table in font_tables {
        if prefixes.iter().any(|p| table.starts_with(p)) {
            found_unnecessary_tables.push(table);
        }
    }

    if !found_unnecessary_tables.is_empty() {
        let message = format!(
            "This font contains the following unnecessary tables:\n\n    {}",
            found_unnecessary_tables.join("\n    ")
        );
        let mut status = Status::fail("unnecessary-tables", &message);
        status.add_metadata(Metadata::FontProblem {
            message: message.clone(),
            context: Some(json!({ "unnecessary_tables": found_unnecessary_tables })),
        });
        problems.push(status);
    }

    return_result(problems)
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::unnecessary_tables;
    use fontspector_checkapi::{
        codetesting::{
            add_table, assert_messages_contain, assert_pass, assert_results_contain, remove_table,
            run_check_with_config, test_able,
        },
        StatusCode, TestableType,
    };
    use serde_json::json;

    fn default_unnecessary_tables_config() -> HashMap<String, serde_json::Value> {
        HashMap::from([(
            "monotype/unnecessary_tables".to_string(),
            json!({
                "unnecessary_table_tags": [
                    "LINO", "SLST", "FFTM", "PfEd", "TeX ", "BDF ", "bdat", "bloc", "ADFH", "SPRK"
                ],
                "unnecessary_table_tags_cff": ["gasp", "cvt ", "fpgm", "glyf", "loca", "prep"],
                "unnecessary_table_tags_ttf": ["VORG"],
                "unnecessary_table_tags_variable": ["kern"],
                "unnecessary_table_prefixes": ["TSI", "MT", "ASC", "edt", "ABA"],
            }),
        )])
    }

    fn tags_from_strs(tags: &[&str]) -> Vec<[u8; 4]> {
        tags.iter()
            .map(|s| {
                let b = s.as_bytes();
                [b[0], b[1], b[2], b[3]]
            })
            .collect()
    }

    #[test]
    fn test_invalid_tag_length_fails_config() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let config = HashMap::from([(
            "monotype/unnecessary_tables".to_string(),
            json!({
                "unnecessary_table_tags": ["TOOLONG"],
                "unnecessary_table_tags_cff": [],
                "unnecessary_table_tags_ttf": [],
                "unnecessary_table_tags_variable": [],
                "unnecessary_table_prefixes": [],
            }),
        )]);
        let results =
            run_check_with_config(unnecessary_tables, TestableType::Single(&testable), config);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("invalid-config".to_string()),
        );
    }

    #[test]
    fn test_non_ascii_tag_fails_config() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let config = HashMap::from([(
            "monotype/unnecessary_tables".to_string(),
            json!({
                "unnecessary_table_tags": ["Täg "],
                "unnecessary_table_tags_cff": [],
                "unnecessary_table_tags_ttf": [],
                "unnecessary_table_tags_variable": [],
                "unnecessary_table_prefixes": [],
            }),
        )]);
        let results =
            run_check_with_config(unnecessary_tables, TestableType::Single(&testable), config);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("invalid-config".to_string()),
        );
    }

    #[test]
    fn test_missing_config_key_fails() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let config = HashMap::from([(
            "monotype/unnecessary_tables".to_string(),
            json!({
                "unnecessary_table_tags": ["LINO"],
            }),
        )]);
        let results =
            run_check_with_config(unnecessary_tables, TestableType::Single(&testable), config);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("invalid-config".to_string()),
        );
    }

    #[test]
    fn test_invalid_prefix_length_fails_config() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let config = HashMap::from([(
            "monotype/unnecessary_tables".to_string(),
            json!({
                "unnecessary_table_tags": [],
                "unnecessary_table_tags_cff": [],
                "unnecessary_table_tags_ttf": [],
                "unnecessary_table_tags_variable": [],
                "unnecessary_table_prefixes": ["ABCD"],
            }),
        )]);
        let results =
            run_check_with_config(unnecessary_tables, TestableType::Single(&testable), config);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("invalid-config".to_string()),
        );
    }

    #[test]
    fn test_ttf_font_pass() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check_with_config(
            unnecessary_tables,
            TestableType::Single(&testable),
            default_unnecessary_tables_config(),
        );
        assert_pass(&results);
    }

    #[test]
    fn test_ttf_variable_font_pass() {
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results = run_check_with_config(
            unnecessary_tables,
            TestableType::Single(&testable),
            default_unnecessary_tables_config(),
        );
        assert_pass(&results);
    }

    #[test]
    fn test_cff_font_pass() {
        let testable = test_able("source-sans-pro/OTF/SourceSansPro-Black.otf");
        let results = run_check_with_config(
            unnecessary_tables,
            TestableType::Single(&testable),
            default_unnecessary_tables_config(),
        );
        assert_pass(&results);
    }

    #[test]
    fn test_cff2_font_pass() {
        let testable = test_able("source-sans-pro/VAR/SourceSansVariable-Italic.otf");
        let results = run_check_with_config(
            unnecessary_tables,
            TestableType::Single(&testable),
            default_unnecessary_tables_config(),
        );
        assert_pass(&results);
    }

    #[test]
    fn test_unnecessary_tables_ttf() {
        let mut testable = test_able("mada/Mada-Regular.ttf");
        let mut unnecessary_table_tags = tags_from_strs(&[
            "LINO", "SLST", "FFTM", "PfEd", "TeX ", "BDF ", "bdat", "bloc", "ADFH", "SPRK", "VORG",
        ]);
        let additional_tags = tags_from_strs(&["MT01", "TSI0", "ASC0", "edt0", "ABA0"]);
        unnecessary_table_tags.extend(additional_tags);

        for optional in unnecessary_table_tags.iter() {
            remove_table(&mut testable, optional);
        }

        for optional in unnecessary_table_tags.iter() {
            let mut test_font = testable.clone();
            add_table(&mut test_font, optional);
            let results = run_check_with_config(
                unnecessary_tables,
                TestableType::Single(&test_font),
                default_unnecessary_tables_config(),
            );
            assert_results_contain(
                &results,
                StatusCode::Fail,
                Some("unnecessary-tables".to_string()),
            );
            let tag_str = std::str::from_utf8(optional.as_slice()).unwrap();
            assert_messages_contain(&results, tag_str);
        }
    }

    #[test]
    fn test_custom_prefix_override() {
        let mut testable = test_able("mada/Mada-Regular.ttf");
        for tag in [b"MT01", b"TSI0", b"ASC0", b"edt0", b"ABA0"] {
            remove_table(&mut testable, tag);
        }
        add_table(&mut testable, b"ZZZ0");

        let mut config = default_unnecessary_tables_config();
        config.insert(
            "monotype/unnecessary_tables".to_string(),
            json!({
                "unnecessary_table_tags": [],
                "unnecessary_table_tags_cff": [],
                "unnecessary_table_tags_ttf": [],
                "unnecessary_table_tags_variable": [],
                "unnecessary_table_prefixes": ["ZZZ"],
            }),
        );

        let results =
            run_check_with_config(unnecessary_tables, TestableType::Single(&testable), config);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("unnecessary-tables".to_string()),
        );
        assert_messages_contain(&results, "ZZZ0");
    }

    #[test]
    fn test_unnecessary_tables_cff() {
        let mut testable = test_able("source-sans-pro/OTF/SourceSansPro-Black.otf");
        let unnecessary_table_tags = tags_from_strs(&[
            "LINO", "SLST", "FFTM", "PfEd", "TeX ", "BDF ", "bdat", "bloc", "ADFH", "SPRK", "gasp",
            "cvt ", "fpgm", "glyf", "loca", "prep",
        ]);

        for optional in unnecessary_table_tags.iter() {
            remove_table(&mut testable, optional);
        }

        for optional in unnecessary_table_tags.iter() {
            let mut test_font = testable.clone();
            add_table(&mut test_font, optional);
            let results = run_check_with_config(
                unnecessary_tables,
                TestableType::Single(&test_font),
                default_unnecessary_tables_config(),
            );
            assert_results_contain(
                &results,
                StatusCode::Fail,
                Some("unnecessary-tables".to_string()),
            );
            let tag_str = std::str::from_utf8(optional.as_slice()).unwrap();
            assert_messages_contain(&results, tag_str);
        }
    }
}
