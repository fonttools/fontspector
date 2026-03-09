use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, Metadata};
use serde_json::json;

#[check(
    id = "no_vert_and_vrt2",
    title = "Ensure fonts don't have both vert and vrt2 GSUB features.",
    rationale = "
        The OpenType specification states that the 'vert' feature should
        never be used in conjunction with 'vrt2'. The 'vrt2' feature is a
        superset of 'vert' and having both present can cause issues on
        some platforms. For example, Kinto Sans fonts failed to install
        on Windows due to this problem.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/215"
)]
fn no_vert_and_vrt2(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let has_vert = f.has_feature(true, "vert");
    let has_vrt2 = f.has_feature(true, "vrt2");
    if has_vert && has_vrt2 {
        let msg = "This font has both 'vert' and 'vrt2' GSUB features. \
             The OpenType spec says 'vert' should never be used with \
             'vrt2', as 'vrt2' is a superset of 'vert'. Please remove \
             the 'vert' feature.";
        let mut status = Status::fail("has-vert-and-vrt2", msg);
        status.add_metadata(Metadata::TableProblem {
            table_tag: "GSUB".to_string(),
            field_name: None,
            actual: Some(json!("both vert and vrt2 present")),
            expected: Some(json!("only one of vert or vrt2")),
            message: msg.to_string(),
        });
        Ok(Box::new(vec![status].into_iter()))
    } else {
        Ok(Status::just_one_pass())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::codetesting::{assert_pass, run_check, test_able};

    use super::no_vert_and_vrt2;

    #[test]
    fn test_pass_no_vert_features() {
        // A normal font without vert/vrt2 should pass
        let testable = test_able("mada/Mada-Regular.ttf");
        let results = run_check(no_vert_and_vrt2, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_fail_both_vert_and_vrt2() {
        // Inject a GSUB table containing both 'vert' and 'vrt2' features
        // into Mada-Regular, which should trigger the check failure.
        use fontations::skrifa::raw::types::Tag;
        use fontations::skrifa::FontRef;
        use fontations::write::FontBuilder;
        use fontspector_checkapi::codetesting::assert_results_contain;
        use fontspector_checkapi::StatusCode;

        // Minimal valid GSUB table with both vert and vrt2 features
        #[rustfmt::skip]
        const GSUB_VERT_VRT2: &[u8] = &[
            0x00, 0x01, 0x00, 0x00, // version 1.0
            0x00, 0x0a,             // ScriptList offset
            0x00, 0x20,             // FeatureList offset
            0x00, 0x36,             // LookupList offset
            // ScriptList
            0x00, 0x01,             // scriptCount = 1
            0x44, 0x46, 0x4c, 0x54, // 'DFLT'
            0x00, 0x08,             // offset to Script table
            // Script table
            0x00, 0x04,             // defaultLangSysOffset
            0x00, 0x00,             // langSysCount = 0
            // Default LangSys
            0x00, 0x00,             // lookupOrder
            0xff, 0xff,             // reqFeatureIndex
            0x00, 0x02,             // featureIndexCount = 2
            0x00, 0x00, 0x00, 0x01, // featureIndices [0, 1]
            // FeatureList
            0x00, 0x02,             // featureCount = 2
            0x76, 0x65, 0x72, 0x74, // 'vert'
            0x00, 0x0e,             // offset to Feature[0]
            0x76, 0x72, 0x74, 0x32, // 'vrt2'
            0x00, 0x12,             // offset to Feature[1]
            // Feature[0]
            0x00, 0x00, 0x00, 0x00, // featureParams=0, lookupCount=0
            // Feature[1]
            0x00, 0x00, 0x00, 0x00, // featureParams=0, lookupCount=0
            // LookupList
            0x00, 0x00,             // lookupCount = 0
        ];

        let mut testable = test_able("mada/Mada-Regular.ttf");
        let f = FontRef::new(&testable.contents).unwrap();
        let gsub_tag = Tag::new(b"GSUB");

        // Rebuild font replacing the GSUB table
        let mut builder = FontBuilder::new();
        builder.add_raw(gsub_tag, GSUB_VERT_VRT2);
        for table_record in f.table_directory.table_records() {
            let tag = table_record.tag.get();
            if tag != gsub_tag {
                if let Some(table_data) = f.table_data(tag) {
                    builder.add_raw(tag, table_data);
                }
            }
        }
        testable.contents = builder.build();

        let results = run_check(no_vert_and_vrt2, testable);
        assert_results_contain(
            &results,
            StatusCode::Fail,
            Some("has-vert-and-vrt2".to_string()),
        );
    }
}
