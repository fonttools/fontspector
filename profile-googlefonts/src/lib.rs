//! The google fonts profile for Fontspector
pub mod checks;

pub(crate) mod constants;
use fontspector_checkapi::{prelude::*, ProfileBuilder, Registry};

mod network_conditions;
mod utils;
use serde_json::json;
use std::collections::HashMap;

pub(crate) const LICENSE: FileType = FileType {
    pattern: "{LICENSE,OFL}.txt",
};

pub(crate) const IMAGE: FileType = FileType {
    pattern: "*.{png,jpg,jpeg,jxl,gif,svg}",
};

pub(crate) fn seems_like_gf_repo(c: &TestableCollection) -> bool {
    c.iter().next().is_some_and(|f| {
        let Some(grandparent) = f.filename.parent().and_then(|p| p.parent()) else {
            return false;
        };

        grandparent
            .file_name()
            .is_some_and(|n| n == "ofl" || n == "apache" || n == "ufl")
    })
}

/// The main plugin struct for the Google Fonts profile.
pub struct GoogleFonts;
impl fontspector_checkapi::ProfileProvider for GoogleFonts {
    fn register(&self, cr: &mut Registry) -> Result<(), FontspectorError> {
        let mdpb = FileType::new("METADATA.pb");
        let desc = FileType::new("*.en_us.html");
        cr.register_filetype("MDPB", mdpb);
        cr.register_filetype("DESC", desc);
        cr.register_filetype("IMAGE", IMAGE);
        cr.register_filetype("LICENSE", LICENSE);

        let builder = ProfileBuilder::new()
            .include_profile("universal")
            .exclude_check("adobefonts/STAT_strings"); // We have our own stricter version

        #[cfg(feature = "check")]
        let builder = builder
            .add_section("Article Checks")
            .add_and_register_check(checks::googlefonts::article::images);

        let builder = builder.add_section("Metadata Checks");

        #[cfg(feature = "check")]
        let builder = builder.add_and_register_check(checks::googlefonts::metadata::axes);
        #[cfg(all(feature = "check", not(target_family = "wasm")))]
        let builder = builder.add_and_register_check(checks::googlefonts::metadata::broken_links);
        #[cfg(feature = "check")]
        let builder = builder
            .add_and_register_check(checks::googlefonts::metadata::can_render_samples)
            .add_and_register_check(checks::googlefonts::metadata::category)
            .add_and_register_check(checks::googlefonts::metadata::consistent_repo_urls)
            .add_and_register_check(checks::googlefonts::metadata::consistent_with_fonts);
        #[cfg(all(feature = "check", not(target_family = "wasm")))]
        let builder =
            builder.add_and_register_check(checks::googlefonts::metadata::designer_profiles);

        #[cfg(feature = "check")]
        let builder = builder
            .add_and_register_check(checks::googlefonts::metadata::escaped_strings)
            .add_and_register_check(checks::googlefonts::metadata::family_directory_name)
            .add_and_register_check(checks::googlefonts::metadata::familyname)
            .add_and_register_check(checks::googlefonts::metadata::has_regular)
            .add_and_register_check(checks::googlefonts::metadata::primary_script)
            .add_and_register_check(checks::googlefonts::metadata::valid_primary_script_language)
            .add_and_register_check(checks::googlefonts::metadata::regular_is_400)
            .add_and_register_check(checks::googlefonts::metadata::subsets_correct) // Replacement for metadata/unsupported_subsets
            .add_and_register_check(checks::googlefonts::metadata::unreachable_subsetting)
            .add_and_register_check(checks::googlefonts::metadata::validate)
            .add_and_register_check(checks::googlefonts::metadata::valid_nameid25)
            .add_and_register_check(checks::googlefonts::metadata::weightclass);

        let builder = builder.add_section("Glyphset Checks");
        #[cfg(feature = "check")]
        let builder =
            builder.add_and_register_check(checks::googlefonts::glyphsets::shape_languages);
        #[cfg(feature = "check")]
        let builder = builder.add_and_register_check(checks::googlefonts::tofu);
        let builder = builder
            .add_and_register_check(checks::googlefonts::separator_glyphs)
            .add_section("Description Checks");

        #[cfg(all(feature = "check", not(target_family = "wasm")))]
        let builder =
            builder.add_and_register_check(checks::googlefonts::description::broken_links);

        #[cfg(feature = "check")]
        let builder = builder
            .add_and_register_check(checks::googlefonts::description::eof_linebreak)
            .add_and_register_check(checks::googlefonts::description::git_url)
            .add_and_register_check(checks::googlefonts::description::has_article)
            .add_and_register_check(checks::googlefonts::description::has_unsupported_elements)
            .add_and_register_check(checks::googlefonts::description::min_length)
            .add_and_register_check(checks::googlefonts::description::no_free_word)
            .add_and_register_check(checks::googlefonts::description::urls)
            .add_and_register_check(checks::googlefonts::description::valid_html);

        let builder = builder
            .add_section("Family Checks")
            .add_and_register_check(checks::googlefonts::family::equal_codepoint_coverage)
            .add_and_register_check(checks::googlefonts::family::file_size)
            .add_and_register_check(checks::googlefonts::family::italics_have_roman_counterparts)
            .add_and_register_check(checks::googlefonts::family::tnum_horizontal_metrics)
            .add_section("Name table checks")
            .add_and_register_check(checks::googlefonts::name::family_name_compliance)
            .add_and_register_check(checks::googlefonts::name::line_breaks)
            .add_section("Licensing Checks")
            .add_and_register_check(checks::googlefonts::family::has_license);

        #[cfg(feature = "check")]
        let builder = builder
            .add_and_register_check(checks::googlefonts::font_copyright)
            .add_and_register_check(checks::googlefonts::license::OFL_body_text)
            .add_and_register_check(checks::googlefonts::license::OFL_copyright);

        #[cfg(feature = "check")]
        let builder = builder
            .add_and_register_check(checks::googlefonts::metadata::copyright)
            .add_and_register_check(checks::googlefonts::metadata::license)
            .add_and_register_check(checks::googlefonts::metadata::reserved_font_name);

        let builder = builder
            .add_and_register_check(checks::googlefonts::name::license)
            .add_and_register_check(checks::googlefonts::name::license_url)
            .add_and_register_check(checks::googlefonts::name::rfn)
            .add_section("Repository Checks")
            .add_and_register_check(checks::googlefonts::repo::ascii_filenames)
            .add_and_register_check(checks::googlefonts::repo::dirname_matches_nameid_1)
            .add_and_register_check(checks::googlefonts::repo::vf_has_static_fonts)
            .add_section("Shaping Checks")
            .add_and_register_check(checks::dotted_circle);

        #[cfg(feature = "check")]
        let builder = builder.add_and_register_check(checks::soft_dotted);

        #[cfg(all(feature = "check", not(target_family = "wasm")))]
        let builder = builder
            //            Realistically Simon is the only person who uses this check, and it can wait until he needs it again.
            //            checks::shaping::collides
            .add_and_register_check(checks::shaping::forbidden)
            .add_and_register_check(checks::shaping::regression);

        let builder = builder.add_section("Outline Checks");

        #[cfg(feature = "check")]
        let builder = builder
            .add_and_register_check(checks::outline::alignment_miss)
            .add_and_register_check(checks::outline::colinear_vectors)
            .add_and_register_check(checks::outline::direction)
            .add_and_register_check(checks::outline::jaggy_segments)
            .add_and_register_check(checks::outline::overlapping_path_segments)
            .add_and_register_check(checks::outline::semi_vertical)
            .add_and_register_check(checks::outline::short_segments);

        let builder = builder.add_section("Font File Checks");

        #[cfg(all(feature = "check", not(target_family = "wasm")))]
        let builder = builder.add_and_register_check(checks::googlefonts::axes_match);

        #[cfg(feature = "check")]
        let builder = builder.add_and_register_check(checks::googlefonts::glyph_coverage);
        #[cfg(feature = "check")]
        let builder = builder.add_and_register_check(checks::googlefonts::old_ttfautohint);

        let builder = builder
            .add_and_register_check(checks::googlefonts::axisregistry::fvar_axis_defaults)
            .add_and_register_check(checks::googlefonts::axisregistry::fvar_axis_ranges)
            .add_and_register_check(checks::googlefonts::canonical_filename)
            .add_and_register_check(checks::googlefonts::cjk_vertical_metrics)
            .add_and_register_check(checks::googlefonts::color_fonts)
            .add_and_register_check(checks::googlefonts::font_names)
            .add_and_register_check(checks::googlefonts::fstype)
            .add_and_register_check(checks::googlefonts::fvar_instances)
            .add_and_register_check(checks::googlefonts::gasp)
            .add_and_register_check(checks::googlefonts::has_ttfautohint_params)
            .add_and_register_check(checks::googlefonts::meta::script_lang_tags)
            .add_and_register_check(checks::googlefonts::name::description_max_length)
            .add_and_register_check(checks::googlefonts::name::familyname_first_char)
            .add_and_register_check(checks::googlefonts::name::mandatory_entries)
            .add_and_register_check(checks::googlefonts::name::illegal_particles)
            .add_and_register_check(checks::googlefonts::name::version_format)
            .add_and_register_check(checks::googlefonts::parametric_axes_hidden)
            .add_and_register_check(checks::googlefonts::render_own_name)
            .add_and_register_check(checks::googlefonts::STAT::axis_order)
            .add_and_register_check(checks::googlefonts::STAT::axisregistry)
            .add_and_register_check(checks::googlefonts::STAT::compulsory_axis_values)
            .add_and_register_check(checks::googlefonts::STAT::opsz_not_elided)
            .add_and_register_check(checks::STAT_strings)
            .add_and_register_check(checks::googlefonts::unitsperem)
            .add_and_register_check(checks::googlefonts::use_typo_metrics)
            .add_and_register_check(checks::googlefonts::varfont::has_HVAR)
            .add_and_register_check(checks::googlefonts::varfont::slnt_needs_italic)
            .add_and_register_check(checks::googlefonts::vendor_id)
            .add_and_register_check(checks::googlefonts::version_bump)
            .add_and_register_check(checks::googlefonts::vertical_metrics);

        #[cfg(feature = "check")]
        let builder = builder
            .add_and_register_check(checks::googlefonts::vertical_metrics_regressions)
            .add_and_register_check(checks::googlefonts::cjk_vertical_metrics_regressions);

        let builder = builder
            // includes_production_subsets merged into metadata/subsets_correct
            .add_and_register_check(checks::googlefonts::weightclass)
            // Pending review
            .exclude_check("cmap/format_12")
            .exclude_check("empty_letters")
            .exclude_check("inconsistencies_between_fvar_STAT")
            .exclude_check("no_mac_entries")
            .exclude_check("typographic_family_name")
            .exclude_check("vtt_volt_data")
            // Configuration defaultss
            .with_configuration_defaults(
                "file_size",
                HashMap::from([
                    ("WARN_SIZE".to_string(), json!(1048576)),   // 1Mb
                    ("FAIL_SIZE".to_string(), json!(9437184)),   // 9Mb
                    ("FATAL_SIZE".to_string(), json!(10485760)), // 10Mb
                ]),
            )
            .with_configuration_defaults(
                "googlefonts/family/file_size",
                HashMap::from([
                    ("FATAL_SIZE".to_string(), json!(26214400)), // 25Mb
                ]),
            );

        builder.build("googlefonts", cr)
    }
}
