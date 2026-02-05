use fontations::{
    skrifa::raw::TableProvider,
    write::{from_obj::ToOwnedTable, tables::maxp::Maxp},
};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, TestFont};

enum VersionStatus {
    Ok,
    NeedsUpgrade,
    NeedsDowngrade,
}

fn check_maxp_version(f: &TestFont) -> Result<VersionStatus, FontspectorError> {
    let maxp = f.font().maxp()?;
    let found_version = maxp.version().to_major_minor();
    Ok(if f.has_table(b"glyf") {
        if found_version == (1, 0) {
            VersionStatus::Ok
        } else {
            VersionStatus::NeedsUpgrade
        }
    } else if found_version == (0, 5) {
        VersionStatus::Ok
    } else {
        VersionStatus::NeedsDowngrade
    })
}

#[check(
    id = "opentype/maxp_version",
    title = "Check version of maxp table is correct for outlines",
    proposal = "https://github.com/fonttools/fontspector/issues/378",
    rationale = r#"
        According to the [OpenType specification](https://learn.microsoft.com/en-us/typography/opentype/spec/maxp):

        >  Fonts with CFF or CFF2 outlines must use Version 0.5 of this table, specifying only the numGlyphs field. Fonts with TrueType outlines must use Version 1.0 of this table, where all data is required.

        Acrobat Reader refuses to open PDFs with embedded fonts which have the wrong version of the maxp table.
    "#,
    hotfix = fix_maxp_version,
)]
fn maxp_version(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let status = check_maxp_version(&f)?;
    Ok(match status {
        VersionStatus::Ok => Status::just_one_pass(),
        VersionStatus::NeedsUpgrade => Status::just_one_fail(
            "version-upgrade-needed",
            "maxp table version is 1.0, but should be 0.5 for CFF outlines",
        ),
        VersionStatus::NeedsDowngrade => Status::just_one_fail(
            "version-downgrade-needed",
            "maxp table version is 0.5, but should be 1.0 for TrueType outlines",
        ),
    })
}

fn fix_maxp_version(t: &mut Testable) -> FixFnResult {
    let f = testfont!(t);
    let status = check_maxp_version(&f)?;
    match status {
        VersionStatus::Ok => Ok(false),
        VersionStatus::NeedsUpgrade => {
            // Too complex, refuse
            Err(FontspectorError::Fix(
                "Cannot upgrade maxp version from 1.0 to 0.5 automatically".to_string(),
            ))
        }
        VersionStatus::NeedsDowngrade => {
            #[allow(clippy::unwrap_used)] // We know there's a maxp table
            let mut maxp: Maxp = f.font().maxp().unwrap().to_owned_table();
            maxp.max_points = None;
            maxp.max_contours = None;
            maxp.max_composite_points = None;
            maxp.max_composite_contours = None;
            maxp.max_zones = None;
            maxp.max_twilight_points = None;
            maxp.max_storage = None;
            maxp.max_function_defs = None;
            maxp.max_instruction_defs = None;
            maxp.max_stack_elements = None;
            maxp.max_size_of_instructions = None;
            maxp.max_component_elements = None;
            maxp.max_component_depth = None;
            let font = f.rebuild_with_new_table(&maxp)?;
            t.set(font);
            Ok(true)
        }
    }
}

#[cfg(test)]
mod tests {
    use fontspector_checkapi::{
        codetesting::{run_check, test_able},
        StatusCode,
    };

    use super::*;

    #[allow(clippy::unwrap_used)]
    #[test]
    fn test_maxp_version() {
        // Source Sans Pro OTF should be OK
        let mut ssp_otf = test_able("source-sans-pro/OTF/SourceSansPro-Regular.otf");
        let result = run_check(maxp_version, ssp_otf.clone()).unwrap();
        assert_eq!(result.worst_status(), StatusCode::Pass);

        // Steal maxp table from a TTF
        let ssp_ttf = test_able("source-sans-pro/TTF/SourceSansPro-Regular.ttf");
        let maxp: Maxp = TTF
            .from_testable(&ssp_ttf)
            .unwrap()
            .font()
            .maxp()
            .unwrap()
            .to_owned_table();
        let ssp_otf_bad_maxp = TTF
            .from_testable(&ssp_otf)
            .unwrap()
            .rebuild_with_new_table(&maxp)
            .unwrap();
        ssp_otf.set(ssp_otf_bad_maxp);
        // That should fail
        let result = run_check(maxp_version, ssp_otf.clone()).unwrap();
        assert_eq!(result.worst_status(), StatusCode::Fail);

        // Fix it
        let fixed = (maxp_version.hotfix.unwrap())(&mut ssp_otf).unwrap();
        assert!(fixed);
    }
}
