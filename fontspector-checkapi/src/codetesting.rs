#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::LazyLock,
};

// No bad thing if we panic in tests
use crate::{
    prelude::*, testfont, Check, CheckResult, Context, FileTypeConvert, StatusCode, TestFont,
};
use skrifa::{
    raw::{types::NameId, TableProvider},
    GlyphId, GlyphId16, GlyphNames, MetadataProvider,
};
use write_fonts::{
    from_obj::{FromTableRef, ToOwnedTable},
    tables::{
        cmap::Cmap,
        glyf::{GlyfLocaBuilder, Glyph},
        head::Head,
        name::{Name, NameRecord},
        os2::Os2,
    },
    FontBuilder,
};

/// The root of the workspace, used to locate test resources
// The usual thing to use here is env!("CARGO_MANIFEST_DIR"), but that's a pain
// when we're in a workspace - if you're running `cargo test` inside a package,
// the manifest dir is the package root; if you're running `cargo test -p foo`,
// the manifest dir is the workspace root. So ask Cargo for the workspace root
// and go from there.
static WORKSPACE_ROOT: LazyLock<PathBuf> = LazyLock::new(|| {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = std::path::Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
});

/// Return a pathname for a file in the test resources directory
pub fn test_file(fname: impl AsRef<Path>) -> PathBuf {
    let mut workspace_root = WORKSPACE_ROOT.clone();

    workspace_root.push("resources/test/");
    workspace_root.join(fname)
}

/// Return a Testable from a file in the test resources directory
pub fn test_able(fname: impl AsRef<Path>) -> Testable {
    let path = test_file(fname);
    Testable::new(path).expect("Failed to load test file")
}

/// Run a check on a font and return the result
pub fn run_check(check: Check<'_>, font: Testable) -> Option<CheckResult> {
    run_check_with_config(check, TestableType::Single(&font), HashMap::new())
}

/// Run a check on a font or collection with a given configuration and return the result
pub fn run_check_with_config(
    check: Check<'_>,
    things: TestableType<'_>,
    config: HashMap<String, serde_json::Value>,
) -> Option<CheckResult> {
    run_check_with_config_and_network(check, things, config, false)
}

/// Run a check on a font or collection with a given configuration and explicit network toggle.
pub fn run_check_with_config_and_network(
    check: Check<'_>,
    things: TestableType<'_>,
    config: HashMap<String, serde_json::Value>,
    skip_network: bool,
) -> Option<CheckResult> {
    let ctx: Context = Context {
        skip_network,
        network_timeout: Some(10),
        configuration: config,
        check_metadata: check.metadata(),
        full_lists: true,
        cache: Default::default(),
        overrides: vec![],
        check_id: None,
    };
    check.run(&things, &ctx, None)
}

/// Assert that a check passes
///
/// Takes a `CheckResult` and asserts that the worst status is `Pass`
pub fn assert_pass(check_result: &Option<CheckResult>) {
    let status = check_result.as_ref().unwrap().worst_status();
    assert_eq!(
        status,
        StatusCode::Pass,
        "Expected check to pass, but got status {:?}",
        status
    );
}
/// Assert that a check is skipped
///
/// Takes a `CheckResult` and asserts that the worst status is `Skip`
pub fn assert_skip(check_result: &Option<CheckResult>) {
    let status = check_result.as_ref().unwrap().worst_status();
    assert_eq!(status, StatusCode::Skip);
}

/// Assert that a check result contains an expected status and code
pub fn assert_results_contain(
    check_result: &Option<CheckResult>,
    severity: StatusCode,
    code: Option<String>,
) {
    let subresults = &check_result.as_ref().unwrap().subresults;
    assert!(subresults
        .iter()
        .any(|subresult| subresult.severity == severity && subresult.code == code),
        "Could not find result with severity {:?} and code {:?} in check results.\nResults found:\n- {}",
        severity,
        code,
        subresults
            .iter()
            .map(|subresult| subresult.to_string())
            .collect::<Vec<_>>()
            .join("\n- ")
    );
}

/// Assert that a check result contains an expected message substring
pub fn assert_messages_contain(check_result: &Option<CheckResult>, wanted_message: &str) {
    assert_messages_contain_impl(check_result, wanted_message, true);
}

/// Assert that a check result does not contain an expected message substring
pub fn assert_messages_dont_contain(check_result: &Option<CheckResult>, wanted_message: &str) {
    assert_messages_contain_impl(check_result, wanted_message, false);
}

/// Implementation of assert_messages_contain and assert_messages_dont_contain
fn assert_messages_contain_impl(
    check_result: &Option<CheckResult>,
    wanted_message: &str,
    positive: bool,
) {
    if check_result.is_none() {
        panic!("Check result was None");
    }
    let result = check_result.as_ref().unwrap();
    let mut found = false;
    for subresult in &result.subresults {
        if let Some(message) = &subresult.message {
            if message.contains(wanted_message) {
                found = true;
                break;
            }
        }
    }
    if found != positive {
        let all_messages: Vec<String> = result
            .subresults
            .iter()
            .filter_map(|s| s.message.clone())
            .collect();
        panic!(
            "Could not find message '{}' in check results.\nMessages found:\n- {}",
            wanted_message,
            all_messages.join("\n- ")
        );
    }
}

/// Manipulate a font by changing a name table entry (for testing purposes only)
pub fn set_name_entry(
    font: &mut Testable,
    platform: u16,
    encoding: u16,
    language: u16,
    nameid: NameId,
    new_string: String,
) {
    let f = TTF.from_testable(font).unwrap();
    let name = f.font().name().unwrap();

    let new_record = NameRecord::new(
        platform,
        encoding,
        language,
        nameid,
        new_string.to_string().into(),
    );
    let mut new_records: Vec<NameRecord> = name
        .name_record()
        .iter()
        .filter(|record| record.name_id() != nameid)
        .map(|r| {
            #[allow(clippy::unwrap_used)]
            NameRecord::new(
                r.platform_id(),
                r.encoding_id(),
                r.language_id(),
                r.name_id(),
                r.string(name.string_data())
                    .unwrap()
                    .chars()
                    .collect::<String>()
                    .to_string()
                    .into(),
            )
        })
        .collect();
    new_records.push(new_record);
    new_records.sort_by_key(|r| (r.platform_id, r.encoding_id, r.language_id, r.name_id));
    let new_nametable = Name::new(new_records);
    let new_bytes = FontBuilder::new()
        .add_table(&new_nametable)
        .unwrap()
        .copy_missing_tables(f.font())
        .build();

    font.contents = new_bytes;
}

/// Manipulate a font by removing name table entries for a given name ID.
pub fn remove_name_entry(font: &mut Testable, nameid: NameId) {
    let f = TTF.from_testable(font).unwrap();
    let name = f.font().name().unwrap();

    let mut new_records: Vec<NameRecord> = name
        .name_record()
        .iter()
        .filter(|record| record.name_id() != nameid)
        .map(|r| {
            #[allow(clippy::unwrap_used)]
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
    new_records.sort_by_key(|r| (r.platform_id, r.encoding_id, r.language_id, r.name_id));
    let new_nametable = Name::new(new_records);
    let new_bytes = FontBuilder::new()
        .add_table(&new_nametable)
        .unwrap()
        .copy_missing_tables(f.font())
        .build();

    font.contents = new_bytes;
}

/// Remove a codepoint from the cmap of a font
pub fn deencode_glyph(t: &mut Testable, codepoint: u32) -> Result<(), FontspectorError> {
    deencode_glyphs(t, &[codepoint])
}

/// Return the glyph ID for a glyph name, if the font has a 'post' or 'CFF' table
/// providing glyph names.
pub fn glyph_id_for_name(f: &TestFont<'_>, glyph_name: &str) -> Option<GlyphId> {
    GlyphNames::new(&f.font())
        .iter()
        .find(|(_gid, name)| name.as_str() == glyph_name)
        .map(|(gid, _name)| gid)
}

/// Read a glyph from the 'glyf' table by name, as an owned write-fonts glyph.
///
/// Returns `None` if the font is not a TrueType font, or the glyph name is not
/// present. Empty glyphs are returned as [`Glyph::Empty`].
pub fn glyph_by_name(font: &Testable, glyph_name: &str) -> Option<Glyph> {
    let f = TTF.from_testable(font)?;
    let gid = glyph_id_for_name(&f, glyph_name)?;
    let loca = f.font().loca(None).ok()?;
    let glyf = f.font().glyf().ok()?;
    match loca.get_glyf(gid, &glyf).ok()? {
        Some(glyph) => Some(Glyph::from_table_ref(&glyph)),
        None => Some(Glyph::Empty),
    }
}

/// Replace a glyph in the 'glyf' table by name (for testing purposes only).
///
/// This rebuilds the 'glyf', 'loca' and 'head' tables using write-fonts, so the
/// glyph data and loca offsets stay consistent.
pub fn set_glyph(
    font: &mut Testable,
    glyph_name: &str,
    new_glyph: &Glyph,
) -> Result<(), FontspectorError> {
    let f = TTF
        .from_testable(font)
        .ok_or_else(|| FontspectorError::General("Not a TrueType font".to_string()))?;
    let target = glyph_id_for_name(&f, glyph_name).ok_or_else(|| {
        FontspectorError::General(format!("No glyph named '{glyph_name}' in font"))
    })?;
    let loca = f.font().loca(None)?;
    let glyf = f.font().glyf()?;
    let mut builder = GlyfLocaBuilder::new();
    for gid in f.all_glyphs() {
        if gid == target {
            builder.add_glyph(new_glyph)?;
        } else if let Some(glyph) = loca.get_glyf(gid, &glyf)? {
            builder.add_glyph(&Glyph::from_table_ref(&glyph))?;
        } else {
            builder.add_glyph(&Glyph::Empty)?;
        }
    }
    let (glyf, loca, loca_format) = builder.build();
    let mut head: Head = f.font().head()?.to_owned_table();
    head.index_to_loc_format = loca_format as i16;
    let new_bytes = FontBuilder::new()
        .add_table(&glyf)?
        .add_table(&loca)?
        .add_table(&head)?
        .copy_missing_tables(f.font())
        .build();
    font.contents = new_bytes;
    Ok(())
}

/// Replace the GDEF table's GlyphClassDef with the given (glyph name, class)
/// pairs (for testing purposes only).
///
/// This discards any other GDEF subtables (attachment points, ligature carets,
/// mark attachment classes and mark glyph sets).
pub fn set_gdef_glyph_classes(
    font: &mut Testable,
    classes: &[(&str, u16)],
) -> Result<(), FontspectorError> {
    use write_fonts::tables::{gdef::Gdef, layout::ClassDef};

    let f = TTF
        .from_testable(font)
        .ok_or_else(|| FontspectorError::General("Not a TrueType font".to_string()))?;
    let mut pairs = Vec::with_capacity(classes.len());
    for (glyph_name, class) in classes {
        let gid = glyph_id_for_name(&f, glyph_name).ok_or_else(|| {
            FontspectorError::General(format!("No glyph named '{glyph_name}' in font"))
        })?;
        let gid = GlyphId16::try_from(gid)
            .map_err(|_| FontspectorError::General(format!("Glyph {gid} is out of range")))?;
        pairs.push((gid, *class));
    }
    let class_def: ClassDef = pairs.into_iter().collect();
    let gdef = Gdef::new(Some(class_def), None, None, None);
    font.contents = f.rebuild_with_new_table(&gdef)?;
    Ok(())
}

/// Remove a set of codepoints from a font's cmap (for testing purposes only).
pub fn deencode_glyphs(t: &mut Testable, codepoints: &[u32]) -> Result<(), FontspectorError> {
    let f = testfont!(t);
    let charmap = f.font().charmap();
    let mappings: Vec<_> = charmap
        .mappings()
        .filter(|(cp, _)| !codepoints.contains(cp))
        .collect();
    let new_cmap = Cmap::from_mappings(
        mappings
            .into_iter()
            .map(|(c, gid)| (char::from_u32(c).unwrap_or('\0'), gid)),
    )
    .map_err(|e| FontspectorError::General(format!("Failed to create new cmap: {e}")))?;
    t.set(f.rebuild_with_new_table(&new_cmap)?);
    Ok(())
}

/// Set the same advance width for every glyph in a font (for testing purposes only).
///
/// This rewrites both 'hmtx' and 'hhea', keeping `hhea.numberOfHMetrics` and
/// `hhea.advanceWidthMax` consistent with the new metrics.
pub fn set_all_advance_widths(t: &mut Testable, advance: u16) -> Result<(), FontspectorError> {
    use write_fonts::tables::{
        hhea::Hhea,
        hmtx::{Hmtx, LongMetric},
    };

    let f = testfont!(t);
    let num_glyphs = f.font().maxp()?.num_glyphs();
    let hmtx = Hmtx {
        h_metrics: (0..num_glyphs)
            .map(|_| LongMetric {
                advance,
                side_bearing: 0,
            })
            .collect(),
        left_side_bearings: vec![],
    };
    let mut hhea: Hhea = f.font().hhea()?.to_owned_table();
    hhea.number_of_h_metrics = num_glyphs;
    hhea.advance_width_max = advance.into();
    let new_bytes = FontBuilder::new()
        .add_table(&hhea)?
        .add_table(&hmtx)?
        .copy_missing_tables(f.font())
        .build();
    t.contents = new_bytes;
    Ok(())
}

/// Mutate OS/2 usWeightClass in a font (for testing purposes only).
pub fn set_weight_class(t: &mut Testable, weight_class: u16) -> Result<(), FontspectorError> {
    let f = testfont!(t);
    let mut os2: Os2 = f.font().os2()?.to_owned_table();
    os2.us_weight_class = weight_class;
    t.set(f.rebuild_with_new_table(&os2)?);
    Ok(())
}

/// Manipulate a font by remapping a glyph (for testing purposes only)
pub fn remap_glyph(
    font: &mut Testable,
    codepoint: u32,
    glyphname: &str,
) -> Result<(), FontspectorError> {
    let f = TTF.from_testable(font).unwrap();
    let names = GlyphNames::new(&f.font());
    let Some(glyph) = names
        .iter()
        .find(|(_gid, name)| name.as_str() == glyphname)
        .map(|(gid, _name)| gid)
    else {
        return Err(FontspectorError::General(format!(
            "No glyph named '{}' in font",
            glyphname
        )));
    };
    let once = std::iter::once((codepoint, glyph));
    let new_cmap = Cmap::from_mappings(
        f.font()
            .charmap()
            .mappings()
            .filter(|(cp, _gid)| *cp != codepoint) // Remove existing mapping if there is one
            .chain(once)
            .map(|(cp, gid)| (char::from_u32(cp).unwrap(), gid)),
    )
    .expect("Failed to create new cmap");
    let new_bytes = FontBuilder::new()
        .add_table(&new_cmap)
        .unwrap()
        .copy_missing_tables(f.font())
        .build();

    font.contents = new_bytes;
    Ok(())
}

/// Remove a table from a font (for testing purposes only)
///
/// This rebuilds the font without the specified table using write-fonts FontBuilder.
pub fn remove_table(font: &mut Testable, table_tag: &[u8; 4]) {
    use skrifa::{font::FontRef, Tag};

    let f = FontRef::new(&font.contents).unwrap();
    let tag_to_remove = Tag::new(table_tag);

    let mut builder = FontBuilder::new();
    for table_record in f.table_directory.table_records() {
        let tag = table_record.tag.get();
        if tag != tag_to_remove {
            if let Some(table_data) = f.table_data(tag) {
                builder.add_raw(tag, table_data);
            }
        }
    }

    font.contents = builder.build();
}

/// Add a dummy table to a font (for testing purposes only)
///
/// This adds a table with minimal dummy data using write-fonts FontBuilder.
/// The table won't be valid but will be detected by has_table().
pub fn add_table(font: &mut Testable, table_tag: &[u8; 4]) {
    use skrifa::{font::FontRef, Tag};

    let f = FontRef::new(&font.contents).unwrap();
    let new_tag = Tag::new(table_tag);

    let mut builder = FontBuilder::new();

    // Copy all existing tables
    for table_record in f.table_directory.table_records() {
        let tag = table_record.tag.get();
        if let Some(table_data) = f.table_data(tag) {
            builder.add_raw(tag, table_data);
        }
    }

    // Add the new dummy table (4 bytes of zeros)
    let dummy_data: &[u8] = &[0u8; 4];
    builder.add_raw(new_tag, dummy_data);

    font.contents = builder.build();
}
