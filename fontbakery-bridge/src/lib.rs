//!  Fontbakery Bridge
//!
//!  This module provides a bridge to run Fontbakery checks from Rust.
//!  It allows running Python checks that take a single Font or TTFont object as an argument.
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use std::ffi::CString;

use fontspector_checkapi::{prelude::*, StatusCode};
use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyList, PyTuple},
};
use serde_json::json;
/// The profile implementation for the Fontbakery Bridge.
pub struct FontbakeryBridge;

// We isolate the Python part to avoid type/result madness.
fn python_checkrunner_impl(
    module: &str,
    function: &str,
    testable: &Testable,
) -> PyResult<CheckFnResult> {
    let filename = testable.filename.to_string_lossy();
    Python::with_gil(|py| {
        let module = PyModule::import(py, module)?;
        let check = module.getattr(function)?;

        // Let's check this check's mandatory arguments
        let args = check.getattr("mandatoryArgs")?.extract::<Vec<String>>()?;
        if args.len() != 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Expected exactly one mandatory argument".to_string(),
            ));
        }
        let arg = if args[0] == "font" {
            // Convert the Testable to a Python Font object
            let testable = PyModule::import(py, "fontbakery.testable")?;
            let font = testable.getattr("Font")?;
            font.call1((filename,))?
        } else if args[0] == "ttFont" {
            // Convert the Testable to a Python TTFont object
            let ttlib = PyModule::import(py, "fontTools.ttLib")?;
            let ttfont = ttlib.getattr("TTFont")?;
            ttfont.call1((filename,))?
        } else {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Unknown mandatory argument".to_string(),
            ));
        };

        let mut checkresult = check.call1((arg,))?;
        let mut messages: Vec<Status> = vec![];

        // If the checkresult is a single tuple, turn it into a list of tuples, and get a generator
        if checkresult.is_instance_of::<PyTuple>() {
            let checkresults = vec![checkresult];
            checkresult = PyList::new(py, checkresults)?
                .getattr("__iter__")?
                .call0()?;
        }

        // Now convert the Fontbakery status to our StatusList
        while let Ok(value) = checkresult.getattr("__next__")?.call0() {
            // Value is a tuple of status and message
            let status_str = value.get_item(0)?.getattr("name")?.extract::<String>()?;
            let status = StatusCode::from_string(&status_str).ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Fontbakery returned unknown status code".to_string(),
                )
            })?;
            let code = if value.get_item(1)?.hasattr("code")? {
                Some(value.get_item(1)?.getattr("code")?.extract::<String>()?)
            } else {
                None
            };
            let message = if value.get_item(1)?.hasattr("message")? {
                value.get_item(1)?.getattr("message")?
            } else {
                value.get_item(1)?
            }
            .extract::<String>()?;

            messages.push(Status {
                message: Some(message),
                severity: status,
                code,
                metadata: None,
            });
        }
        Ok(return_result(messages))
    })
}

// This wrapper will work for any fontbakery check that takes a single
// Font or ttFont object as an argument.
fn python_checkrunner(c: &Testable, context: &Context) -> CheckFnResult {
    let module = context
        .check_metadata
        .get("module")
        .ok_or_else(|| FontspectorError::Python("No module specified".to_string()))?
        .as_str()
        .ok_or_else(|| {
            FontspectorError::Python("module in metadata was not a string!".to_string())
        })?;
    let function = context
        .check_metadata
        .get("function")
        .ok_or_else(|| FontspectorError::Python("No function specified".to_string()))?
        .as_str()
        .ok_or_else(|| {
            FontspectorError::Python("function in metadata was not a string!".to_string())
        })?;
    python_checkrunner_impl(module, function, c)
        .unwrap_or_else(|e| Err(FontspectorError::Python(format!("Python error: {}", e))))
}

/// This function registers all checks from a Python module with the given name
pub fn register_python_checks(
    modulename: &str,
    source: &str,
    cr: &mut Registry,
) -> Result<(), String> {
    Python::with_gil(|py| {
        // Assert that we have loaded the FB prelude
        let _prelude = PyModule::import(py, "fontbakery.prelude")?;
        let callable = PyModule::import(py, "fontbakery.callable")?;
        let full_source = "from fontbakery.prelude import *\n\n".to_string() + source;
        let module = PyModule::from_code(
            py,
            &CString::new(full_source)
                .map_err(|_| PyValueError::new_err("Failed to create CString from source"))?,
            &CString::new(format!("{}.py", modulename))
                .map_err(|_| PyValueError::new_err("Failed to create CString from module name"))?,
            &CString::new(modulename)
                .map_err(|_| PyValueError::new_err("Failed to create CString from module name"))?,
        )?;
        log::debug!("Loaded module {}", modulename);
        // Find all functions in the module which are checks
        let checktype = callable.getattr("FontBakeryCheck")?;
        for name in module.dir()?.iter() {
            log::debug!("Looking at module attribute {}", name);
            let name_str: String = name.extract()?;
            let obj = module.getattr(name.downcast()?)?;
            if let Ok(true) = obj.is_instance(&checktype) {
                let id: String = obj.getattr("id")?.extract()?;
                // Check the mandatory arguments
                let args = obj.getattr("mandatoryArgs")?.extract::<Vec<String>>()?;
                if args.len() != 1 || !(args[0] == "font" || args[0] == "ttFont") {
                    log::warn!(
                        "Can't load check {}; unable to support arguments: {}",
                        id,
                        args.join(", ")
                    );
                    continue;
                }
                let title: String = obj
                    .getattr("__doc__")
                    .and_then(|doc| doc.extract())
                    .unwrap_or("An untitled check".to_string());
                let py_rationale = obj.getattr("rationale")?;
                let rationale: String = if py_rationale.is_instance_of::<PyList>() {
                    let r: Vec<String> = py_rationale.extract()?;
                    r.join(", ")
                } else {
                    py_rationale.extract().unwrap_or("No rationale".to_string())
                };
                let py_proposal = obj.getattr("proposal")?;
                let proposals: Vec<String> = if py_proposal.is_instance_of::<PyList>() {
                    let r: Vec<String> = py_proposal.extract()?;
                    r.iter().map(|s| s.to_string()).collect()
                } else {
                    let s: String = py_proposal.extract().unwrap_or("No proposal".to_string());
                    vec![s.to_string()]
                };

                // Collect into a Vec<&str> instead of a slice
                // let leaked_proposals = leak_vec_string(proposals);
                let leaked_proposals: &[&'static str] = proposals
                    .into_iter()
                    .map(|s| s.leak() as &'static str)
                    .collect::<Vec<&'static str>>()
                    .leak();

                log::info!("Registered check: {}", id);
                let metadata = json!({
                    "module": modulename,
                    "function": name_str,
                });
                cr.register_check(Check {
                    id: id.leak(),
                    title: title.leak(),
                    rationale: rationale.leak(),
                    proposal: leaked_proposals,
                    hotfix: None,
                    fix_source: None,
                    applies_to: "TTF",
                    flags: CheckFlags::default(),
                    implementation: CheckImplementation::CheckOne(&python_checkrunner),
                    _metadata: Some(metadata.to_string().leak()),
                })
            }
        }
        Ok(())
    })
    .map_err(|e: PyErr| format!("Error loading checks: {}", e))
}

impl fontspector_checkapi::Plugin for FontbakeryBridge {
    fn register(&self, cr: &mut Registry) -> Result<(), String> {
        pyo3::prepare_freethreaded_python();
        // Load needed FB modules
        let ok: PyResult<()> = Python::with_gil(|py| {
            #[allow(clippy::unwrap_used)] // Static strings
            PyModule::from_code(
                py,
                &CString::new(include_str!("../fontbakery/Lib/fontbakery/callable.py")).map_err(
                    |_| PyValueError::new_err("Failed to create CString from callable source"),
                )?,
                &CString::new("callable.py").unwrap(),
                &CString::new("fontbakery.callable").unwrap(),
            )?;
            #[allow(clippy::unwrap_used)] // Static strings
            PyModule::from_code(
                py,
                &CString::new(include_str!("../fontbakery/Lib/fontbakery/status.py")).map_err(
                    |_| PyValueError::new_err("Failed to create CString from status source"),
                )?,
                &CString::new("status.py").unwrap(),
                &CString::new("fontbakery.status").unwrap(),
            )?;
            #[allow(clippy::unwrap_used)] // Static strings
            PyModule::from_code(
                py,
                &CString::new(include_str!("../fontbakery/Lib/fontbakery/message.py")).map_err(
                    |_| PyValueError::new_err("Failed to create CString from message source"),
                )?,
                &CString::new("message.py").unwrap(),
                &CString::new("fontbakery.message").unwrap(),
            )?;
            Ok(())
        });
        ok.map_err(|e| format!("Error loading FB modules: {}", e))?;

        // register_python_checks(
        //     "fontbakery.checks.opentype.kern",
        //     include_str!("../fontbakery/Lib/fontbakery/checks/opentype/kern.py"),
        //     cr,
        // )?;
        // register_python_checks(
        //     "fontbakery.checks.opentype.cff",
        //     include_str!("../fontbakery/Lib/fontbakery/checks/opentype/cff.py"),
        //     cr,
        // )?;
        // register_python_checks(
        //     "fontbakery.checks.opentype.gdef",
        //     include_str!("../fontbakery/Lib/fontbakery/checks/opentype/gdef.py"),
        //     cr,
        // )?;
        // register_python_checks(
        //     "fontbakery.checks.opentype.gpos",
        //     include_str!("../fontbakery/Lib/fontbakery/checks/opentype/gpos.py"),
        //     cr,
        // )?;
        // register_python_checks(
        //     "fontbakery.checks.opentype.head",
        //     include_str!("../fontbakery/Lib/fontbakery/checks/opentype/head.py"),
        //     cr,
        // )?;
        // register_python_checks(
        //     "fontbakery.checks.opentype.hhea",
        //     include_str!("../fontbakery/Lib/fontbakery/checks/opentype/hhea.py"),
        //     cr,
        // )?;
        // register_python_checks(
        //     "fontbakery.checks.opentype.os2",
        //     include_str!("../fontbakery/Lib/fontbakery/checks/opentype/os2.py"),
        //     cr,
        // )?;
        // register_python_checks(
        //     "fontbakery.checks.some_other_checks",
        //     include_str!("../fontbakery/Lib/fontbakery/checks/some_other_checks.py"),
        //     cr,
        // )?;
        // register_python_checks(
        //     "fontbakery.checks.glyphset",
        //     include_str!("../fontbakery/Lib/fontbakery/checks/glyphset.py"),
        //     cr,
        // )?;
        // register_python_checks(
        //     "fontbakery.checks.metrics",
        //     include_str!("../fontbakery/Lib/fontbakery/checks/metrics.py"),
        //     cr,
        // )?;
        // register_python_checks(
        //     "fontbakery.checks.hinting",
        //     include_str!("../fontbakery/Lib/fontbakery/checks/hinting.py"),
        //     cr,
        // )?;
        cr.register_profile(
            "fontbakery",
            Profile::from_toml(
                r#"
        [sections]
        "Test profile" = [
            "hinting_impact",
            "opentype/name/empty_records",
            "opentype/monospace",
            "opentype/cff_call_depth",
        ]
        "#,
            )
            .map_err(|_| "Couldn't parse profile")?,
        )
    }
}

#[cfg(not(target_family = "wasm"))]
#[allow(missing_docs)]
pluginator::plugin_implementation!(fontspector_checkapi::Plugin, FontbakeryBridge);
