#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::LazyLock,
};

// No bad thing if we panic in tests
use crate::{prelude::*, Check, CheckResult, Context, FileTypeConvert, StatusCode};
use fontations::{
    skrifa::{
        raw::{types::NameId, TableProvider},
        GlyphNames, MetadataProvider,
    },
    write::{
        tables::{
            cmap::Cmap,
            name::{Name, NameRecord},
        },
        FontBuilder,
    },
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
    let ctx: Context = Context {
        skip_network: false,
        network_timeout: Some(10),
        configuration: config,
        check_metadata: check.metadata(),
        full_lists: true,
        cache: Default::default(),
        overrides: vec![],
    };
    check.run(&things, &ctx, None)
}

/// Assert that a check passes
///
/// Takes a `CheckResult` and asserts that the worst status is `Pass`
pub fn assert_pass(check_result: &Option<CheckResult>) {
    let status = check_result.as_ref().unwrap().worst_status();
    assert_eq!(status, StatusCode::Pass);
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
    let new_nametable = Name::new(new_records);
    let new_bytes = FontBuilder::new()
        .add_table(&new_nametable)
        .unwrap()
        .copy_missing_tables(f.font())
        .build();

    font.contents = new_bytes;
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
/// This rebuilds the font without the specified table by manipulating raw bytes.
/// We use raw byte manipulation to preserve the original font structure.
pub fn remove_table(font: &mut Testable, table_tag: &[u8; 4]) {
    let data = &font.contents;

    // Parse the table directory header
    let sfnt_version = &data[0..4];
    let num_tables = u16::from_be_bytes([data[4], data[5]]) as usize;

    // Find tables to keep (all except the one we're removing)
    let mut tables_to_keep = Vec::new();
    for i in 0..num_tables {
        let record_offset = 12 + i * 16;
        let tag = &data[record_offset..record_offset + 4];
        if tag != table_tag {
            let checksum = u32::from_be_bytes([
                data[record_offset + 4],
                data[record_offset + 5],
                data[record_offset + 6],
                data[record_offset + 7],
            ]);
            let offset = u32::from_be_bytes([
                data[record_offset + 8],
                data[record_offset + 9],
                data[record_offset + 10],
                data[record_offset + 11],
            ]) as usize;
            let length = u32::from_be_bytes([
                data[record_offset + 12],
                data[record_offset + 13],
                data[record_offset + 14],
                data[record_offset + 15],
            ]) as usize;

            tables_to_keep.push((tag.to_vec(), checksum, offset, length));
        }
    }

    let new_num_tables = tables_to_keep.len() as u16;

    // Calculate search range, entry selector, range shift
    let mut search_range = 1u16;
    let mut entry_selector = 0u16;
    while search_range * 2 <= new_num_tables {
        search_range *= 2;
        entry_selector += 1;
    }
    search_range *= 16;
    let range_shift = new_num_tables * 16 - search_range;

    // Build new font
    let mut new_data = Vec::new();

    // Write header
    new_data.extend_from_slice(sfnt_version);
    new_data.extend_from_slice(&new_num_tables.to_be_bytes());
    new_data.extend_from_slice(&search_range.to_be_bytes());
    new_data.extend_from_slice(&entry_selector.to_be_bytes());
    new_data.extend_from_slice(&range_shift.to_be_bytes());

    // Calculate new offsets - table data starts after header and table records
    let header_size = 12 + tables_to_keep.len() * 16;
    let mut current_offset = header_size;

    // Pad to 4-byte boundary
    while current_offset % 4 != 0 {
        current_offset += 1;
    }

    // Write table records with updated offsets
    let mut table_data_with_offsets = Vec::new();
    for (tag, checksum, old_offset, length) in &tables_to_keep {
        new_data.extend_from_slice(tag);
        new_data.extend_from_slice(&checksum.to_be_bytes());
        new_data.extend_from_slice(&(current_offset as u32).to_be_bytes());
        new_data.extend_from_slice(&(*length as u32).to_be_bytes());

        // Store table data for later
        table_data_with_offsets.push((&data[*old_offset..*old_offset + *length], current_offset));

        // Update offset for next table, with 4-byte padding
        current_offset += length;
        while current_offset % 4 != 0 {
            current_offset += 1;
        }
    }

    // Pad header to table data start
    while new_data.len()
        < table_data_with_offsets
            .first()
            .map(|(_, o)| *o)
            .unwrap_or(header_size)
    {
        new_data.push(0);
    }

    // Write table data
    for (table_bytes, offset) in table_data_with_offsets {
        // Pad to correct offset
        while new_data.len() < offset {
            new_data.push(0);
        }
        new_data.extend_from_slice(table_bytes);
    }

    font.contents = new_data;
}

/// Add a dummy table to a font (for testing purposes only)
///
/// This adds a table with minimal dummy data. The table won't be valid
/// but will be detected by has_table().
pub fn add_table(font: &mut Testable, table_tag: &[u8; 4]) {
    let data = &font.contents;

    // Parse existing tables
    let sfnt_version = &data[0..4];
    let num_tables = u16::from_be_bytes([data[4], data[5]]) as usize;

    let mut existing_tables = Vec::new();
    for i in 0..num_tables {
        let record_offset = 12 + i * 16;
        let tag = data[record_offset..record_offset + 4].to_vec();
        let checksum = u32::from_be_bytes([
            data[record_offset + 4],
            data[record_offset + 5],
            data[record_offset + 6],
            data[record_offset + 7],
        ]);
        let offset = u32::from_be_bytes([
            data[record_offset + 8],
            data[record_offset + 9],
            data[record_offset + 10],
            data[record_offset + 11],
        ]) as usize;
        let length = u32::from_be_bytes([
            data[record_offset + 12],
            data[record_offset + 13],
            data[record_offset + 14],
            data[record_offset + 15],
        ]) as usize;

        existing_tables.push((tag, checksum, offset, length));
    }

    // Add new dummy table (4 bytes of zeros)
    let dummy_data = [0u8; 4];
    existing_tables.push((table_tag.to_vec(), 0, 0, 4)); // offset will be recalculated

    let new_num_tables = existing_tables.len() as u16;

    // Calculate search range, entry selector, range shift
    let mut search_range = 1u16;
    let mut entry_selector = 0u16;
    while search_range * 2 <= new_num_tables {
        search_range *= 2;
        entry_selector += 1;
    }
    search_range *= 16;
    let range_shift = new_num_tables * 16 - search_range;

    // Build new font
    let mut new_data = Vec::new();

    // Write header
    new_data.extend_from_slice(sfnt_version);
    new_data.extend_from_slice(&new_num_tables.to_be_bytes());
    new_data.extend_from_slice(&search_range.to_be_bytes());
    new_data.extend_from_slice(&entry_selector.to_be_bytes());
    new_data.extend_from_slice(&range_shift.to_be_bytes());

    // Calculate new offsets
    let header_size = 12 + existing_tables.len() * 16;
    let mut current_offset = header_size;
    while current_offset % 4 != 0 {
        current_offset += 1;
    }

    // Write table records with updated offsets
    let mut table_data_list: Vec<(Vec<u8>, usize)> = Vec::new();
    for (tag, checksum, old_offset, length) in &existing_tables {
        new_data.extend_from_slice(tag);
        new_data.extend_from_slice(&checksum.to_be_bytes());
        new_data.extend_from_slice(&(current_offset as u32).to_be_bytes());
        new_data.extend_from_slice(&(*length as u32).to_be_bytes());

        // Get table data (either from original font or dummy data for new table)
        let table_bytes = if tag == table_tag {
            dummy_data.to_vec()
        } else {
            data[*old_offset..*old_offset + *length].to_vec()
        };
        table_data_list.push((table_bytes, current_offset));

        current_offset += length;
        while current_offset % 4 != 0 {
            current_offset += 1;
        }
    }

    // Pad header
    while new_data.len()
        < table_data_list
            .first()
            .map(|(_, o)| *o)
            .unwrap_or(header_size)
    {
        new_data.push(0);
    }

    // Write table data
    for (table_bytes, offset) in table_data_list {
        while new_data.len() < offset {
            new_data.push(0);
        }
        new_data.extend_from_slice(&table_bytes);
    }

    font.contents = new_data;
}
