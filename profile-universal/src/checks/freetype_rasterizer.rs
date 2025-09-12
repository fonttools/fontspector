use fontspector_checkapi::prelude::*;
use freetype;

#[check(
    id = "freetype_rasterizer",
    rationale = "Malformed fonts can cause FreeType to crash.",
    proposal = "https://github.com/fonttools/fontbakery/issues/3642",
    title = "Ensure that the font can be rasterized by FreeType."
)]
pub fn freetype_rasterizer(f: &Testable, _context: &Context) -> CheckFnResult {
    let library = freetype::Library::init().map_err(|e| {
        FontspectorError::General(format!("Failed to initialize FreeType library: {e:?}"))
    })?;
    match library.new_memory_face(f.contents.clone(), 0) {
        Ok(face) => {
            if let Err(failed) = face
                .set_char_size(40 * 64, 0, 50, 0)
                .and_then(|_| face.load_char(0x2705, freetype::face::LoadFlag::RENDER))
            {
                return Ok(Status::just_one_fail("freetype-crash", &failed.to_string()));
            }
        }
        Err(err) => return Ok(Status::just_one_fail("freetype-crash", &err.to_string())),
    }
    Ok(Status::just_one_pass())
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::codetesting::{
        assert_pass, assert_results_contain, run_check, test_able,
    };
    use fontspector_checkapi::StatusCode;

    #[test]
    fn test_check_freetype_rasterizer_good() {
        let testable = test_able("cabin/Cabin-Regular.ttf");
        let results = run_check(super::freetype_rasterizer, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_check_freetype_rasterizer_good_varfont() {
        let testable = test_able("source-sans-pro/VAR/SourceSansVariable-Italic.ttf");
        let results = run_check(super::freetype_rasterizer, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_check_freetype_rasterizer_fail_ancho() {
        let testable = test_able("ancho/AnchoGX.ttf");
        let results = run_check(super::freetype_rasterizer, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("freetype-crash".to_string()),
        );
    }

    #[test]
    fn test_check_freetype_rasterizer_fail_rubik() {
        let testable = test_able("rubik/Rubik-Italic.ttf");
        let results = run_check(super::freetype_rasterizer, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("freetype-crash".to_string()),
        );
    }
}
