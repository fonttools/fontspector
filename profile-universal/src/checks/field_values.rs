use fontations::{
    read::FontRef,
    skrifa::{instance::Location, raw::TableProvider, MetadataProvider},
    types::NameId,
};
use fontdrasil::coords::{CoordConverter, DesignCoord, NormalizedCoord, UserCoord};
use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use tabled::{Table, Tabled};

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
                skip!("unconfigured", "No entry for this file")
            }
        } else {
            config
        };

        let serialized = font_to_json(&font.font());
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

pub fn font_to_json(font: &FontRef) -> Value {
    let mut map = Map::new();

    // Some tables are serialized by using read_font's traversal feature; typically those which
    // are just a set of fields and values (or are so complicated we haven't yet been bothered
    // to write our own serializers for them...)
    for table in font.table_directory.table_records().iter() {
        let key = table.tag().to_string();
        let value = match table.tag().into_bytes().as_ref() {
            b"head" => font.head().map(|t| <dyn SomeTable>::serialize(&t)),
            b"hhea" => font.hhea().map(|t| <dyn SomeTable>::serialize(&t)),
            b"vhea" => font.vhea().map(|t| <dyn SomeTable>::serialize(&t)),
            // b"avar" => font.avar().map(|t| <dyn SomeTable>::serialize(&t)),
            b"maxp" => font.maxp().map(|t| <dyn SomeTable>::serialize(&t)),
            b"OS/2" => font.os2().map(|t| <dyn SomeTable>::serialize(&t)),
            b"post" => font.post().map(|t| <dyn SomeTable>::serialize(&t)),
            b"STAT" => font.stat().map(|t| <dyn SomeTable>::serialize(&t)),
            _ => continue,
        };
        map.insert(
            key,
            value.unwrap_or_else(|_| Value::String("Could not parse".to_string())),
        );
    }

    // Other tables require a bit of massaging to produce information which makes sense to test.
    map.insert("name".to_string(), serialize_name_table(font));
    map.insert("fvar".to_string(), serialize_fvar_table(font));
    Value::Object(flatten_map(&map))
}

use fontations::read::traversal::{FieldType, SomeArray, SomeTable};
use serde_json::{json, Map, Number, Value};

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

fn serialize_fvar_table(font: &FontRef) -> Value {
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
            if let Ok(location) = font.denormalize_location(instance.location()) {
                instances_map.insert(name, location.into());
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
pub(crate) trait ToValue {
    fn serialize(&self) -> Value;
}

impl<'a> ToValue for FieldType<'a> {
    fn serialize(&self) -> Value {
        match self {
            Self::I8(arg0) => Value::Number((*arg0).into()),
            Self::U8(arg0) => Value::Number((*arg0).into()),
            Self::I16(arg0) => Value::Number((*arg0).into()),
            Self::I24(arg0) => Value::Number((Into::<i32>::into(*arg0)).into()),
            Self::U16(arg0) => Value::Number((*arg0).into()),
            Self::I32(arg0) => Value::Number((*arg0).into()),
            Self::U32(arg0) => Value::Number((*arg0).into()),
            Self::U24(arg0) => {
                let u: u32 = (*arg0).into();
                Value::Number(u.into())
            }
            Self::Tag(arg0) => Value::String(arg0.to_string()),
            Self::FWord(arg0) => Value::Number(arg0.to_i16().into()),
            Self::UfWord(arg0) => Value::Number(arg0.to_u16().into()),
            Self::MajorMinor(arg0) => Value::String(format!("{}.{}", arg0.major, arg0.minor)),
            Self::Version16Dot16(arg0) => Value::String(format!("{}", *arg0)),
            Self::F2Dot14(arg0) => {
                Value::Number(Number::from_f64(arg0.to_f32() as f64).unwrap_or(0.into()))
            }
            Self::Fixed(arg0) => Value::Number(Number::from(arg0.to_i32())),
            Self::LongDateTime(arg0) => Value::Number(arg0.as_secs().into()),
            Self::GlyphId16(arg0) => Value::String(format!("g{}", arg0.to_u16())),
            Self::NameId(arg0) => Value::String(arg0.to_string()),
            Self::StringOffset(string) => match &string.target {
                Ok(arg0) => Value::String(arg0.as_ref().iter_chars().collect()),
                Err(_) => Value::Null,
            },
            Self::ArrayOffset(array) => match &array.target {
                Ok(arg0) => arg0.as_ref().serialize(),
                Err(_) => Value::Null,
            },
            Self::BareOffset(arg0) => Value::String(format!("0x{:04X}", arg0.to_u32())),
            Self::ResolvedOffset(arg0) => {
                arg0.target.as_ref().map_or(Value::Null, |t| t.serialize())
            }
            Self::Record(arg0) => (arg0 as &(dyn SomeTable<'a> + 'a)).serialize(),
            Self::Array(arg0) => arg0.serialize(),
            Self::Unknown => Value::String("no repr available".to_string()),
        }
    }
}

impl<'a> ToValue for dyn SomeArray<'a> + 'a {
    fn serialize(&self) -> Value {
        let mut out = vec![];
        let mut idx = 0;
        while let Some(val) = self.get(idx) {
            out.push(val.serialize());
            idx += 1;
        }
        Value::Array(out)
    }
}

impl<'a> ToValue for dyn SomeTable<'a> + 'a {
    fn serialize(&self) -> Value {
        let mut field_num = 0;
        let mut map = Map::new();
        while let Some(field) = self.get_field(field_num) {
            let camel_case_name = stringcase::camel_case(field.name);
            map.insert(camel_case_name, field.value.serialize());
            field_num += 1;
        }
        Value::Object(map)
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
fn poor_mans_denormalize(
    peak: f32,
    axis: &fontations::read::tables::fvar::VariationAxisRecord,
) -> f32 {
    if peak > 0.0 {
        lerp(
            axis.default_value().to_f32(),
            axis.max_value().to_f32(),
            peak,
        )
    } else {
        lerp(
            axis.default_value().to_f32(),
            axis.min_value().to_f32(),
            -peak,
        )
    }
}

pub trait DenormalizeLocation {
    /// Given a normalized location tuple, turn it back into a friendly representation in userspace
    fn denormalize_location(
        &self,
        location: Location,
    ) -> Result<Map<String, Value>, FontspectorError>;
}

impl DenormalizeLocation for FontRef<'_> {
    fn denormalize_location(
        &self,
        location: Location,
    ) -> Result<Map<String, Value>, FontspectorError> {
        let all_axes = self.fvar()?.axes()?;
        let mut map = Map::new();
        for (axis_index, axis) in all_axes.iter().enumerate() {
            // Start with a default convertor, may edit later
            let mut converter = CoordConverter::unmapped(
                UserCoord::new(axis.min_value().to_f64()),
                UserCoord::new(axis.max_value().to_f64()),
                UserCoord::new(axis.min_value().to_f64()),
            );
            // If there is an avar table, we denormalize its mappings and use it
            if let Ok(avar) = self.avar() {
                if let Some(Ok(segment_map)) = avar.axis_segment_maps().get(axis_index) {
                    let default_idx = segment_map
                        .axis_value_maps
                        .iter()
                        .position(|avm| avm.from_coordinate().to_f32() == 0.0)
                        .unwrap_or(0);
                    let normalized_map = segment_map
                        .axis_value_maps
                        .iter()
                        .map(|axis_value_map| {
                            (
                                UserCoord::new(poor_mans_denormalize(
                                    axis_value_map.from_coordinate().to_f32(),
                                    axis,
                                ) as f64),
                                DesignCoord::new(poor_mans_denormalize(
                                    axis_value_map.to_coordinate().to_f32(),
                                    axis,
                                ) as f64),
                            )
                        })
                        .collect::<Vec<_>>();
                    converter = CoordConverter::new(normalized_map, default_idx);
                }
            }
            let coord = location
                .coords()
                .get(axis_index)
                .ok_or(FontspectorError::General(
                    "Not enough axes in fvar table".to_string(),
                ))?
                .to_f32() as f64;
            let normalized_value = NormalizedCoord::new(coord);
            // Denormalize to userspace!
            map.insert(
                axis.axis_tag().to_string(),
                json!(normalized_value.to_user(&converter).to_f64()),
            );
        }
        Ok(map)
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
