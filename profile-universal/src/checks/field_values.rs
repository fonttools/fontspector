use fontations::{
    read::FontRef,
    skrifa::{raw::TableProvider, MetadataProvider},
    types::NameId,
};
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
    hhea.ascent = 927
    \"OS/2.sxHeight\" = 518 # Key needs to be escaped because of / in OS/2
    name.versionString = { \"en-us\" = \"Version 1.008\" } # Languages must be present
    ```

    Alternatively, the configuration can be specialized on a per-font basis:

    ```
    [field_values.\"Foo-Regular.ttf\"]
    hhea.ascent = 990
    [field_values.\"Foo-Bold.ttf\"]
    hhea.ascent = 1020
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
        let config_for_this_font = if config.values().all(|v| v.is_object()) {
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
            if found != Some(value) {
                incorrect.push(TableEntry {
                    field: key.clone(),
                    expected: value.clone().to_string(),
                    found: found
                        .cloned()
                        .map_or("<Not present>".to_string(), |f| f.to_string()),
                });
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
            b"fvar" => font.fvar().map(|t| <dyn SomeTable>::serialize(&t)),
            b"avar" => font.avar().map(|t| <dyn SomeTable>::serialize(&t)),
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

    // Other tables require a bit of massaging to produce information which makes sense to diff.
    map.insert("name".to_string(), serialize_name_table(font));
    Value::Object(flatten_map(&map))
}

use fontations::read::traversal::{FieldType, SomeArray, SomeTable};
use serde_json::{Map, Number, Value};

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
