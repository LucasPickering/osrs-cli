use crate::{
    commands::Command,
    config::{AnimaPlant, Compost, FarmingHerbsConfig, HerbPatch},
    error::OsrsError,
    utils::{context::CommandContext, fmt, math},
};
use derive_more::{Add, Display, Div, Sum};
use prettytable::{cell, row, Table};
use std::fmt::Display;
use structopt::StructOpt;
use strum::{EnumIter, IntoEnumIterator};

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

impl FarmingHerbsConfig {
    /// Get the number of "harvest lives" that each patch has. This is dependent
    /// solely on the type of compost used, and we assume that all patches are
    /// using the same compost, as defined in the config.
    ///
    /// See https://oldschool.runescape.wiki/w/Farming#Variable_crop_yield
    fn harvest_lives(&self) -> u32 {
        match self.compost {
            None => 3,
            Some(Compost::Normal) => 4,
            Some(Compost::Supercompost) => 5,
            Some(Compost::Ultracompost) => 6,
        }
    }

    /// Get the amount of XP gained for spreading one bucket of the configured
    /// compost type. Returns zero if the player isn't using compost
    fn compost_xp(&self) -> f64 {
        match self.compost {
            None => 0.0,
            Some(Compost::Normal) => 18.0,
            Some(Compost::Supercompost) => 26.0,
            Some(Compost::Ultracompost) => 36.0,
        }
    }

    /// Calculate the "chance to save" bonus based on **equipped items** only.
    ///
    /// See https://oldschool.runescape.wiki/w/Farming#Variable_crop_yield
    fn calc_item_chance_to_save(&self) -> f64 {
        let mut bonus = 0.0;
        if self.magic_secateurs {
            bonus += 0.1;
        }
        if self.farming_cape {
            bonus += 0.05;
        }
        bonus
    }
}

impl Display for FarmingHerbsConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Magic secateurs: {}",
            fmt::fmt_bool(self.magic_secateurs)
        )?;
        writeln!(f, "Farming cape: {}", fmt::fmt_bool(self.farming_cape))?;
        writeln!(
            f,
            "Bottomless bucket: {}",
            fmt::fmt_bool(self.bottomless_bucket)
        )?;
        writeln!(f, "Compost: {}", fmt::fmt_option(self.compost))?;
        writeln!(f, "Anima plant: {}", fmt::fmt_option(self.anima_plant))?;
        // TODO include brief info on each patch (yield+XP bonus and disease
        // free)
        writeln!(
            f,
            "Patches: {}",
            self.patches
                .iter()
                .map(|patch| patch.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        Ok(())
    }
}

impl HerbPatch {
    /// The odds that an herb growing in this patch survives from seed to
    /// adulthood, assuming it is not monitored at all
    fn calc_survival_chance(&self, herb_cfg: &FarmingHerbsConfig) -> f64 {
        // https://oldschool.runescape.wiki/w/Disease_(Farming)#Reducing_disease_risk
        let disease_chance_per_cycle = if self.disease_free {
            0.0
        } else {
            // Base chance is based on compost
            let base_chance = match herb_cfg.compost {
                None => 27.0 / 128.0,
                Some(Compost::Normal) => 14.0 / 128.0,
                Some(Compost::Supercompost) => 6.0 / 128.0,
                Some(Compost::Ultracompost) => 3.0 / 128.0,
            };

            // Iasor reduces chance by 80%
            let modifier = match herb_cfg.anima_plant {
                Some(AnimaPlant::Iasor) => 0.2,
                _ => 1.0,
            };

            // Rate can't be lower than 1/128
            f64::max(base_chance * modifier, 1.0 / 128.0)
        };

        // All herbs have 4 growth cycles, and we want to find the chance of
        // exactly 0 disease instances in 3 (n-1) trials. We use n-1 because
        // the last growth cycle can't have disease
        // https://oldschool.runescape.wiki/w/Seeds#Herb_seeds
        math::binomial(disease_chance_per_cycle, 3, 0)
    }

    /// Calculate the chance to "save a life" when picking an herb. This is
    /// variable based on the herb, farming level, and applicable yield bonuses.
    ///
    /// See https://oldschool.runescape.wiki/w/Farming#Variable_crop_yield
    fn calc_chance_to_save(
        &self,
        herb_cfg: &FarmingHerbsConfig,
        farming_level: u32,
        herb: Herb,
    ) -> f64 {
        let item_bonus = herb_cfg.calc_item_chance_to_save();
        let diary_bonus = self.yield_bonus_pct as f64 / 100.0;
        let attas_bonus = match herb_cfg.anima_plant {
            Some(AnimaPlant::Attas) => 0.05,
            _ => 0.0,
        };

        let (chance1, chance99) = herb.chance_to_save();

        // This comes straight from the wiki, it's a lot easier to read in
        // their formatting. The formatted formula doesn't mention anything
        // about the `floor` though, but it's in the calculator source
        // https://oldschool.runescape.wiki/w/Calculator:Template/Farming/Herbs2?action=edit
        (f64::floor((chance1 * (99.0 - farming_level as f64) / 98.0)
            + (chance99 * (farming_level as f64 - 1.0) / 98.0))
                * (1.0 + item_bonus)
                * (1.0 + diary_bonus)
                // Attas doesn't appear in the formula on the page above, but
                // it's also in the calculator source (see link above)
                * (1.0 + attas_bonus)
            + 1.0)
            / 256.0
    }

    /// Calculate the expected yield of this patch, **assuming it is fully
    /// grown**. I.e., this **doesn't** take into account the chance of the
    /// patch dying before adulthood.
    ///
    /// See https://oldschool.runescape.wiki/w/Farming#Variable_crop_yield
    fn calc_expected_yield(
        &self,
        herb_cfg: &FarmingHerbsConfig,
        farming_level: u32,
        herb: Herb,
    ) -> f64 {
        herb_cfg.harvest_lives() as f64
            / (1.0 - self.calc_chance_to_save(herb_cfg, farming_level, herb))
    }

    /// Calculate stats (survival chance, yield, etc.) for this patch given
    /// some info on the player/herb. Yield and XP values here **do** take into
    /// account the survival chance.
    fn calc_patch_stats(
        &self,
        farming_level: u32,
        herb_cfg: &FarmingHerbsConfig,
        herb: Herb,
    ) -> PatchStats {
        let survival_chance = self.calc_survival_chance(herb_cfg);
        // IMPORTANT: We multiply by survival chance here to account for lost
        // patches.
        let expected_yield =
            self.calc_expected_yield(herb_cfg, farming_level, herb)
                * survival_chance;
        let expected_xp = herb_cfg.compost_xp()
            + herb.xp_per_plant()
            + herb.xp_per_harvest() * expected_yield;

        PatchStats {
            survival_chance,
            expected_yield,
            expected_xp,
        }
    }
}

/// Statistics on a particular herb+patch combo.
#[derive(Copy, Clone, Debug, Add, Div, Sum)]
struct PatchStats {
    /// The chance of a patch getting to fully growth, i.e. the opposite of the
    /// chance of it dying of disease.
    survival_chance: f64,
    /// Expected yield from a patch, **factoring in the survival chance**.
    /// E.g. if survival chance is 50% and we expected to get 6 herbs per
    /// **fully grown** patch, then expected yield will be 3.0.
    expected_yield: f64,
    /// Expected amount of XP gained from planting **and** harvesting it,
    /// **including** the XP for applying compost.
    expected_xp: f64,
}

/// The different types of herbs that a player can grow in an herb patch
#[derive(Copy, Clone, Debug, Display, EnumIter)]
enum Herb {
    #[display(fmt = "Guam leaf")]
    Guam,
    Marrentill,
    Tarromin,
    Harralander,
    Goutweed,
    #[display(fmt = "Ranarr weed")]
    Ranarr,
    Toadflax,
    #[display(fmt = "Irit leaf")]
    Irit,
    Avantoe,
    Kwuarm,
    Snapdragon,
    Cadantine,
    Lantadyma,
    #[display(fmt = "Dwarf weed")]
    Dwarf,
    Torstol,
}

impl Herb {
    /// Get the "chance to save" for an herb at level 1 and level 99. All other
    /// level's values can be linearly interpolated from there.
    ///
    /// See https://oldschool.runescape.wiki/w/Farming#Variable_crop_yield
    fn chance_to_save(self) -> (f64, f64) {
        // Values are ripped from https://oldschool.runescape.wiki/w/Calculator:Farming/Herbs/Template
        match self {
            Self::Guam => (25.0, 80.0),
            Self::Marrentill => (28.0, 80.0),
            Self::Tarromin => (31.0, 80.0),
            Self::Harralander => (36.0, 80.0),
            Self::Goutweed => (39.0, 80.0),
            Self::Ranarr => (39.0, 80.0),
            Self::Toadflax => (43.0, 80.0),
            Self::Irit => (46.0, 80.0),
            Self::Avantoe => (50.0, 80.0),
            Self::Kwuarm => (54.0, 80.0),
            Self::Snapdragon => (57.0, 80.0),
            Self::Cadantine => (60.0, 80.0),
            Self::Lantadyma => (64.0, 80.0),
            Self::Dwarf => (67.0, 80.0),
            Self::Torstol => (71.0, 80.0),
        }
    }

    /// The amount of Farming XP gained for *planting* one seed of this herb
    fn xp_per_plant(self) -> f64 {
        match self {
            Self::Guam => 11.0,
            Self::Marrentill => 13.5,
            Self::Tarromin => 16.0,
            Self::Harralander => 21.5,
            Self::Goutweed => 105.0,
            Self::Ranarr => 27.0,
            Self::Toadflax => 34.0,
            Self::Irit => 43.0,
            Self::Avantoe => 54.5,
            Self::Kwuarm => 69.0,
            Self::Snapdragon => 87.5,
            Self::Cadantine => 106.5,
            Self::Lantadyma => 134.5,
            Self::Dwarf => 170.5,
            Self::Torstol => 199.5,
        }
    }

    /// The amount of Farming XP gained for *harvesting* one herb
    fn xp_per_harvest(self) -> f64 {
        match self {
            Self::Guam => 12.5,
            Self::Marrentill => 15.0,
            Self::Tarromin => 18.0,
            Self::Harralander => 24.0,
            Self::Goutweed => 45.0,
            Self::Ranarr => 30.5,
            Self::Toadflax => 38.5,
            Self::Irit => 48.5,
            Self::Avantoe => 61.5,
            Self::Kwuarm => 78.0,
            Self::Snapdragon => 98.5,
            Self::Cadantine => 120.0,
            Self::Lantadyma => 151.5,
            Self::Dwarf => 192.0,
            Self::Torstol => 224.5,
        }
    }
}
