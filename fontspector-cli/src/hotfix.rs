use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use fontspector_checkapi::{prelude::*, CheckResult, DialogFieldType, HotfixFunction, Metadata};
use serde_json::Value;
use std::io::Write;
use termimad::MadSkin;

use crate::reporters::terminal::check_id_link;

pub(crate) fn run_hotfix(
    testable: &mut Testable,
    modified: &mut bool,
    result: &mut CheckResult,
    fix: &HotfixFunction,
) {
    let mut options = None;
    let mut header_shown = false;

    // If we have a metadata containing a FixNeedsMoreInformation, we can run the dialog first to get the options for the fix
    if let Some(Metadata::FixNeedsMoreInformation(dialog)) = result
        .subresults
        .iter()
        .flat_map(|s| &s.metadata)
        .find(|m| matches!(m, Metadata::FixNeedsMoreInformation(_)))
    {
        show_header(
            testable,
            &result.section,
            &result.check_id,
            &result.check_name,
        );
        options = run_dialog(dialog);
        header_shown = true;
    }

    loop {
        match fix(testable, options) {
            Ok(FixResult::MoreInfoNeeded(dialog)) => {
                if !header_shown {
                    show_header(
                        testable,
                        &result.section,
                        &result.check_id,
                        &result.check_name,
                    );
                    header_shown = true;
                }

                options = run_dialog(&dialog);
                continue;
            }
            Ok(hotfix_result) => {
                if matches!(hotfix_result, FixResult::Fixed) {
                    *modified = true;
                }
                result.hotfix_result = Some(hotfix_result);
                return;
            }
            Err(e) => {
                result.hotfix_result = Some(FixResult::FixFailed(e.to_string()));
                return;
            }
        }
    }
}

fn show_header(
    testable: &mut Testable,
    section: &Option<String>,
    check_id: &str,
    check_name: &str,
) {
    let skin = MadSkin::default();
    let filename = testable.filename.to_string_lossy();
    let _ = writeln!(std::io::stdout(), "Testing: {filename}");
    if let Some(sectionname) = section {
        let _ = writeln!(std::io::stdout(), "  Section: {sectionname}\n");
    }
    let _ = writeln!(std::io::stdout(), ">> {:}\n", check_id_link(check_id));

    let _ = writeln!(std::io::stdout(), "   {:}\n", check_name);
    eprintln!("{}", skin.term_text("More information was needed to fix this problem, so we will ask you some questions to help the process.\n"));
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
                if let Ok(Some(selection)) = Select::with_theme(&ColorfulTheme::default())
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
                if let Ok(input) = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt(&field.prompt)
                    .interact_text()
                {
                    replies.0.insert(field.key.clone(), Value::String(input));
                }
            }
            DialogFieldType::Number => {
                if let Ok(input) = Input::with_theme(&ColorfulTheme::default())
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
                if let Ok(confirmation) = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(&field.prompt)
                    .interact()
                {
                    replies
                        .0
                        .insert(field.key.clone(), Value::Bool(confirmation));
                }
            }
        }
    }
    Some(replies)
}
