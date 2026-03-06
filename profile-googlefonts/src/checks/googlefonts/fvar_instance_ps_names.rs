use fontations::skrifa::MetadataProvider;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

#[check(
    id = "googlefonts/fvar_instance_ps_names",
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
            problems.push(Status::fail(
                "missing-ps-name",
                &format!(
                    "Named instance '{subfamily}' (index {index}) lacks a PostScript name entry.",
                ),
            ));
        }
    }
    return_result(problems)
}
