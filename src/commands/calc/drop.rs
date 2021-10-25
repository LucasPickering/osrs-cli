use std::fmt::Display;

use crate::{
    commands::Command,
    error::OsrsError,
    utils::{context::CommandContext, fmt, math},
};
use lazy_static::lazy_static;
use regex::Regex;
use structopt::StructOpt;

/// Parse a probability string, which can be an integer, decimal, fraction, or
/// percentage. Also enforces that the probability is in [0, 1].
fn parse_probability(s: &str) -> anyhow::Result<f64> {
    lazy_static! {
        // regex to match an integer, decimal, fraction, or percentage
        // see test cases for positive+negative examples
        static ref RE: Regex =
            Regex::new(r"^\s*(?P<num>[\d.]+)\s*(?:(/\s*(?P<denom>[\d.]+)\s*)|(?P<pct>%))?\s*$")
                .unwrap();
    }
    let prob: f64 = match RE.captures(s) {
        None => {
            return Err(OsrsError::ArgsError(format!(
            "Invalid probability: {}; Try a decimal, percentage, or fraction.",
            s,
        ))
            .into())
        }
        Some(caps) => {
            // numerator is a required group so unwrap is safe
            let numerator: f64 = caps.name("num").unwrap().as_str().parse()?;

            let denom_opt = caps.name("denom");
            let is_pct = caps.name("pct").is_some();

            match (denom_opt, is_pct) {
                (Some(denom_match), false) => {
                    let denominator: f64 = denom_match.as_str().parse()?;
                    numerator / denominator
                }
                (None, false) => numerator,
                (None, true) => numerator / 100.0,
                // This case shouldn't be possible because the regex makes them
                // mutually exclusive
                (Some(_), true) => {
                    panic!("Received both fraction and percentage!")
                }
            }
        }
    };

    // Make sure the value is in range
    if (0.0..=1.0).contains(&prob) {
        Ok(prob)
    } else {
        Err(OsrsError::ArgsError(format!(
            "Probability must be in range [0, 1], but got: {}",
            prob,
        ))
        .into())
    }
}

/// Parse an input string for the target number of success into a numerical
/// range. This looks for exact values, a range <= to a given value, or >= to
/// a given value.
fn parse_target_range(s: &str) -> anyhow::Result<TargetRange> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)([-+]?)$").unwrap();
    }
    match RE.captures(s) {
        // no buen
        None => {
            Err(OsrsError::ArgsError(format!("Invalid target range: {}", s))
                .into())
        }
        // buen
        Some(caps) => {
            // Both these groups match always so if the regex matches, they
            // should both have values
            let k: usize = caps.get(1).unwrap().as_str().parse()?;
            let sign = caps.get(2).unwrap().as_str();
            let result = match sign {
                "" => TargetRange::Eq(k),
                "-" => TargetRange::Lte(k),
                "+" => TargetRange::Gte(k),
                // Regex shouldn't let any other values through
                other => panic!("Regex allowed invalid sign char: {}", other),
            };
            Ok(result)
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum TargetRange {
    Eq(usize),
    Lte(usize),
    Gte(usize),
}

impl TargetRange {
    /// Convert to an iterator of values. This should cover all values in the
    /// target range, as a sub-set of `[0, iterations]`.
    fn as_values(&self, iterations: usize) -> Box<dyn Iterator<Item = usize>> {
        match self {
            Self::Eq(k) => Box::new(*k..=*k),
            Self::Lte(k) => Box::new(0..=*k),
            Self::Gte(k) => Box::new(*k..=iterations),
        }
    }
}

impl Display for TargetRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eq(k) => write!(f, "{}", k),
            Self::Lte(k) => write!(f, "≤{}", k),
            Self::Gte(k) => write!(f, "≥{}", k),
        }
    }
}

/// Calculate the probability of getting a drop.
#[derive(Debug, StructOpt)]
pub struct CalcDropCommand {
    /// The probability of a success. Typically your drop rate. Supports
    /// decimal, percentage, or fractions. E.g., `0.02`, `2%`, and `1/50` are
    /// all supported and equivalent.
    #[structopt(short, long, parse(try_from_str = parse_probability))]
    probability: f64,
    /// The number of chances for your drop, e.g. kill count or harvest count.
    #[structopt(short = "n", long)]
    iterations: usize,
    /// The target number of successes. Use just a number for an exact value,
    /// or `+`/`-` for ranges. E.g., `1+` means "1 or more successes", `3-`
    /// means "3 or fewer successes", etc.
    #[structopt(short, long,parse(try_from_str = parse_target_range), default_value = "1+")]
    target: TargetRange,
}

impl Command for CalcDropCommand {
    fn execute(&self, _context: &CommandContext) -> anyhow::Result<()> {
        // Valid probability
        if !(0.0..=1.0).contains(&self.probability) {
            return Err(OsrsError::ArgsError(format!(
                "Probability must be between 0 and 1, got: {}",
                self.probability
            ))
            .into());
        }

        // Do the cumulative distribution function, which is just to sum
        // up the probability of all the values in the
        // range. https://en.wikipedia.org/wiki/Binomial_distribution#Cumulative_distribution_function

        let result_prob: f64 = math::binomial_cdf(
            self.probability,
            self.iterations,
            &mut self.target.as_values(self.iterations),
        );

        println!(
            "{} chance of {} successes in {} attempts",
            fmt::fmt_probability_long(result_prob),
            self.target,
            self.iterations
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_parse_probability() {
        // basic success cases
        assert_approx_eq!(parse_probability("0").unwrap(), 0.0);
        assert_approx_eq!(parse_probability("1").unwrap(), 1.0);
        assert_approx_eq!(parse_probability("0.5").unwrap(), 0.5);
        assert_approx_eq!(parse_probability("50%").unwrap(), 0.5);
        assert_approx_eq!(parse_probability("1/2").unwrap(), 0.5);
        assert_approx_eq!(parse_probability("0.5%").unwrap(), 0.005);

        // whitespace is ignored
        assert_approx_eq!(parse_probability(" 0.5 ").unwrap(), 0.5);
        assert_approx_eq!(parse_probability(" 1 / 2 ").unwrap(), 0.5);
        assert_approx_eq!(parse_probability(" 50 % ").unwrap(), 0.5);

        // fractions w/ decimals
        assert_approx_eq!(parse_probability("1 / 25.6").unwrap(), 0.0390625);
    }

    #[test]
    fn test_parse_probability_errors() {
        // basic success cases
        assert!(parse_probability("5").is_err());
        assert!(parse_probability("%5").is_err());
        assert!(parse_probability("-5").is_err());
        assert!(parse_probability("1 / 2 / 3").is_err());
        assert!(parse_probability("0.5.5").is_err());
        assert!(parse_probability("5% / 5").is_err());
        assert!(parse_probability("1/2%").is_err());
    }
}
