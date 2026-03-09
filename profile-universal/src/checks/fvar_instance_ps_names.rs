use fontations::skrifa::MetadataProvider;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "fvar_instance_ps_names",
    rationale = "
        Named instances in variable fonts should have PostScript name entries.
        Without PostScript names, some applications (notably Adobe products)
        may have trouble printing or embedding the font in PDFs.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/168",
    title = "Ensure fvar instances have PostScript names."
)]
fn fvar_instance_ps_names(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    skip!(!f.is_variable_font(), "not-variable", "Not a variable font");
    let instances = f.font().named_instances();
    skip!(
        instances.is_empty(),
        "no-instances",
        "Font has no named instances."
    );

    let mut problems = vec![];
    for (index, instance) in instances.iter().enumerate() {
        if instance.postscript_name_id().is_none() {
            let subfamily = f
                .get_name_entry_strings(instance.subfamily_name_id())
                .next()
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("instance #{index}"));
            let message = format!(
                "Named instance '{subfamily}' (index {index}) lacks a PostScript name entry.",
            );
            let mut status = Status::fail("missing-ps-name", &message);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "fvar".to_string(),
                field_name: Some("postScriptNameID".to_string()),
                actual: Some(
                    json!({ "instance": subfamily, "index": index, "postScriptNameID": null }),
                ),
                expected: Some(json!(
                    "postScriptNameID should be set for all named instances"
                )),
                message,
            });
            problems.push(status);
        }
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontations::{
        skrifa::raw::TableProvider,
        write::{from_obj::ToOwnedTable, tables::fvar::Fvar, FontBuilder},
    };
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, assert_skip, run_check, test_able},
        FileTypeConvert, StatusCode, TTF,
    };

    use super::fvar_instance_ps_names;

    #[test]
    fn test_skip_non_variable() {
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(fvar_instance_ps_names, testable);
        assert_skip(&results);
    }

    #[test]
    fn test_pass_instances_with_ps_names() {
        // Inter's instances lack PostScript names, so we add them
        let mut testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();

        let mut fvar: Fvar = f.font().fvar().unwrap().to_owned_table();
        // Add a postScriptNameID to every instance
        for instance in fvar.axis_instance_arrays.instances.iter_mut() {
            instance.post_script_name_id = Some(instance.subfamily_name_id);
        }

        let new_bytes = FontBuilder::new()
            .add_table(&fvar)
            .unwrap()
            .copy_missing_tables(f.font())
            .build();

        testable.contents = new_bytes;

        let results = run_check(fvar_instance_ps_names, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_fail_instance_missing_ps_name() {
        // Inter's instances lack PostScript names by default
        let testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let results = run_check(fvar_instance_ps_names, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("missing-ps-name".to_string()),
        );
    }

    #[test]
    fn test_skip_no_instances() {
        // Start with Inter, then clear all instances from fvar
        let mut testable = test_able("varfont/inter/Inter[slnt,wght].ttf");
        let f = TTF.from_testable(&testable).unwrap();

        let mut fvar: Fvar = f.font().fvar().unwrap().to_owned_table();
        fvar.axis_instance_arrays.instances.clear();

        let new_bytes = FontBuilder::new()
            .add_table(&fvar)
            .unwrap()
            .copy_missing_tables(f.font())
            .build();

        testable.contents = new_bytes;

        let results = run_check(fvar_instance_ps_names, testable);
        assert_skip(&results);
    }
}
