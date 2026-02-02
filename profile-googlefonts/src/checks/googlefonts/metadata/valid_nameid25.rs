use fontations::{
    read::TableProvider,
    skrifa::string::StringId,
    write::{from_obj::ToOwnedTable, tables::name::Name},
};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use google_fonts_axisregistry::build_variations_ps_name;

// This is not actually googlefonts/metadata (in the sense of METADATA.pb) related, but we
// keep the check ID for legacy reasons.

#[check(
    id = "googlefonts/metadata/valid_nameid25",
    rationale = "
        
        Due to a bug in (at least) Adobe Indesign, name ID 25
        needs to be different for Italic VFs than their Upright counterparts.
        Google Fonts chooses to append \"Italic\" here.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/3024 and https://github.com/googlefonts/gftools/issues/297 and https://typo.social/@arrowtype/110430680157544757",
    title = "Check name ID 25 to end with \"Italic\" for Italic VFs.",
    hotfix = fix_nameid25,
)]
fn valid_nameid25(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    skip!(
        !f.is_variable_font(),
        "not-variable",
        "Font is not a variable font."
    );
    if let Some(style) = f.style() {
        if style.contains("Italic") {
            for name in f.get_name_entry_strings(StringId::new(25)) {
                if !name.ends_with("Italic") {
                    problems.push(Status::fail(
                        "nameid25-missing-italic",
                        "Name ID 25 must end with \"Italic\" for Italic fonts.",
                    ))
                }
                if name.contains(' ') {
                    problems.push(Status::fail(
                        "nameid25-has-spaces",
                        "Name ID 25 must not contain spaces.",
                    ))
                }
            }
        }
    }
    return_result(problems)
}

fn fix_nameid25(t: &mut Testable) -> FixFnResult {
    let f = testfont!(t);
    if !f.is_variable_font() {
        return Ok(false);
    }
    let mut name_table: Name = f.font().name()?.to_owned_table();
    build_variations_ps_name(&mut name_table, &f.font(), None);
    f.rebuild_with_new_table(&name_table)?;
    Ok(true)
}
