use dialoguer::{Confirm, Input, Select};
use fontspector_checkapi::{prelude::*, DialogFieldType, HotfixFunction};
use serde_json::Value;

pub(crate) fn run_hotfix(
    file: &String,
    testable: &mut Testable,
    modified: &mut bool,
    id: String,
    fix: &HotfixFunction,
) -> Option<FixResult> {
    let mut options = None;
    loop {
        log::info!("Trying to fix {file} with {id}");
        match fix(testable, options) {
            Ok(FixResult::MoreInfoNeeded(dialog)) => {
                log::info!("Check {id} needs more info to fix {file}");
                options = run_dialog(&dialog);
                continue;
            }
            Ok(hotfix_result) => {
                if matches!(hotfix_result, FixResult::Fixed) {
                    *modified = true;
                }
                return Some(hotfix_result);
            }
            Err(e) => return Some(FixResult::FixFailed(e.to_string())),
        }
    }
}

fn run_dialog(dialog: &MoreInfoRequest) -> Option<MoreInfoReplies> {
    let mut replies = MoreInfoReplies::default();
    for field in &dialog.0 {
        match &field.field_type {
            DialogFieldType::Choice(choices) => {
                let choice_keys = choices.iter().map(|x| x.value.clone()).collect::<Vec<_>>();
                let items = choices
                    .iter()
                    .map(|x| x.description.clone())
                    .collect::<Vec<_>>();
                if let Ok(Some(selection)) = Select::new()
                    .with_prompt(&field.prompt)
                    .items(&items)
                    .interact_opt()
                {
                    replies.0.insert(
                        field.key.clone(),
                        Value::String(choice_keys.get(selection).cloned().unwrap_or_default()),
                    );
                }
            }
            DialogFieldType::Text => {
                if let Ok(input) = Input::new().with_prompt(&field.prompt).interact_text() {
                    replies.0.insert(field.key.clone(), Value::String(input));
                }
            }
            DialogFieldType::Number => {
                if let Ok(input) = Input::new()
                    .with_prompt(&field.prompt)
                    .validate_with(|input: &String| -> Result<(), &str> {
                        if input.parse::<f64>().is_ok() {
                            Ok(())
                        } else {
                            Err("Please enter a valid number")
                        }
                    })
                    .interact_text()
                {
                    if let Ok(number) = input.parse::<f64>() {
                        #[allow(clippy::unwrap_used)]
                        replies.0.insert(
                            field.key.clone(),
                            Value::Number(serde_json::Number::from_f64(number).unwrap()),
                        );
                    }
                }
            }
            DialogFieldType::Boolean => {
                if let Ok(confirmation) = Confirm::new().with_prompt(&field.prompt).interact() {
                    replies
                        .0
                        .insert(field.key.clone(), Value::Bool(confirmation));
                }
            }
        }
    }
    Some(replies)
}
