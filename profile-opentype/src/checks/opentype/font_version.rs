use fontations::skrifa::{
    raw::{types::NameId, TableProvider},
    MetadataProvider,
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

fn parse_version(v: impl Iterator<Item = char>) -> String {
    let mut result = String::new();
    let mut periods = 0;
    for c in v.skip_while(|c| !c.is_ascii_digit()) {
        if c.is_ascii_digit() {
            result.push(c);
        } else if c == '.' {
            periods += 1;
            if periods > 1 {
                break;
            }
            result.push(c);
        } else {
            break;
        }
    }
    result
}

#[check(
    id = "opentype/font_version",
    title = "Checking font version fields (head and name table).",
    rationale = "
        The OpenType specification provides for two fields which contain
        the version number of the font: fontRevision in the head table,
        and nameID 5 in the name table. If these fields do not match,
        different applications will report different version numbers for
        the font.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
)]
fn font_version(f: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    let head_version = font.font().head()?.font_revision().to_f32();
    let name_id_5_version = font
        .font()
        .localized_strings(NameId::VERSION_STRING)
        .english_or_first()
        .ok_or(FontspectorError::General("No name ID 5".to_string()))?
        .chars();
    let name_id_5_version_str = parse_version(name_id_5_version);

    if name_id_5_version_str.is_empty() {
        return Err(FontspectorError::General(
            "No version string in name table".to_string(),
        ));
    }
    let name_id_5_version = name_id_5_version_str.parse::<f32>().map_err(|e| {
        FontspectorError::General(format!("Could not parse name ID 5 version as float: {e}"))
    })?;
    let warn_tolerance = 1.0 / (0x10000 as f32);
    let fail_tolerance = 1.0 / 2000.0;
    let mut problems = vec![];
    if (head_version - name_id_5_version).abs() > fail_tolerance {
        let msg = format!(
            "Font version mismatch: head table: {head_version}, name table: {name_id_5_version}"
        );
        let mut status = Status::fail("mismatch", &msg);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "head/name".to_string(),
            field_name: Some("fontRevision/version string".to_string()),
            actual: Some(json!({
                "head_version": head_version,
                "name_version": name_id_5_version
            })),
            expected: Some(json!("Versions should match")),
            message: msg,
        });
        problems.push(status);
    } else if (head_version - name_id_5_version).abs() >= warn_tolerance {
        let msg = format!(
            "Font version mismatch: head table: {head_version}, name table: {name_id_5_version}"
        );
        let mut status = Status::warn("near-mismatch", &msg);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "head/name".to_string(),
            field_name: Some("fontRevision/version string".to_string()),
            actual: Some(json!({
                "head_version": head_version,
                "name_version": name_id_5_version
            })),
            expected: Some(json!("Versions should match exactly")),
            message: msg,
        });
        problems.push(status);
    }
    return_result(problems)
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use fontations::{skrifa::raw::TableProvider, write::from_obj::ToOwnedTable};
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, set_name_entry, test_able},
        StatusCode,
    };

    #[test]
    fn test_parser() {
        let v = "Version 2.000; ttfautohint (v1.8.4.7-5d5b)";
        let parsed = parse_version(v.chars());
        assert_eq!(parsed, "2.000");

        let v = "Version 1.2.3";
        let parsed = parse_version(v.chars());
        assert_eq!(parsed, "1.2");
    }

    #[test]
    fn test_font_version_pass() {
        let testable = test_able("nunito/Nunito-Regular.ttf");
        let result = run_check(font_version, testable);
        assert_pass(&result);
    }

    #[test]
    fn test_font_version_near_mismatch() {
        let mut testable = test_able("nunito/Nunito-Regular.ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut head: fontations::write::tables::head::Head =
            f.font().head().unwrap().to_owned_table();
        head.font_revision = fontations::skrifa::raw::types::Fixed::from_f64(1.00098);
        testable.set(f.rebuild_with_new_table(&head).unwrap());
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::VERSION_STRING,
            "Version 1.001".to_string(),
        );
        set_name_entry(
            &mut testable,
            1,
            0,
            0,
            NameId::VERSION_STRING,
            "Version 1.001".to_string(),
        );
        let result = run_check(font_version, testable);
        assert_results_contain(&result, StatusCode::Warn, Some("near-mismatch".to_string()));
    }

    #[test]
    fn test_font_version_fail_mismatch() {
        let mut testable = test_able("nunito/Nunito-Regular.ttf");
        let f = TTF.from_testable(&testable).unwrap();
        let mut head: fontations::write::tables::head::Head =
            f.font().head().unwrap().to_owned_table();
        head.font_revision = fontations::skrifa::raw::types::Fixed::from_f64(3.1);
        testable.set(f.rebuild_with_new_table(&head).unwrap());
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::VERSION_STRING,
            "Version 3.000".to_string(),
        );
        set_name_entry(
            &mut testable,
            1,
            0,
            0,
            NameId::VERSION_STRING,
            "Version 3.000".to_string(),
        );
        let result = run_check(font_version, testable);
        assert_results_contain(&result, StatusCode::Fail, Some("mismatch".to_string()));
    }
}
