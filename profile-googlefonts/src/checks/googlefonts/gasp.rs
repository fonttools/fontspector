use fontations::{
    skrifa::raw::{tables::gasp::GaspRangeBehavior, TableProvider},
    types::Tag,
    write::FontBuilder,
};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Metadata};
use serde_json::json;
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
        let msg = "Font is missing the 'gasp' table";
        let mut status = Status::fail(
            "lacks-gasp",
            &format!("Font is missing the 'gasp' table. Try exporting the font with autohinting enabled.\n{NON_HINTING_MESSAGE}")
        );
        status.add_metadata(Metadata::TableProblem {
            table_tag: "gasp".to_string(),
            field_name: None,
            actual: Some(json!("missing")),
            expected: Some(json!("gasp table present")),
            message: msg.to_string(),
        });
        problems.push(status);
        return return_result(problems);
    }
    let gasp_table = f.font().gasp()?;
    if gasp_table.gasp_ranges().is_empty() {
        let msg = "The 'gasp' table has no values";
        let mut status = Status::fail(
            "empty",
            &format!("The 'gasp' table has no values.\n{NON_HINTING_MESSAGE}"),
        );
        status.add_metadata(Metadata::TableProblem {
            table_tag: "gasp".to_string(),
            field_name: Some("gasp_ranges".to_string()),
            actual: Some(json!(0)),
            expected: Some(json!("> 0")),
            message: msg.to_string(),
        });
        problems.push(status);
        return return_result(problems);
    }
    if !gasp_table
        .gasp_ranges()
        .iter()
        .any(|r| r.range_max_ppem == 0xFFFF)
    {
        let msg = "The 'gasp' table does not have an entry that applies for all font sizes";
        let mut status = Status::warn(
            "lacks-ffff-range",
            "The 'gasp' table does not have an entry that applies for all font sizes. The gaspRange value for such entry should be set to 0xFFFF.",
        );
        status.add_metadata(Metadata::TableProblem {
            table_tag: "gasp".to_string(),
            field_name: Some("gasp_ranges".to_string()),
            actual: Some(json!("no 0xFFFF range")),
            expected: Some(json!("0xFFFF range required")),
            message: msg.to_string(),
        });
        problems.push(status);
        return return_result(problems);
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
            let msg = format!(
                "The gasp table has a range of {} that may be unnecessary",
                range.range_max_ppem
            );
            let mut status = Status::warn("non-ffff-range", &msg);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "gasp".to_string(),
                field_name: Some("range_max_ppem".to_string()),
                actual: Some(json!(range.range_max_ppem.to_string())),
                expected: Some(json!("0xFFFF")),
                message: msg.clone(),
            });
            problems.push(status);
        } else if range.range_gasp_behavior.get().bits() != 0x0f {
            let bits = range.range_gasp_behavior.get().bits();
            let msg = format!(
                "The gasp range 0xFFFF value 0x{:02X} should be set to 0x0F",
                bits
            );
            let mut status = Status::warn("unset-flags", &msg);
            status.add_metadata(Metadata::TableProblem {
                table_tag: "gasp".to_string(),
                field_name: Some("gasp_behavior".to_string()),
                actual: Some(json!(format!("0x{:02X}", bits))),
                expected: Some(json!("0x0F")),
                message: msg.clone(),
            });
            problems.push(status);
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
