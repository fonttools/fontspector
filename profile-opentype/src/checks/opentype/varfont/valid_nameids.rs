use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use skrifa::{raw::types::NameId, raw::TableProvider, MetadataProvider};

#[check(
    id = "opentype/varfont/valid_nameids",
    title = "Validates that all of the name IDs in an instance record are within the correct range",
    rationale = r#"
        According to the 'fvar' documentation in OpenType spec v1.9
        https://docs.microsoft.com/en-us/typography/opentype/spec/fvar

        The axisNameID field provides a name ID that can be used to obtain strings
        from the 'name' table that can be used to refer to the axis in application
        user interfaces. The name ID must be greater than 255 and less than 32768.

        The postScriptNameID field provides a name ID that can be used to obtain
        strings from the 'name' table that can be treated as equivalent to name
        ID 6 (PostScript name) strings for the given instance. Values of 6 and
        "undefined" can be used; otherwise, values must be greater than 255 and
        less than 32768.

        The subfamilyNameID field provides a name ID that can be used to obtain
        strings from the 'name' table that can be treated as equivalent to name
        ID 17 (typographic subfamily) strings for the given instance. Values of
        2 or 17 can be used; otherwise, values must be greater than 255 and less
        than 32768.
    "#,
    proposal = "https://github.com/fonttools/fontbakery/issues/3703",
    proposal = "https://github.com/Monotype/fontspector/issues/19",
    proposal = "https://github.com/Monotype/fontspector/issues/33"
)]
fn valid_nameids(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    skip!(!f.is_variable_font(), "not-variable", "Not a variable font");
    let mut problems = vec![];
    let valid_nameid = |n: NameId| (256..32768).contains(&n.to_u16());
    let valid_subfamily_nameid = |n: NameId| matches!(n.to_u16(), 2 | 17 | 256..32768);

    let nameids_in_font = f
        .font()
        .name()?
        .name_record()
        .iter()
        .map(|record| record.name_id())
        .collect::<Vec<_>>();

    // Do the axes first
    for axis in f.font().axes().iter() {
        let axis_name_id = axis.name_id();
        if !nameids_in_font.contains(&axis_name_id) {
            problems.push(Status::fail(
                "axis-nameid-missing-in-name-table",
                &format!(
                    "Axis name ID {} is missing in the name table.",
                    axis_name_id.to_u16()
                ),
            ));
        }
        if !valid_nameid(axis_name_id) {
            problems.push(Status::fail(
                &format!("invalid-axis-nameid:{}", axis_name_id.to_u16()),
                &format!(
                    "Axis name ID {} ({}) is out of range. It must be greater than 255 and less than 32768.",
                    axis_name_id.to_u16(), f.get_name_entry_strings(axis_name_id).next().unwrap_or_default()
                ),
            ));
        }
    }

    for instance in f.font().named_instances().iter() {
        let subfamily_name_id = instance.subfamily_name_id();
        if !nameids_in_font.contains(&subfamily_name_id) {
            problems.push(Status::fail(
                "subfamily-nameid-missing-in-name-table",
                &format!(
                    "Instance subfamily name ID {} is missing in the name table.",
                    subfamily_name_id.to_u16()
                ),
            ));
        }
        if let Some(n) = instance.postscript_name_id() {
            if !nameids_in_font.contains(&n) {
                problems.push(Status::fail(
                    "postscript-nameid-missing-in-name-table",
                    &format!(
                        "Instance PostScript name ID {} is missing in the name table.",
                        n.to_u16()
                    ),
                ));
            }
            if n != NameId::new(6) && !valid_nameid(n) {
                problems.push(Status::fail(
                        &format!("invalid-postscript-nameid:{}", n.to_u16()),
                        &format!(
                            "PostScript name ID {} ({}) is out of range. It must be greater than 255 and less than 32768, or 6 or 0xFFFF.",
                            n.to_u16(), f.get_name_entry_strings(n).next().unwrap_or_default()
                        ),
                    ));
            }
        }
        if !valid_subfamily_nameid(subfamily_name_id) {
            problems.push(Status::fail(
                &format!("invalid-subfamily-nameid:{}", subfamily_name_id.to_u16()),
                &format!(
                    "Instance subfamily name ID {} ({}) is out of range. It must be greater than 255 and less than 32768.",
                    subfamily_name_id.to_u16(), f.get_name_entry_strings(subfamily_name_id).next().unwrap_or_default()
                ),
            ));
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
        FileTypeConvert, StatusCode, TTF,
    };
    use write_fonts::{
        from_obj::ToOwnedTable,
        tables::{fvar::Fvar, name::Name},
        types::NameId,
    };

    fn cabin() -> fontspector_checkapi::Testable {
        test_able("cabinvf/Cabin[wdth,wght].ttf")
    }

    fn apply_fvar(testable: &mut fontspector_checkapi::Testable, update: impl FnOnce(&mut Fvar)) {
        let f = TTF.from_testable(testable).unwrap();
        let mut fvar: Fvar = f.font().fvar().unwrap().to_owned_table();
        update(&mut fvar);
        testable.set(f.rebuild_with_new_table(&fvar).unwrap());
    }

    fn ensure_name_ids(testable: &mut fontspector_checkapi::Testable, name_ids: &[u16]) {
        for &nid in name_ids {
            set_name_entry(
                testable,
                3,
                1,
                0x409,
                NameId::new(nid),
                format!("fvar name {nid}"),
            );
        }
    }

    #[test]
    fn test_valid_nameids_pass() {
        let result = run_check(valid_nameids, cabin());
        assert_pass(&result);
    }

    #[test]
    fn test_valid_nameids_fail() {
        let mut testable = test_able("mutatorsans-vf/MutatorSans-VF.ttf");
        let f = TTF.from_testable(&testable).unwrap();

        let name: Name = Name::default(); // empty 'name' table to trigger mismatch
        testable.set(f.rebuild_with_new_table(&name).unwrap());

        let result = run_check(valid_nameids, testable);
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("axis-nameid-missing-in-name-table".to_string()),
        );
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("subfamily-nameid-missing-in-name-table".to_string()),
        );
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("postscript-nameid-missing-in-name-table".to_string()),
        );
    }

    #[test]
    fn test_valid_axis_nameid() {
        let mut testable = cabin();
        apply_fvar(&mut testable, |fvar| {
            fvar.axis_instance_arrays.axes[0].axis_name_id = NameId::new(32767);
            fvar.axis_instance_arrays.axes[1].axis_name_id = NameId::new(256);
        });
        ensure_name_ids(&mut testable, &[256, 32767]);
        assert_pass(&run_check(valid_nameids, testable.clone()));

        apply_fvar(&mut testable, |fvar| {
            fvar.axis_instance_arrays.axes[0].axis_name_id = NameId::new(32768);
            fvar.axis_instance_arrays.axes[1].axis_name_id = NameId::new(255);
        });
        let result = run_check(valid_nameids, testable.clone());
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("invalid-axis-nameid:32768".to_string()),
        );
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("invalid-axis-nameid:255".to_string()),
        );

        apply_fvar(&mut testable, |fvar| {
            fvar.axis_instance_arrays.axes[0].axis_name_id = NameId::new(128);
            fvar.axis_instance_arrays.axes[1].axis_name_id = NameId::new(36000);
        });
        let result = run_check(valid_nameids, testable);
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("invalid-axis-nameid:128".to_string()),
        );
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("invalid-axis-nameid:36000".to_string()),
        );
    }

    #[test]
    fn test_valid_subfamily_nameid() {
        let mut testable = cabin();
        apply_fvar(&mut testable, |fvar| {
            let instances = &mut fvar.axis_instance_arrays.instances;
            instances[0].subfamily_name_id = NameId::new(2);
            instances[1].subfamily_name_id = NameId::new(17);
            instances[2].subfamily_name_id = NameId::new(256);
            instances[3].subfamily_name_id = NameId::new(32767);
        });
        ensure_name_ids(&mut testable, &[2, 17, 256, 32767]);
        assert_pass(&run_check(valid_nameids, testable.clone()));

        apply_fvar(&mut testable, |fvar| {
            let instances = &mut fvar.axis_instance_arrays.instances;
            instances[2].subfamily_name_id = NameId::new(255);
            instances[3].subfamily_name_id = NameId::new(32768);
        });
        let result = run_check(valid_nameids, testable.clone());
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("invalid-subfamily-nameid:255".to_string()),
        );
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("invalid-subfamily-nameid:32768".to_string()),
        );

        apply_fvar(&mut testable, |fvar| {
            let instances = &mut fvar.axis_instance_arrays.instances;
            instances[0].subfamily_name_id = NameId::new(3);
            instances[1].subfamily_name_id = NameId::new(18);
            instances[2].subfamily_name_id = NameId::new(256);
            instances[3].subfamily_name_id = NameId::new(32767);
        });
        ensure_name_ids(&mut testable, &[3, 18, 256, 32767]);
        let result = run_check(valid_nameids, testable);
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("invalid-subfamily-nameid:3".to_string()),
        );
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("invalid-subfamily-nameid:18".to_string()),
        );
    }

    #[test]
    fn test_valid_postscript_nameid() {
        let mut testable = cabin();
        apply_fvar(&mut testable, |fvar| {
            let instances = &mut fvar.axis_instance_arrays.instances;
            // All-or-none: use Some(0xFFFF) for "undefined" so instance size stays consistent.
            instances[0].post_script_name_id = Some(NameId::new(6));
            instances[1].post_script_name_id = Some(NameId::new(0xFFFF));
            instances[2].post_script_name_id = Some(NameId::new(256));
            instances[3].post_script_name_id = Some(NameId::new(32767));
        });
        ensure_name_ids(&mut testable, &[6, 256, 32767]);
        assert_pass(&run_check(valid_nameids, testable.clone()));

        apply_fvar(&mut testable, |fvar| {
            let instances = &mut fvar.axis_instance_arrays.instances;
            instances[0].post_script_name_id = Some(NameId::new(6));
            instances[1].post_script_name_id = Some(NameId::new(0xFFFF));
            instances[2].post_script_name_id = Some(NameId::new(255));
            instances[3].post_script_name_id = Some(NameId::new(32768));
        });
        let result = run_check(valid_nameids, testable.clone());
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("invalid-postscript-nameid:255".to_string()),
        );
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("invalid-postscript-nameid:32768".to_string()),
        );

        apply_fvar(&mut testable, |fvar| {
            let instances = &mut fvar.axis_instance_arrays.instances;
            instances[0].post_script_name_id = Some(NameId::new(3));
            instances[1].post_script_name_id = Some(NameId::new(18));
            instances[2].post_script_name_id = Some(NameId::new(256));
            instances[3].post_script_name_id = Some(NameId::new(32767));
        });
        ensure_name_ids(&mut testable, &[3, 18, 256, 32767]);
        let result = run_check(valid_nameids, testable);
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("invalid-postscript-nameid:3".to_string()),
        );
        assert_results_contain(
            &result,
            StatusCode::Fail,
            Some("invalid-postscript-nameid:18".to_string()),
        );
    }
}
