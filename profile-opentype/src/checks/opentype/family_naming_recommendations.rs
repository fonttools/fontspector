use fontations::skrifa::raw::types::NameId;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

const NAME_LIMITS: [(NameId, usize); 6] = [
    (NameId::FULL_NAME, 63),
    (NameId::POSTSCRIPT_NAME, 63),
    (NameId::FAMILY_NAME, 31),
    (NameId::SUBFAMILY_NAME, 31),
    (NameId::TYPOGRAPHIC_FAMILY_NAME, 31),
    (NameId::TYPOGRAPHIC_SUBFAMILY_NAME, 31),
];

#[check(
    id = "opentype/family_naming_recommendations",
    rationale = "
        This check ensures that the length of various family name and style
        name strings in the name table are within the maximum length
        recommended by the OpenType specification.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Font follows the family naming recommendations?"
)]
fn family_naming_recommendations(t: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(t);

    let mut problems = vec![];
    for (name_id, max_length) in NAME_LIMITS.iter() {
        for name in font.get_name_entry_strings(*name_id) {
            if name.len() > *max_length {
                problems.push(Status::info(
                    "bad-entries",
                    &format!(
                        "{:?} (\"{}\") is too long ({} > {})",
                        name_id,
                        name,
                        name.len(),
                        max_length
                    ),
                ));
            }
        }
    }
    return_result(problems)
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, set_name_entry, test_able},
        StatusCode,
    };

    #[test]
    fn test_family_naming_recommendations_pass() {
        let testable = test_able("mada/Mada-Medium.ttf");
        let result = run_check(family_naming_recommendations, testable);
        assert_pass(&result);
    }

    #[test]
    fn test_family_naming_recommendations_fail_postscript_too_long() {
        let mut testable = test_able("mada/Mada-Medium.ttf");
        let long_name = "A".repeat(64);
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::POSTSCRIPT_NAME,
            long_name,
        );
        let result = run_check(family_naming_recommendations, testable);
        assert_results_contain(&result, StatusCode::Info, Some("bad-entries".to_string()));
    }

    #[test]
    fn test_family_naming_recommendations_fail_family_name_too_long() {
        let mut testable = test_able("mada/Mada-Medium.ttf");
        let long_name = "A".repeat(32);
        set_name_entry(&mut testable, 3, 1, 0x0409, NameId::FAMILY_NAME, long_name);
        let result = run_check(family_naming_recommendations, testable);
        assert_results_contain(&result, StatusCode::Info, Some("bad-entries".to_string()));
    }
}
