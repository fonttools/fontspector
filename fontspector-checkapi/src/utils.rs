use std::fmt::Display;

use crate::{CheckFnResult, Context, Status, StatusCode};

/// Formats a list of items as a Markdown bullet list.
pub fn bullet_list<I>(context: &Context, items: I) -> String
where
    I: IntoIterator,
    I::Item: Display,
{
    let mut items = items.into_iter();
    let first_nine = items.by_ref().take(9);
    let mut list = first_nine
        .map(|item| format!("* {item}"))
        .collect::<Vec<_>>();

    if context.full_lists {
        list.extend(items.map(|item| format!("* {item}")));
    } else {
        let remainder = items.count();
        if remainder > 0 {
            list.push(format!("... and {remainder} others"));
        }
    }
    list.join("\n")
}

/// Asserts that all the values in a list are the same.
///
/// Each value is passed as a tuple of three values:
/// * The element to compare
/// * A displayable value to include in the message
/// * A label for the value to include in the message
///
/// For example:
///
/// ```rust,ignore
///     &[
///      (0b00000001, "Italic", "FontA.ttf"),
///      (0b00000001, "Italic", "FontB.ttf"),
///      (0b00100000, "Bold",   "FontC.ttf"),
///     ]
/// ```
///
/// The values are compared for equality.
/// If they are not equal, a failure status is returned, with a message listing all the values.
/// If they are, a pass status is returned.
pub fn assert_all_the_same<T, U, V>(
    _context: &Context,
    values: &[(T, U, V)],
    code: &str,
    message_start: &str,
    severity: StatusCode,
) -> CheckFnResult
where
    T: Eq,
    U: Display,
    V: Display,
{
    #[allow(clippy::indexing_slicing)] // If we are inside .all, then there must be a first element
    let ok = values.iter().all(|(a, _, _)| a == &values[0].0);
    if ok {
        Ok(Status::just_one_pass())
    } else {
        let message = format!(
            "{}\n\nThe following values were found:\n\n{}",
            message_start,
            bullet_list(_context, values.iter().map(|(_, a, b)| format!("{a}: {b}")))
        );
        if StatusCode::Fail == severity {
            Ok(Status::just_one_fail(code, &message))
        } else {
            Ok(Status::just_one_warn(code, &message))
        }
    }
}
