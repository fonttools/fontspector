use fontations::skrifa::raw::{tables::os2::SelectionFlags, types::NameId, TableProvider};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "opentype/fsselection_wws",
    rationale = "
        According to the OpenType specification, OS/2.fsSelection bit 8 (WWS)
        should be set if the font has name table strings consistent with a
        weight/width/slope family without requiring use of name IDs 21 and 22.

        Conversely, if name IDs 21 and 22 are present (indicating the font
        names are not WWS-conformant), the WWS bit should not be set.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/577",
    title = "Check that OS/2 fsSelection WWS bit is set correctly."
)]
fn fsselection_wws(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let fs_flags = f.font().os2()?.fs_selection();
    let wws_set = fs_flags.contains(SelectionFlags::WWS);
    let has_wws_family = f
        .get_name_entry_strings(NameId::WWS_FAMILY_NAME)
        .next()
        .is_some();
    let has_wws_subfamily = f
        .get_name_entry_strings(NameId::WWS_SUBFAMILY_NAME)
        .next()
        .is_some();
    let has_wws_names = has_wws_family || has_wws_subfamily;

    let mut problems = vec![];

    if wws_set && has_wws_names {
        problems.push(Status::warn(
            "wws-with-wws-names",
            "OS/2 fsSelection WWS bit is set, but the font has name IDs 21/22 \
             (WWS Family/Subfamily). The WWS bit should only be set when the \
             font's naming is already WWS-conformant without needing IDs 21/22.",
        ));
    }

    if !wws_set && !has_wws_names {
        problems.push(Status::warn(
            "no-wws-without-wws-names",
            "OS/2 fsSelection WWS bit is not set, and the font does not have \
             name IDs 21/22 (WWS Family/Subfamily). If the font's naming is \
             WWS-conformant, the WWS bit should be set.",
        ));
    }

    return_result(problems)
}
