use crate::{reporters::csv::CsvReporter, Args};
use fontspector_checkapi::{CheckResult, Registry, StatusCode};
use jinja::JinjaTemplatedReporter;
use json::JsonReporter;
use std::{collections::HashMap, path::PathBuf};

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

pub(crate) mod badges;
pub(crate) mod csv;
#[cfg(feature = "duckdb")]
pub(crate) mod duckdb;
pub(crate) mod jinja;
pub(crate) mod json;
pub(crate) mod terminal;

/// The results of all checks in a check run
pub struct RunResults {
    results: Vec<CheckResult>,
}

impl RunResults {
    /// Iterate over each check
    pub fn iter(&self) -> impl Iterator<Item = &CheckResult> {
        self.results.iter()
    }
    /// Iterate over each check mutably
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut CheckResult> {
        self.results.iter_mut()
    }

    /// Get the worst status of all checks
    pub fn worst_status(&self) -> StatusCode {
        self.results
            .iter()
            .map(|r| r.worst_status())
            .max()
            .unwrap_or(StatusCode::Pass)
    }

    /// Get a summary of the results by status code
    pub fn summary(&self) -> HashMap<StatusCode, i32> {
        let mut summary = HashMap::new();
        for result in self.results.iter() {
            for subresult in result.subresults.iter() {
                let entry = summary.entry(subresult.severity).or_insert(0);
                *entry += 1;
            }
        }
        summary
    }

    /// Organize the results by testable and section
    pub fn organize(&self) -> OrganisedResults {
        let mut organised_results: OrganisedResults = HashMap::new();
        for checkresult in self.iter() {
            let section = organised_results
                .entry(
                    checkresult
                        .filename
                        .clone()
                        .unwrap_or("All fonts".to_string()),
                )
                .or_default();
            let results = section
                .entry(
                    checkresult
                        .section
                        .clone()
                        .unwrap_or("No section".to_string()),
                )
                .or_default();
            results.push(checkresult.clone());
        }
        organised_results
    }

    pub fn len(&self) -> usize {
        self.results.len()
    }
}

impl From<Vec<CheckResult>> for RunResults {
    fn from(val: Vec<CheckResult>) -> Self {
        RunResults { results: val }
    }
}

pub type OrganisedResults<'a> = HashMap<String, HashMap<String, Vec<CheckResult>>>;

pub trait Reporter {
    fn report(&self, organised_results: &RunResults, args: &Args, registry: &Registry);
}

pub fn create_user_home_templates_directory(force: bool) -> PathBuf {
    #[allow(clippy::expect_used)] // Something seriously wrong here if this fails
    let home = homedir::my_home()
        .expect("Couldn't get home directory")
        .expect("No home directory found");
    let templates_dir = home.join(".fontspector/");
    if !templates_dir.exists() {
        std::fs::create_dir_all(&templates_dir).unwrap_or_else(|e| {
            println!("Couldn't create {:?}: {}", templates_dir.to_str(), e);
            std::process::exit(1);
        });
    }
    let buf_reader = std::io::Cursor::new(TEMPLATES_ZIP);
    #[allow(clippy::expect_used)] // Internal error
    let mut zip =
        zip::ZipArchive::new(buf_reader).expect("Internal error: bundled templates zip is invalid");
    for i in 0..zip.len() {
        #[allow(clippy::expect_used)] // Internal error
        let mut file = zip
            .by_index(i)
            .expect("Internal error: couldn't read from templates zip file");
        let path = templates_dir.join(file.mangled_name());
        if !path.exists() || force {
            // Create any intermediate subdirectories
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent).unwrap_or_else(|e| {
                        println!("Couldn't create {:?}: {}", parent.to_str(), e);
                        std::process::exit(1);
                    });
                }
            }
            if file.is_dir() {
                continue;
            }
            let mut writer = std::fs::File::create(&path).unwrap_or_else(|e| {
                println!("Couldn't create template file {:?}: {}", path.to_str(), e);
                std::process::exit(1)
            });

            std::io::copy(&mut file, &mut writer).unwrap_or_else(|e| {
                println!("Couldn't write template file {:?}: {}", path.to_str(), e);
                std::process::exit(1)
            });
        }
    }
    templates_dir
}

pub(crate) fn any_stdout(args: &Args) -> Result<bool, String> {
    let yes_stdout = Some("-".to_string());
    let count_stdout: usize = if args.json == yes_stdout { 1 } else { 0 }
        + (if args.csv == yes_stdout { 1 } else { 0 })
        + (if args.html == yes_stdout { 1 } else { 0 })
        + if args.ghmarkdown == yes_stdout { 1 } else { 0 };
    match count_stdout {
        1 => Ok(true),
        0 => Ok(false),
        _ => Err("Only one of --json, --csv, or --ghmarkdown can be stdout".to_string()),
    }
}

pub(crate) fn process_reporter_args(args: &Args, reporters: &mut Vec<Box<dyn Reporter>>) {
    if let Some(jsonfile) = args.json.as_ref() {
        reporters.push(Box::new(JsonReporter::new(jsonfile)));
    }
    if let Some(mdfile) = args.ghmarkdown.as_ref() {
        reporters.push(Box::new(JinjaTemplatedReporter::new_markdown(
            mdfile,
            args.update_templates,
        )));
    }
    if let Some(htmlfile) = args.html.as_ref() {
        reporters.push(Box::new(JinjaTemplatedReporter::new_html(
            htmlfile,
            args.update_templates,
        )));
    }
    if let Some(csvfile) = args.csv.as_ref() {
        reporters.push(Box::new(CsvReporter::new(csvfile)));
    }
    #[cfg(feature = "duckdb")]
    if let Some(duckdbfile) = args.duckdb.as_ref() {
        reporters.push(Box::new(crate::reporters::duckdb::DuckDbReporter::new(
            duckdbfile,
        )));
    }

    if let Some(directory) = args.badges.as_ref() {
        reporters.push(Box::new(badges::BadgesReporter::new(directory)));
    }
}
