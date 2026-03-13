use std::collections::HashMap;

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

impl FixResult {
    /// A helper function to check if the fix was successful
    pub fn is_success(&self) -> bool {
        matches!(self, FixResult::Fixed)
    }
}

#[derive(Debug, Clone, Serialize)]
/// A request for more information from the user, in order to apply a fix.
pub struct MoreInfoRequest(pub Vec<DialogField>);

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

impl DialogField {
    /// A helper function to create a choice field from a list of (value, description) pairs
    pub fn new_choice(key: &str, prompt: &str, options: Vec<(&str, &str)>) -> Self {
        DialogField {
            key: key.to_string(),
            prompt: prompt.to_string(),
            field_type: DialogFieldType::Choice(
                options
                    .into_iter()
                    .map(|(value, description)| Choice {
                        value: value.to_string(),
                        description: description.to_string(),
                    })
                    .collect(),
            ),
        }
    }

    /// A helper function to create a text field
    pub fn new_text(key: &str, prompt: &str) -> Self {
        DialogField {
            key: key.to_string(),
            prompt: prompt.to_string(),
            field_type: DialogFieldType::Text,
        }
    }

    /// A helper function to create a number field
    pub fn new_number(key: &str, prompt: &str) -> Self {
        DialogField {
            key: key.to_string(),
            prompt: prompt.to_string(),
            field_type: DialogFieldType::Number,
        }
    }

    /// A helper function to create a boolean field
    pub fn new_boolean(key: &str, prompt: &str) -> Self {
        DialogField {
            key: key.to_string(),
            prompt: prompt.to_string(),
            field_type: DialogFieldType::Boolean,
        }
    }
}

/// A collection of user responses to a dialog request, mapping field keys to values
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MoreInfoReplies(pub HashMap<String, Value>);

/// The function signature for a hotfix function
pub type HotfixFunction =
    dyn Fn(&mut Testable, Option<MoreInfoReplies>) -> Result<FixResult, FontspectorError>;
