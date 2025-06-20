use std::collections::HashMap;

use js_sys::{Reflect, Uint8Array};
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use fontspector_checkapi::{
    Check, CheckResult, Context, Plugin, Profile, Registry, Testable, TestableCollection,
    TestableType,
};
use profile_adobe::Adobe;
use profile_fontwerk::Fontwerk;
use profile_googlefonts::GoogleFonts;
use profile_iso15008::Iso15008;
use profile_microsoft::Microsoft;
use profile_opentype::OpenType;
use profile_universal::Universal;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn register_profiles<'a>() -> Registry<'a> {
    let mut registry = Registry::new();
    OpenType
        .register(&mut registry)
        .expect("Couldn't register opentype profile, fontspector bug");
    Universal
        .register(&mut registry)
        .expect("Couldn't register universal profile, fontspector bug");
    GoogleFonts
        .register(&mut registry)
        .expect("Couldn't register googlefonts profile, fontspector bug");
    Iso15008
        .register(&mut registry)
        .expect("Couldn't register iso15008 profile, fontspector bug");
    Adobe
        .register(&mut registry)
        .expect("Couldn't register Adobe profile, fontspector bug");
    Microsoft
        .register(&mut registry)
        .expect("Couldn't register Microsoft profile, fontspector bug");
    Fontwerk
        .register(&mut registry)
        .expect("Couldn't register Fontwerk profile, fontspector bug");

    for (name, toml) in [("fontbureau", include_str!("../../profiles/fontbureau.toml"))] {
        let profile = Profile::from_toml(toml).expect("Couldn't load profile, fontspector bug");
        registry
            .register_profile(name, profile)
            .expect("Couldn't register profile, fontspector bug");
    }

    registry
}

#[wasm_bindgen]
pub fn check_fonts(fonts: &JsValue, profile: &str) -> Result<String, JsValue> {
    console_error_panic_hook::set_once();
    let registry = register_profiles();
    let testables: Vec<Testable> = Reflect::own_keys(fonts)?
        .into_iter()
        .map(|filename| {
            let file: JsValue = Reflect::get(fonts, &filename).unwrap();
            let contents = Uint8Array::new(&file).to_vec();

            Testable {
                filename: filename.as_string().unwrap().into(),
                source: None,
                contents,
            }
        })
        .collect();
    let collection = TestableCollection::from_testables(testables, None);

    let profile = registry
        .get_profile(profile)
        .ok_or_else(|| format!("Could not find profile {:?}", profile))?;
    let context = Context {
        skip_network: true,
        network_timeout: None,
        configuration: HashMap::new(),
        check_metadata: serde_json::Value::Null,
        full_lists: false,
        cache: Default::default(),
        overrides: vec![],
    };
    let all_testables: Vec<TestableType> = collection.collection_and_files().collect();

    let checkorder: Vec<(String, &TestableType, &Check, Context)> = profile.check_order(
        &[],
        &[],
        &registry,
        context,
        &HashMap::new(),
        &all_testables,
    );

    let results: Vec<CheckResult> = checkorder
        .iter()
        .map(|(sectionname, testable, check, context)| {
            (
                testable,
                check,
                check.run(testable, context, Some(sectionname)),
            )
        })
        .flat_map(|(_, _, result)| result)
        .collect();
    serde_json::to_string(&results).map_err(|e| e.to_string().into())
}

#[wasm_bindgen]
pub fn dump_checks() -> Result<String, JsValue> {
    console_error_panic_hook::set_once();
    let registry = register_profiles();
    let mut checks: HashMap<&'static str, Value> = HashMap::new();
    for (profilename, profile) in registry.iter_profiles() {
        for (section_name, check_ids) in profile.sections.iter() {
            for check in check_ids {
                let Some(check) = registry.checks.get(check) else {
                    continue;
                };
                let json_check = checks.entry(check.id).or_insert_with(|| {
                    json!({
                        "description": check.title,
                        "rationale": check.rationale,
                        "proposal": check.proposal,
                        "sections": [],
                        "profiles": [],
                    })
                });
                if let Some(sections) = json_check.get_mut("sections").and_then(Value::as_array_mut)
                {
                    if !sections.contains(&json!(section_name)) {
                        // Avoid duplicates
                        // This is a bit inefficient, but the number of sections is small
                        // enough that it shouldn't matter.
                        sections.push(json!(section_name));
                    }
                }
                if let Some(profiles) = json_check.get_mut("profiles").and_then(Value::as_array_mut)
                {
                    profiles.push(json!(profilename));
                }
            }
        }
    }
    serde_json::to_string_pretty(&checks).map_err(|e| e.to_string().into())
}
