// profile-googlefonts/src/checks/googlefonts/metadata/valid_stroke_and_classifications.rs

use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

const VALID_CLASSIFICATIONS: &[&str] = &["Display", "Handwriting", "Monospace", "Symbols"];
const VALID_STROKES: &[&str] = &["Serif", "Slab Serif", "Sans Serif"];

#[check(
    id = "googlefonts/metadata/valid_stroke_and_classifications",
    title = "METADATA.pb stroke and classifications have valid values",
    rationale = "
        The METADATA.pb file can only contain specific predefined values for the
        'stroke' and 'classifications' fields:
        
        Valid stroke values:
        - Serif
        - Slab Serif
        - Sans Serif
        
        Valid classifications values:
        - Display
        - Handwriting
        - Monospace
        - Symbols
        
        Any other values are invalid and will cause issues with the Google Fonts API.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/XXXX"
)]
fn valid_stroke_and_classifications(t: &Testable, _context: &Context) -> CheckFnResult {
    // Accéder au metadata via la fonction helper du module parent
    let metadata = super::family_proto(t)?;
    
    let mut statuses = vec![];
    
    // Vérifier stroke
    if let Some(stroke) = metadata.stroke.as_ref() {
        if !stroke.is_empty() && !VALID_STROKES.contains(&stroke.as_str()) {
            statuses.push(Status::fail(
                "invalid-stroke",
                &format!(
                    "METADATA.pb stroke field contains invalid value '{}'. Valid values are: {}",
                    stroke,
                    VALID_STROKES.join(", ")
                ),
            ));
        }
    }
    
    // Vérifier classifications
    for classification in &metadata.classifications {
        if !VALID_CLASSIFICATIONS.contains(&classification.as_str()) {
            statuses.push(Status::fail(
                "invalid-classification",
                &format!(
                    "METADATA.pb classifications field contains invalid value '{}'. Valid values are: {}",
                    classification,
                    VALID_CLASSIFICATIONS.join(", ")
                ),
            ));
        }
    }
    
    // Si on a trouvé des problèmes, retourner les erreurs
    if !statuses.is_empty() {
        return Ok(Box::new(statuses.into_iter()));
    }
    
    // Tout est OK
    Ok(Box::new(std::iter::once(Status::pass())))
}