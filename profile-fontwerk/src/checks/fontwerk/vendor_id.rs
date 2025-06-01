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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use fontspector_checkapi::StatusCode;

    #[test]
    fn test_vendor_id_fontwerk() {

        let testable = Testable::new_with_contents(
            "Montserrat-Regular.ttf",
            include_bytes!("../../../../fontspector-py/data/test/montserrat/Montserrat-Regular.ttf")
                .to_vec(),
        );
        let result = vendor_id_impl(&testable, &Context::default())
            .unwrap()
            .next()
            .unwrap();

        // Check that the vendor ID is WERK. It is expected that this test will fail
        // because the Montserrat font does not have the vendor ID set to WERK.
        assert_eq!(result.severity, StatusCode::Fail);
        assert_eq!(result.message, Some("OS/2 achVendID value is 'ULA ', but should be 'WERK'.".to_string()));
    }
}
