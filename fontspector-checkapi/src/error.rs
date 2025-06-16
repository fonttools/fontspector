use std::sync::PoisonError;

use thiserror::Error;

#[derive(Error, Debug, Clone)]
/// An error that occurred during the check process
/// This is used to return errors from check functions
pub enum FontspectorError {
    /// A problem with skrifa reading the font binary
    #[error("Error reading font file: {0}")]
    FontRead(#[from] fontations::read::ReadError),
    /// A problem with skrifa writing the font binary
    #[error("Error writing font file: {0}")]
    FontWrite(#[from] fontations::write::error::Error),
    /// A problem with skrifa producing a the font binary
    #[error("Error building font file: {0}")]
    FontBuild(#[from] fontations::write::BuilderError),
    /// A problem with skrifa outline code
    #[error("Error drawing glyph: {0}")]
    Draw(#[from] fontations::skrifa::outline::DrawError),
    /// Just a skip
    #[error("Skipping check: {message} [{code}]")]
    Skip {
        /// The code for the skip
        code: &'static str,
        /// The message for the skip
        message: &'static str,
    },
    /// We thought something was a TTF file, but it wasn't
    #[error(
        "Inappropriate file type: {filename} was not a valid ${expected} file: {more_details}"
    )]
    InappropriateFile {
        /// The expected file type
        expected: &'static str,
        /// The filename that was not the expected type
        filename: String,
        /// Additional details about the error
        more_details: String,
    },
    /// Something went wrong doing Python things
    #[error("Python error: {0}")]
    Python(String),
    /// Invalid UTF-8 was found
    #[error("Invalid UTF-8: {0}")]
    InvalidUtf8(#[from] std::str::Utf8Error),
    /// Invalid JSON was found
    #[error("Invalid JSON: {0}")]
    InvalidJson(#[from] std::sync::Arc<serde_json::Error>),
    /// An error occurred while reading a file
    #[error("Error reading file: {0}")]
    FileRead(#[from] std::sync::Arc<std::io::Error>),
    /// An error occured when reading from the network
    #[error("A network error occurred: {0}")]
    Network(String), // Not going to depend on reqwest here, so we use a String
    /// We tried to use the network, but it was disabled
    #[error("Network access is disabled, but this check requires it")]
    NetworkAccessDisabled,
    /// A shaping engine returned an error
    #[error("Shaping engine error: {0}")]
    Shaping(String),
    /// A problem serializing or deserializing data to the cache
    #[error("Cache serialization/deserialization error: {0}")]
    CacheSerialization(String),
    /// A problem with the cache
    #[error("Cache error: {0}")]
    CachePoison(String),
    /// Something else happened when checking the font
    #[error("Something went wrong: {0}")]
    General(String),
    /// Something else happened when fixing the font
    #[error("Something went wrong while fixing: {0}")]
    Fix(String),
}

impl From<std::string::FromUtf8Error> for FontspectorError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        FontspectorError::InvalidUtf8(err.utf8_error())
    }
}

impl From<serde_json::Error> for FontspectorError {
    fn from(err: serde_json::Error) -> Self {
        FontspectorError::InvalidJson(std::sync::Arc::new(err))
    }
}

impl From<std::io::Error> for FontspectorError {
    fn from(err: std::io::Error) -> Self {
        FontspectorError::FileRead(std::sync::Arc::new(err))
    }
}

impl<T> From<PoisonError<T>> for FontspectorError {
    fn from(err: PoisonError<T>) -> Self {
        FontspectorError::CachePoison(err.to_string())
    }
}

impl FontspectorError {
    /// Create a skip error with a code and message
    pub fn skip(code: &'static str, message: &'static str) -> Self {
        FontspectorError::Skip { code, message }
    }
}
