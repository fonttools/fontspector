//! Adobe Fonts Profile for Fontspector Checks
mod checks;

use fontspector_checkapi::{Override, ProfileBuilder, Registry, StatusCode};

///  This is the main plugin struct for the Adobe Fonts profile.
pub struct Adobe;
impl fontspector_checkapi::Plugin for Adobe {
    fn register(&self, cr: &mut Registry) -> Result<(), String> {
        let builder = ProfileBuilder::new()
            .include_profile("universal")
            .add_section("Adobe Fonts Checks")
            // "Adobe Fonts Checks" = ["adobefonts/family/consistent_upm", "adobefonts/nameid_1_win_english", "adobefonts/unsupported_tables", "adobefonts/STAT_strings"]
            .add_and_register_check(checks::adobefonts::family::consistent_upm)
            .add_and_register_check(checks::adobefonts::nameid_1_win_english)
            .add_and_register_check(checks::adobefonts::unsupported_tables)
            .add_and_register_check(checks::adobefonts::STAT_strings)
            .exclude_check("opentype/xavgcharwidth")
            .exclude_check("designspace_has_consistent_codepoints")
            .exclude_check("designspace_has_consistent_glyphset")
            .exclude_check("designspace_has_consistent_groups")
            .exclude_check("designspace_has_default_master")
            .exclude_check("designspace_has_sources")
            .exclude_check("name/no_copyright_on_description")
            .exclude_check("ufolint")
            .exclude_check("ufo_features_default_languagesystem")
            .exclude_check("ufo_recommended_fields")
            .exclude_check("ufo_required_fields")
            .exclude_check("ufo_unnecessary_fields")
            .exclude_check("STAT_strings")
            .exclude_check("transformed_components")
            .exclude_check("unreachable_glyphs")
            .exclude_check("whitespace_ink")
            .with_overrides("whitespace_glyphs", vec![
                Override::new("missing-whitespace-glyph-0x00A0", StatusCode::Warn,  "For Adobe, this is not as severe as assessed in the original check for 0x00A0.")
            ]).with_overrides("valid_glyphnames", vec![
                Override::new("found-invalid-names", StatusCode::Warn, "")
            ])
            .with_overrides("family/win_ascent_and_descent", vec![
                Override::new("ascent", StatusCode::Warn, "For Adobe, this is not as severe as assessed in the original check."),
                Override::new("descent", StatusCode::Warn, "For Adobe, this is not as severe as assessed in the original check.")
            ])
            .with_overrides("os2_metrics_match_hhea", vec![
                Override::new("ascender", StatusCode::Warn, ""),
                Override::new("descender", StatusCode::Warn, ""),
                Override::new("lineGap", StatusCode::Warn, "")
            ])
            .with_overrides("fontbakery_version", vec![
                Override::new("connection-error", StatusCode::Skip, "For Adobe, users shouldn't be bothered with a failed check if their internet connection isn't functional.")
            ])
            .with_overrides("opentype/name/match_familyname_fullfont", vec![
                Override::new("mismatch-font-names", StatusCode::Warn, "Many CFF OpenType fonts in circulation are built with the Microsoft platform Full font name string identical to the PostScript FontName in the CFF Name INDEX. This practice was documented in the OpenType spec until version 1.5.")
            ])
            .with_overrides("varfont/bold_wght_coord", vec![
                Override::new("no-bold-instance", StatusCode::Warn, "Adobe strongly recommends, but does not require having a Bold instance."),
                Override::new("wght-not-700", StatusCode::Warn, "Adobe strongly recommends (but does not require) that instance should have coordinate 700 on the 'wght' axis.")
            ])
            .with_overrides("opentype/fvar/regular_coords_correct", vec![
                Override::new("no-regular-instance", StatusCode::Warn, "Adobe strongly recommends, but does not require having a Regular instance.")
            ])
            .with_overrides("opentype/varfont/valid_default_instance_nameids", vec![
                Override::new("invalid-default-instance-subfamily-name", StatusCode::Warn, "Adobe and the OpenType spec strongly recommend following these guidelines, but they are not hard requirements so we are relaxing this to WARN rather than FAIL.\nFonts that do not meet these guidelines might behave inconsistently so please carefully consider trying to meet them."),
                Override::new("invalid-default-instance-postscript-name", StatusCode::Warn, "Adobe and the OpenType spec strongly recommend following these guidelines, but they are not hard requirements so we are relaxing this to WARN rather than FAIL.\nFonts that do not meet these guidelines might behave inconsistently so please carefully consider trying to meet them.")
            ])
            .with_overrides("inconsistencies_between_fvar_STAT", vec![
                Override::new("missing-fvar-instance-axis-value", StatusCode::Warn, "Adobe and Fontwerk strongly recommend following this guideline, but it is not a hard requirement so we are relaxing this to WARN rather than FAIL.\nFonts that do not meet this guideline might behave inconsistently so please carefully consider trying to meet it.")
            ])
            .with_overrides("opentype/weight_class_fvar", vec![
                Override::new("bad-weight-class", StatusCode::Warn, "Adobe and Fontwerk strongly recommend following this guideline, but it is not a hard requirement so we are relaxing this to WARN rather than FAIL.\nFonts that do not meet this guideline might behave inconsistently so please carefully consider trying to meet it.")
            ]);
        builder.build("adobefonts", cr)
    }
}

#[cfg(not(target_family = "wasm"))]
pluginator::plugin_implementation!(fontspector_checkapi::Plugin, Adobe);
