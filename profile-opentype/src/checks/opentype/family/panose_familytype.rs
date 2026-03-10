use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, FileTypeConvert, StatusCode, TestFont};

#[check(
    id = "opentype/family/panose_familytype",
    title = "Fonts have consistent PANOSE family type?",
    rationale = "
        The [PANOSE value](https://monotype.github.io/panose/) in the OS/2 table is a
        way of classifying a font based on its visual appearance and characteristics.

        The first field in the PANOSE classification is the family type: 2 means Latin
        Text, 3 means Latin Script, 4 means Latin Decorative, 5 means Latin Symbol.
        This check ensures that within a family, all fonts have the same family type.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
    implementation = "all"
)]
fn panose_familytype(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let fonts = TTF.from_collection(c);
    let mut problems = vec![];
    let (ok, missing): (Vec<&TestFont>, Vec<&TestFont>) =
        fonts.iter().partition(|f| f.font().os2().is_ok());
    for font in missing {
        problems.push(Status::error(
            None,
            &format!(
                "Font {} is missing an OS/2 table",
                font.filename.to_string_lossy()
            ),
        ));
    }
    if !problems.is_empty() {
        return return_result(problems);
    }
    let panose_values = ok
        .iter()
        .map(|f| {
            #[allow(clippy::unwrap_used, clippy::indexing_slicing)] // Surely we can index a PANOSE.
            let panose_first = f.font().os2().unwrap().panose_10()[0];
            let panose_name = match panose_first {
                2 => "Latin Text".to_string(),
                3 => "Latin Script".to_string(),
                4 => "Latin Decorative".to_string(),
                5 => "Latin Symbol".to_string(),
                _ => format!("Unknown ({panose_first})"),
            };

            #[allow(clippy::unwrap_used)]
            (
                panose_first,
                panose_name,
                f.filename.file_name().unwrap().to_string_lossy(),
            )
        })
        .collect::<Vec<_>>();
    assert_all_the_same(
        _context,
        &panose_values,
        "inconsistency",
        "PANOSE family type is not the same across this family. In order to fix this, please make sure that the panose.bFamilyType value is the same in the OS/2 table of all of this family's font files.",
        StatusCode::Warn
    )
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use fontations::{skrifa::raw::TableProvider, write::from_obj::ToOwnedTable};
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check_with_config, test_able},
        StatusCode, TestableCollection, TestableType,
    };
    use std::collections::HashMap;

    #[test]
    fn test_panose_familytype_pass() {
        let testables: Vec<_> = [
            "mada/Mada-Black.ttf",
            "mada/Mada-ExtraLight.ttf",
            "mada/Mada-Medium.ttf",
            "mada/Mada-SemiBold.ttf",
            "mada/Mada-Bold.ttf",
            "mada/Mada-Light.ttf",
            "mada/Mada-Regular.ttf",
        ]
        .iter()
        .map(|f| test_able(f))
        .collect();
        let collection = TestableCollection {
            testables,
            directory: "".to_string(),
        };
        let result = run_check_with_config(
            panose_familytype,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_pass(&result);
    }

    #[test]
    fn test_panose_familytype_inconsistency() {
        let mut testables: Vec<_> = [
            "mada/Mada-Black.ttf",
            "mada/Mada-ExtraLight.ttf",
            "mada/Mada-Medium.ttf",
            "mada/Mada-SemiBold.ttf",
            "mada/Mada-Bold.ttf",
            "mada/Mada-Light.ttf",
            "mada/Mada-Regular.ttf",
        ]
        .iter()
        .map(|f| test_able(f))
        .collect();
        // Modify the first font's panose family type
        let new_bytes = {
            let f = TTF.from_testable(&testables[0]).unwrap();
            let mut os2: fontations::write::tables::os2::Os2 =
                f.font().os2().unwrap().to_owned_table();
            os2.panose_10[0] = os2.panose_10[0].wrapping_add(1);
            f.rebuild_with_new_table(&os2).unwrap()
        };
        testables[0].set(new_bytes);
        let collection = TestableCollection {
            testables,
            directory: "".to_string(),
        };
        let result = run_check_with_config(
            panose_familytype,
            TestableType::Collection(&collection),
            HashMap::new(),
        );
        assert_results_contain(&result, StatusCode::Warn, Some("inconsistency".to_string()));
    }
}
