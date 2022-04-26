//! Utilities related to formatting values into strings

use num_format::{Locale, ToFormattedString};

/// Format the given number.
pub fn fmt_int<T: ToFormattedString>(num: &T) -> String {
    // Formatting is hard-coded to English locale, because the system locale
    // detection wasn't working on Windows (num-format wouldn't compile on
    // v0.4.0). We could try again in the future, but since OSRS is hard-coded
    // to English, not much of a reason for the CLI to be more flexible.
    let locale = Locale::en;
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
