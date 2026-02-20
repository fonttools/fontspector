use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{error::FontspectorError, Override};
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone, Serialize, Deserialize, Hash)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[serde(rename_all = "UPPERCASE")]
/// A severity level for a single check subresult
pub enum StatusCode {
    /// Skip: the check didn't run because some condition was not met
    Skip,
    /// Pass: there's no problem here
    Pass,
    /// Info: the check returned some useful information, but no problems
    Info,
    /// Warn: a problem which should be manually reviewed
    Warn,
    /// Fail: a problem materially affects the correctness of the font
    Fail,
    /// Fatal: a critical font defect that would break font-serving infrastructure
    Fatal,
    /// Error: something went wrong with the check itself
    ///
    /// An Error is when something which returns a `Result<>` gave us
    /// an `Err` - for example a file couldn't be found or couldn't be
    /// parsed, even though we did our best to check for things. In
    /// other words, something went wrong with the check infrastructure,
    /// not the font itself.
    Error,
}

impl FromStr for StatusCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SKIP" => Ok(StatusCode::Skip),
            "INFO" => Ok(StatusCode::Info),
            "PASS" => Ok(StatusCode::Pass),
            "WARN" => Ok(StatusCode::Warn),
            "FAIL" => Ok(StatusCode::Fail),
            "FATAL" => Ok(StatusCode::Fatal),
            "ERROR" => Ok(StatusCode::Error),
            _ => Err(()),
        }
    }
}

impl StatusCode {
    /// Return an iterator over all status codes
    ///
    /// Used to provide a list of possible status codes to the user when
    /// selecting the minimum reported status.
    pub fn all() -> impl Iterator<Item = StatusCode> {
        vec![
            StatusCode::Skip,
            StatusCode::Info,
            StatusCode::Pass,
            StatusCode::Warn,
            StatusCode::Fail,
            StatusCode::Fatal,
            StatusCode::Error,
        ]
        .into_iter()
    }

    /// Convert a string to a status code
    ///
    /// This is used when the status code comes from an external source,
    /// such as FontBakery.
    pub fn from_string(s: &str) -> Option<StatusCode> {
        FromStr::from_str(s).ok()
    }
}
impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            StatusCode::Pass => write!(f, "PASS"),
            StatusCode::Skip => write!(f, "SKIP"),
            StatusCode::Fail => write!(f, "FAIL"),
            StatusCode::Fatal => write!(f, "FATAL"),
            StatusCode::Warn => write!(f, "WARN"),
            StatusCode::Info => write!(f, "INFO"),
            StatusCode::Error => write!(f, "ERROR"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
/// Metadata about a check result, which can be used by the reporter to provide
/// additional information about the check result. This is intended to make the
/// results of checks machine readable, for display in font editors or other tools.
pub enum Metadata {
    /// A problem with a specific glyph.
    GlyphProblem {
        /// The name of the glyph
        glyph_name: String,
        /// The ID of the glyph
        glyph_id: u32,
        /// A specific location within the font's design space, in user-space coordinates.
        #[serde(skip_serializing_if = "Option::is_none")]
        userspace_location: Option<HashMap<String, f32>>,
        /// A specific location within the glyph's coordinate space.
        #[serde(skip_serializing_if = "Option::is_none")]
        position: Option<(f32, f32)>,
        /// The value that was found.
        #[serde(skip_serializing_if = "Option::is_none")]
        actual: Option<serde_json::Value>,
        /// The value that was expected.
        #[serde(skip_serializing_if = "Option::is_none")]
        expected: Option<serde_json::Value>,
        /// A description of the problem to show to the user.
        message: String,
    },
    /// A problem with a specific OpenType table.
    TableProblem {
        /// The tag of the table
        table_tag: String,
        /// The field within the table which has the problem, if any.
        #[serde(skip_serializing_if = "Option::is_none")]
        field_name: Option<String>,
        /// The value of the field which has the problem, if any.
        #[serde(skip_serializing_if = "Option::is_none")]
        actual: Option<serde_json::Value>,
        /// The expected value of the field, if any.
        #[serde(skip_serializing_if = "Option::is_none")]
        expected: Option<serde_json::Value>,
        /// A description of the problem to show to the user.
        message: String,
    },
    /// A problem which is not specific to a glyph or table.
    FontProblem {
        /// A description of the problem to show to the user.
        message: String,
        /// Additional context about the problem
        #[serde(skip_serializing_if = "Option::is_none")]
        context: Option<serde_json::Value>,
    },
    /// A catch-all for other kinds of structured data.
    Other(serde_json::Value),
}
#[derive(Debug, Clone, Serialize)]
/// A status message from a check
///
/// This is a subresult, in the sense that a check may return multiple failures
/// and warnings; all the subresults then get wrapped into a [crate::CheckResult]
/// which is the final result of the check.
pub struct Status {
    /// A message to explain the status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The severity of the status
    pub severity: StatusCode,
    /// A code to identify the status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Additional metadata provided to the reporter
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<Metadata>,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "**{:}**: ", self.severity)?;
        if let Some(code) = self.code.as_ref() {
            write!(f, "[{code}]: ")?;
        }
        if let Some(message) = self.message.as_ref() {
            write!(f, "{message:}")?;
        }
        Ok(())
    }
}

impl Status {
    /// Return a single pass result from a check
    pub fn just_one_pass() -> Box<dyn Iterator<Item = Status>> {
        Box::new(vec![Status::pass()].into_iter())
    }

    /// Return a single warn result from a check
    pub fn just_one_warn(code: &str, message: &str) -> Box<dyn Iterator<Item = Status>> {
        Box::new(vec![Status::warn(code, message)].into_iter())
    }

    /// Return a single info result from a check
    pub fn just_one_info(code: &str, message: &str) -> Box<dyn Iterator<Item = Status>> {
        Box::new(vec![Status::info(code, message)].into_iter())
    }

    /// Return a single fail result from a check
    pub fn just_one_fail(code: &str, message: &str) -> Box<dyn Iterator<Item = Status>> {
        Box::new(vec![Status::fail(code, message)].into_iter())
    }

    /// Return a single fatal result from a check
    pub fn just_one_fatal(code: &str, message: &str) -> Box<dyn Iterator<Item = Status>> {
        Box::new(vec![Status::fatal(code, message)].into_iter())
    }

    /// Return a single skip result from a check
    pub fn just_one_skip(code: &str, message: &str) -> Box<dyn Iterator<Item = Status>> {
        Box::new(vec![Status::skip(code, message)].into_iter())
    }

    /// Create a status with a pass severity
    pub fn pass() -> Self {
        Self {
            message: None,
            code: None,
            severity: StatusCode::Pass,
            metadata: Vec::new(),
        }
    }
    /// Create a status with a fail severity
    pub fn fail(code: &str, message: &str) -> Self {
        Self {
            message: Some(message.to_string()),
            code: Some(code.to_string()),
            severity: StatusCode::Fail,
            metadata: Vec::new(),
        }
    }
    /// Create a status with a warning severity
    pub fn warn(code: &str, message: &str) -> Self {
        Self {
            message: Some(message.to_string()),
            code: Some(code.to_string()),
            severity: StatusCode::Warn,
            metadata: Vec::new(),
        }
    }
    /// Create a status with an info severity
    pub fn skip(code: &str, message: &str) -> Self {
        Self {
            message: Some(message.to_string()),
            code: Some(code.to_string()),
            severity: StatusCode::Skip,
            metadata: Vec::new(),
        }
    }
    /// Create a status with an info severity
    pub fn info(code: &str, message: &str) -> Self {
        Self {
            message: Some(message.to_string()),
            code: Some(code.to_string()),
            severity: StatusCode::Info,
            metadata: Vec::new(),
        }
    }
    /// Create a status with a fatal severity
    pub fn fatal(code: &str, message: &str) -> Self {
        Self {
            message: Some(message.to_string()),
            code: Some(code.to_string()),
            severity: StatusCode::Fatal,
            metadata: Vec::new(),
        }
    }
    /// Create a status with an error severity
    pub fn error(code: Option<&str>, message: &str) -> Self {
        Self {
            message: Some(message.to_string()),
            code: code.map(|x| x.to_string()),
            severity: StatusCode::Error,
            metadata: Vec::new(),
        }
    }

    /// Append metadata to the status
    pub fn add_metadata(&mut self, metadata: Metadata) -> &mut Self {
        self.metadata.push(metadata);
        self
    }

    /// Apply an override to the status
    ///
    /// Overrides are provided by the profile or by the user's configuration file;
    /// they are used to override the severity of a check result.
    pub fn process_override(&mut self, overrides: &[Override]) {
        let code = self.code.as_deref();
        if let Some(override_) = overrides.iter().find(|x| Some(x.code.as_str()) == code) {
            self.severity = override_.status;
            self.message = Some(format!(
                "{} (Overriden: {})",
                self.message
                    .clone()
                    .unwrap_or("No original message".to_string()),
                override_.reason
            ))
        }
    }
}

/// A list of statuses
pub type StatusList = Box<dyn Iterator<Item = Status>>;
/// The expected return type of a check implementation function
pub type CheckFnResult = Result<StatusList, FontspectorError>;
