//!  Build script for fetching OpenType script and language tags.
//!  This script fetches the tags from the Microsoft documentation,
//!  parses the HTML to extract the tags, and writes them to Rust source files.
//!  The generated files are then included in the crate for use in the API.
use scraper::{Html, Selector};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::LazyLock;

#[allow(clippy::unwrap_used)] // it's a constant!
static TAG_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"'([A-Za-z0-9\s]{4})'").unwrap());
const SCRIPT_TAGS_URL: &str =
    "https://learn.microsoft.com/en-gb/typography/opentype/spec/scripttags";
const LANG_TAGS_URL: &str =
    "https://learn.microsoft.com/en-gb/typography/opentype/spec/languagetags";

fn out_dir_path(name: &str) -> PathBuf {
    #[allow(clippy::unwrap_used)]
    // we're a build script, if this isn't set something is badly wrong
    let out_dir = env::var_os("OUT_DIR").unwrap();
    PathBuf::from(out_dir).join(name)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch and parse script tags
    let script_response = minreq::get(SCRIPT_TAGS_URL)
        .send()
        .map_err(|e| format!("Failed to fetch OpenType script tags from {SCRIPT_TAGS_URL}: {e}"))?;
    let script_html = Html::parse_document(script_response.as_str()?);
    #[allow(clippy::unwrap_used)] // it's a constant!
    let table_selector = Selector::parse("table").unwrap();
    #[allow(clippy::unwrap_used)] // it's a constant!
    let row_selector = Selector::parse("tr").unwrap();
    #[allow(clippy::unwrap_used)] // it's a constant!
    let column_selector = Selector::parse("td").unwrap();

    let mut script_tags = vec![];

    if let Some(table) = script_html.select(&table_selector).next() {
        for row in table.select(&row_selector) {
            let columns: Vec<_> = row.select(&column_selector).collect();
            if columns.len() == 3 {
                #[allow(clippy::indexing_slicing)] // We just checked the length
                let tag_text = columns[1]
                    .text()
                    .collect::<Vec<_>>()
                    .join("")
                    .trim()
                    .to_string();
                let tag = if let Some(captures) = TAG_RE.captures(&tag_text) {
                    captures[1].to_string()
                } else {
                    tag_text.replace("\u{a0}", " ")
                };

                script_tags.push(tag);
            }
        }
    }
    if script_tags.is_empty() {
        return Err("No script tags found".into());
    }

    // Fetch and parse language tags
    let lang_response = minreq::get(LANG_TAGS_URL)
        .send()
        .map_err(|e| format!("Failed to fetch OpenType language tags from {LANG_TAGS_URL}: {e}"))?;
    let lang_html = Html::parse_document(lang_response.as_str()?);

    let mut language_tags = vec![];

    if let Some(table) = lang_html.select(&table_selector).next() {
        for row in table.select(&row_selector) {
            let columns: Vec<_> = row.select(&column_selector).collect();
            if columns.len() == 3 {
                #[allow(clippy::indexing_slicing)] // We just checked the length
                let tag_text = columns[1]
                    .text()
                    .collect::<Vec<_>>()
                    .join("")
                    .trim()
                    .to_string();
                let tag = if let Some(captures) = TAG_RE.captures(&tag_text) {
                    captures[1].to_string()
                } else {
                    tag_text
                };

                language_tags.push(tag);
            }
        }
    }
    if language_tags.is_empty() {
        return Err("No language tags found".into());
    }
    // Write script tags to file

    let mut script_file = File::create(out_dir_path("script_tags.rs"))?;
    writeln!(
        script_file,
        "/// Valid OpenType script names
pub const VALID_SCRIPT_TAGS: [&str; {}] = [ {} ];",
        script_tags.len(),
        script_tags
            .iter()
            .map(|tag| format!("\"{tag}\""))
            .collect::<Vec<_>>()
            .join(", "),
    )?;

    // Write language tags to file
    let mut language_file = File::create(out_dir_path("language_tags.rs"))?;
    writeln!(
        language_file,
        "/// Valid OpenType language names
pub const VALID_LANG_TAGS: [&str; {}] = [ {} ];",
        language_tags.len(),
        language_tags
            .iter()
            .map(|tag| format!("\"{tag}\""))
            .collect::<Vec<_>>()
            .join(", "),
    )?;

    Ok(())
}
