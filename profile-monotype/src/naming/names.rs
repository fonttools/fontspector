//
// names.rs
//
// Copyright © 2026 Monotype Imaging, Inc. All rights reserved
//

use fontspector_checkapi::TestFont;
use skrifa::string::StringId;

use crate::naming::{family_naming_info, StyleParts};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NameStrings {
    pub full_name: Option<String>,
    pub postscript_name: Option<String>,
    pub family_name: Option<String>,
    pub style_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontNaming {
    pub names: NameStrings,
    pub style: StyleParts,
    pub family_name: String,
    pub family_names: Vec<String>,
    pub product_variety: String,
    pub foundries: Vec<String>,
}

fn name_ids_for_fallback(id: u16, fallback: bool) -> Vec<StringId> {
    if fallback {
        match id {
            21 | 16 | 1 => return [21, 16, 1].map(StringId::new).to_vec(),
            22 | 17 | 2 => return [22, 17, 2].map(StringId::new).to_vec(),
            _ => {}
        }
    }
    vec![StringId::new(id)]
}

pub fn name_string_with_fallback(font: &TestFont, id: u16, fallback: bool) -> Option<String> {
    font.get_best_name(&name_ids_for_fallback(id, fallback))
}

pub fn extract(font: &TestFont) -> FontNaming {
    let full = name_string_with_fallback(font, 4, false);
    let ps = name_string_with_fallback(font, 6, false);
    let fam = name_string_with_fallback(font, 21, true);
    let sty = name_string_with_fallback(font, 22, true);
    let (family_name, family_names, style, product_variety, foundries) = family_naming_info(
        full.as_deref().unwrap_or(""),
        ps.as_deref().unwrap_or(""),
        fam.as_deref().unwrap_or(""),
        sty.as_deref().unwrap_or(""),
        true,
    );
    FontNaming {
        names: NameStrings {
            full_name: full,
            postscript_name: ps,
            family_name: fam,
            style_name: sty,
        },
        style,
        family_name,
        family_names,
        product_variety,
        foundries,
    }
}

#[cfg(test)]
mod tests {
    use super::{extract, name_string_with_fallback};
    use fontspector_checkapi::{TestFont, Testable};
    use std::collections::HashMap;
    use write_fonts::{
        tables::{
            maxp::Maxp,
            name::{Name, NameRecord},
        },
        types::NameId,
        FontBuilder,
    };

    fn font_from_name_ids(testable: &Testable) -> TestFont<'_> {
        match TestFont::new_from_data(&testable.filename, &testable.contents) {
            Ok(font) => font,
            Err(e) => panic!("TestFont from fixture: {e}"),
        }
    }

    fn build_testable(name_ids: HashMap<NameId, &str>) -> Testable {
        let mut builder = FontBuilder::new();
        if let Err(e) = builder.add_table(&Maxp::default()) {
            panic!("maxp table: {e}");
        }
        let mut name_table = Name::default();
        let mut new_records = Vec::new();
        for (nid, s) in name_ids.iter() {
            let name_rec = NameRecord::new(3, 1, 1033, *nid, (*s).to_string().into());
            new_records.push(name_rec);
        }
        new_records.sort();
        name_table.name_record = new_records;
        if let Err(e) = builder.add_table(&name_table) {
            panic!("name table: {e}");
        }
        Testable::new_with_contents("fixture.ttf", builder.build().clone())
    }

    #[test]
    fn name_string_with_fallback_22_uses_name_id_2() {
        let name_ids = HashMap::from([
            (NameId::FAMILY_NAME, "Garamond Classico"),
            (NameId::SUBFAMILY_NAME, "Italic"),
            (NameId::FULL_NAME, "Garamond Classico Italic"),
            (NameId::POSTSCRIPT_NAME, "GaramondClassico-Italic"),
        ]);
        let testable = build_testable(name_ids);
        let font = font_from_name_ids(&testable);
        let sty = name_string_with_fallback(&font, 22, true);
        assert_eq!(sty.as_deref(), Some("Italic"));
    }

    #[test]
    fn name_string_with_fallback_21_uses_name_id_1() {
        let name_ids = HashMap::from([
            (NameId::FAMILY_NAME, "Garamond Classico"),
            (NameId::SUBFAMILY_NAME, "Italic"),
            (NameId::FULL_NAME, "Garamond Classico Italic"),
            (NameId::POSTSCRIPT_NAME, "GaramondClassico-Italic"),
        ]);
        let testable = build_testable(name_ids);
        let font = font_from_name_ids(&testable);
        let fam = name_string_with_fallback(&font, 21, true);
        assert_eq!(fam.as_deref(), Some("Garamond Classico"));
    }

    #[test]
    fn extract_family_name_xcn_xblk_extra_black_extra_condensed() {
        let name_ids = HashMap::from([
            (NameId::FAMILY_NAME, "Family Name XCn XBlk"),
            (NameId::SUBFAMILY_NAME, "Regular"),
            (NameId::FULL_NAME, "Family Name"),
            (NameId::POSTSCRIPT_NAME, "FamilyName-XCnXBlk"),
            (NameId::TYPOGRAPHIC_FAMILY_NAME, "Family Name XCond"),
            (NameId::TYPOGRAPHIC_SUBFAMILY_NAME, "XBlack"),
        ]);
        let testable = build_testable(name_ids);
        let font = font_from_name_ids(&testable);
        let ex = extract(&font);
        assert_eq!(ex.style.weight, "ExtraBlack");
        assert_eq!(ex.style.width, "ExtraCondensed");
    }

    #[test]
    fn extract_garamond_classico_italic_weight_empty() {
        let name_ids = HashMap::from([
            (NameId::FAMILY_NAME, "Garamond Classico"),
            (NameId::SUBFAMILY_NAME, "Italic"),
            (NameId::FULL_NAME, "Garamond Classico Italic"),
            (NameId::POSTSCRIPT_NAME, "GaramondClassico-Italic"),
        ]);
        let testable = build_testable(name_ids);
        let font = font_from_name_ids(&testable);
        let ex = extract(&font);
        assert_eq!(ex.style.weight, "");
    }

    #[test]
    fn extract_helvetica_now_xblk_extra_black() {
        let name_ids = HashMap::from([
            (NameId::FAMILY_NAME, "Helvetica Now XBlk"),
            (NameId::SUBFAMILY_NAME, "Regular"),
            (NameId::FULL_NAME, "Helvetica Now XBlk"),
            (NameId::POSTSCRIPT_NAME, "HelveticaNow-XBlk"),
            (NameId::TYPOGRAPHIC_FAMILY_NAME, "Helvetica Now XBlk"),
            (NameId::TYPOGRAPHIC_SUBFAMILY_NAME, "Regular"),
        ]);
        let testable = build_testable(name_ids);
        let font = font_from_name_ids(&testable);
        let ex = extract(&font);
        assert_eq!(ex.style.weight, "ExtraBlack");
    }

    #[test]
    fn extract_helvetica_now_display_regular_weight() {
        let name_ids = HashMap::from([
            (NameId::FAMILY_NAME, "Helvetica Now Display"),
            (NameId::SUBFAMILY_NAME, "Regular"),
            (NameId::FULL_NAME, "Helvetica Now Display"),
            (NameId::POSTSCRIPT_NAME, "HelveticaNowDisplay-Reg"),
            (NameId::TYPOGRAPHIC_FAMILY_NAME, "Helvetica Now"),
            (NameId::TYPOGRAPHIC_SUBFAMILY_NAME, "Display Reg"),
        ]);
        let testable = build_testable(name_ids);
        let font = font_from_name_ids(&testable);
        let ex = extract(&font);
        assert_eq!(ex.style.weight, "Regular");
    }
}
