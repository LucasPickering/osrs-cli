use crate::{
    commands::Command,
    config::FarmingHerbsConfig,
    error::OsrsError,
    utils::{
        context::CommandContext,
        farm::{Herb, PatchStats},
        fmt,
        hiscore::HiscorePlayer,
        skill::Skill,
    },
};
use anyhow::Context;
use prettytable::{cell, row, Table};
use structopt::StructOpt;
use strum::IntoEnumIterator;

// TODO add command for setting herb config more easily

/// Calculate yield, XP, and profit related to farming herbs
// Note: there are slight differences between this and the calculator on the
// wiki, I ran through all the scenarios and I think the differences are all
// either negligible or the fault of the wiki (e.g. it doesn't handle the +5%
// on the Hosidius patch).
#[derive(Debug, StructOpt)]
pub struct CalcFarmHerbCommand {
    /// Farming level (affects crop yield). If provided, this will override
    /// hiscores lookup for a player.
    #[structopt(short = "l", long = "lvl")]
    farming_level: Option<usize>,

    /// The player to pull a farming level from. If not given, will use the
    /// default player in the config.
    #[structopt(short, long)]
    player: Vec<String>,
}

impl Command for CalcFarmHerbCommand {
    fn execute(&self, context: &CommandContext) -> anyhow::Result<()> {
        let herb_cfg = &context.config().farming.herbs;

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
        let farming_level = match self {
            Self {
                farming_level: Some(farming_level),
                ..
            } => *farming_level,
            Self {
                farming_level: None,
                player,
            } => {
                // This error message isn't the best, but hopefully it gets the
                // point across
                let username = context
                    .config()
                    .get_username(player)
                    .context("Error loading farming level")?;
                let player = HiscorePlayer::load(&username)?;
                player.skill(Skill::Farming).level
            }
        };

        // Print a little prelude to give the user some info
        println!("Farming level: {}", farming_level);
        println!("{}", &herb_cfg);
        println!();
        println!("Survival chance is an average across all patches. Yield values take into account survival chance.");

        let mut table = Table::new();
        table.set_format(
            *prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE,
        );
        table.set_titles(row!["Herb", r->"Survival Chance", r->"Yield per Run", r->"XP per Run"]);

        // Calculate expected results for each patch
        for herb in Herb::iter() {
            let herb_stats =
                calc_total_patch_stats(farming_level, herb_cfg, herb);
            // TODO include price data here
            table.add_row(row![
                herb.to_string(),
                r->fmt::fmt_probability(herb_stats.survival_chance),
                r->format!("{:.3}", herb_stats.expected_yield),
                r->format!("{:.1}", herb_stats.expected_xp),
            ]);
        }

        table.printstd();
        Ok(())
    }
}

/// Calculate output statistics for *all* patches. Survival chance will be
/// average across all patches (including disease-free, which have a chance of
/// 100%), and yield/XP will be totaled across all patches to provide per-run
/// expected output.
fn calc_total_patch_stats(
    farming_level: usize,
    herb_cfg: &FarmingHerbsConfig,
    herb: Herb,
) -> PatchStats {
    let mut total_stats = herb_cfg.patches.iter().fold(
        PatchStats {
            survival_chance: 0.0,
            expected_yield: 0.0,
            expected_xp: 0.0,
        },
        |mut acc, patch| {
            let patch_stats =
                patch.calc_patch_stats(farming_level, herb_cfg, herb);
            // We aggregate survival chance here, then we'll turn it into an
            // average below
            acc.survival_chance += patch_stats.survival_chance;
            // Yield and XP should be pure totals
            acc.expected_yield += patch_stats.expected_yield;
            acc.expected_xp += patch_stats.expected_xp;
            acc
        },
    );
    // Convert survival chance from total => average
    total_stats.survival_chance /= herb_cfg.patches.len() as f64;
    total_stats
}
