use std::{collections::HashMap, process::exit};

use crate::{
    reporters::{Reporter, RunResults},
    Args,
};
use fontspector_checkapi::{CheckResult, Registry, StatusCode};
use serde_json::json;
pub(crate) struct BadgesReporter {
    directory: String,
}
fn color_for(percentage: f32) -> String {
    if percentage > 90.0 {
        "brightgreen".to_string()
    } else if percentage > 70.0 {
        "green".to_string()
    } else if percentage > 50.0 {
        "yellow".to_string()
    } else if percentage > 30.0 {
        "orange".to_string()
    } else {
        "red".to_string()
    }
}
impl BadgesReporter {
    pub fn new(directory: &str) -> Self {
        // Create the directory if it doesn't exist
        if !std::path::Path::new(directory).exists() {
            std::fs::create_dir_all(directory).unwrap_or_else(|e| {
                eprintln!("Error creating directory: {e}");
                exit(1);
            });
        }
        Self {
            directory: directory.to_string(),
        }
    }

    fn make_section(&self, section_name: &str, error_state: bool, score: i64, out_of: i64) {
        let filename = section_name.replace(" ", "") + ".json";
        let (message, color) = if error_state {
            ("ERRORED".to_string(), "red".to_string())
        } else if out_of > 0 {
            let percentage = score as f32 / out_of as f32 * 100.0;
            (format!("{percentage:.0}%"), color_for(percentage))
        } else {
            ("SKIP".to_string(), "inactive".to_string())
        };
        let result = json!({
            "schemaVersion": 1,
            "label": section_name,
            "message": message,
            "color": color,
        });
        let file_path = format!("{}/{}", self.directory, filename);
        let file = std::fs::File::create(file_path).unwrap_or_else(|e| {
            eprintln!("Error creating file: {e}");
            exit(1);
        });
        serde_json::to_writer_pretty(file, &result).unwrap_or_else(|e| {
            eprintln!("Error writing to file: {e}");
            exit(1);
        });
    }
}

impl Reporter for BadgesReporter {
    fn report(&self, results: &RunResults, _args: &Args, registry: &Registry) {
        let mut organised_results: HashMap<String, Vec<CheckResult>> = HashMap::new();
        // Just organize by section
        for checkresult in results.iter() {
            let section = organised_results
                .entry(
                    checkresult
                        .section
                        .clone()
                        .unwrap_or("No section".to_string()),
                )
                .or_default();
            section.push(checkresult.clone());
        }
        // And now score
        let mut total_score = 0;
        let mut total_out_of = 0;
        for (sectionname, checkresults) in organised_results.iter() {
            let mut score = 0;
            let mut out_of = 0;
            let mut error_state = false;
            for checkresult in checkresults.iter() {
                if checkresult.worst_status() == StatusCode::Skip {
                    continue;
                }
                if checkresult.worst_status() == StatusCode::Error {
                    error_state = true;
                    break;
                }
                let severity = registry
                    .checks
                    .get(&checkresult.check_id)
                    .map(|ck| ck.metadata())
                    .and_then(|md| {
                        md.as_object()
                            .and_then(|md| md.get("severity"))
                            .and_then(|s| s.as_i64())
                    })
                    .unwrap_or(5);
                if checkresult.worst_status() == StatusCode::Pass {
                    score += severity;
                }
                out_of += severity;
            }
            self.make_section(sectionname, error_state, score, out_of);
            total_out_of += out_of;
            total_score += score;
        }
        self.make_section("Fontspector QA", false, total_score, total_out_of);
        log::info!(
            "A set of badges in JSON format has been saved to \"{}/\".",
            self.directory,
        );
    }
}
