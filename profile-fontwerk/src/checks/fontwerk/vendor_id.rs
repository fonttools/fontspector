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

    use fontations::{
        write::{
            tables::os2::Os2,
            tables::maxp::Maxp,
            FontBuilder,
        },
        skrifa::{Tag},
    };

    #[test]
    fn test_vendor_id_fontwerk() {

        let vendor_ids = [b"UKWN", b"WERK", b"TEST", b"ABCD"];
        let expected = [
            (StatusCode::Fail, Some("OS/2 achVendID value is 'UKWN', but should be 'WERK'.".to_string())),
            (StatusCode::Pass, None),
            (StatusCode::Fail, Some("OS/2 achVendID value is 'TEST', but should be 'WERK'.".to_string())),
            (StatusCode::Fail, Some("OS/2 achVendID value is 'ABCD', but should be 'WERK'.".to_string())),
        ];

        for (i, &vend_id) in vendor_ids.iter().enumerate() {
            let mut builder = FontBuilder::new();
            // We need to add a default maxp table, because otherwise 
            // Testable::new_with_contents complains "Not a TTF file"
            builder.add_table(&Maxp::default()).unwrap(); 
            let mut os2 = Os2::default();
            os2.ach_vend_id = Tag::new(vend_id);
            builder.add_table(&os2).unwrap();

            let testable = Testable::new_with_contents(
                "demo.ttf",
                builder.build().clone(),
            );
            let result = vendor_id_impl(&testable, &Context::default())
                .unwrap()
                .next()
                .unwrap();

            assert_eq!(result.severity, expected[i].0);
            assert_eq!(result.message, expected[i].1);
        }
    }
}