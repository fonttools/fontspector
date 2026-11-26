use fontspector_checkapi::{prelude::*, supported_name_entry_encoding, testfont, FileTypeConvert};
use skrifa::raw::TableProvider;

#[check(
    id = "opentype/name/empty_records",
    title = "Check name table for empty records.",
    rationale = "Check the name table for empty records, as this can cause problems in Adobe apps.",
    proposal = "https://github.com/fonttools/fontbakery/pull/2369"
)]
fn empty_records(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let name = f.font().name()?;
    let mut problems: Vec<Status> = vec![];
    for record in name.name_record() {
        if !supported_name_entry_encoding(record.platform_id(), record.encoding_id()) {
            continue;
        }
        if record
            .string(name.string_data())?
            .to_string()
            .trim()
            .is_empty()
        {
            problems.push(Status::fail(
                "empty-record",
                &format!(
                    "Empty name record found for name ID={} platform ID={} encoding ID={} language ID={}",
                    record.name_id(),
                    record.platform_id(),
                    record.encoding_id(),
                    record.language_id(),
                ),
            ));
        }
    }
    return_result(problems)
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use fontspector_checkapi::{
        codetesting::{
            assert_messages_contain, assert_pass, assert_results_contain, run_check,
            set_name_entry, test_able,
        },
        supported_name_entry_encoding, FileTypeConvert, StatusCode, Testable, TTF,
    };
    use skrifa::raw::types::NameId;
    use skrifa::{raw::TableProvider, Tag};
    use write_fonts::FontBuilder;

    #[test]
    fn test_empty_records_pass() {
        let testable = test_able("source-sans-pro/OTF/SourceSansPro-Regular.otf");
        let result = run_check(empty_records, testable);
        assert_pass(&result);
    }

    #[test]
    fn test_empty_records_fail() {
        let mut testable = test_able("source-sans-pro/OTF/SourceSansPro-Regular.otf");
        // Set a name entry to an empty string
        set_name_entry(
            &mut testable,
            3,
            1,
            0x0409,
            NameId::UNIQUE_ID,
            "".to_string(),
        );
        let result = run_check(empty_records, testable);
        assert_results_contain(&result, StatusCode::Fail, Some("empty-record".to_string()));
        assert_messages_contain(&result, "platform ID=3 encoding ID=1 language ID=1033");
    }

    #[test]
    fn test_empty_records_skips_unsupported_name_entry_encodings() {
        assert!(
            !supported_name_entry_encoding(1, 1),
            "test fixture relies on platform 1 / encoding 1 being unsupported"
        );

        let mut testable = test_able("source-sans-pro/OTF/SourceSansPro-Regular.otf");
        append_unsupported_nonempty_name_record(&mut testable);

        let font = TTF.from_testable(&testable).expect("ttf");
        let name = font.font().name().expect("name table");
        assert!(
            name.name_record().iter().any(|record| {
                record.platform_id() == 1 && record.encoding_id() == 1 && record.length() > 0
            }),
            "fixture must expose a non-empty unsupported (1, 1) name record"
        );

        let result = run_check(empty_records, testable);
        assert_pass(&result);
    }

    fn utf16_be_bytes(text: &str) -> Vec<u8> {
        text.encode_utf16()
            .flat_map(|unit| unit.to_be_bytes())
            .collect()
    }

    fn encode_name_table_v0(records: &[(u16, u16, u16, u16, Vec<u8>)]) -> Vec<u8> {
        let storage_offset = (6 + records.len() * 12) as u16;
        let mut table = Vec::new();
        table.extend_from_slice(&0u16.to_be_bytes());
        table.extend_from_slice(&(records.len() as u16).to_be_bytes());
        table.extend_from_slice(&storage_offset.to_be_bytes());

        let mut storage = Vec::new();
        for (platform_id, encoding_id, language_id, name_id, string_bytes) in records {
            let offset = storage.len() as u16;
            let length = string_bytes.len() as u16;
            table.extend_from_slice(&platform_id.to_be_bytes());
            table.extend_from_slice(&encoding_id.to_be_bytes());
            table.extend_from_slice(&language_id.to_be_bytes());
            table.extend_from_slice(&name_id.to_be_bytes());
            table.extend_from_slice(&length.to_be_bytes());
            table.extend_from_slice(&offset.to_be_bytes());
            storage.extend_from_slice(string_bytes);
        }
        table.extend_from_slice(&storage);
        table
    }

    /// Patch the name table after load: add a (1, 1) record with non-empty bytes.
    ///
    /// write-fonts cannot build that entry; raw table bytes are injected instead.
    /// Fontations treats the encoding as unknown and decodes it as an empty string.
    fn append_unsupported_nonempty_name_record(testable: &mut Testable) {
        let font_wrapper = TTF.from_testable(testable).expect("ttf");
        let font = font_wrapper.font();
        let name = font.name().expect("name table");
        let string_data = name.string_data();

        let mut records: Vec<(u16, u16, u16, u16, Vec<u8>)> = name
            .name_record()
            .iter()
            .map(|record| {
                let start = record.string_offset().to_u32() as usize;
                let end = start + record.length() as usize;
                let bytes = string_data
                    .as_bytes()
                    .get(start..end)
                    .unwrap_or_default()
                    .to_vec();
                (
                    record.platform_id(),
                    record.encoding_id(),
                    record.language_id(),
                    record.name_id().to_u16(),
                    bytes,
                )
            })
            .collect();

        records.push((1, 1, 0, 1, utf16_be_bytes("UnsupportedEncodingFamily")));
        records.sort_by_key(|record| (record.0, record.1, record.2, record.3));

        let name_bytes = encode_name_table_v0(&records);
        let mut builder = FontBuilder::new();
        builder.add_raw(Tag::new(b"name"), name_bytes);
        builder.copy_missing_tables(font);
        testable.set(builder.build());
    }
}
