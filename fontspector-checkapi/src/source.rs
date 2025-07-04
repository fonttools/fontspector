use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use glyphslib::Font;
use norad::designspace::DesignSpaceDocument;
use norad::Font as UfoFont;

use crate::FontspectorError;

/// A source of a font, which can be a Glyphs file, a UFO file, or a DesignSpace document.
#[derive(Debug, Clone)]
pub enum Source {
    /// A source that contains a Glyphs font.
    ///
    /// This may be either Glyphs 2 or Glyphs 3 format.
    Glyphs(Box<Font>),
    /// A source that contains a UFO font.
    Ufo(Box<UfoFont>),
    /// A source that contains a DesignSpace document.
    Designspace(DesignSpaceDocument),
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
            "designspace" => Ok(Source::Designspace(DesignSpaceDocument::load(path)?)),
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
            Source::Ufo(font) => font
                .save(&self.file)
                .map_err(|e| FontspectorError::SaveError {
                    path: self.file.clone(),
                    error: e.to_string(),
                }),
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
