use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::FontspectorError;

/// A source file that can be loaded (and saved) from a path.
pub struct SourceFile {
    /// The font source.
    pub source: babelfont::Font,
    /// The path to the file, if available.
    pub file: PathBuf,
}

impl SourceFile {
    /// Load a source file from a given path.
    pub fn new(path: &Path) -> Result<Self, FontspectorError> {
        if !path.exists() {
            return Err(FontspectorError::FileNotFound(path.to_path_buf()));
        }
        let source = babelfont::load(path)
            .map_err(|_e| FontspectorError::UnrecognizedSource(path.to_path_buf()))?;
        // A better error will come in time
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
        self.source
            .save(&self.file)
            .map_err(|e| FontspectorError::SaveError {
                path: self.file.clone(),
                error: e.to_string(),
            })
    }
}
