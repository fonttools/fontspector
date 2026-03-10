use std::sync::LazyLock;

use fontations::skrifa::raw::TableProvider;
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use hashbrown::HashSet;

const VENDOR_IDS_FILE: &str = include_str!("../../../resources/vendor_ids.txt");
static VENDOR_IDS: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    VENDOR_IDS_FILE
        .lines()
        .filter(|line| !line.is_empty())
        .collect()
});

const BAD_VIDS: [&str; 4] = ["UKWN", "ukwn", "PfEd", "PYRS"];
const SUGGEST_MICROSOFT_VENDORLIST_WEBSITE: &str = "If you registered it recently, then it's safe to ignore this warning message. Otherwise, you should set it to your own unique 4 character code, and register it with Microsoft at https://www.microsoft.com/typography/links/vendorlist.aspx\n";

#[check(
    id = "googlefonts/vendor_id",
    rationale = "
        
        Microsoft keeps a list of font vendors and their respective contact info. This
        list is updated regularly and is indexed by a 4-char \"Vendor ID\" which is
        stored in the achVendID field of the OS/2 table.

        Registering your ID is not mandatory, but it is a good practice since some
        applications may display the type designer / type foundry contact info on some
        dialog and also because that info will be visible on Microsoft's website:

        https://docs.microsoft.com/en-us/typography/vendors/

        This check verifies whether or not a given font's vendor ID is registered in
        that list or if it has some of the default values used by the most common
        font editors.

        Each new FontBakery release includes a cached copy of that list of vendor IDs.
        If you registered recently, you're safe to ignore warnings emitted by this
        check, since your ID will soon be included in one of our upcoming releases.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/3943",
    proposal = "https://github.com/fonttools/fontbakery/issues/4829",
    title = "Checking OS/2 achVendID."
)]
fn vendor_id(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    let font_vendor_id = f.font().os2()?.ach_vend_id().to_string();
    if font_vendor_id.is_empty() {
        problems.push(Status::warn(
            "not-set",
            &format!("OS/2 VendorID is not set.\n{SUGGEST_MICROSOFT_VENDORLIST_WEBSITE}"),
        ));
    } else if BAD_VIDS.contains(&font_vendor_id.as_str()) {
        problems.push(Status::warn(
            "bad",
            &format!(
                "OS/2 VendorID is '{font_vendor_id}', a font editor default.\n{SUGGEST_MICROSOFT_VENDORLIST_WEBSITE}"
            ),
        ));
    } else if !VENDOR_IDS.contains(&font_vendor_id.as_str()) {
        problems.push(Status::warn(
            "unknown",
            &format!(
                "OS/2 VendorID value '{font_vendor_id}' is not yet recognized.\n{SUGGEST_MICROSOFT_VENDORLIST_WEBSITE}"
            ),
        ));
    }
    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use fontspector_checkapi::{
        codetesting::{assert_pass, assert_results_contain, run_check, test_able},
        FileTypeConvert, StatusCode,
    };

    use super::vendor_id;

    fn set_vendor_id(testable: &mut fontspector_checkapi::Testable, vid: &str) {
        use fontations::{skrifa::raw::TableProvider, write::from_obj::ToOwnedTable};

        let f = fontspector_checkapi::prelude::TTF
            .from_testable(testable)
            .unwrap();
        let mut os2: fontations::write::tables::os2::Os2 = f.font().os2().unwrap().to_owned_table();
        let vid_bytes: [u8; 4] = vid.as_bytes().try_into().unwrap();
        os2.ach_vend_id = fontations::types::Tag::new(&vid_bytes);
        testable.set(f.rebuild_with_new_table(&os2).unwrap());
    }

    #[test]
    fn test_warn_bad_vids() {
        for bad_vid in &["UKWN", "ukwn", "PfEd", "PYRS"] {
            let mut testable = test_able("merriweather/Merriweather-Regular.ttf");
            set_vendor_id(&mut testable, bad_vid);
            let results = run_check(vendor_id, testable);
            assert_results_contain(&results, StatusCode::Warn, Some("bad".to_string()));
        }
    }

    #[test]
    fn test_warn_unknown_vid() {
        let mut testable = test_able("merriweather/Merriweather-Regular.ttf");
        set_vendor_id(&mut testable, "????");
        let results = run_check(vendor_id, testable);
        assert_results_contain(&results, StatusCode::Warn, Some("unknown".to_string()));
    }

    #[test]
    fn test_pass_known_vid() {
        let mut testable = test_able("merriweather/Merriweather-Regular.ttf");
        set_vendor_id(&mut testable, "APPL");
        let results = run_check(vendor_id, testable);
        assert_pass(&results);
    }

    #[test]
    fn test_pass_goog_vid() {
        let mut testable = test_able("merriweather/Merriweather-Regular.ttf");
        set_vendor_id(&mut testable, "GOOG");
        let results = run_check(vendor_id, testable);
        assert_pass(&results);
    }
}
