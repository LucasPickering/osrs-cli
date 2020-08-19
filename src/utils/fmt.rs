//! Utilities related to formatting values into strings

use std::fmt::Display;

/// Format a probability value (0 to 1) into a percentage string.
pub fn fmt_probability(probability: f64) -> String {
    format!("{:.1}%", probability * 100.0)
}

/// Format a probability value (0 to 1) into a percentage string, with extra
/// decimal places.
pub fn fmt_probability_long(probability: f64) -> String {
    format!("{:.4}%", probability * 100.0)
}

/// Format a boolean into a yes/no string
pub fn fmt_bool(b: bool) -> &'static str {
    if b {
        "Yes"
    } else {
        "No"
    }
}

/// Format an option into a string of either the contained value (if `Some`) or
/// the string `"None"`.
pub fn fmt_option<T: Display>(opt: Option<T>) -> String {
    match opt {
        Some(value) => value.to_string(),
        None => "None".to_string(),
    }
}
