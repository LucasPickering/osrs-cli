use crate::{
    commands::Command,
    config::FarmingHerbsConfig,
    error::OsrsError,
    utils::{
        context::CommandContext,
        farm::{Herb, PatchStats},
        fmt, hiscore,
        magic::Spell,
        skill::Skill,
    },
};
use anyhow::Context;
use prettytable::{cell, color, row, Attr, Row, Table};
use structopt::StructOpt;
use strum::IntoEnumIterator;

/// Calculate yield, XP, and profit related to farming herbs. Configure your
/// herb patches, farming buffs, etc. with the `config set-herb` subcommand,
/// then use this to view statistics on individual herb types.
// Note: there are slight differences between this and the calculator on the
// wiki, I ran through all the scenarios and I think the differences are all
// either negligible or the fault of the wiki (e.g. it doesn't handle the +5%
// on the Hosidius patch).
#[derive(Debug, StructOpt)]
pub struct CalcFarmHerbCommand {
    /// Farming level (affects crop yield). If provided, this will override
    /// hiscores lookup for a player.
    #[structopt(short = "l", long = "lvl", alias = "level", alias = "farm")]
    farming_level: Option<usize>,

    /// Magic level (affects Resurrect Crops success rate). If provided, this
    /// will override hiscores lookup for a player.
    #[structopt(long = "magic-lvl", alias = "magic")]
    magic_level: Option<usize>,

    /// The player to pull levels from. If not given, will use the default
    /// player in the config.
    #[structopt(short, long)]
    player: Vec<String>,
}

impl Command for CalcFarmHerbCommand {
    fn execute(&self, context: &CommandContext) -> anyhow::Result<()> {
        let cfg = context.config();
        let herb_cfg = &cfg.farming.herbs;

        // Make sure at least one patch is configured
        if herb_cfg.patches.is_empty() {
            return Err(OsrsError::Unconfigured {
                key: "farming.herbs.patches".into(),
                message: "Configure your herb patches to use this calculator."
                    .into(),
            }
            .into());
        }

        // Look for farming level, in this order:
        // 1. --lvl param
        // 2. --player param
        // 3. Default player in config
        // 4. Freak out
        let farming_level = hiscore::get_level_from_args(
            cfg,
            Skill::Farming,
            &self.player,
            self.farming_level,
        )
        .context("Error getting farming level")?;

        // If the user wants to use Resurrect Crops, grab their magic level.
        // This will error out if we can't get a level, so we only do it when
        // actually needed
        let magic_level = if herb_cfg.resurrect_crops {
            let level =
                // We use the same logic as grabbing farming level
                hiscore::get_level_from_args(
                    cfg,
                    Skill::Magic,
                    &self.player,
                    self.magic_level,
                )
                .context("Error getting magic level")?;

            // Make sure the player can actually cast the spell
            let required_level = Spell::ResurrectCrops.level();
            if level < required_level {
                return Err(OsrsError::InvalidConfig(format!(
                    "Resurrect Crops requires level {}, \
                        but player has level {}",
                    required_level, level
                ))
                .into());
            }

            Some(level)
        } else {
            None
        };

        // Print a little prelude to give the user some info
        println!("Farming level: {}", farming_level);
        // Only print magic level if it's being used
        if let Some(magic_level) = magic_level {
            println!("Magic level: {}", magic_level);
        }
        println!("{}", &herb_cfg);
        println!();
        println!(
            "Survival chance is an average across all patches.\
                Yield values take into account survival chance."
        );

        // Calculate stats for each herb
        let herb_stats: Vec<(Herb, PatchStats)> = Herb::iter()
            // Don't show herbs that the player can't grow
            .filter(|herb| herb.farming_level() <= farming_level)
            .map(|herb| {
                Ok((
                    herb,
                    self.calc_total_patch_stats(
                        farming_level,
                        magic_level,
                        herb_cfg,
                        herb,
                    )?,
                ))
            })
            .collect::<anyhow::Result<_>>()?;

        let table = self.build_table(herb_stats);
        table.printstd();

        Ok(())
    }
}

impl CalcFarmHerbCommand {
    /// Calculate output statistics for *all* patches. Survival chance will be
    /// average across all patches (including disease-free, which have a chance
    /// of 100%), and yield/XP/profit will be totaled across all patches to
    /// provide per-run expected output.
    fn calc_total_patch_stats(
        &self,
        farming_level: usize,
        magic_level: Option<usize>,
        herb_cfg: &FarmingHerbsConfig,
        herb: Herb,
    ) -> anyhow::Result<PatchStats> {
        let mut total_stats = PatchStats::default();

        for patch in &herb_cfg.patches {
            // Map the patch name + the modifiers for all patches into a
            // combined HerbPatch value, which picks out only the
            // relevant modifiers. This makes it easy to pass around
            // the modifier context that we need, and nothing more.
            let patch_stats = patch.calc_patch_stats(
                farming_level,
                magic_level,
                herb_cfg,
                herb,
            )?;

            // We aggregate survival chance here, then we'll turn it into an
            // average below
            total_stats.survival_chance += patch_stats.survival_chance;

            // Prices should be the same across all patches since they use the
            // same herb
            total_stats.seed_price = patch_stats.seed_price;
            total_stats.grimy_herb_price = patch_stats.grimy_herb_price;

            // Yield and XP should be pure totals
            total_stats.expected_yield += patch_stats.expected_yield;
            total_stats.expected_xp += patch_stats.expected_xp;
            total_stats.expected_profit += patch_stats.expected_profit;
        }

        // Convert survival chance from total => average
        total_stats.survival_chance /= herb_cfg.patches.len() as f64;
        Ok(total_stats)
    }

    /// Build a nice formatted table with all the output we want. Input is a
    /// list of herbs (in the order to be displayed), each one with its
    /// associated XP/GP stats.
    fn build_table(&self, herb_stats: Vec<(Herb, PatchStats)>) -> Table {
        let mut table = Table::new();
        table.set_format(
            *prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE,
        );
        table.set_titles(row![
            "Herb",
            r->"Lvl",
            r->"Surv%",
            r->"Yield/Run",
            r->"XP/Run",
            r->"Seed$",
            r->"Herb$",
            r->"Profit/Run",
        ]);

        if herb_stats.is_empty() {
            return table;
        }

        // Find the highest profit value. We'll use this to style each row
        // according to its relative profitability
        let max_profitability = herb_stats
            .iter()
            .map(|(_, stats)| stats.expected_profit)
            .max()
            .unwrap() as f64; // we know the list isn't empty at this point

        // Calculate expected results for each patch
        for (herb, stats) in herb_stats {
            let mut cells = vec![
                cell!(herb.to_string()),
                // Numeric columns are all right-aligned
                cell!(r->herb.farming_level()),
                cell!(r->fmt::fmt_probability(stats.survival_chance)),
                cell!(r->format!("{:.3}", stats.expected_yield)),
                cell!(r->format!("{:.1}", stats.expected_xp)),
                cell!(r->fmt::fmt_price(stats.seed_price)),
                cell!(r->fmt::fmt_price(stats.grimy_herb_price)),
                cell!(r->fmt::fmt_int(&stats.expected_profit)),
            ];

            // Style each row according to profitability
            for cell in &mut cells {
                if stats.expected_profit >= 0 {
                    cell.style(Attr::ForegroundColor(color::BRIGHT_GREEN));

                    let profitability_factor =
                        stats.expected_profit as f64 / max_profitability;
                    // The most profitable should be very obvious
                    if (profitability_factor - 1.0).abs() < f64::EPSILON {
                        cell.style(Attr::Underline(true));
                    }

                    // Anything close in profitablility should also be
                    // highlighted
                    if profitability_factor >= 0.8 {
                        cell.style(Attr::ForegroundColor(color::GREEN));
                    }
                } else {
                    cell.style(Attr::ForegroundColor(color::BRIGHT_RED));
                };
            }

            table.add_row(Row::new(cells));
        }

        table
    }
}
