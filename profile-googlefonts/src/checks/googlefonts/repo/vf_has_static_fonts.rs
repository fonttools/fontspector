use fontations::skrifa::string::StringId;
use fontspector_checkapi::{prelude::*, FileTypeConvert, TestFont};

fn is_manually_hinted(font: &TestFont) -> bool {
    let is_hinted = font.has_table(b"fpgm");
    let is_vtt_hinted = font.has_table(b"TSI5");
    if !is_hinted {
        return false;
    }
    if is_vtt_hinted {
        return true;
    }
    !font
        .get_name_entry_strings(StringId::VERSION_STRING)
        .any(|name| name.contains("ttfautohint"))
}

#[check(
    id = "googlefonts/repo/vf_has_static_fonts",
    rationale = "
        Variable font family directories kept in the google/fonts git repo may include
        a static/ subdir containing static fonts, if manual hinting is used on
        these fonts. Otherwise, the directory should be removed.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2654",
    title = "A static fonts directory, if present, must contain manually hinted fonts",
    implementation = "all"
)]
fn vf_has_static_fonts(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let statics = c
        .iter()
        .filter(|t| t.filename.components().any(|c| c.as_os_str() == "static"))
        .collect::<Vec<_>>();
    if !statics.is_empty()
        && statics
            .iter()
            .filter_map(|t| TTF.from_testable(t))
            .any(|t| !is_manually_hinted(&t))
    {
        return Ok(Status::just_one_warn("not-manually-hinted",
            "There is a 'static' dir but it contains fonts which are not manually hinted. Delete the directory."));
    }
    Ok(Status::just_one_pass())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::vf_has_static_fonts;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, test_able},
        StatusCode, Testable, TestableCollection, TestableType,
    };

    fn run(files: Vec<Testable>) -> Option<fontspector_checkapi::CheckResult> {
        let collection = TestableCollection::from_testables(files, None);
        fontspector_checkapi::codetesting::run_check_with_config(
            vf_has_static_fonts,
            TestableType::Collection(&collection),
            HashMap::new(),
        )
    }

    fn clone_to_static_path(src: &str, basename: &str) -> Testable {
        let t = test_able(src);
        Testable::new_with_contents(
            format!("ofl/testfamily/static/{basename}"),
            t.contents.clone(),
        )
    }

    #[test]
    fn test_check_repo_vf_has_static_fonts() {
        // Pass with no static directory files.
        assert_pass(&run(vec![
            test_able("cabinvf/Cabin[wdth,wght].ttf"),
            test_able("cabinvf/Cabin-Italic[wdth,wght].ttf"),
        ]));

        // Pass when static dir contains manually hinted statics.
        assert_pass(&run(vec![
            test_able("cabinvf/Cabin[wdth,wght].ttf"),
            test_able("cabinvf/Cabin-Italic[wdth,wght].ttf"),
            clone_to_static_path(
                "ibmplexsans-vf/IBMPlexSansVar-Roman.ttf",
                "IBMPlexSansVar-Roman.ttf",
            ),
            clone_to_static_path(
                "ibmplexsans-vf/IBMPlexSansVar-Italic.ttf",
                "IBMPlexSansVar-Italic.ttf",
            ),
        ]));

        // Warn when static dir contains non-manually-hinted fonts.
        assert_results_contain(
            &run(vec![
                test_able("cabinvf/Cabin[wdth,wght].ttf"),
                clone_to_static_path(
                    "overpassmono/OverpassMono-Regular.ttf",
                    "OverpassMono-Regular.ttf",
                ),
            ]),
            StatusCode::Warn,
            Some("not-manually-hinted".to_string()),
        );
    }
}
