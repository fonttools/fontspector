use std::collections::HashSet;

use fontations::skrifa::raw::types::NameId;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "opentype/name/postscript_name_consistency",
    rationale = "
        The PostScript name entries in the font's 'name' table should be
        consistent across platforms.

        This is the TTF/CFF2 equivalent of the CFF 'name/postscript_vs_cff' check.
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/2394",
    title = "Name table ID 6 (PostScript name) must be consistent across platforms."
)]
fn postscript_name_consistency(t: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    // Fontbakery had this just for non-CFF fonts, but I think we don't want
    // inconsistent PostScript names in *any* font!
    let psnames: HashSet<_> = font
        .get_name_entry_strings(NameId::POSTSCRIPT_NAME)
        .collect();
    if psnames.len() > 1 {
        return Ok(Status::just_one_fail(
            "inconsistency",
            &format!(
                "More than one entry found for PostScript name; we got: {:?}",
                psnames.into_iter().collect::<Vec<String>>().join(", ")
            ),
        ));
    }
    Ok(Status::just_one_pass())
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use fontations::skrifa::raw::types::NameId;
    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        StatusCode,
    };

    #[test]
    fn test_postscript_name_consistency_pass() {
        let testable = test_able("source-sans-pro/TTF/SourceSansPro-Regular.ttf");
        let result = run_check(postscript_name_consistency, testable);
        assert_pass(&result);
    }

    #[test]
    fn test_postscript_name_consistency_fail() {
        use fontations::{
            skrifa::raw::TableProvider,
            write::tables::name::{Name, NameRecord},
        };
        let mut testable = test_able("source-sans-pro/TTF/SourceSansPro-Regular.ttf");
        // Add a Mac platform name entry with a different PostScript name
        // We must keep existing entries and add a new one (not replace)
        let f = TTF.from_testable(&testable).unwrap();
        let name = f.font().name().unwrap();
        let mut new_records: Vec<NameRecord> = name
            .name_record()
            .iter()
            .map(|r| {
                NameRecord::new(
                    r.platform_id(),
                    r.encoding_id(),
                    r.language_id(),
                    r.name_id(),
                    r.string(name.string_data())
                        .unwrap()
                        .chars()
                        .collect::<String>()
                        .into(),
                )
            })
            .collect();
        // Add a Mac platform entry with a different PS name
        new_records.push(NameRecord::new(
            1,
            0,
            0,
            NameId::POSTSCRIPT_NAME,
            "YetAnotherFontName".to_string().into(),
        ));
        new_records.sort_by(|a, b| {
            a.platform_id
                .cmp(&b.platform_id)
                .then(a.encoding_id.cmp(&b.encoding_id))
                .then(a.language_id.cmp(&b.language_id))
                .then(a.name_id.cmp(&b.name_id))
        });
        let new_nametable = Name::new(new_records);
        let new_bytes = fontations::write::FontBuilder::new()
            .add_table(&new_nametable)
            .unwrap()
            .copy_missing_tables(f.font())
            .build();
        testable.set(new_bytes);
        let result = run_check(postscript_name_consistency, testable);
        assert_results_contain(&result, StatusCode::Fail, Some("inconsistency".to_string()));
    }
}
