use fontdrasil::coords::{NormalizedCoord, NormalizedLocation, UserSpace};
use fontdrasil::types::Axes;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use serde_json::{json, Map, Value};
use skrifa::raw::tables::{
    head::Head, hhea::Hhea, maxp::Maxp, os2::Os2, post::Post, stat::AxisValue, stat::Stat,
    vhea::Vhea,
};
use skrifa::raw::types::Fixed;
use skrifa::raw::FontRef;
use skrifa::{raw::TableProvider, MetadataProvider};
use tabled::{Table, Tabled};
use write_fonts::types::NameId;

#[derive(Tabled)]
struct TableEntry {
    field: String,
    expected: String,
    found: String,
}
#[check(
    id = "field_values",
    rationale = "Some foundries want to know that, for example, vertical metrics are exactly set to certain values, flags are set in a certain way, name table entries are just so, and so on.

    This check expects to find a table of field names and field values in the configuration file, and checks to ensure that the font's metadata matches these expected values.

    Example:

    ```
    [field_values]
    hhea.ascender = 927
    \"OS/2.sxHeight\" = 518 # Key needs to be escaped because of / in OS/2
    name.versionString = { \"en-us\" = \"Version 1.008\" } # Languages must be present
    fvar.axes = {
        wght = { name = \"Weight\", min = 100, max = 900, default = 400 }
    }
    fvar.namedInstances = {
        Thin = { wght = 100 },
        Light = { wght = 300 },
        ...
    }
    ```

    Alternatively, the configuration can be specialized on a per-font basis:

    ```
    [field_values.\"Foo-Regular.ttf\"]
    hhea.ascender = 990
    [field_values.\"Foo-Bold.ttf\"]
    hhea.ascender = 1020
    ```

    Field names should be `camelcase` forms of the names given in the OpenType specification.
    ",
    proposal = "https://github.com/fonttools/fontspector/issues/404",
    title = "Ensure field data is as expected."
)]
fn field_values(t: &Testable, context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    let config = context.local_config("field_values");
    skip!(
        config.is_null(),
        "unconfigured",
        "No configuration found for field_values"
    );
    if let Some(config) = config.as_object() {
        // If the config is a table of tables, specialize it by font filename
        let config_for_this_font = if config.values().all(|v| v.is_object())
            && config
                .keys()
                .all(|k| k.ends_with(".ttf") || k.ends_with(".otf"))
        {
            if let Some(specific) =
                config.get(&t.basename().unwrap_or("<Unnamed Font>".to_string()))
            {
                if let Some(specific) = specific.as_object() {
                    specific
                } else {
                    return Err(FontspectorError::General(
                        "Configuration for field_values is not an object".to_string(),
                    ));
                }
            } else {
                skip!("unconfigured", "No entry for this file");
            }
        } else {
            config
        };

        let serialized = font_to_json(&font.font(), font.fontdrasil_axes()?.as_ref());
        let mut incorrect = vec![];
        for (key, value) in config_for_this_font.iter() {
            let found = serialized.get(key);
            match found {
                Some(found) if equal_enough(found, value) => continue,
                Some(found) => {
                    incorrect.push(TableEntry {
                        field: key.clone(),
                        expected: value.clone().to_string(),
                        found: found.to_string(),
                    });
                }
                None => {
                    // No field for this key? Maybe the key was spelt wrong
                    #[expect(
                        clippy::unwrap_used,
                        reason = "Since the config is validated to be an object, we can be sure that this unwrap won't panic"
                    )]
                    let suggestion = did_you_mean(
                        key,
                        serialized.as_object().unwrap().keys().cloned().collect(),
                    );
                    if let Some(suggestion) = suggestion {
                        incorrect.push(TableEntry {
                            field: key.clone(),
                            expected: value.clone().to_string(),
                            found: format!(
                                "<Not present> (Did you mean {}?
                            )",
                                suggestion
                            ),
                        });
                        continue;
                    } else {
                        incorrect.push(TableEntry {
                            field: key.clone(),
                            expected: value.clone().to_string(),
                            found: "<Not present>".to_string(),
                        });
                        continue;
                    }
                }
            }
        }
        if incorrect.is_empty() {
            return Ok(Status::just_one_pass());
        } else {
            let mut table = Table::new(incorrect);
            table.with(tabled::settings::Style::markdown());
            return Ok(Status::just_one_fail(
                "incorrect-field-values",
                &format!("The following fields have incorrect values:\n\n{}", table),
            ));
        }
    } else {
        return Err(FontspectorError::General(
            "Configuration for field_values is not an object".to_string(),
        ));
    }
}

fn did_you_mean(key: &str, options: Vec<String>) -> Option<String> {
    let mut best_distance = usize::MAX;
    let mut best_option = None;
    for option in options {
        let distance = edit_distance(key, &option);
        if distance < best_distance {
            best_distance = distance;
            best_option = Some(option);
        }
    }
    // Only suggest if the distance is reasonably small; otherwise we might be suggesting something completely different
    if best_distance <= 3 {
        best_option
    } else {
        None
    }
}

#[allow(clippy::indexing_slicing, clippy::needless_range_loop)] // It's not my code; taken from edit_distance crate
pub fn edit_distance(a: impl AsRef<str>, b: impl AsRef<str>) -> usize {
    let len_a = a.as_ref().chars().count();
    let len_b = b.as_ref().chars().count();
    if len_a < len_b {
        return edit_distance(b, a);
    }
    // handle special case of 0 length
    if len_a == 0 {
        return len_b;
    } else if len_b == 0 {
        return len_a;
    }

    let len_b = len_b + 1;

    let mut pre;
    let mut tmp;
    let mut cur = vec![0; len_b];

    // initialize string b
    for i in 1..len_b {
        cur[i] = i;
    }

    // calculate edit distance
    for (i, ca) in a.as_ref().chars().enumerate() {
        // get first column for this row
        pre = cur[0];
        cur[0] = i + 1;
        for (j, cb) in b.as_ref().chars().enumerate() {
            tmp = cur[j + 1];
            cur[j + 1] = std::cmp::min(
                // deletion
                tmp + 1,
                std::cmp::min(
                    // insertion
                    cur[j] + 1,
                    // match or substitution
                    pre + if ca == cb { 0 } else { 1 },
                ),
            );
            pre = tmp;
        }
    }
    cur[len_b - 1]
}

// Flatten the map: `{ table: { field: value} }` -> ` { table.field: value }`
fn flatten_map(
    map: &serde_json::Map<String, serde_json::Value>,
) -> serde_json::Map<String, serde_json::Value> {
    let mut out = serde_json::Map::new();
    for (key, value) in map.iter() {
        if let serde_json::Value::Object(obj) = value {
            for (subkey, subvalue) in obj.iter() {
                out.insert(format!("{}.{}", key, subkey), subvalue.clone());
            }
        } else {
            out.insert(key.clone(), value.clone());
        }
    }
    out
}

// This code taken from diffenator3's ttj crate

pub fn font_to_json(font: &FontRef, axes: Option<&Axes>) -> Value {
    let mut map = Map::new();

    // Some tables are serialized by hand below; typically those which are just a set of
    // fields and values (or are so complicated we haven't yet been bothered to write our
    // own serializers for them...)
    for table in font.table_directory.table_records().iter() {
        let key = table.tag().to_string();
        let value = match table.tag().into_bytes().as_ref() {
            b"head" => font.head().map(|t| serialize_head(&t)),
            b"hhea" => font.hhea().map(|t| serialize_hhea(&t)),
            b"vhea" => font.vhea().map(|t| serialize_vhea(&t)),
            b"maxp" => font.maxp().map(|t| serialize_maxp(&t)),
            b"OS/2" => font.os2().map(|t| serialize_os2(&t)),
            b"post" => font.post().map(|t| serialize_post(&t)),
            b"STAT" => font.stat().map(|t| serialize_stat(&t)),
            _ => continue,
        };
        map.insert(
            key,
            value.unwrap_or_else(|_| Value::String("Could not parse".to_string())),
        );
    }

    // Other tables require a bit of massaging to produce information which makes sense to test.
    map.insert("name".to_string(), serialize_name_table(font));
    map.insert("fvar".to_string(), serialize_fvar_table(font, axes));
    Value::Object(flatten_map(&map))
}

// The field names of the serialized tables are the camelCase forms of the names given in
// the OpenType specification, which are also the names this check has always used.
fn json_object(fields: impl IntoIterator<Item = (&'static str, Value)>) -> Value {
    Value::Object(
        fields
            .into_iter()
            .map(|(field, value)| (field.to_string(), value))
            .collect(),
    )
}

fn bare_offset(offset: u32) -> Value {
    Value::String(format!("0x{:04X}", offset))
}

// 16.16 fixed point numbers have at most five decimal places; rounding off anything
// beyond that avoids exposing the floating point representation to the user
// (1.008 rather than 1.0080032348632812).
fn fixed_to_json(value: Fixed) -> Value {
    let value = (value.to_f64() * 100_000.0).round() / 100_000.0;
    json!(value)
}

fn serialize_head(head: &Head) -> Value {
    json_object([
        ("version", json!(head.version().to_string())),
        ("fontRevision", fixed_to_json(head.font_revision())),
        ("checksumAdjustment", json!(head.checksum_adjustment())),
        ("magicNumber", json!(head.magic_number())),
        ("flags", json!(head.flags().bits())),
        ("unitsPerEm", json!(head.units_per_em())),
        ("created", json!(head.created().as_secs())),
        ("modified", json!(head.modified().as_secs())),
        ("xMin", json!(head.x_min())),
        ("yMin", json!(head.y_min())),
        ("xMax", json!(head.x_max())),
        ("yMax", json!(head.y_max())),
        ("macStyle", json!(head.mac_style().bits())),
        ("lowestRecPpem", json!(head.lowest_rec_ppem())),
        ("fontDirectionHint", json!(head.font_direction_hint())),
        ("indexToLocFormat", json!(head.index_to_loc_format())),
        ("glyphDataFormat", json!(head.glyph_data_format())),
    ])
}

fn serialize_hhea(hhea: &Hhea) -> Value {
    json_object([
        ("version", json!(hhea.version().to_string())),
        ("ascender", json!(hhea.ascender().to_i16())),
        ("descender", json!(hhea.descender().to_i16())),
        ("lineGap", json!(hhea.line_gap().to_i16())),
        ("advanceWidthMax", json!(hhea.advance_width_max().to_u16())),
        (
            "minLeftSideBearing",
            json!(hhea.min_left_side_bearing().to_i16()),
        ),
        (
            "minRightSideBearing",
            json!(hhea.min_right_side_bearing().to_i16()),
        ),
        ("xMaxExtent", json!(hhea.x_max_extent().to_i16())),
        ("caretSlopeRise", json!(hhea.caret_slope_rise())),
        ("caretSlopeRun", json!(hhea.caret_slope_run())),
        ("caretOffset", json!(hhea.caret_offset())),
        ("metricDataFormat", json!(hhea.metric_data_format())),
        ("numberOfHMetrics", json!(hhea.number_of_h_metrics())),
    ])
}

fn serialize_vhea(vhea: &Vhea) -> Value {
    json_object([
        ("version", json!(vhea.version().to_string())),
        ("ascender", json!(vhea.ascender().to_i16())),
        ("descender", json!(vhea.descender().to_i16())),
        ("lineGap", json!(vhea.line_gap().to_i16())),
        (
            "advanceHeightMax",
            json!(vhea.advance_height_max().to_u16()),
        ),
        (
            "minTopSideBearing",
            json!(vhea.min_top_side_bearing().to_i16()),
        ),
        (
            "minBottomSideBearing",
            json!(vhea.min_bottom_side_bearing().to_i16()),
        ),
        ("yMaxExtent", json!(vhea.y_max_extent().to_i16())),
        ("caretSlopeRise", json!(vhea.caret_slope_rise())),
        ("caretSlopeRun", json!(vhea.caret_slope_run())),
        ("caretOffset", json!(vhea.caret_offset())),
        ("metricDataFormat", json!(vhea.metric_data_format())),
        (
            "numberOfLongVerMetrics",
            json!(vhea.number_of_long_ver_metrics()),
        ),
    ])
}

fn serialize_maxp(maxp: &Maxp) -> Value {
    let mut fields = vec![
        ("version", json!(maxp.version().to_string())),
        ("numGlyphs", json!(maxp.num_glyphs())),
    ];
    for (field, value) in [
        ("maxPoints", maxp.max_points()),
        ("maxContours", maxp.max_contours()),
        ("maxCompositePoints", maxp.max_composite_points()),
        ("maxCompositeContours", maxp.max_composite_contours()),
        ("maxZones", maxp.max_zones()),
        ("maxTwilightPoints", maxp.max_twilight_points()),
        ("maxStorage", maxp.max_storage()),
        ("maxFunctionDefs", maxp.max_function_defs()),
        ("maxInstructionDefs", maxp.max_instruction_defs()),
        ("maxStackElements", maxp.max_stack_elements()),
        ("maxSizeOfInstructions", maxp.max_size_of_instructions()),
        ("maxComponentElements", maxp.max_component_elements()),
        ("maxComponentDepth", maxp.max_component_depth()),
    ] {
        if let Some(value) = value {
            fields.push((field, json!(value)));
        }
    }
    json_object(fields)
}

fn serialize_os2(os2: &Os2) -> Value {
    let mut fields = vec![
        ("version", json!(os2.version())),
        ("xAvgCharWidth", json!(os2.x_avg_char_width())),
        ("usWeightClass", json!(os2.us_weight_class())),
        ("usWidthClass", json!(os2.us_width_class())),
        ("fsType", json!(os2.fs_type())),
        ("ySubscriptXSize", json!(os2.y_subscript_x_size())),
        ("ySubscriptYSize", json!(os2.y_subscript_y_size())),
        ("ySubscriptXOffset", json!(os2.y_subscript_x_offset())),
        ("ySubscriptYOffset", json!(os2.y_subscript_y_offset())),
        ("ySuperscriptXSize", json!(os2.y_superscript_x_size())),
        ("ySuperscriptYSize", json!(os2.y_superscript_y_size())),
        ("ySuperscriptXOffset", json!(os2.y_superscript_x_offset())),
        ("ySuperscriptYOffset", json!(os2.y_superscript_y_offset())),
        ("yStrikeoutSize", json!(os2.y_strikeout_size())),
        ("yStrikeoutPosition", json!(os2.y_strikeout_position())),
        ("sFamilyClass", json!(os2.s_family_class())),
        (
            "panose10",
            Value::Array(os2.panose_10().iter().map(|byte| json!(byte)).collect()),
        ),
        ("ulUnicodeRange1", json!(os2.ul_unicode_range_1())),
        ("ulUnicodeRange2", json!(os2.ul_unicode_range_2())),
        ("ulUnicodeRange3", json!(os2.ul_unicode_range_3())),
        ("ulUnicodeRange4", json!(os2.ul_unicode_range_4())),
        ("achVendId", json!(os2.ach_vend_id().to_string())),
        ("fsSelection", json!(os2.fs_selection().bits())),
        ("usFirstCharIndex", json!(os2.us_first_char_index())),
        ("usLastCharIndex", json!(os2.us_last_char_index())),
        ("sTypoAscender", json!(os2.s_typo_ascender())),
        ("sTypoDescender", json!(os2.s_typo_descender())),
        ("sTypoLineGap", json!(os2.s_typo_line_gap())),
        ("usWinAscent", json!(os2.us_win_ascent())),
        ("usWinDescent", json!(os2.us_win_descent())),
    ];
    for (field, value) in [
        (
            "ulCodePageRange1",
            os2.ul_code_page_range_1().map(|value| json!(value)),
        ),
        (
            "ulCodePageRange2",
            os2.ul_code_page_range_2().map(|value| json!(value)),
        ),
        ("sxHeight", os2.sx_height().map(|value| json!(value))),
        ("sCapHeight", os2.s_cap_height().map(|value| json!(value))),
        (
            "usDefaultChar",
            os2.us_default_char().map(|value| json!(value)),
        ),
        ("usBreakChar", os2.us_break_char().map(|value| json!(value))),
        (
            "usMaxContext",
            os2.us_max_context().map(|value| json!(value)),
        ),
        (
            "usLowerOpticalPointSize",
            os2.us_lower_optical_point_size().map(|value| json!(value)),
        ),
        (
            "usUpperOpticalPointSize",
            os2.us_upper_optical_point_size().map(|value| json!(value)),
        ),
    ] {
        if let Some(value) = value {
            fields.push((field, value));
        }
    }
    json_object(fields)
}

fn serialize_post(post: &Post) -> Value {
    // The glyph name strings are reachable through the (gappy) string data, and were
    // never serialized meaningfully; only the name indices are reported.
    let mut fields = vec![
        ("version", json!(post.version().to_string())),
        ("italicAngle", fixed_to_json(post.italic_angle())),
        (
            "underlinePosition",
            json!(post.underline_position().to_i16()),
        ),
        (
            "underlineThickness",
            json!(post.underline_thickness().to_i16()),
        ),
        ("isFixedPitch", json!(post.is_fixed_pitch())),
        ("minMemType42", json!(post.min_mem_type42())),
        ("maxMemType42", json!(post.max_mem_type42())),
        ("minMemType1", json!(post.min_mem_type1())),
        ("maxMemType1", json!(post.max_mem_type1())),
    ];
    if let Some(num_glyphs) = post.num_glyphs() {
        fields.push(("numGlyphs", json!(num_glyphs)));
    }
    if let Some(glyph_name_index) = post.glyph_name_index() {
        fields.push((
            "glyphNameIndex",
            Value::Array(
                glyph_name_index
                    .iter()
                    .map(|index| json!(index.get()))
                    .collect(),
            ),
        ));
    }
    json_object(fields)
}

fn serialize_stat(stat: &Stat) -> Value {
    let mut fields = vec![
        ("version", json!(stat.version().to_string())),
        ("designAxisSize", json!(stat.design_axis_size())),
        ("designAxisCount", json!(stat.design_axis_count())),
        ("designAxesOffset", serialize_design_axes(stat)),
        ("axisValueCount", json!(stat.axis_value_count())),
        (
            "offsetToAxisValueOffsets",
            serialize_axis_value_offsets(stat),
        ),
    ];
    if let Some(name_id) = stat.elided_fallback_name_id() {
        fields.push(("elidedFallbackNameId", json!(name_id.to_string())));
    }
    json_object(fields)
}

fn serialize_design_axes(stat: &Stat) -> Value {
    if stat.design_axis_count() == 0 {
        return bare_offset(stat.design_axes_offset().to_u32());
    }
    match stat.design_axes() {
        Ok(axes) => Value::Array(
            axes.iter()
                .map(|axis| {
                    json_object([
                        ("axisTag", json!(axis.axis_tag().to_string())),
                        ("axisNameId", json!(axis.axis_name_id().to_string())),
                        ("axisOrdering", json!(axis.axis_ordering())),
                    ])
                })
                .collect(),
        ),
        Err(_) => Value::Null,
    }
}

fn serialize_axis_value_offsets(stat: &Stat) -> Value {
    match stat.offset_to_axis_values() {
        Some(Ok(axis_values)) => json_object([(
            "axisValueOffsets",
            Value::Array(
                axis_values
                    .axis_values()
                    .iter()
                    .map(|value| {
                        value
                            .as_ref()
                            .map_or(Value::Null, |value| serialize_axis_value(value))
                    })
                    .collect(),
            ),
        )]),
        Some(Err(_)) => Value::Null,
        None => bare_offset(stat.offset_to_axis_value_offsets().offset().to_u32()),
    }
}

fn serialize_axis_value(value: &AxisValue) -> Value {
    match value {
        AxisValue::Format1(value) => json_object([
            ("format", json!(value.format())),
            ("axisIndex", json!(value.axis_index())),
            ("flags", json!(value.flags().bits())),
            ("valueNameId", json!(value.value_name_id().to_string())),
            ("value", fixed_to_json(value.value())),
        ]),
        AxisValue::Format2(value) => json_object([
            ("format", json!(value.format())),
            ("axisIndex", json!(value.axis_index())),
            ("flags", json!(value.flags().bits())),
            ("valueNameId", json!(value.value_name_id().to_string())),
            ("nominalValue", fixed_to_json(value.nominal_value())),
            ("rangeMinValue", fixed_to_json(value.range_min_value())),
            ("rangeMaxValue", fixed_to_json(value.range_max_value())),
        ]),
        AxisValue::Format3(value) => json_object([
            ("format", json!(value.format())),
            ("axisIndex", json!(value.axis_index())),
            ("flags", json!(value.flags().bits())),
            ("valueNameId", json!(value.value_name_id().to_string())),
            ("value", fixed_to_json(value.value())),
            ("linkedValue", fixed_to_json(value.linked_value())),
        ]),
        AxisValue::Format4(value) => json_object([
            ("format", json!(value.format())),
            ("axisCount", json!(value.axis_count())),
            ("flags", json!(value.flags().bits())),
            ("valueNameId", json!(value.value_name_id().to_string())),
            (
                "axisValues",
                Value::Array(
                    value
                        .axis_values()
                        .iter()
                        .map(|record| {
                            json_object([
                                ("axisIndex", json!(record.axis_index())),
                                ("value", fixed_to_json(record.value())),
                            ])
                        })
                        .collect(),
                ),
            ),
        ]),
    }
}

fn serialize_name_table<'a>(font: &(impl MetadataProvider<'a> + TableProvider<'a>)) -> Value {
    let mut map = Map::new();
    if let Ok(name) = font.name() {
        let mut ids: Vec<NameId> = name.name_record().iter().map(|x| x.name_id()).collect();
        ids.sort_by_key(|id| id.to_u16());
        for id in ids {
            let strings = font.localized_strings(id);
            if strings.clone().next().is_some() {
                let mut localized = Map::new();
                for string in font.localized_strings(id) {
                    localized.insert(
                        string.language().unwrap_or("default").to_string(),
                        Value::String(string.to_string()),
                    );
                }
                map.insert(
                    stringcase::camel_case(&id.to_string()),
                    Value::Object(localized),
                );
            }
        }
    }
    Value::Object(map)
}

fn serialize_fvar_table(font: &FontRef, axes: Option<&Axes>) -> Value {
    let mut map = Map::new();
    if !font.axes().is_empty() {
        let mut axes_map = Map::new();
        for axis in font.axes().iter() {
            axes_map.insert(
                axis.tag().to_string(),
                json!(
                    {
                        "name": font.localized_strings(axis.name_id()).english_or_first().map_or("Unknown axis".to_string(), |f| f.to_string()),
                        "min": axis.min_value(),
                        "max": axis.max_value(),
                        "default": axis.default_value(),
                    }
                ),
            );
        }
        map.insert("axes".to_string(), Value::Object(axes_map));
    }
    if !font.named_instances().is_empty() {
        let mut instances_map = Map::new();
        for instance in font.named_instances().iter() {
            let name = font
                .localized_strings(instance.subfamily_name_id())
                .english_or_first()
                .map_or("Unknown instance".to_string(), |f| f.to_string());
            if let Some(axes) = axes {
                let location_normalized = instance.location();
                let mut location_normalized_fontdrasil: NormalizedLocation =
                    NormalizedLocation::new();
                for (tag_ix, coord) in location_normalized.coords().iter().enumerate() {
                    if let Some(tag) = axes.axis_order().get(tag_ix) {
                        location_normalized_fontdrasil
                            .insert(*tag, NormalizedCoord::new(coord.to_f64()));
                    }
                }
                if let Ok(location_user_fontdrasil) =
                    location_normalized_fontdrasil.convert::<UserSpace>(axes)
                {
                    instances_map.insert(
                        name,
                        location_user_fontdrasil
                            .iter()
                            .map(|(tag, coord)| (tag.to_string(), json!(&coord.to_f64())))
                            .collect(),
                    );
                }
            }
        }
        map.insert("namedInstances".to_string(), Value::Object(instances_map));
    }
    Value::Object(map)
}

fn equal_enough(v1: &Value, v2: &Value) -> bool {
    match (v1, v2) {
        (Value::Number(n1), Value::Number(n2)) => {
            // Allow numbers to be equal if they are close enough, to account for floating point imprecision
            let f1 = n1.as_f64().unwrap_or(0.0);
            let f2 = n2.as_f64().unwrap_or(0.0);
            (f1 - f2).abs() < 0.01
        }
        (Value::String(s1), Value::String(s2)) => s1 == s2,
        (Value::Object(o1), Value::Object(o2)) => {
            // For objects, we require all keys in the config to be present and correct in the found value, but the found value can have extra keys
            o1.iter().all(|(k, v)| {
                if let Some(found_value) = o2.get(k) {
                    equal_enough(v, found_value)
                } else {
                    false
                }
            })
        }
        _ => v1 == v2,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use fontspector_checkapi::codetesting::{assert_pass, run_check_with_config, test_able};

    #[test]
    fn test_field_values_simple() {
        let t1 = test_able("mada/Mada-Regular.ttf");

        let config = json!({
                "hhea.ascender": 900,
                "hhea.descender": -300,
                "OS/2.sxHeight": 486,
        });
        let results = run_check_with_config(
            field_values,
            TestableType::Single(&t1),
            HashMap::from_iter([("field_values".to_string(), config)]),
        );
        assert_pass(&results);
    }

    #[test]
    fn test_field_values_per_font() {
        let t1 = test_able("mada/Mada-Regular.ttf");

        let config = json!({
                "hhea.ascender": 900,
                "hhea.descender": -300,
                "OS/2.sxHeight": 486,
        });
        let not_my_config = json!({
                "hhea.ascender": 901,
                "hhea.descender": -301,
                "OS/2.sxHeight": 485,
        });
        let results = run_check_with_config(
            field_values,
            TestableType::Single(&t1),
            HashMap::from_iter([(
                "field_values".to_string(),
                json!({
                   "Mada-Regular.ttf": config,
                   "Mada-Nonsuch.ttf": not_my_config
                }),
            )]),
        );
        assert_pass(&results);
    }

    #[test]
    fn test_field_values_variable() {
        let t1 = test_able("cabinvf/Cabin[wdth,wght].ttf");

        let config: Value = json!({
                "fvar.axes": {
                    "wdth": { "name": "Width", "min": 75, "max": 100, "default": 100 },
                    "wght": { "name": "Weight", "min": 400, "max": 700, "default": 400 },
                },
                "fvar.namedInstances": {
                    "Regular": { "wdth": 100, "wght": 400 },
                    "Medium": { "wdth": 100, "wght": 500 },
                    "SemiBold": { "wdth": 100, "wght": 600 },
                    "Bold": { "wdth": 100, "wght": 700 },
                },
        });
        let results = run_check_with_config(
            field_values,
            TestableType::Single(&t1),
            HashMap::from_iter([("field_values".to_string(), config)]),
        );
        println!("Results: {:#?}", results);
        assert_pass(&results);
    }

    #[test]
    fn test_field_values_serialized_tables() {
        let t1 = test_able("shantell/ShantellSans[BNCE,INFM,SPAC,wght].ttf");

        let config = json!({
                "head.fontRevision": 1.011,
                "maxp.numGlyphs": 1728,
                "post.version": "2.0",
                "post.underlinePosition": -105,
                "STAT.designAxisCount": 5,
                "STAT.axisValueCount": 14,
                "STAT.elidedFallbackNameId": "SUBFAMILY_NAME",
        });
        let results = run_check_with_config(
            field_values,
            TestableType::Single(&t1),
            HashMap::from_iter([("field_values".to_string(), config)]),
        );
        assert_pass(&results);
    }
}
