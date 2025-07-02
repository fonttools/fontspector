use std::{cell::RefCell, collections::HashMap};

use crate::{
    reporters::{Reporter, RunResults},
    Args,
};
use fontspector_checkapi::{CheckResult, Registry, StatusCode};
use indexmap::IndexMap;
use serde_json::json;
use std::io::Write;
use tera::{Context, Tera, Value};

use super::create_user_home_templates_directory;

pub(crate) struct JinjaTemplatedReporter {
    filename: String,
    tera: RefCell<Tera>,
    name: String,
    main_template: String,
}

fn percent_of(v: &Value, options: &HashMap<String, Value>) -> tera::Result<Value> {
    let v = v.as_f64().unwrap_or(0.0);
    let total = options
        .get("total")
        .unwrap_or(&Value::Null)
        .as_f64()
        .unwrap_or(100.0);
    Ok(format!("{:.0}%", v / total * 100.0).into())
}

fn unindent(v: &Value, _options: &HashMap<String, Value>) -> tera::Result<Value> {
    let v = v.as_str().unwrap_or("");
    let v = v.trim_start();
    Ok(v.into())
}

fn emoticon(v: &Value, _options: &HashMap<String, Value>) -> tera::Result<Value> {
    let v = v.as_str().unwrap_or("");
    let v = match v {
        "ERROR" => "💥",
        "FATAL" => "☠",
        "FAIL" => "🔥",
        "WARN" => "⚠️",
        "INFO" => "ℹ️",
        "SKIP" => "⏩",
        "PASS" => "✅",
        "DEBUG" => "🔎",
        _ => "❓",
    };
    Ok(v.into())
}

fn markdown(v: &Value, _options: &HashMap<String, Value>) -> tera::Result<Value> {
    let v = v.as_str().unwrap_or("");
    let v = markdown::to_html_with_options(v, &markdown::Options::gfm())
        .map_err(|e| tera::Error::msg(format!("Error converting to markdown: {e}")))?;
    Ok(v.into())
}

fn basename(v: &Value, _options: &HashMap<String, Value>) -> tera::Result<Value> {
    let v = v.as_str().unwrap_or("");
    let v = std::path::Path::new(v)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(v);
    Ok(v.into())
}

impl JinjaTemplatedReporter {
    pub fn new_html(filename: &str, update_templates: bool) -> Self {
        Self::new(filename, update_templates, "HTML", "html", "main.html")
    }

    pub fn new_markdown(filename: &str, update_templates: bool) -> Self {
        Self::new(
            filename,
            update_templates,
            "Markdown",
            "markdown",
            "main.markdown",
        )
    }

    pub fn new(
        filename: &str,
        update_templates: bool,
        name: &str,
        template_directory: &str,
        main_template: &str,
    ) -> Self {
        let homedir = create_user_home_templates_directory(update_templates);
        #[allow(clippy::expect_used)] // Internal error
        let mut tera = Tera::new(&format!(
            "{}/templates/{}/*",
            homedir
                .to_str()
                .expect("Internal error reading template directory"),
            template_directory
        ))
        .unwrap_or_else(|e| {
            log::error!("Error parsing {name} templates: {e:?}");
            std::process::exit(1);
        });
        tera.register_filter("percent", percent_of);
        tera.register_filter("unindent", unindent);
        tera.register_filter("emoticon", emoticon);
        tera.register_filter("markdown", markdown);
        tera.register_filter("basename", basename);
        Self {
            tera: RefCell::new(tera),
            filename: filename.to_string(),
            name: name.to_string(),
            main_template: main_template.to_string(),
        }
    }
}
impl Reporter for JinjaTemplatedReporter {
    fn report(&self, results: &RunResults, args: &Args, registry: &Registry) {
        let mut fatal_checks = HashMap::new();
        let mut experimental_checks = HashMap::new();
        let mut other_checks = HashMap::new();
        let all_fonts = "All fonts".to_string();
        let log_status = args.loglevel;
        self.tera.borrow_mut().register_tester(
            "omitted",
            move |value: Option<&Value>, _params: &[Value]| {
                let Some(value) = value else {
                    return Ok(true);
                };
                // If we were sent a list, find the ["worst_status"] of all of them
                // and see if any are not "omitted"
                if let Some(list) = value.as_array() {
                    for item in list.iter() {
                        let worst = item.as_object().and_then(|o| {
                            o.get("worst_status")
                                .and_then(|s| s.as_str())
                                .and_then(StatusCode::from_string)
                        });
                        if let Some(status) = worst {
                            if status >= log_status {
                                return Ok(false);
                            }
                        } else {
                            return Err(tera::Error::msg(format!(
                                "Error parsing status code {item}"
                            )));
                        }
                    }
                    return Ok(true);
                }
                let value = value.as_str().ok_or(tera::Error::msg(format!(
                    "'omitted' tester requires a string value, not {value:?}"
                )))?;
                let Some(status) = StatusCode::from_string(value) else {
                    return Err(tera::Error::msg(format!(
                        "Error parsing status code {value}"
                    )));
                };
                Ok(status < log_status)
            },
        );
        // Checks by filename
        for result in results.iter() {
            let filename = result.filename.as_ref().unwrap_or(&all_fonts).as_str();
            if result.worst_status() < args.loglevel {
                continue;
            }
            if registry.is_experimental(&result.check_id) {
                experimental_checks
                    .entry(filename)
                    .or_insert_with(Vec::new)
                    .push(result);
            } else if result.is_error() {
                fatal_checks
                    .entry(filename)
                    .or_insert_with(Vec::new)
                    .push(result);
            } else {
                other_checks
                    .entry(filename)
                    .or_insert_with(Vec::new)
                    .push(result);
            }
        }
        // Sort other_checks by status, worst first
        for file in other_checks.values_mut() {
            file.sort_by_key(|a| std::cmp::Reverse(a.worst_status()));
        }

        // Checks by section
        let summary = results.summary();
        let mut by_section_by_check: IndexMap<String, IndexMap<String, Vec<&CheckResult>>> =
            IndexMap::new();
        for checkresult in results.iter() {
            let section = by_section_by_check
                .entry(
                    checkresult
                        .section
                        .clone()
                        .unwrap_or("No section".to_string()),
                )
                .or_default();
            let check = section.entry(checkresult.check_id.clone()).or_default();
            check.push(checkresult);
        }
        // Sort check results by worst status, highest first
        for section in by_section_by_check.values_mut() {
            for check in section.values_mut() {
                check.sort_by_key(|a| std::cmp::Reverse(a.worst_status()));
            }
        }

        // If there are no bad results in the section, remove it
        by_section_by_check.retain(|_section, checks| {
            checks
                .iter()
                .any(|(_check, results)| results.iter().any(|r| r.worst_status() >= args.loglevel))
        });

        let proposals: HashMap<String, Vec<String>> = registry
            .checks
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    v.proposal.iter().map(|s| s.to_string()).collect(),
                )
            })
            .collect();
        let descriptions: HashMap<String, String> = registry
            .checks
            .iter()
            .map(|(k, v)| (k.clone(), v.title.to_string()))
            .collect();
        let mut summary_keys: Vec<String> = summary.keys().map(|k| k.to_string()).collect();
        summary_keys.reverse();
        let val: serde_json::Value = json!({
            "version": env!("CARGO_PKG_VERSION"),
            "summary": &summary,
            "summary_keys": summary_keys,
            "by_section_by_check": by_section_by_check,
            // "omitted": vec![],
            "fatal_checks": fatal_checks,
            "other_checks": other_checks,
            "experimental_checks": experimental_checks,
            "succinct": args.succinct,
            "total": results.len(),
            "proposals": proposals,
            "descriptions": descriptions,
            "ISSUE_URL": "https://github.com/fonttools/fontspector/issues",
        });
        let context = &Context::from_serialize(val).unwrap_or_else(|e| {
            log::error!("Error creating {} context: {:}", self.name, e);
            std::process::exit(1);
        });

        let rendered = self
            .tera
            .borrow()
            .render(&self.main_template, context)
            .unwrap_or_else(|e| {
                log::error!("Error rendering {} report: {:?}", self.name, e);
                std::process::exit(1);
            });
        if self.filename == "-" {
            let _ = writeln!(std::io::stdout(), "{rendered}");
            return;
        }
        std::fs::write(&self.filename, rendered).unwrap_or_else(|e| {
            eprintln!(
                "Error writing {} report to {:}: {:}",
                self.name, self.filename, e
            );
            std::process::exit(1);
        });
        let _ = writeln!(
            std::io::stdout(),
            "{} report written to {}",
            self.name,
            self.filename
        );
    }
}
