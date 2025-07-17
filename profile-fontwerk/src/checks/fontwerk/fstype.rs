use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "fontwerk/embedding_bit",
    rationale = "
        According to Fontwerk the value of the OS/2.fstype field must be Print & Preview (Bit 4).
    ",
    proposal = "https://github.com/ollimeier/fontspector/issues/4",
    title = "Checking embedding bit (OS/2 fsType)."
)]
fn fstype(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let fstype_val = f.font().os2()?.fs_type();
    Ok(if fstype_val == 4 {
        Status::just_one_pass()
    } else {
        Status::just_one_fail(
            "fstype",
            &format!("OS/2 fsType must be set to Print & Preview (Bit 4), found {fstype_val} instead."),
        )
    })
}
