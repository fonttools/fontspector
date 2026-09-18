use std::collections::HashSet;

use fontspector_checkapi::{prelude::*, FileTypeConvert};
use skrifa::raw::TableProvider;

#[check(
    id = "adobefonts/family/consistent_upm",
    rationale = "
        
        While not required by the OpenType spec, we (Adobe) expect that a group
        of fonts designed & produced as a family have consistent units per em.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/2372",
    title = "Fonts have consistent Units Per Em?",
    implementation = "all"
)]
fn consistent_upm(c: &TestableCollection, _context: &Context) -> CheckFnResult {
    let fonts = TTF.from_collection(c);
    let upms: HashSet<_> = fonts
        .iter()
        .flat_map(|font| font.font().head())
        .map(|head| head.units_per_em())
        .collect();

    if upms.len() > 1 {
        Ok(Status::just_one_fail(
            "inconsistent-upem",
            &format!(
                "Fonts have different units per em: {}.",
                upms.iter()
                    .map(|upm| upm.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        ))
    } else {
        Ok(Status::just_one_pass())
    }
}
