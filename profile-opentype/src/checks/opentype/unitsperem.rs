use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "opentype/unitsperem",
    title = "Checking unitsPerEm value is reasonable.",
    rationale = "
        According to the OpenType spec:

        The value of unitsPerEm at the head table must be a value
        between 16 and 16384. Any value in this range is valid.

        In fonts that have TrueType outlines, a power of 2 is recommended
        as this allows performance optimizations in some rasterizers.

        But 1000 is a commonly used value. And 2000 may become
        increasingly more common on Variable Fonts.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",  // legacy check
)]
fn unitsperem(f: &Testable, _context: &Context) -> CheckFnResult {
    let mut problems = vec![];
    match testfont!(f).font().head()?.units_per_em() {
        bad_upem if !(16..=16384).contains(&bad_upem) => {
            let msg = format!(
                "unitsPerEm value must be a value between 16 and 16384. {bad_upem} is out of range"
            );
            let mut status = Status::fail("out-of-range", &msg);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "head".to_string(),
                field_name: Some("unitsPerEm".to_string()),
                actual: Some(json!(bad_upem)),
                expected: Some(json!("16-16384")),
                message: msg,
            });
            problems.push(status);
        }
        16 | 32 | 64 | 128 | 256 | 512 | 1024 | 2048 | 4096 | 8192 | 16384 | 1000 | 2000 => {
            // Valid values
        }
        upem => {
            let msg = format!("In order to optimize performance on some legacy renderers, the value of unitsPerEm at the head table should ideally be a power of 2 between 16 to 16384. And values of 1000 and 2000 are also common and may be just fine as well. But we got {upem} instead.");
            let mut status = Status::warn("suboptimal", &msg);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "head".to_string(),
                field_name: Some("unitsPerEm".to_string()),
                actual: Some(json!(upem)),
                expected: Some(json!("power of 2 or 1000/2000")),
                message: msg,
            });
            problems.push(status);
        }
    }
    return_result(problems)
}
