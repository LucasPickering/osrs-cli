use crate::{
    commands::Command,
    config::{FarmingHerbsConfig, HerbPatch},
    error::OsrsError,
    utils::context::CommandContext,
};
use structopt::StructOpt;

impl FarmingHerbsConfig {
    fn calc_global_yield_bonus_pct(&self) -> u32 {
        let mut bonus = 0;
        if self.magic_secateurs {
            bonus += 10;
        }
        if self.farming_cape {
            bonus += 5;
        }
        bonus
    }
}

impl HerbPatch {
    fn calc_survival_chance(&self) -> f32 {
        if self.disease_free {
            1.0
        } else {
            todo!()
        }
    }

    fn calc_expected_yield(
        &self,
        farming_level: u32,
        global_cfg: &FarmingHerbsConfig,
    ) -> f32 {
        let global_bonus = global_cfg.calc_global_yield_bonus_pct();
        todo!()
    }
}

/// Calculate yield and XP from farming herbs.
#[derive(Debug, StructOpt)]
pub struct CalcFarmHerbCommand {}

#[derive(Debug)]
struct PatchStats {
    survival_chance: f32,
    expected_yield: f32,
}

impl Command for CalcFarmHerbCommand {
    fn execute(&self, context: &CommandContext) -> anyhow::Result<()> {
        let herb_cfg = &context.config().farming.herbs;

        // Make sure at least one patch is configured
        if herb_cfg.patches.is_empty() {
            return Err(OsrsError::Unconfigured {
                key: "farming.herbs.patches".into(),
                msg: "Configure your herb patches to use this calculator."
                    .into(),
            }
            .into());
        }

        let global_yield_bonus_pct = herb_cfg.calc_global_yield_bonus_pct();
        let patch_stats: Vec<PatchStats> = herb_cfg
            .patches
            .iter()
            .map(|patch| PatchStats {
                survival_chance: patch.calc_survival_chance(),
                expected_yield: patch.calc_expected_yield(
                    1, // TODO
                    global_yield_bonus_pct,
                ),
            })
            .collect();

        dbg!(patch_stats);

        Ok(())
    }
}
