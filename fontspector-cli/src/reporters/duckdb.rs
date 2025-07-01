use std::path::Path;

use super::RunResults;
use crate::{reporters::Reporter, Args};
use duckdb::{params, Connection, Result};
use fontspector_checkapi::{Registry, StatusCode};
use itertools::Itertools;

pub(crate) struct DuckDbReporter {
    db: Connection,
}

impl DuckDbReporter {
    pub fn new(filename: &str) -> Self {
        let create_tables = !Path::new(&filename).exists();
        let db = Connection::open(filename).unwrap_or_else(|e| {
            println!("Could not open database file {filename}: {e:?}");
            std::process::exit(1);
        });
        if create_tables {
            #[allow(clippy::expect_used)]
            DuckDbReporter::create_tables(&db).expect("Issue creating tables");
        }
        Self { db }
    }

    fn create_tables(db: &Connection) -> Result<(), duckdb::Error> {
        let create_enum = "CREATE TYPE status as ENUM (".to_string()
            + &StatusCode::all().map(|s| format!("'{s}'")).join(", ")
            + ")";
        db.execute(&create_enum, params![])?;
        db.execute(
            "CREATE TABLE results (
                run TIMESTAMP,
                directory TEXT,
                file TEXT,
                section TEXT,
                check_id TEXT,
                status status,
                codes TEXT
            )",
            params![],
        )?;
        Ok(())
    }
}

impl Reporter for DuckDbReporter {
    fn report(&self, results: &RunResults, args: &Args, _registry: &Registry) {
        let organised_results = results.organize();
        let timestamp = chrono::Local::now();
        for (filename, sectionresults) in organised_results
            .iter()
            .sorted_by_key(|(t, _s)| t.to_string())
        {
            let path = Path::new(filename);
            // Work out if it's a collection or an individual file
            let (directory, basefile) = if path.is_dir() {
                #[allow(clippy::unwrap_used)]
                (path.file_name().unwrap().to_str().unwrap(), "All files")
            } else {
                #[allow(clippy::unwrap_used)]
                (
                    path.parent()
                        .unwrap()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap(),
                    path.file_name().unwrap().to_str().unwrap(),
                )
            };
            let mut app = self.db.appender("results").unwrap_or_else(|e| {
                println!("Error creating appender: {e}");
                std::process::exit(1);
            });
            for (section, results) in sectionresults.iter() {
                for result in results.iter() {
                    let subresults = result
                        .subresults
                        .iter()
                        .filter(|c| c.severity >= args.loglevel)
                        .collect::<Vec<_>>();
                    if let Err(e) = app.append_row(params![
                        // Format timestamp as YYYY-MM-DD hh:mm:ss
                        timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
                        directory,
                        basefile,
                        section.clone(),
                        result.check_id.clone(),
                        result.worst_status().to_string(),
                        subresults
                            .iter()
                            .map(|r| r.code.as_deref().unwrap_or_default())
                            .join(" ")
                    ]) {
                        println!("Error inserting into database: {e}");
                        std::process::exit(1);
                    }
                }
            }
        }
    }
}
