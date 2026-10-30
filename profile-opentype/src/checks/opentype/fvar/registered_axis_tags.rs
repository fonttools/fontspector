use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use skrifa::MetadataProvider;

const REGISTERED_AXIS_TAGS: &[(&str, &str)] = &[
    ("ital", "Italic"),
    ("opsz", "Optical Size"),
    ("slnt", "Slant"),
    ("wdth", "Width"),
    ("wght", "Weight"),
];

#[check(
    id = "opentype/fvar/registered_axis_tags",
    title = "Check registered axis tags",
    rationale = "The OpenType Spec says:

        The following design-variation axes and tags have been registered; 
        the linked pages provide the axis tag descriptions. 
        These are listed in alphabetic order of the tags.

        Axis tag	Name
        'ital'	Italic
        'opsz'	Optical Size
        'slnt'	Slant
        'wdth'	Width
        'wght'	Weight
    ",
    proposal = "https://learn.microsoft.com/en-us/typography/opentype/spec/dvaraxisreg#registered-axis-tags"
)]
fn registered_axis_tags(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    skip!(!f.is_variable_font(), "not-variable", "Not a variable font");

    let mut problems = vec![];

    let axes = f.font().axes();
    for axis in axes.iter() {
        let axis_tag = axis.tag().to_string();
        if let Some((_, registered_name)) = REGISTERED_AXIS_TAGS
            .iter()
            .find(|(tag, _registered_name)| *tag == axis_tag.to_lowercase())
        {
            // This is a registered axis...
            if axis_tag != axis_tag.to_lowercase() {
                problems.push(Status::fail(
                    "invalid-registered-axis-tag",
                    &format!(
                        "The registered axis tag \"{}\" is invalid because it is not in lowercase.",
                        axis_tag,
                    ),
                ));
            }

            // Check axis name matches the registered name.
            let name_id = axis.name_id();
            let axis_name = f.get_best_name(&[name_id]);

            let Some(axis_name) = axis_name else {
                problems.push(Status::fail(
                    "missing-registered-axis-name",
                    &format!(
                        "The axis tag \"{}\" name ID \"{}\" is missing in the name table.",
                        axis_tag, name_id,
                    ),
                ));
                continue;
            };
            if axis_name != *registered_name {
                problems.push(Status::warn(
                    "invalid-registered-axis-name",
                    &format!(
                        "The registered axis tag \"{}\" has an invalid name \"{}\". Expected name is \"{}\".",
                        axis_tag,
                        axis_name,
                        registered_name,
                    ),
                ));
            }
        } else {
            // This axis tag is not registered.
            if axis_tag != axis_tag.to_uppercase() {
                problems.push(Status::fail(
                    "invalid-unregistered-axis-tag",
                    &format!(
                        "The unregistered axis tag \"{}\" is invalid because it is not in uppercase.",
                        axis_tag,
                    ),
                ));
            } else {
                problems.push(Status::info(
                    "unregistered-axis-tag",
                    &format!(
                        "This variable font uses an unregistered axis tag \"{}\".",
                        axis_tag,
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
    use super::registered_axis_tags;
    use fontspector_checkapi::{
        codetesting::{
            assert_messages_contain, assert_pass, assert_results_contain, run_check,
            set_name_entry, test_able,
        },
        FileTypeConvert, StatusCode,
    };
    use skrifa::{
        raw::{types::NameId, TableProvider},
        Tag,
    };
    use write_fonts::from_obj::ToOwnedTable;

    #[test]
    fn test_registered_axis_tags_pass() {
        let testable = test_able("cabinvfbeta/CabinVFBeta.ttf");
        let result = run_check(registered_axis_tags, testable);
        assert_pass(&result);
    }

    #[test]
    fn test_registered_axis_tags_info_unregistered_axis_tags() {
        let testable = test_able("shantell/ShantellSans[BNCE,INFM,SPAC,wght].ttf");
        let result = run_check(registered_axis_tags, testable);
        assert_results_contain(
            &result,
            StatusCode::Info,
            Some("unregistered-axis-tag".to_string()),
        );
    }

    #[test]
    fn test_registered_axis_tags_invalid_tag() {
        let mut testable = test_able("shantell/ShantellSans[BNCE,INFM,SPAC,wght].ttf");
        let f = fontspector_checkapi::prelude::TTF
            .from_testable(&testable)
            .unwrap();
        let mut fvar_new: write_fonts::tables::fvar::Fvar =
            f.font().fvar().unwrap().to_owned_table();
        fvar_new.axis_instance_arrays.axes[0].axis_tag = Tag::new(b"WGHT");
        testable.set(f.rebuild_with_new_table(&fvar_new).unwrap());
        let result = run_check(registered_axis_tags, testable);
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("invalid-registered-axis-tag".to_string()),
        );
    }

    #[test]
    fn test_registered_axis_tags_invalid_unregistered_tag() {
        let mut testable = test_able("shantell/ShantellSans[BNCE,INFM,SPAC,wght].ttf");
        let f = fontspector_checkapi::prelude::TTF
            .from_testable(&testable)
            .unwrap();
        let mut fvar_new: write_fonts::tables::fvar::Fvar =
            f.font().fvar().unwrap().to_owned_table();
        fvar_new.axis_instance_arrays.axes[1].axis_tag = Tag::new(b"infm");
        testable.set(f.rebuild_with_new_table(&fvar_new).unwrap());
        let result = run_check(registered_axis_tags, testable);
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("invalid-unregistered-axis-tag".to_string()),
        );
    }

    #[test]
    fn test_registered_axis_tags_invalid_name() {
        let mut testable = test_able("shantell/ShantellSans[BNCE,INFM,SPAC,wght].ttf");
        set_name_entry(
            &mut testable,
            3,
            1,
            0x409,
            NameId::new(260),
            "This is a wrong name!".to_string(),
        );

        let result = run_check(registered_axis_tags, testable);
        assert_results_contain(
            &result,
            StatusCode::Warn,
            Some("invalid-registered-axis-name".to_string()),
        );
        assert_messages_contain(&result, "Expected name is \"Weight\"");
    }
}
