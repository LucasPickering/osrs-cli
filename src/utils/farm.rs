//! Utility types and functions related to farming.

use crate::{
    config::FarmingHerbsConfig,
    utils::{diary::AchievementDiaryLevel, fmt, math},
};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{EnumIter, IntoStaticStr};

/// Different types of compost that can be applied to a farming patch
#[derive(Copy, Clone, Debug, Display, PartialEq, Serialize, Deserialize)]
pub enum Compost {
    Normal,
    Supercompost,
    Ultracompost,
}

/// A type of plant that has global impact on how other crops grow
/// https://oldschool.runescape.wiki/w/Anima_seed
#[derive(Copy, Clone, Debug, Display, PartialEq, Serialize, Deserialize)]
pub enum AnimaPlant {
    /// https://oldschool.runescape.wiki/w/Kronos_seed
    Kronos,
    /// Increases yield https://oldschool.runescape.wiki/w/Attas_seed
    Attas,
    /// Lowers disease chance https://oldschool.runescape.wiki/w/Iasor_seed
    Iasor,
}

/// The different types of herbs that a player can grow in an herb patch
#[derive(Copy, Clone, Debug, Display, EnumIter)]
pub enum Herb {
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
    pub fn chance_to_save(self) -> (f64, f64) {
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
    pub fn xp_per_plant(self) -> f64 {
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
    pub fn xp_per_harvest(self) -> f64 {
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

/// An herb farming patch. Each patch has unique fields that encompass the
/// different buffs that are applicable.
#[derive(
    Copy, Clone, Debug, PartialEq, IntoStaticStr, Serialize, Deserialize,
)]
#[serde(tag = "name")]
pub enum HerbPatch {
    Ardougne,
    Catherby {
        /// Achievement diary gives a yield buff via increased chance to save a
        /// harvest life.
        /// Medium => +5%
        /// Hard => +10%
        /// Elite => +15%
        kandarin_diary: Option<AchievementDiaryLevel>,
    },
    Falador {
        /// Medium => +10% XP
        falador_diary: Option<AchievementDiaryLevel>,
    },
    FarmingGuild {
        /// Hard => +5% chance to save a harvest life
        kourend_diary: Option<AchievementDiaryLevel>,
    },
    HarmonyIsland,
    Hosidius {
        /// 50%+ Hosidius => disease-free
        fifty_favor: bool,
        /// Hard => +5% chance to save a harvest life
        kourend_diary: Option<AchievementDiaryLevel>,
    },
    PortPhasmatys,
    TrollStronghold,
    Weiss,
}

impl HerbPatch {
    /// Is this patch 100% certified disease-free?
    pub fn disease_free(self) -> bool {
        matches!(
            self,
            Self::TrollStronghold
                | Self::Weiss
                | Self::Hosidius {
                    fifty_favor: true,
                    ..
                }
        )
    }

    /// Calculate the "chance to save a life" that this patch provides. This
    /// will stack with other bonuses (magic secateurs, etc.).
    ///
    /// See https://oldschool.runescape.wiki/w/Farming#Variable_crop_yield
    pub fn chance_to_save_bonus(self) -> f64 {
        match self {
            // Bonus scales based on tiers completed
            Self::Catherby {
                kandarin_diary: Some(diary),
            } => match diary {
                AchievementDiaryLevel::Easy => 0.0,
                AchievementDiaryLevel::Medium => 0.05,
                AchievementDiaryLevel::Hard => 0.10,
                AchievementDiaryLevel::Elite => 0.15,
            },

            // Both get +5% from Kourend medium
            Self::FarmingGuild {
                kourend_diary: Some(diary),
            }
            | Self::Hosidius {
                kourend_diary: Some(diary),
                ..
            } if diary >= AchievementDiaryLevel::Medium => 0.05,

            _ => 0.0,
        }
    }

    /// Get the XP bonus that this patch provides for **all actions** performed
    /// on the patch.
    pub fn xp_bonus(self) -> f64 {
        match self {
            // Falador Medium grants a +10% XP bonus
            Self::Falador {
                falador_diary: Some(diary),
            } if diary >= AchievementDiaryLevel::Medium => 0.10,
            _ => 0.0,
        }
    }

    /// Calculate stats (survival chance, yield, etc.) for this patch given
    /// some info on the player/herb. Yield and XP values here **do** take into
    /// account the survival chance.
    pub fn calc_patch_stats(
        self,
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

    /// The odds that an herb growing in this patch survives from seed to
    /// adulthood, assuming it is not monitored at all.
    fn calc_survival_chance(self, herb_cfg: &FarmingHerbsConfig) -> f64 {
        // https://oldschool.runescape.wiki/w/Disease_(Farming)#Reducing_disease_risk
        let disease_chance_per_cycle = if self.disease_free() {
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
        self,
        herb_cfg: &FarmingHerbsConfig,
        farming_level: u32,
        herb: Herb,
    ) -> f64 {
        let item_bonus = herb_cfg.calc_item_chance_to_save();
        let diary_bonus = self.chance_to_save_bonus() as f64 / 100.0;
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
        self,
        herb_cfg: &FarmingHerbsConfig,
        farming_level: u32,
        herb: Herb,
    ) -> f64 {
        herb_cfg.harvest_lives() as f64
            / (1.0 - self.calc_chance_to_save(herb_cfg, farming_level, herb))
    }
}

impl Display for HerbPatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(self))?;

        let disease_free = self.disease_free();
        let chance_to_save_bonus = self.chance_to_save_bonus();
        let xp_bonus = self.xp_bonus();

        let mut modifiers = Vec::new();
        if disease_free {
            modifiers.push("disease-free".to_owned());
        }
        if chance_to_save_bonus > 0.0 {
            modifiers
                .push(format!("{:+}% yield", chance_to_save_bonus * 100.0));
        }
        if xp_bonus > 0.0 {
            modifiers.push(format!("{:+}% XP", xp_bonus * 100.0));
        }

        if !modifiers.is_empty() {
            write!(f, " ({})", modifiers.join(", "))?;
        }

        Ok(())
    }
}

/// Statistics on a particular herb+patch combo.
#[derive(Copy, Clone, Debug)]
pub struct PatchStats {
    /// The chance of a patch getting to fully growth, i.e. the opposite of the
    /// chance of it dying of disease.
    pub survival_chance: f64,
    /// Expected yield from a patch, **factoring in the survival chance**.
    /// E.g. if survival chance is 50% and we expected to get 6 herbs per
    /// **fully grown** patch, then expected yield will be 3.0.
    pub expected_yield: f64,
    /// Expected amount of XP gained from planting **and** harvesting it,
    /// **including** the XP for applying compost.
    pub expected_xp: f64,
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
        // Last line should be just a `write!` so we don't have a dangling
        // newline at the end
        write!(
            f,
            "Patches: {}",
            self.patches
                .iter()
                .map(HerbPatch::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        Ok(())
    }
}
