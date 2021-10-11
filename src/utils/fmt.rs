//! Utilities related to formatting values into strings

use num_format::{SystemLocale, ToFormattedString};
use std::fmt::Display;

/// Format the given number. The formatting will be based on system locale.
pub fn fmt_int<T: ToFormattedString>(num: &T) -> String {
    let locale = SystemLocale::default().unwrap();
    num.to_formatted_string(&locale)
}

/// Format a GE price. Prices are typically options since any item could
/// potentially have no trade data, so will format `None` as a dash. Otherwise,
/// the price will be formatted as an int (with commas).
pub fn fmt_price(price: Option<usize>) -> String {
    match price {
        Some(price) => fmt_int(&price),
        // If no price is present, show a placeholder
        None => "—".into(),
    }
}

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
