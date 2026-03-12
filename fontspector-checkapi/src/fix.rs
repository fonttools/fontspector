use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{FontspectorError, Testable};

#[derive(Debug, Clone, Serialize)]
/// The result of a fix operation.
pub enum FixResult {
    /// A fix was available, but not requested
    Available,
    /// A fix was requested, but no fix was available
    Unfixable,
    /// More information was needed to fix
    MoreInfoNeeded(MoreInfoRequest),
    /// A fix was applied
    Fixed,
    /// The fix failed, for some reason
    FixFailed(String),
    // /// An error happened while trying to apply the fix
    // FixError(String),
}

#[derive(Debug, Clone, Serialize)]
/// A request for more information from the user, in order to apply a fix.
pub struct MoreInfoRequest(Vec<DialogField>);

/// A field in a dialog request, which can be of various types (choice, text, number, boolean)
#[derive(Debug, Clone, Serialize)]
pub struct DialogField {
    /// A unique key for this field, which will be used to identify the user's response
    pub key: String,
    /// A prompt to show the user, describing what information is needed
    pub prompt: String,
    /// The type of field, which determines how the user's response should be interpreted
    pub field_type: DialogFieldType,
}

/// A single choice in a choice field, with a value and a description
#[derive(Debug, Clone, Serialize)]
pub struct Choice {
    /// The value that will be returned if the user selects this choice
    pub value: String,
    /// A description of this choice to show to the user
    pub description: String,
}

/// The type of a dialog field, which determines how the user input should be requested
#[derive(Debug, Clone, Serialize)]
pub enum DialogFieldType {
    /// A field where the user must choose one of several options
    Choice(Vec<Choice>),
    /// A field where the user can enter freeform text
    Text,
    /// A field where the user can enter a number
    Number,
    /// A field where the user can enter a boolean value (true/false)
    Boolean,
}

/// A collection of user responses to a dialog request, mapping field keys to values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoreInfoReplies(std::collections::HashMap<String, Value>);

/// The function signature for a hotfix function
pub type HotfixFunction =
    dyn Fn(&mut Testable, Option<MoreInfoReplies>) -> Result<FixResult, FontspectorError>;
