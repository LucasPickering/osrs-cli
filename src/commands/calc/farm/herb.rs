use crate::{
    commands::Command,
    config::FarmingHerbsConfig,
    error::OsrsError,
    utils::{
        context::CommandContext,
        farm::{Herb, PatchStats},
        fmt,
    },
};
use prettytable::{cell, row, Table};
use structopt::StructOpt;
use strum::IntoEnumIterator;

// TODO add command for setting herb config more easily
// TODO test all edge cases against the wiki calculator

/// Calculate yield, XP, and profit related to farming herbs
#[derive(Debug, StructOpt)]
pub struct CalcFarmHerbCommand {
    /// Farming level (affects crop yield)
    #[structopt(short = "l", long = "lvl")]
    farming_level: Option<u32>,
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

        // Calculate expected results for each patch
        // TODO grab farming level from hiscores when possible
        let farming_level = self.farming_level.unwrap_or(1);

        // Print a little prelude to give the user some info
        // TODO make yield/XP values per run instead of average per patch
        println!("All values are an average across all patches. Yield values take into account survival chance.");
        println!();
        println!("Farming level: {}", farming_level);
        println!("{}", &herb_cfg);

        let mut table = Table::new();
        table.set_format(
            *prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE,
        );
        table.set_titles(row!["Herb", r->"Survival Chance", r->"Yield per Seed", r->"XP per Seed"]);

        for herb in Herb::iter() {
            let herb_stats =
                calc_average_patch_stats(farming_level, herb_cfg, herb);
            // TODO include price data here
            table.add_row(row![
                herb.to_string(),
                r->fmt::fmt_probability(herb_stats.survival_chance),
                r->format!("{:.2}", herb_stats.expected_yield),
                r->format!("{:.1}", herb_stats.expected_xp),
            ]);
        }

        table.printstd();
        Ok(())
    }
}

/// Calculate output statistics for *all* patches and average them together.
/// Most players plant the same herb in all patches, so a simple average works
/// for that case to give average yield/profit numbers.
///
/// If you really want to min/max you could plant different herbs in different
/// patches but this function ignores those weenies.
fn calc_average_patch_stats(
    farming_level: u32,
    herb_cfg: &FarmingHerbsConfig,
    herb: Herb,
) -> PatchStats {
    herb_cfg
        .patches
        .iter()
        .map(|patch| patch.calc_patch_stats(farming_level, herb_cfg, herb))
        .sum::<PatchStats>()
        / (herb_cfg.patches.len() as f64)
}
