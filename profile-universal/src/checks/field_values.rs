use fontdrasil::coords::{NormalizedCoord, NormalizedLocation, UserSpace};
use fontdrasil::types::Axes;
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
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

// Explicit serializers replace the removed read-fonts/skrifa traversal feature.
// Keys are camelCase OpenType field names so existing field_values configs keep working.

use serde_json::{json, Map, Value};
use skrifa::raw::tables::{
    head::Head,
    hhea::Hhea,
    maxp::Maxp,
    os2::Os2,
    post::Post,
    stat::{AxisRecord, AxisValue, AxisValueRecord, Stat},
    vhea::Vhea,
};
use skrifa::raw::types::{FWord, Fixed, LongDateTime, MajorMinor, UfWord, Version16Dot16};

pub fn font_to_json(font: &FontRef, axes: Option<&Axes>) -> Value {
    let mut map = Map::new();

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

fn major_minor(v: MajorMinor) -> Value {
    Value::String(format!("{}.{}", v.major, v.minor))
}

fn version_16_16(v: Version16Dot16) -> Value {
    Value::String(format!("{}", v))
}

// 16.16 fixed point numbers have at most five decimal places; rounding off anything
// beyond that avoids exposing the floating point representation to the user
// (1.008 rather than 1.0080032348632812).
fn fixed_to_json(value: Fixed) -> Value {
    let value = (value.to_f64() * 100_000.0).round() / 100_000.0;
    json!(value)
}

fn fword(v: FWord) -> Value {
    Value::Number(v.to_i16().into())
}

fn ufword(v: UfWord) -> Value {
    Value::Number(v.to_u16().into())
}

fn long_date_time(v: LongDateTime) -> Value {
    Value::Number(v.as_secs().into())
}

fn insert_opt<T: Into<Value>>(map: &mut Map<String, Value>, key: &str, value: Option<T>) {
    if let Some(v) = value {
        map.insert(key.to_string(), v.into());
    }
}

fn serialize_head(t: &Head) -> Value {
    json!({
        "version": major_minor(t.version()),
        "fontRevision": fixed_to_json(t.font_revision()),
        "checksumAdjustment": t.checksum_adjustment(),
        "magicNumber": t.magic_number(),
        "flags": t.flags().bits(),
        "unitsPerEm": t.units_per_em(),
        "created": long_date_time(t.created()),
        "modified": long_date_time(t.modified()),
        "xMin": t.x_min(),
        "yMin": t.y_min(),
        "xMax": t.x_max(),
        "yMax": t.y_max(),
        "macStyle": t.mac_style().bits(),
        "lowestRecPpem": t.lowest_rec_ppem(),
        "fontDirectionHint": t.font_direction_hint(),
        "indexToLocFormat": t.index_to_loc_format(),
        "glyphDataFormat": t.glyph_data_format(),
    })
}

fn serialize_hhea(t: &Hhea) -> Value {
    json!({
        "version": major_minor(t.version()),
        "ascender": fword(t.ascender()),
        "descender": fword(t.descender()),
        "lineGap": fword(t.line_gap()),
        "advanceWidthMax": ufword(t.advance_width_max()),
        "minLeftSideBearing": fword(t.min_left_side_bearing()),
        "minRightSideBearing": fword(t.min_right_side_bearing()),
        "xMaxExtent": fword(t.x_max_extent()),
        "caretSlopeRise": t.caret_slope_rise(),
        "caretSlopeRun": t.caret_slope_run(),
        "caretOffset": t.caret_offset(),
        "metricDataFormat": t.metric_data_format(),
        "numberOfHMetrics": t.number_of_h_metrics(),
    })
}

fn serialize_vhea(t: &Vhea) -> Value {
    json!({
        "version": version_16_16(t.version()),
        "ascender": fword(t.ascender()),
        "descender": fword(t.descender()),
        "lineGap": fword(t.line_gap()),
        "advanceHeightMax": ufword(t.advance_height_max()),
        "minTopSideBearing": fword(t.min_top_side_bearing()),
        "minBottomSideBearing": fword(t.min_bottom_side_bearing()),
        "yMaxExtent": fword(t.y_max_extent()),
        "caretSlopeRise": t.caret_slope_rise(),
        "caretSlopeRun": t.caret_slope_run(),
        "caretOffset": t.caret_offset(),
        "metricDataFormat": t.metric_data_format(),
        "numberOfLongVerMetrics": t.number_of_long_ver_metrics(),
    })
}

fn serialize_maxp(t: &Maxp) -> Value {
    let mut map = Map::new();
    map.insert("version".to_string(), version_16_16(t.version()));
    map.insert("numGlyphs".to_string(), t.num_glyphs().into());
    insert_opt(&mut map, "maxPoints", t.max_points());
    insert_opt(&mut map, "maxContours", t.max_contours());
    insert_opt(&mut map, "maxCompositePoints", t.max_composite_points());
    insert_opt(&mut map, "maxCompositeContours", t.max_composite_contours());
    insert_opt(&mut map, "maxZones", t.max_zones());
    insert_opt(&mut map, "maxTwilightPoints", t.max_twilight_points());
    insert_opt(&mut map, "maxStorage", t.max_storage());
    insert_opt(&mut map, "maxFunctionDefs", t.max_function_defs());
    insert_opt(&mut map, "maxInstructionDefs", t.max_instruction_defs());
    insert_opt(&mut map, "maxStackElements", t.max_stack_elements());
    insert_opt(
        &mut map,
        "maxSizeOfInstructions",
        t.max_size_of_instructions(),
    );
    insert_opt(&mut map, "maxComponentElements", t.max_component_elements());
    insert_opt(&mut map, "maxComponentDepth", t.max_component_depth());
    Value::Object(map)
}

fn serialize_os2(t: &Os2) -> Value {
    let mut map = Map::new();
    map.insert("version".to_string(), t.version().into());
    map.insert("xAvgCharWidth".to_string(), t.x_avg_char_width().into());
    map.insert("usWeightClass".to_string(), t.us_weight_class().into());
    map.insert("usWidthClass".to_string(), t.us_width_class().into());
    map.insert("fsType".to_string(), t.fs_type().into());
    map.insert("ySubscriptXSize".to_string(), t.y_subscript_x_size().into());
    map.insert("ySubscriptYSize".to_string(), t.y_subscript_y_size().into());
    map.insert(
        "ySubscriptXOffset".to_string(),
        t.y_subscript_x_offset().into(),
    );
    map.insert(
        "ySubscriptYOffset".to_string(),
        t.y_subscript_y_offset().into(),
    );
    map.insert(
        "ySuperscriptXSize".to_string(),
        t.y_superscript_x_size().into(),
    );
    map.insert(
        "ySuperscriptYSize".to_string(),
        t.y_superscript_y_size().into(),
    );
    map.insert(
        "ySuperscriptXOffset".to_string(),
        t.y_superscript_x_offset().into(),
    );
    map.insert(
        "ySuperscriptYOffset".to_string(),
        t.y_superscript_y_offset().into(),
    );
    map.insert("yStrikeoutSize".to_string(), t.y_strikeout_size().into());
    map.insert(
        "yStrikeoutPosition".to_string(),
        t.y_strikeout_position().into(),
    );
    map.insert("sFamilyClass".to_string(), t.s_family_class().into());
    map.insert(
        "panose10".to_string(),
        Value::Array(
            t.panose_10()
                .iter()
                .map(|b| Value::Number((*b).into()))
                .collect(),
        ),
    );
    map.insert("ulUnicodeRange1".to_string(), t.ul_unicode_range_1().into());
    map.insert("ulUnicodeRange2".to_string(), t.ul_unicode_range_2().into());
    map.insert("ulUnicodeRange3".to_string(), t.ul_unicode_range_3().into());
    map.insert("ulUnicodeRange4".to_string(), t.ul_unicode_range_4().into());
    map.insert(
        "achVendId".to_string(),
        Value::String(t.ach_vend_id().to_string()),
    );
    map.insert("fsSelection".to_string(), t.fs_selection().bits().into());
    map.insert(
        "usFirstCharIndex".to_string(),
        t.us_first_char_index().into(),
    );
    map.insert("usLastCharIndex".to_string(), t.us_last_char_index().into());
    map.insert("sTypoAscender".to_string(), t.s_typo_ascender().into());
    map.insert("sTypoDescender".to_string(), t.s_typo_descender().into());
    map.insert("sTypoLineGap".to_string(), t.s_typo_line_gap().into());
    map.insert("usWinAscent".to_string(), t.us_win_ascent().into());
    map.insert("usWinDescent".to_string(), t.us_win_descent().into());
    insert_opt(&mut map, "ulCodePageRange1", t.ul_code_page_range_1());
    insert_opt(&mut map, "ulCodePageRange2", t.ul_code_page_range_2());
    insert_opt(&mut map, "sxHeight", t.sx_height());
    insert_opt(&mut map, "sCapHeight", t.s_cap_height());
    insert_opt(&mut map, "usDefaultChar", t.us_default_char());
    insert_opt(&mut map, "usBreakChar", t.us_break_char());
    insert_opt(&mut map, "usMaxContext", t.us_max_context());
    insert_opt(
        &mut map,
        "usLowerOpticalPointSize",
        t.us_lower_optical_point_size(),
    );
    insert_opt(
        &mut map,
        "usUpperOpticalPointSize",
        t.us_upper_optical_point_size(),
    );
    Value::Object(map)
}

fn serialize_post(t: &Post) -> Value {
    let mut map = Map::new();
    map.insert("version".to_string(), version_16_16(t.version()));
    map.insert("italicAngle".to_string(), fixed_to_json(t.italic_angle()));
    map.insert(
        "underlinePosition".to_string(),
        fword(t.underline_position()),
    );
    map.insert(
        "underlineThickness".to_string(),
        fword(t.underline_thickness()),
    );
    map.insert("isFixedPitch".to_string(), t.is_fixed_pitch().into());
    map.insert("minMemType42".to_string(), t.min_mem_type42().into());
    map.insert("maxMemType42".to_string(), t.max_mem_type42().into());
    map.insert("minMemType1".to_string(), t.min_mem_type1().into());
    map.insert("maxMemType1".to_string(), t.max_mem_type1().into());
    insert_opt(&mut map, "numGlyphs", t.num_glyphs());
    // The glyph name strings are reachable through the (gappy) string data, and were
    // never serialized meaningfully; only the name indices are reported.
    if let Some(indices) = t.glyph_name_index() {
        map.insert(
            "glyphNameIndex".to_string(),
            Value::Array(
                indices
                    .iter()
                    .map(|gid| Value::Number(gid.get().into()))
                    .collect(),
            ),
        );
    }
    Value::Object(map)
}

fn serialize_axis_record(axis: &AxisRecord) -> Value {
    json!({
        "axisTag": axis.axis_tag().to_string(),
        "axisNameId": axis.axis_name_id().to_string(),
        "axisOrdering": axis.axis_ordering(),
    })
}

fn serialize_axis_value_record(rec: &AxisValueRecord) -> Value {
    json!({
        "axisIndex": rec.axis_index(),
        "value": fixed_to_json(rec.value()),
    })
}

fn serialize_name_id(id: NameId) -> Value {
    Value::String(id.to_string())
}

fn serialize_axis_value(value: &AxisValue) -> Value {
    match value {
        AxisValue::Format1(t) => json!({
            "format": t.format(),
            "axisIndex": t.axis_index(),
            "flags": t.flags().bits(),
            "valueNameId": serialize_name_id(t.value_name_id()),
            "value": fixed_to_json(t.value()),
        }),
        AxisValue::Format2(t) => json!({
            "format": t.format(),
            "axisIndex": t.axis_index(),
            "flags": t.flags().bits(),
            "valueNameId": serialize_name_id(t.value_name_id()),
            "nominalValue": fixed_to_json(t.nominal_value()),
            "rangeMinValue": fixed_to_json(t.range_min_value()),
            "rangeMaxValue": fixed_to_json(t.range_max_value()),
        }),
        AxisValue::Format3(t) => json!({
            "format": t.format(),
            "axisIndex": t.axis_index(),
            "flags": t.flags().bits(),
            "valueNameId": serialize_name_id(t.value_name_id()),
            "value": fixed_to_json(t.value()),
            "linkedValue": fixed_to_json(t.linked_value()),
        }),
        AxisValue::Format4(t) => {
            let axis_values: Vec<Value> = t
                .axis_values()
                .iter()
                .map(serialize_axis_value_record)
                .collect();
            json!({
                "format": t.format(),
                "axisCount": t.axis_count(),
                "flags": t.flags().bits(),
                "valueNameId": serialize_name_id(t.value_name_id()),
                "axisValues": axis_values,
            })
        }
    }
}

fn serialize_stat(t: &Stat) -> Value {
    // Field names/nesting mirror the removed SomeTable traversal for Stat.
    let mut map = Map::new();
    map.insert("version".to_string(), major_minor(t.version()));
    map.insert("designAxisSize".to_string(), t.design_axis_size().into());
    map.insert("designAxisCount".to_string(), t.design_axis_count().into());
    map.insert(
        "designAxesOffset".to_string(),
        match t.design_axes() {
            Ok(axes) => Value::Array(axes.iter().map(serialize_axis_record).collect()),
            Err(_) => Value::Null,
        },
    );
    map.insert("axisValueCount".to_string(), t.axis_value_count().into());
    map.insert(
        "offsetToAxisValueOffsets".to_string(),
        match t.offset_to_axis_values() {
            Some(Ok(axis_values)) => {
                let values: Vec<Value> = axis_values
                    .axis_values()
                    .iter()
                    .map(|item| match item {
                        Ok(v) => serialize_axis_value(&v),
                        Err(_) => Value::Null,
                    })
                    .collect();
                json!({ "axisValueOffsets": values })
            }
            Some(Err(_)) => Value::Null,
            None => Value::Null,
        },
    );
    if let Some(name_id) = t.elided_fallback_name_id() {
        map.insert(
            "elidedFallbackNameId".to_string(),
            serialize_name_id(name_id),
        );
    }
    Value::Object(map)
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
}
