use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert, Testable};

#[check(
    id = "colr_glyph_entry_with_no_layers",
    rationale = "
        A rendering bug in Windows 10 when a glyph in the COLR table is present, 
        but without any layers specified.

        See https://github.com/fonttools/fontspector/issues/402
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/402",
    title = "COLR glyph entry with no layers"
)]
fn colr_glyph_entry_with_no_layers(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    skip!(!f.has_table(b"COLR"), "no-colr", "No COLR table.");

    let colr = f.font().colr()?;

    let mut bad_glyphs = vec![];

    if let Some(records) = colr.base_glyph_records() {
        if let Ok(colr_glyph_entries) = records {
            for base_glyph in colr_glyph_entries {
                if base_glyph.num_layers == 0 {
                    let name = f.glyph_name_for_id(base_glyph.glyph_id());
                    if let Some(name) = name {
                        bad_glyphs.push(name.to_string());
                    } else {
                        bad_glyphs.push(format!("{:?}", base_glyph.glyph_id()));
                    }
                }
            }
        }
    }

    let status = if bad_glyphs.is_empty() {
        Status::just_one_pass()
    } else {
        Status::just_one_fail(
            "bad-colr-glyphs",
            &format!("This font has colr glyphs with no layers:\n\n{bad_glyphs:?}"),
        )
    };

    Ok(status)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use fontspector_checkapi::{Context, Testable};

    #[test]
    fn test_colr_glyph_entry_with_no_layers() {
        let contents = include_bytes!(
            "../../../fontspector-py/data/test/color_fonts/AmiriQuranColored_colr_glyph_no_layers.ttf"
        );

        let testable = Testable::new_with_contents("demo.ttf", contents.to_vec());
        let context = Context {
            ..Default::default()
        };
        let result = colr_glyph_entry_with_no_layers_impl(&testable, &context)
            .unwrap()
            .next()
            .unwrap();

        let expected_message = "This font has colr glyphs with no layers:\n\n[\"space\"]";
        assert_eq!(result.message, Some(expected_message.to_string()));
    }
}
