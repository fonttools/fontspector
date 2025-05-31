use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "fontwerk/vendor_id",
    rationale = "
        Vendor ID must be WERK for Fontwerk fonts.
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/3579",
    title = "Check OS/2 achVendID is WERK"
)]
fn vendor_id(f: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    let os2_vendor_id = font.font().os2()?.ach_vend_id().to_string();
    if os2_vendor_id.as_str() == "WERK" {
        Ok(Status::just_one_pass())
    } else {
        Ok(Status::just_one_fail(
            "bad-vendor-id",
            &format!(
                "OS/2 achVendID value is '{}', but should be 'WERK'.",
                os2_vendor_id, 
            ),
        ))
    }
}
