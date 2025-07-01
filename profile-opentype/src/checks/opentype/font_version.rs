use fontations::skrifa::{
    raw::{types::NameId, TableProvider},
    MetadataProvider,
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

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
    if (head_version - name_id_5_version).abs() > fail_tolerance {
        return Ok(Status::just_one_fail(
            "mismatch",
            &format!(
                "Font version mismatch: head table: {head_version}, name table: {name_id_5_version}"
            ),
        ));
    }
    if (head_version - name_id_5_version).abs() >= warn_tolerance {
        return Ok(Status::just_one_warn(
            "near-mismatch",
            &format!(
                "Font version mismatch: head table: {head_version}, name table: {name_id_5_version}"
            ),
        ));
    }
    Ok(Status::just_one_pass())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let v = "Version 2.000; ttfautohint (v1.8.4.7-5d5b)";
        let parsed = parse_version(v.chars());
        assert_eq!(parsed, "2.000");

        let v = "Version 1.2.3";
        let parsed = parse_version(v.chars());
        assert_eq!(parsed, "1.2");
    }
}
