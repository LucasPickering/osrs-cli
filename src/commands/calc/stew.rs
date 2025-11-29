use std::{io::Write, iter};

use crate::{
    commands::Command,
    error::OsrsError,
    utils::{context::CommandContext, fmt, math, table::TableExt},
};
use async_trait::async_trait;
use comfy_table::{presets, Cell, CellAlignment, Row, Table};
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

#[async_trait(?Send)]
impl<O: Write> Command<O> for CalcStewCommand {
    async fn execute(
        &self,
        mut context: CommandContext<O>,
    ) -> anyhow::Result<()>
    where
        O: 'async_trait,
    {
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
        table
            .load_preset(presets::ASCII_BORDERS_ONLY_CONDENSED)
            .set_aligned_header(
                iter::once((Cell::new("Doses/Stew"), CellAlignment::Right))
                    // Add one col for each boost number (1-5)
                    .chain((1..=MAX_BOOST).map(|boost| {
                        let cell = style_cell(
                            Cell::new(format!("≥+{}", boost))
                                .set_alignment(CellAlignment::Right),
                            boost == self.boost,
                            false,
                        );
                        (cell, CellAlignment::Right)
                    })),
            );

        for (doses_per_stew, dose_probabilities) in probabilities.doses_iter() {
            table.add_row(Row::from(
                iter::once(Cell::new(fmt::fmt_int(&doses_per_stew.0)))
                    // Calculate prob for hitting each boost value (1-5)
                    .chain(dose_probabilities.into_iter().map(
                        |(boost, prob)| {
                            let boost_matches = boost == Boost(self.boost);
                            style_cell(
                                Cell::new(fmt::fmt_probability(prob)),
                                // Bold the column of the requested boost level
                                boost_matches,
                                // Highlight cell with highest probability
                                boost_matches
                                    && doses_per_stew == optimal_doses_per_stew,
                            )
                        },
                    )),
            ));
        }

        // Styling doesn 't work on wasm so this caption is pointless
        if cfg!(not(wasm)) {
            context.println(
                "The bolded column indicates the requested boost. \
                The green cell is the optimal number of doses to use per stew, to \
                maximize your odds of hitting the boost.
                ",
            )?;
        }
        context.print_table(&table)?;

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

/// Apply ANSI styling to a cell. This needs to be a separate function so
/// its functionality can vary for wasm vs native (since ANSI terminal
/// stuff isn't supported in wasm)
#[cfg(not(wasm))]
fn style_cell(mut cell: Cell, bold: bool, color: bool) -> Cell {
    use comfy_table::{Attribute, Color};

    if bold {
        cell = cell.add_attribute(Attribute::Bold);
    }
    if color {
        cell = cell.fg(Color::Green);
    }
    cell
}

/// Placehold to match the native call signature
#[cfg(wasm)]
fn style_cell(cell: Cell, _bold: bool, _color: bool) -> Cell {
    cell
}
