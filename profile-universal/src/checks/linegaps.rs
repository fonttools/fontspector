use fontations::{
    skrifa::raw::TableProvider,
    write::{
        from_obj::ToOwnedTable,
        tables::{hhea, os2::Os2},
        FontBuilder,
    },
};
use fontspector_checkapi::{
    prelude::*, source::find_or_add_cp, testfont, FileTypeConvert, Source, SourceFile,
};

#[check(
    id = "linegaps",
    rationale = "
        The LineGap value is a space added to the line height created by the union
        of the (typo/hhea)Ascender and (typo/hhea)Descender. It is handled differently
        according to the environment.

        This leading value will be added above the text line in most desktop apps.
        It will be shared above and under in web browsers, and ignored in Windows
        if Use_Typo_Metrics is disabled.

        For better linespacing consistency across platforms,
        (typo/hhea)LineGap values must be 0.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/4133",
    proposal = "https://googlefonts.github.io/gf-guide/metrics.html",
    title = "Checking Vertical Metric linegaps.",
    hotfix = fix_linegaps,
    fix_source = sourcefix_linegaps,
)]
fn linegaps(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);

    let os2 = f
        .font()
        .os2()
        .map_err(|_| FontspectorError::General("OS/2 table missing".to_string()))?;
    let hhea = f
        .font()
        .hhea()
        .map_err(|_| FontspectorError::General("hhea table missing".to_string()))?;
    let mut problems = vec![];
    if hhea.line_gap().to_i16() != 0 {
        problems.push(Status::warn("hhea", "hhea lineGap is not equal to 0."));
    }
    if os2.s_typo_line_gap() != 0 {
        problems.push(Status::warn("OS/2", "OS/2 sTypoLineGap is not equal to 0."));
    }
    return_result(problems)
}

fn fix_linegaps(t: &mut Testable) -> FixFnResult {
    let f = testfont!(t);
    let mut hhea: hhea::Hhea = f.font().hhea()?.to_owned_table();
    let mut changed = false;

    if hhea.line_gap.to_i16() != 0 {
        hhea.line_gap = 0.into();
        changed = true;
    }

    let mut os2: Os2 = f.font().os2()?.to_owned_table();
    if os2.s_typo_line_gap != 0 {
        os2.s_typo_line_gap = 0;
        changed = true;
    }

    if changed {
        let mut builder = FontBuilder::new();
        builder.add_table(&hhea)?;
        builder.add_table(&os2)?;
        builder.copy_missing_tables(f.font());
        let new_bytes = builder.build();
        t.set(new_bytes);
    }

    Ok(changed)
}

fn sourcefix_linegaps(s: &mut SourceFile) -> FixFnResult {
    fn fix_a_ufo(font: &mut norad::Font) -> FixFnResult {
        let was_broken = (font.font_info.open_type_hhea_line_gap.is_some()
            && font.font_info.open_type_hhea_line_gap != Some(0))
            || (font.font_info.open_type_os2_typo_line_gap.is_some()
                && font.font_info.open_type_os2_typo_line_gap != Some(0));
        font.font_info.open_type_hhea_line_gap = Some(0);
        font.font_info.open_type_os2_typo_line_gap = Some(0);
        Ok(was_broken)
    }
    fn fix_custom_parameters(cps: &mut Vec<glyphslib::common::CustomParameter>) -> FixFnResult {
        Ok(
            find_or_add_cp(cps, "typoLineGap", glyphslib::Plist::Integer(0))?
                || find_or_add_cp(cps, "hheaLineGap", glyphslib::Plist::Integer(0))?,
        )
    }
    match s.source {
        Source::Ufo(ref mut font) => fix_a_ufo(font),
        Source::Designspace(ref mut ds) => ds.apply_fix(&fix_a_ufo),
        Source::Glyphs(ref mut font) => match &mut **font {
            glyphslib::Font::Glyphs2(glyphs2) => {
                let mut changed = false;
                for master in glyphs2.masters.iter_mut() {
                    changed |= fix_custom_parameters(&mut master.custom_parameters)?;
                }
                Ok(changed)
            }
            glyphslib::Font::Glyphs3(glyphs3) => {
                let mut changed = false;
                for master in glyphs3.masters.iter_mut() {
                    changed |= fix_custom_parameters(&mut master.custom_parameters)?;
                }
                Ok(changed)
            }
        },
    }
}
