use std::iter;

use crate::{
    commands::Command,
    error::OsrsError,
    utils::{context::CommandContext, fmt, math},
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
    fn execute(&self, _context: &CommandContext) -> anyhow::Result<()> {
        if self.boost > MAX_BOOST {
            return Err(OsrsError::ArgsError(format!(
                "Maximum boost is {} levels",
                MAX_BOOST
            ))
            .into());
        }

        let probabilities = Probabilities::calculate(Doses(self.total_doses));
        // Figure out which cell the highlight for the user
        let optimal_doses_per_stew =
            probabilities.optimal_doses(Boost(self.boost));

        let mut table = Table::new();
        table.set_format(
            *prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE,
        );

        table.set_titles(Row::new(
            iter::once(Cell::new_align("Doses/Stew", Alignment::RIGHT))
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

        for (doses_per_stew, dose_probabilities) in probabilities.doses_iter() {
            table.add_row(Row::new(
                iter::once(Cell::new_align(
                    &fmt::fmt_int(&doses_per_stew.0),
                    Alignment::RIGHT,
                ))
                // Calculate prob for hitting each boost value (1-5)
                .chain(dose_probabilities.into_iter().map(|(boost, prob)| {
                    let mut cell = Cell::new_align(
                        &fmt::fmt_probability(prob),
                        Alignment::RIGHT,
                    );

                    // Bold the column for the requested boost level
                    if boost == Boost(self.boost) {
                        cell.style(Attr::Bold);
                        // Highlight the cell with the highest probability
                        if doses_per_stew == optimal_doses_per_stew {
                            cell.style(Attr::ForegroundColor(color::GREEN));
                        }
                    }

                    cell
                }))
                .collect(),
            ));
        }

        println!(
            "The bolded column indicates the requested boost. \
            The green cell is the optimal number of doses to use per stew, to \
            maximize your odds of hitting the boost.
            "
        );
        table.printstd();

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Boost(usize);

impl Boost {
    /// Convert an array index to a doses value (from 0-indexed to 1-indexed)
    fn from_index(index: usize) -> Self {
        Self(index + 1)
    }

    /// Convert this value to an array index (from 1-indexed to 0-indexed)
    fn to_index(self) -> usize {
        self.0 - 1
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Doses(usize);

impl Doses {
    /// Convert an array index to a doses value (from 0-indexed to 1-indexed)
    fn from_index(index: usize) -> Self {
        Self(index + 1)
    }

    /// Convert this value to an array index (from 1-indexed to 0-indexed)
    fn to_index(self) -> usize {
        self.0 - 1
    }
}

struct Probabilities {
    /// A table of doses:boost probabilities. Lookup by doses **then** boost.
    probabilities: [[f64; MAX_BOOST]; MAX_DOSES],
}

impl Probabilities {
    /// Build a table of probabilities for each dose/boost level
    fn calculate(total_doses: Doses) -> Self {
        let mut probabilities = [
            [0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0],
        ];

        // Use newtypes to make sure we don't mix up boosts/doses, and also
        // to differentiate 0-indexed (usize) from 1-indexed (newtypes).
        #[allow(clippy::needless_range_loop)]
        for doses_per_stew in 1..=MAX_DOSES {
            let doses_per_stew = Doses(doses_per_stew);
            for boost in 1..=MAX_BOOST {
                let boost = Boost(boost);
                probabilities[doses_per_stew.to_index()][boost.to_index()] =
                    Self::prob_for_stews(boost, total_doses, doses_per_stew)
            }
        }

        Self { probabilities }
    }

    /// Calculate the probability of hitting AT LEAST the specified boost level
    /// AT LEAST once in `n` trials, where `n` is the total number of doses we
    /// have available over the number of doses per stew.
    fn prob_for_stews(
        boost: Boost,
        total_doses: Doses,
        doses_per_stew: Doses,
    ) -> f64 {
        let total_stews = total_doses.0 / doses_per_stew.0; // rounded down

        // +1 on the boost index because CUMULATIVE_PROBS has an extra column
        // for boost=0
        let prob_per_stew =
            CUMULATIVE_PROBS[doses_per_stew.to_index()][boost.to_index() + 1];

        math::binomial_cdf(prob_per_stew, total_stews, &mut (1..=total_stews))
    }

    /// Based on a given probabilities table, calculate the optimal number of
    /// doses to use per stew to get the highest probability of hitting the
    /// requested boost level.
    fn optimal_doses(&self, target_boost: Boost) -> Doses {
        let mut best: (Doses, f64) = (Doses(1), 0.0);
        for (dose_idx, probabilities) in self.probabilities.iter().enumerate() {
            let prob = probabilities[target_boost.to_index()];
            if prob >= best.1 {
                best = (Doses::from_index(dose_idx), prob);
            }
        }
        best.0
    }

    /// Get an iterator for this table. Outer iterator is table rows
    /// (probabilities for a dose count), inner iterators are (boost,
    /// probability) pairs.
    fn doses_iter(
        &self,
    ) -> impl Iterator<Item = (Doses, Vec<(Boost, f64)>)> + '_ {
        self.probabilities.iter().enumerate().map(
            |(doses_idx, row_probabilities)| {
                (
                    Doses::from_index(doses_idx),
                    row_probabilities
                        .iter()
                        .enumerate()
                        .map(|(boost_idx, prob)| {
                            (Boost::from_index(boost_idx), *prob)
                        })
                        .collect(),
                )
            },
        )
    }
}
