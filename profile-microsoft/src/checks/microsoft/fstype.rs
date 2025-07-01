use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "microsoft/fstype",
    rationale = "
        
        The value of the OS/2.fstype field must be 8 (Editable embedding), meaning,
        according to the OpenType spec:
        
        \"Editable embedding: the font may be embedded, and may be temporarily loaded
        on other systems. As with Preview & Print embedding, documents containing
        Editable fonts may be opened for reading. In addition, editing is permitted,
        including ability to format new text using the embedded font, and changes
        may be saved.\" 
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/4657",
    title = "Checking OS/2 fsType."
)]
fn fstype(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let fstype_val = f.font().os2()?.fs_type();
    Ok(if fstype_val == 8 {
        Status::just_one_pass()
    } else {
        Status::just_one_fail(
            "fstype",
            &format!("OS/2 fsType must be set to 8, found {fstype_val} instead."),
        )
    })
}
