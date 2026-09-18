use fontspector_checkapi::{FontspectorError, TestFont};
use google_fonts_axisregistry::{build_fvar_instances, build_name_table, build_stat};

pub(crate) fn build_expected_font<'a>(
    font: &'a TestFont,
    siblings: &[&'a TestFont],
) -> Result<Vec<u8>, FontspectorError> {
    let mut new_binary = build_name_table(font.font_data(), None, None, &[], None)
        .map_err(|e| FontspectorError::General(e.to_string()))?;
    if font.is_variable_font() {
        new_binary = build_fvar_instances(&new_binary, None)
            .map_err(|e| FontspectorError::General(e.to_string()))?;
        // And again...
        let siblings: Vec<_> = siblings.iter().map(|x| x.font_data()).collect();
        new_binary = build_stat(&new_binary, &siblings)
            .map_err(|e| FontspectorError::General(e.to_string()))?;
    }
    Ok(new_binary)
}
