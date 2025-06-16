use crate::{
    constants::{OutlineType, RIBBI_STYLE_NAMES, STATIC_STYLE_NAMES},
    error::FontspectorError,
    filetype::FileTypeConvert,
    Context, FileType, Testable,
};
use fontations::{
    read::TopLevelTable,
    skrifa::{
        font::FontRef,
        outline::{DrawSettings, OutlinePen},
        prelude::Size,
        raw::{
            tables::{
                gdef::GlyphClassDef,
                glyf::Glyph,
                gpos::{PairPos, PairPosFormat1, PairPosFormat2, PositionSubtables},
                head::MacStyle,
                layout::{Feature, FeatureRecord},
                os2::SelectionFlags,
            },
            ReadError, TableProvider,
        },
        setting::VariationSetting,
        string::StringId,
        GlyphId, GlyphId16, GlyphNames, MetadataProvider, Tag,
    },
    write::{validate::Validate, FontWrite},
};
use itertools::Either;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    error::Error,
    fmt::{Debug, Formatter},
    path::{Path, PathBuf},
};

/// A Font to be tested
pub struct TestFont<'a> {
    /// The path to the font file
    pub filename: PathBuf,
    /// The font's binary data
    font_data: &'a [u8],
    // Try to avoid caching stuff here unless you really need to, the conversion Testable->TestFont
    // should be cheap as it is run for each check.
    /// The number of glyphs in the font
    pub glyph_count: usize,
}

impl Debug for TestFont<'_> {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "<TestFont:{}>", self.filename.display())
    }
}

/// A file type for TrueType fonts
pub const TTF: FileType = FileType {
    pattern: "*.[ot]tf",
};

impl<'a> FileTypeConvert<'a, TestFont<'a>> for FileType<'a> {
    fn from_testable(&self, t: &'a Testable) -> Option<TestFont<'a>> {
        self.applies(t)
            .then(|| TestFont::new_from_data(&t.filename, &t.contents))
            .transpose()
            .unwrap_or(None)
    }
}

impl TestFont<'_> {
    /// Create a new TestFont from a file path and binary data
    pub fn new_from_data<'a>(
        filename: &Path,
        font_data: &'a [u8],
    ) -> Result<TestFont<'a>, Box<dyn Error>> {
        let font = FontRef::new(font_data)?;
        let glyph_count = font.maxp()?.num_glyphs().into();
        Ok(TestFont {
            filename: filename.to_path_buf(),
            font_data,
            glyph_count,
        })
    }

    /// A [read-fonts](https://docs.rs/read-fonts/) font object
    pub fn font(&self) -> FontRef {
        #[allow(clippy::expect_used)] // We just tested for it in the initializer
        FontRef::new(self.font_data).expect("Can't happen")
    }

    /// Get the font's style name
    ///
    /// For a variable font, we try to determine the style from the default location
    /// and whether the name contains `Italic`. For a static font, we try to match
    /// the style name part of the filename to a list of known styles.
    pub fn style(&self) -> Option<&str> {
        if self.is_variable_font() {
            if let Some(default_location) = self.default_location() {
                if default_location.get("wght") == Some(&700.0) {
                    if self.filename.to_str()?.contains("Italic") {
                        return Some("BoldItalic");
                    } else {
                        return Some("Bold");
                    }
                } else {
                    if self.filename.to_str()?.contains("Italic") {
                        return Some("Italic");
                    }
                    return Some("Regular");
                }
            }
        }
        if let Some(style_part) = self.filename.file_stem()?.to_str()?.split('-').next_back() {
            for styles in STATIC_STYLE_NAMES.iter() {
                if style_part == styles.replace(" ", "") {
                    return Some(style_part);
                }
            }
        }
        None
    }

    /// Is this a RIBBI font?
    ///
    /// A RIBBI font is one that is Regular, Italic, Bold, BoldItalic. We determine
    /// the style using the [style](TestFont::style) method.
    pub fn is_ribbi(&self) -> bool {
        self.style()
            .is_some_and(|s| RIBBI_STYLE_NAMES.iter().any(|r| r == &s))
    }

    /// Is this font italic?
    pub fn is_italic(&self) -> Result<bool, ReadError> {
        let font = self.font();
        let os2 = font.os2()?;
        if os2.fs_selection().contains(SelectionFlags::ITALIC) {
            return Ok(true);
        }
        let head = font.head()?;
        if head.mac_style().contains(MacStyle::ITALIC) {
            return Ok(true);
        }
        if self
            .get_name_entry_strings(StringId::FULL_NAME)
            .any(|x| x.to_lowercase().contains("italic"))
        {
            return Ok(true);
        }
        let post = font.post()?;
        if post.italic_angle().to_f32() != 0.0 {
            return Ok(true);
        }
        Ok(false)
    }

    /// Does this font contain a given TrueType table?
    pub fn has_table(&self, table: &[u8; 4]) -> bool {
        self.font().table_data(Tag::new(table)).is_some()
    }

    /// Return the GDEF class for a glyph
    pub fn gdef_class(&self, glyph_id: impl Into<GlyphId>) -> GlyphClassDef {
        if let Some(Ok(class_def)) = self
            .font()
            .gdef()
            .ok()
            .and_then(|gdef| gdef.glyph_class_def())
        {
            GlyphId16::try_from(glyph_id.into())
                .map(|gid| class_def.get(gid))
                .map_or(GlyphClassDef::Unknown, GlyphClassDef::new)
        } else {
            GlyphClassDef::Unknown
        }
    }

    /// Return the OS/2 table FsSelection flags
    pub fn get_os2_fsselection(&self) -> Result<SelectionFlags, FontspectorError> {
        let os2 = self.font().os2()?;
        Ok(os2.fs_selection())
    }

    /// Get a string from the font's name table by Name ID
    pub fn get_name_entry_strings(&self, name_id: StringId) -> impl Iterator<Item = String> + '_ {
        self.font()
            .localized_strings(name_id)
            .map(|s| s.to_string())
    }

    /// Internal implementation for getting a glyph name
    fn glyph_name_for_id_impl(&self, gid: impl Into<GlyphId>, synthesize: bool) -> Option<String> {
        let names = GlyphNames::new(&self.font());
        let proposed_name = names.get(gid.into());
        proposed_name.and_then(|name| {
            if name.is_synthesized() && !synthesize {
                None
            } else {
                Some(name.as_str().to_string())
            }
        })
    }

    /// Get a glyph's name by Glyph ID, if present in the font
    pub fn glyph_name_for_id(&self, gid: impl Into<GlyphId>) -> Option<String> {
        self.glyph_name_for_id_impl(gid, false)
    }
    /// Get a glyph's name by Glyph ID, synthesizing a name if not present in the font
    ///
    /// For example GID 1024 will be named "gid1024".
    pub fn glyph_name_for_id_synthesise(&self, gid: impl Into<GlyphId>) -> String {
        #[allow(clippy::unwrap_used)]
        self.glyph_name_for_id_impl(gid, true).unwrap()
    }
    /// Internal implementation for getting a glyph name by Unicode codepoint
    fn glyph_name_for_unicode_impl(&self, u: impl Into<u32>, synthesize: bool) -> Option<String> {
        self.font()
            .charmap()
            .map(u)
            .and_then(|gid| self.glyph_name_for_id_impl(gid, synthesize))
    }
    /// Get a glyph's name by Unicode codepoint, if present in the font
    pub fn glyph_name_for_unicode(&self, u: impl Into<u32>) -> Option<String> {
        self.glyph_name_for_unicode_impl(u, false)
    }
    /// Get a glyph's name by Unicode codepoint, synthesizing a name if not present in the font
    pub fn glyph_name_for_unicode_synthesise(&self, u: impl Into<u32>) -> String {
        #[allow(clippy::unwrap_used)]
        self.glyph_name_for_unicode_impl(u, true).unwrap()
    }

    /// Retrieve a glyph by ID from the `glyf` table
    pub fn get_glyf_glyph(&self, gid: GlyphId) -> Result<Option<Glyph>, ReadError> {
        let loca = self.font().loca(None)?;
        let glyf = self.font().glyf()?;
        loca.get_glyf(gid, &glyf)
    }

    /// Is this font a variable font?
    pub fn is_variable_font(&self) -> bool {
        self.has_table(b"fvar")
    }

    /// Return the font's outline type
    pub fn outline_type(&self) -> OutlineType {
        if self.has_table(b"glyf") {
            OutlineType::TrueType
        } else {
            OutlineType::CFF
        }
    }

    /// Does this font have a given variation axis?
    pub fn has_axis(&self, axis: &str) -> bool {
        self.is_variable_font() && self.font().axes().iter().any(|a| a.tag() == axis)
    }

    /// The font's default location in userspace coordinates
    pub fn default_location(&self) -> Option<HashMap<String, f32>> {
        Some(
            self.font()
                .fvar()
                .ok()?
                .axes()
                .ok()?
                .iter()
                .map(|axis| {
                    let tag = axis.axis_tag().to_string();
                    let default = axis.default_value().to_f32();
                    (tag, default)
                })
                .collect(),
        )
    }

    /// The set of Unicode codepoints in the font
    pub fn codepoints(&self, context: Option<&Context>) -> HashSet<u32> {
        let get_codepoints = || {
            Ok(self
                .font()
                .charmap()
                .mappings()
                .map(|(u, _gid)| u)
                .collect::<HashSet<u32>>())
        };
        if let Some(context) = context {
            let key = "codepoints:".to_string() + &self.filename.to_string_lossy();
            #[allow(clippy::unwrap_used)] // How can it fail?!
            context
                .cached_question(
                    &key,
                    get_codepoints,
                    |hashset| serde_json::to_value(hashset).unwrap(),
                    |value| {
                        serde_json::from_value(value.clone())
                            .map_err(|e| FontspectorError::CacheSerialization(e.to_string()))
                    },
                )
                .unwrap_or_default()
        } else {
            get_codepoints().unwrap_or_default()
        }
    }

    /// Returns an iterator over the named instances in the font.
    ///
    /// Each item is a tuple of the instance name and a map of axis tag to user coordinate value.
    pub fn named_instances(&self) -> impl Iterator<Item = (String, BTreeMap<String, f32>)> + '_ {
        self.font().named_instances().iter().map(|ni| {
            let instance_name = self
                .font()
                .localized_strings(ni.subfamily_name_id())
                .english_or_first()
                .map(|s| s.chars().collect::<String>())
                .unwrap_or("Unnamed".to_string());
            let coords = ni
                .user_coords()
                .zip(self.font().axes().iter())
                .map(|(coord, axis)| (axis.tag().to_string(), coord));
            (instance_name, coords.collect())
        })
    }

    /// Return the ranges of the font's variation space
    ///
    /// This returns an iterator of items `name, min, default, max` for each axis.
    pub fn axis_ranges(&self) -> impl Iterator<Item = (String, f32, f32, f32)> + '_ {
        self.font().axes().iter().map(|axis| {
            let tag = axis.tag().to_string();
            let min = axis.min_value();
            let max = axis.max_value();
            let def = axis.default_value();
            (tag, min, def, max)
        })
    }

    /// Draw a glyph at the given location using the provided Pen.
    pub fn draw_glyph<I>(
        &self,
        gid: GlyphId,
        pen: &mut impl OutlinePen,
        settings: I,
    ) -> Result<(), FontspectorError>
    where
        I: IntoIterator,
        I::Item: Into<VariationSetting>,
    {
        let glyph = self
            .font()
            .outline_glyphs()
            .get(gid)
            .ok_or_else(|| FontspectorError::skip("no-H", "No H glyph in font"))?;
        let location = self.font().axes().location(settings);
        let settings = DrawSettings::unhinted(Size::unscaled(), &location);

        glyph.draw(settings, pen)?;
        Ok(())
    }

    /// Returns the font's FeatureRecord and associated Feature tables
    ///
    /// If `gsub_only` is true, only searches in the `GSUB` table.
    pub fn feature_records(
        &self,
        gsub_only: bool,
    ) -> impl Iterator<Item = (&FeatureRecord, Result<Feature, ReadError>)> {
        let gsub_featurelist = self
            .font()
            .gsub()
            .ok()
            .and_then(|gsub| gsub.feature_list().ok());
        let gpos_feature_list = self
            .font()
            .gpos()
            .ok()
            .and_then(|gpos| gpos.feature_list().ok());
        let gsub_feature_and_data = gsub_featurelist.map(|list| {
            list.feature_records()
                .iter()
                .map(move |feature| (feature, feature.feature(list.offset_data())))
        });
        let gpos_feature_and_data = gpos_feature_list.map(|list| {
            list.feature_records()
                .iter()
                .map(move |feature| (feature, feature.feature(list.offset_data())))
        });
        let iter = gsub_feature_and_data.into_iter().flatten();
        if gsub_only {
            Either::Left(iter)
        } else {
            Either::Right(iter.chain(gpos_feature_and_data.into_iter().flatten()))
        }
    }

    /// Does the font have a given feature?
    ///
    /// If `gsub_only` is true, only searches in the `GSUB` table.
    pub fn has_feature(&self, gsub_only: bool, tag: &str) -> bool {
        self.feature_records(gsub_only)
            .any(|(f, _)| f.feature_tag() == tag)
    }

    /// An iterator of all glyphs in the font
    pub fn all_glyphs(&self) -> impl Iterator<Item = GlyphId> {
        (0..self.glyph_count as u32).map(GlyphId::from)
    }

    /// An iterator of all glyphs in the font that are CJK
    pub fn cjk_codepoints(&self, context: Option<&Context>) -> impl Iterator<Item = u32> {
        self.codepoints(context)
            .into_iter()
            .filter(|&cp| is_cjk(cp))
    }

    /// Is this font a CJK font?
    ///
    /// A font is considered a CJK font if it contains more than 150 CJK codepoints.
    /// This is because 150 is the minimal number of CJK glyphs to support a Korean font,
    /// which in turn is the smallest CJK set.
    pub fn is_cjk_font(&self, context: Option<&Context>) -> bool {
        self.cjk_codepoints(context).count() > 150
    }

    /// Walk a font's kern pairs
    ///
    /// This function looks at all  the pair positioning rules in a font's GPOS table
    /// gathering information about the kerning pairs. It needs two functions to process
    /// the two different PairPos format tables. See the [tabular_kerning](../../profile-universal/checks/tabular_kerning.html) check for
    /// an example of how to use it.
    pub fn process_kerning<T>(
        &self,
        format1_func: &dyn Fn(PairPosFormat1) -> Result<Vec<T>, ReadError>,
        format2_func: &dyn Fn(PairPosFormat2) -> Result<Vec<T>, ReadError>,
    ) -> Result<Vec<T>, ReadError> {
        let gpos = self.font().gpos()?;
        Ok(
            gpos.lookup_list()?
                .lookups()
                .iter()
                .flatten()
                .flat_map(|l| l.subtables())
                .filter_map(|s| match s {
                    PositionSubtables::Pair(p) => Some(p),
                    _ => None,
                })
                .flat_map(|p| p.iter())
                .flatten()
                .map(|pp| match pp {
                    PairPos::Format1(pp1) => format1_func(pp1),
                    PairPos::Format2(pp2) => format2_func(pp2),
                })
                .flat_map(|v| v.into_iter())
                .flatten()
                .collect(), // NOW WASH YOUR HANDS
        )
    }

    /// Get the best name from a list of name IDs
    pub fn get_best_name(&self, ids: &[StringId]) -> Option<String> {
        for id in ids {
            if let Some(name) = self.font().localized_strings(*id).english_or_first() {
                return Some(name.chars().collect());
            }
        }
        None
    }

    /// Returns the best English family name for a font
    pub fn best_familyname(&self) -> Option<String> {
        self.get_best_name(&[
            StringId::WWS_FAMILY_NAME,
            StringId::TYPOGRAPHIC_FAMILY_NAME,
            StringId::FAMILY_NAME,
        ])
    }

    /// Returns the best English subfamily name for a font
    pub fn best_subfamilyname(&self) -> Option<String> {
        self.get_best_name(&[
            StringId::WWS_SUBFAMILY_NAME,
            StringId::TYPOGRAPHIC_SUBFAMILY_NAME,
            StringId::SUBFAMILY_NAME,
        ])
    }

    /// Returns the font's vertical metrics
    pub fn vertical_metrics(&self) -> Result<VerticalMetrics, FontspectorError> {
        Ok(VerticalMetrics {
            upm: self.font().head()?.units_per_em(),
            os2_typo_ascender: self.font().os2()?.s_typo_ascender(),
            os2_typo_descender: self.font().os2()?.s_typo_descender(),
            os2_typo_linegap: self.font().os2()?.s_typo_line_gap(),
            hhea_ascent: self.font().hhea()?.ascender().to_i16(),
            hhea_descent: self.font().hhea()?.descender().to_i16(),
            hhea_linegap: self.font().hhea()?.line_gap().to_i16(),
            os2_win_ascent: self.font().os2()?.us_win_ascent(),
            os2_win_descent: self.font().os2()?.us_win_descent(),
        })
    }

    /// True if the font's OS/2.fsSelection bit 7 (USE_TYPO_METRICS) is set
    pub fn use_typo_metrics(&self) -> Result<bool, FontspectorError> {
        Ok(self
            .font()
            .os2()?
            .fs_selection()
            .intersects(SelectionFlags::USE_TYPO_METRICS))
    }

    /// Rebuild the font with new tables
    ///
    /// This will create a font with the tables specified, and then copy all other
    /// tables from the original font.
    pub fn rebuild_with_new_tables<T: FontWrite + Validate + TopLevelTable>(
        &self,
        tables: &[T],
    ) -> Result<Vec<u8>, FontspectorError> {
        let mut new_font = fontations::write::FontBuilder::new();
        for table in tables {
            new_font.add_table(table)?;
        }
        new_font.copy_missing_tables(self.font());
        Ok(new_font.build())
    }
}

/// Is a codepoint a CJK character?
fn is_cjk(cp: u32) -> bool {
    crate::constants::CJK_UNICODE_RANGES
        .iter()
        .any(|range| range.contains(&cp))
}

/// An empty [VariationSetting] for use in default location.
pub const DEFAULT_LOCATION: &[VariationSetting] = &[];

/// A font's vertical metrics
pub struct VerticalMetrics {
    /// The font's units per em
    pub upm: u16,
    /// The OS/2 Typographic Ascender
    pub os2_typo_ascender: i16,
    /// The OS/2 Typographic Descender
    pub os2_typo_descender: i16,
    /// The OS/2 Typographic Line Gap
    pub os2_typo_linegap: i16,
    /// The OS/2 Windows Ascender
    pub os2_win_ascent: u16,
    /// The OS/2 Windows Descender
    pub os2_win_descent: u16,
    /// The hhea Ascender
    pub hhea_ascent: i16,
    /// The hhea Descender
    pub hhea_descent: i16,
    /// The hhea Line Gap
    pub hhea_linegap: i16,
}

impl VerticalMetrics {
    /// Scale the vertical metrics to a different UPM
    pub fn scale_to_upm(&self, other_upm: u16) -> VerticalMetrics {
        let scaled_upm = other_upm as f32 / self.upm as f32;
        VerticalMetrics {
            upm: other_upm,
            os2_typo_ascender: (self.os2_typo_ascender as f32 * scaled_upm).ceil() as i16,
            os2_typo_descender: (self.os2_typo_descender as f32 * scaled_upm).ceil() as i16,
            os2_typo_linegap: (self.os2_typo_linegap as f32 * scaled_upm).ceil() as i16,
            os2_win_ascent: (self.os2_win_ascent as f32 * scaled_upm).ceil() as u16,
            os2_win_descent: (self.os2_win_descent as f32 * scaled_upm).ceil() as u16,
            hhea_ascent: (self.hhea_ascent as f32 * scaled_upm).ceil() as i16,
            hhea_descent: (self.hhea_descent as f32 * scaled_upm).ceil() as i16,
            hhea_linegap: (self.hhea_linegap as f32 * scaled_upm).ceil() as i16,
        }
    }
}
