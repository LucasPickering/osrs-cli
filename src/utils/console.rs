//! Utilities related to the console/terminal and interactivity

use dialoguer::{Confirm, MultiSelect, Select};
use std::iter;
use strum::IntoEnumIterator;

/// Generate a yes/no user prompt
pub fn confirm(prompt: &str, default: bool) -> anyhow::Result<bool> {
    Ok(Confirm::new()
        .with_prompt(prompt)
        .default(default)
        .interact()?)
}

/// Generate a single-select user prompt from an enum. The value is optional,
/// so there will always be a `None` option as well as one for each variant of
/// the enum.
pub fn enum_select<T>(
    prompt: &str,
    default: Option<T>,
) -> anyhow::Result<Option<T>>
where
    T: Copy + PartialEq + ToString + IntoEnumIterator,
{
    let choices: Vec<Option<T>> =
        iter::once(None).chain(T::iter().map(Some)).collect();
    let choice_labels: Vec<String> = choices
        .iter()
        .map(|choice| match choice {
            Some(choice) => choice.to_string(),
            None => "None".into(),
        })
        .collect();
    // dialoguer makes you specify default by index rather than value
    let default_index = choices
        .iter()
        .position(|choice| *choice == default)
        .unwrap_or_default();

    // dialoguer passes back the index of the selected choice
    let selected_index = Select::new()
        .with_prompt(prompt)
        .default(default_index)
        .items(&choice_labels)
        .interact()?;

    Ok(choices[selected_index])
}

/// Generate a multi-select user prompt from an enum. There will be one option
/// per enum variant, and the user can freely toggle each option to be on/off.
/// The returned vec will be only the enabled options.
pub fn enum_multi_select<T>(
    prompt: &str,
    default: &[T],
) -> anyhow::Result<Vec<T>>
where
    T: Copy + PartialEq + ToString + IntoEnumIterator,
{
    let choices: Vec<T> = T::iter().collect();
    let choice_labels: Vec<(String, bool)> = choices
        .iter()
        .map(|choice| (choice.to_string(), default.contains(choice)))
        .collect();

    // Get the index of the desired choice from the user
    let selected_indexes = MultiSelect::new()
        .with_prompt(format!("{} (Space to toggle)", prompt))
        .items_checked(&choice_labels)
        .interact()?;

    // Map indexes back to values
    Ok(selected_indexes
        .into_iter()
        .map(|index| choices[index])
        .collect())
}
