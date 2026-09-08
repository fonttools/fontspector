use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use skrifa::{raw::TableProvider, string::StringId};

#[check(
    id = "adobefonts/nameid_1_win_english",
    rationale = "
        
        While not required by the OpenType spec, Adobe Fonts' pipeline requires
        every font to support at least nameID 1 (Font Family name) for platformID 3
        (Windows), encodingID 1 (Unicode), and languageID 1033/0x409 (US-English).
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/3714",
    title = "Font has a good nameID 1, Windows/Unicode/US-English `name` table record?"
)]
fn nameid_1_win_english(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    let name_table = f.font().name()?;
    let nameid1 = name_table
        .name_record()
        .iter()
        .filter(|record| {
            record.name_id() == StringId::FAMILY_NAME
                && record.platform_id() == 3
                && record.encoding_id() == 1
                && record.language_id() == 0x409
        })
        .collect::<Vec<_>>();
    if nameid1.is_empty() {
        problems.push(Status::fail(
            "nameid-1-not-found",
            "Windows nameID 1 US-English record not found.",
        ));
    } else {
        for record in nameid1 {
            let name = record
                .string(name_table.string_data())?
                .chars()
                .collect::<String>(); // Looks like we can *always* decode?
            if name.trim().is_empty() {
                problems.push(Status::fail(
                    "nameid-1-empty",
                    "Windows nameID 1 US-English record is empty.",
                ));
            }
        }
    }

    return_result(problems)
}
