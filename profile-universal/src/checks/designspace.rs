use fontdrasil::coords::{NormalizedCoord, NormalizedLocation};
use fontspector_checkapi::{FontspectorError, TestFont};
use skrifa::{setting::VariationSetting, Tag};

/// Convert a normalized designspace location (one value per fvar axis,
/// as found in gvar tuple peaks) into user-space variation settings.
///
/// Axes at their default (0.0) are omitted from the result.
pub(crate) fn denormalize_location(
    font: &TestFont,
    tuple: &[f32],
) -> Result<Vec<VariationSetting>, FontspectorError> {
    let all_axes = font.fontdrasil_axes()?.ok_or(FontspectorError::General(
        "Axes went away between function calls".to_string(),
    ))?;
    let loc: NormalizedLocation = all_axes
        .iter()
        .zip(tuple)
        .filter(|&(_axis, peak)| *peak != 0.0)
        .map(|(axis, peak)| (axis.tag, NormalizedCoord::new(*peak as f64)))
        .collect();
    let user = loc.to_user(&all_axes).map_err(|e| {
        FontspectorError::General(format!(
            "Failed to convert location to user coordinates: {}",
            e
        ))
    })?;
    // And now back to skrifa
    Ok(user
        .iter()
        .map(|(tag, value)| VariationSetting {
            selector: Tag::new(&tag.to_be_bytes()), // different version madness
            value: value.to_f64() as f32,
        })
        .collect())
}
