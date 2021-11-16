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

        // Calculate expected results for each patch
        for herb in Herb::iter() {
            // Don't show herbs that the user doesn't have the level to grow
            if herb.farming_level() <= farming_level {
                let herb_stats =
                    calc_total_patch_stats(farming_level, herb_cfg, herb)?;

                // TODO add row highlighting for best rows by profit
                table.add_row(row![
                    herb.to_string(),
                    r->herb.farming_level(),
                    r->fmt::fmt_probability(herb_stats.survival_chance),
                    r->format!("{:.3}", herb_stats.expected_yield),
                    r->format!("{:.1}", herb_stats.expected_xp),
                    r->fmt::fmt_price(herb_stats.seed_price),
                    r->fmt::fmt_price(herb_stats.grimy_herb_price),
                    r->fmt::fmt_int(&herb_stats.expected_profit),
                ]);
            }
        }

        table.printstd();
        Ok(())
    }
}

/// Calculate output statistics for *all* patches. Survival chance will be
/// average across all patches (including disease-free, which have a chance of
/// 100%), and yield/XP/profit will be totaled across all patches to provide
/// per-run expected output.
fn calc_total_patch_stats(
    farming_level: usize,
    herb_cfg: &FarmingHerbsConfig,
    herb: Herb,
) -> anyhow::Result<PatchStats> {
    let mut total_stats = PatchStats::default();

    for patch in &herb_cfg.patches {
        // Map the patch name + the modifiers for all patches into a combined
        // HerbPatch value, which picks out only the relevant modifiers. This
        // makes it easy to pass around the modifier context that we need, and
        // nothing more.
        let patch_stats =
            patch.calc_patch_stats(farming_level, herb_cfg, herb)?;

        // We aggregate survival chance here, then we'll turn it into an
        // average below
        total_stats.survival_chance += patch_stats.survival_chance;

        // Prices should be the same across all patches since they use the same
        // herb
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
