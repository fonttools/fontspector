use fontations::skrifa::{GlyphId, MetadataProvider};
use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert, TestFont, Testable};
use std::collections::HashSet;
use std::vec;

const FW_LAT_STD_ENCODED_GLYPHS: [u32; 414] = [
    0x0000, 0x000D, 0x0020, 0x0041, 0x00C1, 0x0102, 0x00C2, 0x00C4, 0x00C0, 0x0100, 0x0104, 0x00C5,
    0x00C3, 0x00C6, 0x0042, 0x0043, 0x0106, 0x010C, 0x00C7, 0x0108, 0x010A, 0x0044, 0x010E, 0x0110,
    0x00D0, 0x0045, 0x00C9, 0x0114, 0x011A, 0x00CA, 0x00CB, 0x0116, 0x00C8, 0x0112, 0x0118, 0x018F,
    0x0046, 0x0047, 0x011E, 0x011C, 0x0122, 0x0120, 0x0048, 0x0126, 0x0124, 0x0049, 0x00CD, 0x012C,
    0x00CE, 0x00CF, 0x0130, 0x00CC, 0x012A, 0x012E, 0x0128, 0x004A, 0x0134, 0x004B, 0x0136, 0x004C,
    0x0139, 0x013D, 0x013B, 0x013F, 0x0141, 0x004D, 0x004E, 0x0143, 0x0147, 0x0145, 0x00D1, 0x014A,
    0x004F, 0x00D3, 0x014E, 0x00D4, 0x00D6, 0x00D2, 0x0150, 0x014C, 0x00D8, 0x00D5, 0x0152, 0x0050,
    0x00DE, 0x0051, 0x0052, 0x0154, 0x0158, 0x0156, 0x0053, 0x015A, 0x0160, 0x015E, 0x015C, 0x0218,
    0x1E9E, 0x0054, 0x0166, 0x0164, 0x0162, 0x021A, 0x0055, 0x00DA, 0x016C, 0x00DB, 0x00DC, 0x00D9,
    0x0170, 0x016A, 0x0172, 0x016E, 0x0168, 0x0056, 0x0057, 0x1E82, 0x0174, 0x1E84, 0x1E80, 0x0058,
    0x0059, 0x00DD, 0x0176, 0x0178, 0x1EF2, 0x005A, 0x0179, 0x017D, 0x017B, 0x0061, 0x00E1, 0x0103,
    0x00E2, 0x00E4, 0x00E0, 0x0101, 0x0105, 0x00E5, 0x00E3, 0x00E6, 0x0062, 0x0063, 0x0107, 0x010D,
    0x00E7, 0x0109, 0x010B, 0x0064, 0x010F, 0x0111, 0x00F0, 0x0065, 0x00E9, 0x0115, 0x011B, 0x00EA,
    0x00EB, 0x0117, 0x00E8, 0x0113, 0x0119, 0x0259, 0x0066, 0x0067, 0x011F, 0x011D, 0x0123, 0x0121,
    0x0068, 0x0127, 0x0125, 0x0069, 0x0131, 0x00ED, 0x012D, 0x00EE, 0x00EF, 0x00EC, 0x012B, 0x012F,
    0x0129, 0x006A, 0x0237, 0x0135, 0x006B, 0x0137, 0x006C, 0x013A, 0x013E, 0x013C, 0x0140, 0x0142,
    0x006D, 0x006E, 0x0144, 0x0148, 0x0146, 0x00F1, 0x014B, 0x006F, 0x00F3, 0x014F, 0x00F4, 0x00F6,
    0x00F2, 0x0151, 0x014D, 0x00F8, 0x00F5, 0x0153, 0x0070, 0x00FE, 0x0071, 0x0072, 0x0155, 0x0159,
    0x0157, 0x0073, 0x015B, 0x0161, 0x015F, 0x015D, 0x0219, 0x00DF, 0x0074, 0x0167, 0x0165, 0x0163,
    0x021B, 0x0075, 0x00FA, 0x016D, 0x00FB, 0x00FC, 0x00F9, 0x0171, 0x016B, 0x0173, 0x016F, 0x0169,
    0x0076, 0x0077, 0x1E83, 0x0175, 0x1E85, 0x1E81, 0x0078, 0x0079, 0x00FD, 0x0177, 0x00FF, 0x1EF3,
    0x007A, 0x017A, 0x017E, 0x017C, 0xFB01, 0xFB02, 0x00AA, 0x00BA, 0x0394, 0x03A9, 0x03BC, 0x03C0,
    0x0030, 0x0031, 0x0032, 0x0033, 0x0034, 0x0035, 0x0036, 0x0037, 0x0038, 0x0039, 0x2044, 0x00BD,
    0x00BC, 0x00BE, 0x00B9, 0x00B2, 0x00B3, 0x00A0, 0x2007, 0x2008, 0x2009, 0x200B, 0xFEFF, 0x2060,
    0x27E8, 0x27E9, 0x002D, 0x2013, 0x2014, 0x2012, 0x2010, 0x2011, 0x005F, 0x0028, 0x0029, 0x007B,
    0x007D, 0x005B, 0x005D, 0x201A, 0x201E, 0x201C, 0x201D, 0x2018, 0x2019, 0x00AB, 0x00BB, 0x2039,
    0x203A, 0x0022, 0x0027, 0x002E, 0x002C, 0x003A, 0x003B, 0x2026, 0x0021, 0x00A1, 0x003F, 0x00BF,
    0x00B7, 0x2022, 0x002A, 0x0023, 0x002F, 0x005C, 0x0192, 0x20BF, 0x00A2, 0x00A4, 0x0024, 0x20AC,
    0x20BA, 0x00A3, 0x00A5, 0x2219, 0x2052, 0x22C5, 0x002B, 0x2212, 0x00D7, 0x00F7, 0x003D, 0x2260,
    0x003E, 0x003C, 0x2265, 0x2264, 0x00B1, 0x2248, 0x007E, 0x00AC, 0x005E, 0x221E, 0x222B, 0x2126,
    0x2206, 0x220F, 0x2211, 0x221A, 0x00B5, 0x2202, 0x0025, 0x2030, 0x25CA, 0x0040, 0x0026, 0x00B6,
    0x00A7, 0x00A9, 0x00AE, 0x2117, 0x2122, 0x00B0, 0x2032, 0x2033, 0x007C, 0x00A6, 0x2020, 0x2113,
    0x2021, 0x212E, 0xFFFD, 0x0308, 0x0307, 0x0300, 0x0301, 0x030B, 0x0302, 0x030C, 0x0306, 0x030A,
    0x0303, 0x0304, 0x0326, 0x0327, 0x0328, 0x00A8, 0x02D9, 0x0060, 0x00B4, 0x02DD, 0x02C6, 0x02C7,
    0x02D8, 0x02DA, 0x02DC, 0x00AF, 0x00B8, 0x02DB,
];
const FW_LAT_STD_UNENCODED_GLYPHS: &[[&str; 2]; 48] = &[
    // [prefered name, alternative name]
    [".notdef", ".notdef"],
    ["zero.tf", "uni0030.tf"],
    ["one.tf", "uni0031.tf"],
    ["two.tf", "uni0032.tf"],
    ["three.tf", "uni0033.tf"],
    ["four.tf", "uni0034.tf"],
    ["five.tf", "uni0035.tf"],
    ["six.tf", "uni0036.tf"],
    ["seven.tf", "uni0037.tf"],
    ["eight.tf", "uni0038.tf"],
    ["nine.tf", "uni0039.tf"],
    ["leftanglebracket-math.case", "uni27E8.case"],
    ["rightanglebracket-math.case", "uni27E9.case"],
    ["hyphen.case", "uni002D.case"],
    ["endash.case", "uni2013.case"],
    ["emdash.case", "uni2014.case"],
    ["figuredash.case", "uni2012.case"],
    ["hyphentwo.case", "uni2010.case"],
    ["nonbreakinghyphen.case", "uni2011.case"],
    ["parenleft.case", "uni0028.case"],
    ["parenright.case", "uni0029.case"],
    ["braceleft.case", "uni007B.case"],
    ["braceright.case", "uni007D.case"],
    ["bracketleft.case", "uni005B.case"],
    ["bracketright.case", "uni005D.case"],
    ["guillemetleft.case", "guillemotleft.case"],
    ["guillemetright.case", "guillemotright.case"],
    ["guilsinglleft.case", "uni2039.case"],
    ["guilsinglright.case", "uni203A.case"],
    ["colon.case", "uni003A.case"],
    ["exclamdown.case", "uni00A1.case"],
    ["questiondown.case", "uni00BF.case"],
    ["periodcentered.case", "uni00B7.case"],
    ["bullet.case", "uni2022.case"],
    ["slash.case", "uni002F.case"],
    ["backslash.case", "uni005C.case"],
    ["at.case", "uni0040.case"],
    ["dieresiscomb.case", "uni0308.case"],
    ["dotaccentcomb.case", "uni0307.case"],
    ["gravecomb.case", "uni0300.case"],
    ["acutecomb.case", "uni0301.case"],
    ["hungarumlautcomb.case", "uni030B.case"],
    ["circumflexcomb.case", "uni0302.case"],
    ["caroncomb.case", "uni030C.case"],
    ["brevecomb.case", "uni0306.case"],
    ["ringcomb.case", "uni030A.case"],
    ["tildecomb.case", "uni0303.case"],
    ["macroncomb.case", "uni0304.case"],
];

#[check(
    id = "fontwerk/glyph_coverage",
    rationale = "
        Fontwerk expects that fonts support at least the minimal
        set of characters defined in the `FW LAT Std` glyph-set.

        GlyphsApp plists can be found here: https://github.com/fontwerk/specifications

    ",
    proposal = "https://github.com/ollimeier/fontspector/issues/1",
    title = "Check Fontwerk Fonts minimum glyph coverage."
)]
fn glyph_coverage(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let codepoints = f.codepoints(Some(context));
    let required_glyphset = FW_LAT_STD_ENCODED_GLYPHS.to_vec();

    // Check encoded glyphs
    let mut missing_glyphs = required_glyphset
        .iter()
        .filter(|&&c| !codepoints.contains(&c))
        .cloned()
        .map(|c| format!("uni{c:04X}"))
        .collect::<Vec<String>>();

    // Check unencoded glyphs
    let all_unencoded_glyphs = get_unencoded_glyphs(f);
    for [unencoded_name, unencoded_name_alt] in FW_LAT_STD_UNENCODED_GLYPHS {
        if !all_unencoded_glyphs.contains(&unencoded_name.to_string())
            && !all_unencoded_glyphs.contains(&unencoded_name_alt.to_string())
        {
            missing_glyphs.push(unencoded_name.to_string());
        }
    }

    let status = if missing_glyphs.is_empty() {
        Status::just_one_pass()
    } else {
        Status::just_one_fail(
            "missing-glyphs",
            &format!("This font is missing required glyphs:\n\n{missing_glyphs:?}"),
        )
    };

    Ok(status)
}

fn get_unencoded_glyphs(test_font: TestFont) -> Vec<String> {
    let mut unencoded_glyphs = vec![];
    let mappings = test_font.font().charmap().mappings().collect::<Vec<_>>();
    let mapped: HashSet<GlyphId> = mappings.iter().map(|(_u, gid)| *gid).collect();
    let unencoded_glyph_ids = test_font
        .all_glyphs()
        .filter(|g| !mapped.contains(g))
        .collect::<Vec<_>>();
    for gid in unencoded_glyph_ids {
        let name = test_font.glyph_name_for_id(gid);
        if let Some(name) = name {
            if !name.is_empty() {
                unencoded_glyphs.push(name);
            }
        }
    }
    unencoded_glyphs
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use fontspector_checkapi::{Context, Testable};

    #[test]
    fn test_glyph_coverage() {
        let contents = include_bytes!(
            "../../../../fontspector-py/data/test/montserrat/Montserrat-Regular.ttf"
        );
        let testable = Testable::new_with_contents("demo.ttf", contents.to_vec());
        let context = Context {
            ..Default::default()
        };
        let result = glyph_coverage_impl(&testable, &context)
            .unwrap()
            .next()
            .unwrap();

        let expected_message = "This font is missing required glyphs:\n\n[\"uniFEFF\", \"uni2060\", \"uni27E8\", \"uni27E9\", \"uni2011\", \"uni20BF\", \"uni2052\", \"uni22C5\", \"uni2117\", \"uniFFFD\", \".null\", \"leftanglebracket-math.case\", \"rightanglebracket-math.case\", \"figuredash.case\", \"hyphentwo.case\", \"nonbreakinghyphen.case\", \"colon.case\", \"exclamdown.case\", \"questiondown.case\", \"ringcomb.case\"]";
        assert_eq!(result.message, Some(expected_message.to_string()));
    }
}
