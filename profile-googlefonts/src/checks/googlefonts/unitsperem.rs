use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

const RECOMMENDED: [u16; 11] = [16, 32, 64, 128, 256, 500, 512, 1000, 1024, 2000, 2048];

#[check(
    id = "googlefonts/unitsperem",
    rationale = "
        
        Even though the OpenType spec allows unitsPerEm to be any value between 16
        and 16384, the Google Fonts project aims at a narrower set of reasonable values.

        Values above 4000 would likely result in unreasonable filesize increases.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Stricter unitsPerEm criteria for Google Fonts."
)]
fn unitsperem(t: &Testable, _context: &Context) -> CheckFnResult {
    let upm = testfont!(t).font().head()?.units_per_em();
    let mut problems = vec![];
    if upm > 4000 {
        let msg = "Font em size (unitsPerEm) is too large (causing filesize bloat)";
        let mut status = Status::fail("large-value", &format!(
                "Font em size (unitsPerEm) is {upm} which may be too large (causing filesize bloat), unless you are sure that the detail level in this font requires that much precision."
            ));
        status.add_metadata(Metadata::TableProblem {
            table_tag: "head".to_string(),
            field_name: Some("unitsPerEm".to_string()),
            actual: Some(json!(upm)),
            expected: Some(json!("<= 4000")),
            message: msg.to_string(),
        });
        problems.push(status);
    } else if upm < 16 {
        let msg = "Font em size (unitsPerEm) is below minimum recommended value";
        let mut status = Status::fail("bad-value", &format!(
                "Font em size (unitsPerEm) is {upm}. If possible, please consider using 1000. Good values for unitsPerEm, though, are typically these: {RECOMMENDED:?}."
            ));
        status.add_metadata(Metadata::TableProblem {
            table_tag: "head".to_string(),
            field_name: Some("unitsPerEm".to_string()),
            actual: Some(json!(upm)),
            expected: Some(json!(">= 16")),
            message: msg.to_string(),
        });
        problems.push(status);
    } else {
        problems.push(Status::pass());
    }
    return_result(problems)
}
