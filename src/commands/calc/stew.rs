use std::iter;

use crate::{
    commands::Command,
    error::OsrsError,
    utils::{context::CommandContext, math},
};
use prettytable::{color, format::Alignment, Attr, Cell, Row, Table};
use structopt::StructOpt;

/// Maximum number of doses per stew
const MAX_DOSES: usize = 3;

/// Highest possible boost
const MAX_BOOST: usize = 5;

/// For each dose count, the odds of getting AT LEAST that boost (+0 through +5)
/// https://oldschool.runescape.wiki/w/Spicy_stew#Probability
const CUMULATIVE_PROBS: [[f64; MAX_BOOST + 1]; MAX_DOSES] = [
    [0.750, 0.250, 0.000, 0.000, 0.000, 0.000], // 1 dose
    [0.625, 0.375, 0.250, 0.125, 0.000, 0.000], // 2 doses
    [0.583, 0.417, 0.333, 0.250, 0.167, 0.083], // 3 doses
];

fn prob_for_stews(
    boost: usize,
    total_doses: usize,
    doses_per_stew: usize,
) -> f64 {
    let total_stews = total_doses / doses_per_stew; // rounded down
    let prob_per_stew = CUMULATIVE_PROBS[doses_per_stew - 1][boost];
    math::binomial_cdf(prob_per_stew, total_stews, &mut (1..=total_stews))
}

/// Calculate probabilities related to spicy stew level boosts.
#[derive(Debug, StructOpt)]
pub struct CalcStewCommand {
    /// The MINIMUM number of levels to boost. Boosting more levels that this
    /// value WILL be included in the output probabilities.
    #[structopt(short, long)]
    boost: usize,
    /// The total number of doses you have of the relevant spice.
    #[structopt(short = "d", long = "doses")]
    total_doses: usize,
}

impl Command for CalcStewCommand {
    fn execute(&self, context: &CommandContext) -> anyhow::Result<()> {
        if self.boost > MAX_BOOST {
            return Err(OsrsError::ArgsError(format!(
                "Maximum boost is {} levels",
                MAX_BOOST
            ))
            .into());
        }

        let mut table = Table::new();
        table.set_format(
            *prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE,
        );

        table.set_titles(Row::new(
            iter::once(Cell::new_align(&"Doses/Stew", Alignment::RIGHT))
                // Add one col for each boost number (1-5)
                .chain((1..=MAX_BOOST).map(|boost| {
                    let mut cell = Cell::new_align(
                        &format!("≥+{}", boost),
                        Alignment::RIGHT,
                    );
                    if boost == self.boost {
                        cell.style(Attr::Bold);
                    }
                    cell
                }))
                .collect(),
        ));

        for doses_per_stew in 1..=MAX_DOSES {
            table.add_row(Row::new(
                iter::once(Cell::new_align(
                    &context.fmt_num(&doses_per_stew),
                    Alignment::RIGHT,
                ))
                // Calculate prob for hitting each boost value (1-5)
                .chain((1..=MAX_BOOST).map(|boost| {
                    let prob =
                        prob_for_stews(boost, self.total_doses, doses_per_stew);
                    let mut cell = Cell::new_align(
                        &format!("{:.1}%", prob * 100.0),
                        Alignment::RIGHT,
                    );

                    // Add some extra conditional styling
                    if boost == self.boost {
                        cell.style(Attr::Bold);
                        if prob > 0.0 {
                            cell.style(Attr::ForegroundColor(color::GREEN));
                        }
                    }

                    cell
                }))
                .collect(),
            ));
        }

        println!(
            "The green bolded column indicates the requested boost. \
            Use the number of doses with the highest probability in that column.
            "
        );
        table.printstd();

        Ok(())
    }
}
