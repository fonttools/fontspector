use fontations::{
    skrifa::raw::{tables::gasp::GaspRangeBehavior, TableProvider},
    types::Tag,
    write::FontBuilder,
};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use tabled::builder::Builder;

const NON_HINTING_MESSAGE: &str =  "If you are dealing with an unhinted font, it can be fixed by running the fonts through the command 'gftools fix-nonhinting'\nGFTools is available at https://pypi.org/project/gftools/";

fn gasp_meaning(value: GaspRangeBehavior) -> String {
    let mut meaning = vec![];
    if value.intersects(GaspRangeBehavior::GASP_GRIDFIT) {
        meaning.push("- Use grid-fitting");
    }
    if value.intersects(GaspRangeBehavior::GASP_DOGRAY) {
        // 🗦🐶🗧
        meaning.push("- Use grayscale rendering");
    }
    if value.intersects(GaspRangeBehavior::GASP_SYMMETRIC_GRIDFIT) {
        meaning.push("- Use gridfitting with ClearType symmetric smoothing");
    }
    if value.intersects(GaspRangeBehavior::GASP_SYMMETRIC_SMOOTHING) {
        meaning.push("- Use smoothing along multiple axes with ClearType®");
    }
    meaning.join("\n\t")
}

#[check(
    id = "googlefonts/gasp",
    rationale = "
        
        Traditionally version 0 'gasp' tables were set so that font sizes below 8 ppem
        had no grid fitting but did have antialiasing. From 9-16 ppem, just grid
        fitting.
        And fonts above 17ppem had both antialiasing and grid fitting toggled on.
        The use of accelerated graphics cards and higher resolution screens make this
        approach obsolete. Microsoft's DirectWrite pushed this even further with much
        improved rendering built into the OS and apps.

        In this scenario it makes sense to simply toggle all 4 flags ON for all font
        sizes.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    hotfix = fix_unhinted_font,
    title = "Is the Grid-fitting and Scan-conversion Procedure ('gasp') table
set to optimize rendering?"
)]
fn gasp(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    skip!(
        f.has_table(b"CFF ") || f.has_table(b"CFF2"),
        "not-ttf",
        "Skip gasp table test, because CFF font."
    );
    let mut problems = vec![];
    if !f.has_table(b"gasp") {
        return Ok(Status::just_one_fail(
            "lacks-gasp",
            &format!("Font is missing the 'gasp' table. Try exporting the font with autohinting enabled.\n{}"
                , NON_HINTING_MESSAGE)
        ));
    }
    let gasp_table = f.font().gasp()?;
    if gasp_table.gasp_ranges().is_empty() {
        return Ok(Status::just_one_fail(
            "empty",
            &format!("The 'gasp' table has no values.\n{}", NON_HINTING_MESSAGE),
        ));
    }
    if !gasp_table
        .gasp_ranges()
        .iter()
        .any(|r| r.range_max_ppem == 0xFFFF)
    {
        return Ok(Status::just_one_warn(
            "lacks-ffff-range",
            "The 'gasp' table does not have an entry that applies for all font sizes. The gaspRange value for such entry should be set to 0xFFFF.",
        ));
    }
    let md_table = Builder::from_iter(gasp_table.gasp_ranges().iter().map(|r| {
        vec![
            format!("PPM <= {}", r.range_max_ppem),
            gasp_meaning(r.range_gasp_behavior.get()),
        ]
    }));
    problems.push(Status::info(
        "ranges",
        &format!(
            "These are the ppm ranges declared on the gasp table:\n\n{}\n",
            md_table.build().with(tabled::settings::Style::markdown())
        ),
    ));
    for range in gasp_table.gasp_ranges() {
        if range.range_max_ppem != 0xFFFF {
            problems.push(Status::warn(
                "non-ffff-range",
                &format!(
                    "The gasp table has a range of {} that may be unneccessary.",
                    range.range_max_ppem
                ),
            ));
        } else if range.range_gasp_behavior.get().bits() != 0x0f {
            problems.push(Status::warn(
                "unset-flags",
                &format!(
                    "The gasp range 0xFFFF value 0x{:02X} should be set to 0x0F.",
                    range.range_gasp_behavior.get().bits()
                ),
            ));
        }
    }
    return_result(problems)
}

fn fix_unhinted_font(t: &mut Testable) -> FixFnResult {
    let f = testfont!(t);
    if f.has_table(b"fpgm") || (f.has_table(b"prep") && f.has_table(b"gasp")) {
        return Ok(false);
    }
    let new_gasp = fontations::write::tables::gasp::Gasp {
        version: 0,
        gasp_ranges: vec![fontations::write::tables::gasp::GaspRange {
            range_max_ppem: 0xFFFF,
            range_gasp_behavior: GaspRangeBehavior::GASP_GRIDFIT
                | GaspRangeBehavior::GASP_DOGRAY
                | GaspRangeBehavior::GASP_SYMMETRIC_GRIDFIT
                | GaspRangeBehavior::GASP_SYMMETRIC_SMOOTHING,
        }],
        num_ranges: 1,
    };
    // PUSHW[] 511 SCANCTRL[] PUSHB[] 4 SCANTYPE[]
    let new_prep = b"\xb8\x01\xff\x85\xb0\x04\x8d";
    let mut new_font = FontBuilder::new();
    new_font.add_table(&new_gasp)?;
    new_font.add_raw(Tag::new(b"prep"), new_prep);
    new_font.copy_missing_tables(f.font());
    let new_bytes = new_font.build();
    t.set(new_bytes);
    Ok(true)
}
