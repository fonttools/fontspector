#![allow(clippy::expect_used, clippy::unwrap_used)]
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    path::PathBuf,
};

use fontspector_checkapi::prelude::*;
use js_sys::{Reflect, Uint8Array};
use serde::Deserialize;
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use fontspector_checkapi::{
    testfont, Check, CheckResult, Context, FileTypeConvert, HotfixFunction, Plugin, Profile,
    Registry, StatusCode, TestFont, Testable, TestableCollection, TestableType,
};
use profile_adobe::Adobe;
use profile_fontwerk::Fontwerk;
use profile_googlefonts::GoogleFonts;
use profile_iso15008::Iso15008;
use profile_microsoft::Microsoft;
use profile_opentype::OpenType;
use profile_universal::Universal;
use std::io::Write;
use zip::ZipWriter;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

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

    {
        let (name, toml) = ("fontbureau", include_str!("../../profiles/fontbureau.toml"));
        let profile = Profile::from_toml(toml).expect("Couldn't load profile, fontspector bug");
        registry
            .register_profile(name, profile)
            .expect("Couldn't register profile, fontspector bug");
    }

    {
        let (name, toml) = ("workspace", include_str!("../../profiles/workspace.toml"));
        let profile = Profile::from_toml(toml).expect("Couldn't load profile, fontspector bug");
        registry
            .register_profile(name, profile)
            .expect("Couldn't register profile, fontspector bug");
    }
    registry
}

#[wasm_bindgen]
pub fn best_family_name(fonts: &JsValue) -> Result<String, JsValue> {
    let mut names: Vec<String> = Reflect::own_keys(fonts)?
        .into_iter()
        .flat_map(|filename| {
            let file: JsValue = Reflect::get(fonts, &filename).unwrap();
            let contents = Uint8Array::new(&file).to_vec();
            TestFont::new_from_data(&PathBuf::from(filename.as_string().unwrap()), &contents)
                .map(|t| t.best_familyname())
        })
        .flatten()
        .collect::<Vec<String>>();
    names.sort();
    names.dedup();
    if names.is_empty() {
        return Ok("Unknown".to_string());
    }
    Ok(names.join(", "))
}

fn fonts_to_testables(fonts: &JsValue) -> Result<Vec<Testable>, JsValue> {
    Reflect::own_keys(fonts)?
        .into_iter()
        .map(|filename| {
            let file: JsValue = Reflect::get(fonts, &filename)?;
            let contents = Uint8Array::new(&file).to_vec();

            Ok(Testable {
                filename: filename.as_string().unwrap().into(),
                source: None,
                contents,
            })
        })
        .collect::<Result<Vec<Testable>, JsValue>>()
}

#[wasm_bindgen]
pub fn check_fonts(
    fonts: &JsValue,
    profile: &str,
    full_lists: bool,
    loglevels: &str,
) -> Result<String, JsValue> {
    console_error_panic_hook::set_once();
    let registry = register_profiles();
    let testables: Vec<Testable> = fonts_to_testables(fonts)?;
    let min_severity = StatusCode::from_string(loglevels);
    let collection = TestableCollection::from_testables(testables, None);

    let profile = registry
        .get_profile(profile)
        .ok_or_else(|| format!("Could not find profile {profile:?}"))?;
    let context = Context {
        skip_network: true,
        network_timeout: None,
        configuration: HashMap::new(),
        check_metadata: serde_json::Value::Null,
        full_lists,
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

    let mut results: Vec<CheckResult> = checkorder
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
    if let Some(severity) = min_severity {
        // Filter results by severity
        results.retain(|result| result.worst_status() >= severity);
    }
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

#[wasm_bindgen]
pub fn fix_fonts(fonts: &JsValue, requests: &JsValue) -> Result<Uint8Array, JsValue> {
    console_error_panic_hook::set_once();
    let mut testables: Vec<Testable> = fonts_to_testables(fonts)?;
    let registry = register_profiles();
    let mut logfile = String::new();

    // Gather all the testables and their fixes first. Have to do this in multiple passes to
    // avoid mutably borrowing the same testable multiple times.
    let mut to_fix: BTreeMap<String, (&mut Testable, Vec<(String, &HotfixFunction)>)> =
        BTreeMap::new();
    let mut filenames_we_need = BTreeSet::new();
    for request in js_sys::try_iter(requests)?.ok_or_else(|| "not iterable!")? {
        let request = request?;
        let filename = Reflect::get(&request, &JsValue::from_str("filename"))?
            .as_string()
            .ok_or("filename is not a string")?;
        filenames_we_need.insert(filename.clone());
    }
    // Now map them to their testables
    for testable in &mut testables {
        if filenames_we_need.contains(&testable.filename.to_string_lossy().to_string()) {
            to_fix.insert(
                testable.filename.to_string_lossy().to_string(),
                (testable, vec![]),
            );
        }
    }
    // Next pass gathers fix functions
    for request in js_sys::try_iter(requests)?.ok_or_else(|| "not iterable!")? {
        let request = request?;
        let check_id = Reflect::get(&request, &JsValue::from_str("check_id"))?
            .as_string()
            .ok_or("check_id is not a string")?;
        let check = registry
            .checks
            .get(&check_id)
            .ok_or_else(|| format!("Could not find check with id {check_id}"))?;
        let fixer = check
            .hotfix
            .as_ref()
            .ok_or_else(|| format!("Check {check_id} does not have a fixer"))?;
        let filename = Reflect::get(&request, &JsValue::from_str("filename"))?
            .as_string()
            .ok_or("filename is not a string")?;
        if let Some((_, fixes)) = to_fix.get_mut(&filename) {
            fixes.push((check_id.clone(), fixer));
        }
    }

    // Now run all the fixes at once
    for (filename, (testable, fixes)) in to_fix.into_iter() {
        logfile.push_str(&format!("Fixing {filename}:\n"));
        for (check_id, fixer) in fixes {
            match fixer(testable, None) {
                Ok(FixResult::Fixed) => {
                    logfile.push_str(&format!("  - Applied fix for {check_id}\n"))
                }
                Ok(FixResult::FixFailed(s)) => logfile.push_str(&format!(
                    "  - Fix for {check_id} did not apply cleanly ({s}), manual review needed\n"
                )),
                Ok(FixResult::MoreInfoNeeded(_)) => logfile.push_str(&format!(
                    "  - Fix for {check_id} needs more information, manual review needed\n"
                )),
                Ok(_) => {}
                Err(e) => {
                    logfile.push_str(&format!("  - Error applying fix for {check_id}: {e}\n"))
                }
            }
        }
    }

    let mut cur = std::io::Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(&mut cur);
    for testable in testables {
        zip.start_file(
            testable.filename.to_string_lossy(),
            zip::write::SimpleFileOptions::default(),
        )
        .map_err(|e| e.to_string())?;
        zip.write_all(&testable.contents)
            .map_err(|e| e.to_string())?;
    }
    zip.start_file("fix_log.txt", zip::write::SimpleFileOptions::default())
        .map_err(|e| e.to_string())?;
    zip.write_all(logfile.as_bytes())
        .map_err(|e| e.to_string())?;
    zip.finish().map_err(|e| e.to_string())?;
    let zip_data = cur.into_inner();
    let js_array = Uint8Array::from(zip_data.as_slice());
    Ok(js_array)
}
