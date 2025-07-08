use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use glyphslib::Font;
use norad::{designspace::DesignSpaceDocument, Font as UfoFont};

use crate::{prelude::FixFnResult, FontspectorError};

#[derive(Debug, Clone)]
/// Our representation of a DesignSpace document, which contains multiple UFO sources.
pub struct DesignSpace {
    /// The DesignSpace document.
    pub document: DesignSpaceDocument,
    /// The source fonts
    pub sources: HashMap<String, Box<UfoFont>>,
}

impl DesignSpace {
    /// Load a DesignSpace document from a path.
    pub fn load(path: &Path) -> Result<Self, FontspectorError> {
        let document = DesignSpaceDocument::load(path)?;
        let mut sources = HashMap::new();
        for source in &document.sources {
            let ufo_font = UfoFont::load(path.with_file_name(&source.filename))?;
            sources.insert(source.filename.clone(), Box::new(ufo_font));
        }
        Ok(Self { document, sources })
    }

    /// Save the DesignSpace document to a path.
    pub fn save(&self, path: &Path) -> Result<(), FontspectorError> {
        self.document
            .save(path)
            .map_err(|e| FontspectorError::SaveError {
                path: path.to_path_buf(),
                error: e.to_string(),
            })?;
        // Now save all the UFO sources
        for (name, font) in &self.sources {
            font.save(path.with_file_name(name))?;
        }
        Ok(())
    }

    /// Apply to fix to each UFO source in the DesignSpace document.
    pub fn apply_fix(&mut self, fix: &dyn Fn(&mut UfoFont) -> FixFnResult) -> FixFnResult {
        let mut changed = false;
        for font in self.sources.values_mut() {
            if fix(font)? {
                changed = true;
            }
        }
        Ok(changed)
    }
}

/// A source of a font, which can be a Glyphs file, a UFO file, or a DesignSpace document.
#[derive(Debug)]
pub enum Source {
    /// A source that contains a Glyphs font.
    ///
    /// This may be either Glyphs 2 or Glyphs 3 format.
    Glyphs(Box<Font>),
    /// A source that contains a UFO font.
    Ufo(Box<UfoFont>),
    /// A source that contains a DesignSpace document.
    Designspace(Box<DesignSpace>),
}

/// A source file that can be loaded (and saved) from a path.
pub struct SourceFile {
    /// The source of the font.
    pub source: Source,
    /// The path to the file, if available.
    pub file: PathBuf,
}

impl SourceFile {
    /// Load a source file from a given path.
    pub fn new(path: &Path) -> Result<Self, FontspectorError> {
        if !path.exists() {
            return Err(FontspectorError::FileNotFound(path.to_path_buf()));
        }
        let ext = path
            .extension()
            .and_then(OsStr::to_str)
            .ok_or_else(|| FontspectorError::UnrecognizedSource(path.to_path_buf()))?;
        let source = match ext {
            "designspace" => Ok(Source::Designspace(Box::new(DesignSpace::load(path)?))),
            "ufo" => Ok(Source::Ufo(Box::new(UfoFont::load(path)?))),
            "glyphs" | "glyphspackage" => Ok(Source::Glyphs(Box::new(Font::load(path)?))),
            _ => Err(FontspectorError::UnrecognizedSource(path.to_path_buf())),
        }?;
        Ok(Self {
            source,
            file: path.to_path_buf(),
        })
    }

    /// Returns the filename of the source file.
    pub fn filename(&self) -> String {
        // Displaying a PathBuf is a chore, let's have a method for it.
        self.file
            .file_name()
            .and_then(OsStr::to_str)
            .map_or_else(|| "unknown".to_string(), |s| s.to_string())
    }

    /// Saves the source file.
    pub fn save(&self) -> Result<(), FontspectorError> {
        match &self.source {
            Source::Glyphs(font) => {
                font.save(&self.file)
                    .map_err(|e| FontspectorError::SaveError {
                        path: self.file.clone(),
                        error: e.to_string(),
                    })
            }
            Source::Ufo(font) => Ok(font.save(&self.file)?),
            Source::Designspace(doc) => {
                doc.save(&self.file)
                    .map_err(|e| FontspectorError::SaveError {
                        path: self.file.clone(),
                        error: e.to_string(),
                    })
            }
        }
    }
}

// Utility functions for source fixes

/// Find or add a custom parameter in a Glyphs font.
pub fn find_or_add_cp(
    cps: &mut Vec<glyphslib::common::CustomParameter>,
    name: &str,
    value: glyphslib::Plist,
) -> FixFnResult {
    if let Some(cp) = cps.iter_mut().find(|cp| cp.name == name) {
        if cp.value != value {
            log::info!("Setting {name} custom parameter to {value:?} in Glyphs font.");
            cp.value = value;
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        log::info!("Adding {name} custom parameter with value {value:?} in Glyphs font.");
        cps.push(glyphslib::common::CustomParameter {
            name: name.to_string(),
            value,
            disabled: false,
        });
        Ok(true)
    }
}
