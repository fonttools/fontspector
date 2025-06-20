//! Bundle the templates into a zip file and generate a Rust source file for
//! inclusion in the crate.
//!
//! Also includes version information from shadow-rs.
use std::{
    env,
    fs::File,
    io::{BufWriter, Read, Write},
    path::Path,
};

use quote::quote;
use shadow_rs::ShadowBuilder;
use walkdir::WalkDir;
use zip::{write::SimpleFileOptions, ZipWriter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ShadowBuilder::builder().build()?;

    let walkdir = WalkDir::new("../templates");
    let it = walkdir.into_iter();
    let mut buf = [0; 65536];
    let options = SimpleFileOptions::default();
    let mut zip = ZipWriter::new(std::io::Cursor::new(&mut buf[..]));
    let mut buffer = Vec::new();
    for entry in it.flatten() {
        let path = entry.path();
        #[allow(clippy::unwrap_used)] // We put .. in there ourselves.
        let name = path.strip_prefix("..").unwrap();
        let path_as_string = name
            .to_str()
            .map(str::to_owned)
            .unwrap_or_else(|| panic!("{name:?} Is a Non UTF-8 Path"));
        if path.is_file() {
            println!("adding file {path:?} as {name:?} ...");
            zip.start_file(path_as_string, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
            println!("cargo:rerun-if-changed={path:?}");
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {path_as_string:?} as {name:?} ...");
            zip.add_directory(path_as_string, options)?;
        }
    }
    zip.finish()?;
    #[allow(clippy::unwrap_used)] // We're a build script, we expect OUT_DIR to be set.
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("templates.rs");
    let mut file = BufWriter::new(File::create(path)?);
    let output = quote! {
        const TEMPLATES_ZIP: &[u8] = &[#(#buf),*];
    };
    file.write_all(output.to_string().as_bytes())?;
    Ok(())
}
