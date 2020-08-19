use crate::error::OsrsError;
use anyhow::Context;
use config::{Config, File};
use serde::{Deserialize, Serialize};

/// The path to the file where we store configuration.
pub const CONFIG_FILE_PATH: &str = if cfg!(debug_assertions) {
    "./osrs.json"
} else {
    "~/.config/osrs.json"
};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OsrsConfig {
    /// For commands that take a player name, this player will be used when
    /// none is given.
    pub default_player: Option<String>,
    pub farming: FarmingConfig,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FarmingConfig {
    pub herbs: FarmingHerbsConfig,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FarmingHerbsConfig {
    /// The list of herb patches being farmed
    pub patches: Vec<HerbPatch>,
    /// The type of compost being used
    pub compost: Option<Compost>,
    /// Do you have magic secateurs equipped? (10% yield bonus)
    pub magic_secateurs: bool,
    /// Do you have a farming cape equipped? (5% yield bonus)
    pub farming_cape: bool,
    /// Do you have a bottomless bucket?
    pub bottomless_bucket: bool,
    /// Do you have an attas seed planted at the farming guild while
    /// harvesting? (5% chance to save a life on all patches)
    pub attas_plant: bool,
}

/// Different types of compost that can be applied to a farming patch
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Compost {
    Normal,
    Supercompost,
    Ultracompost,
}

/// An herb farming patch. Different patches can have different attributes
/// based on the user's unlocks.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HerbPatch {
    pub name: String,
    /// Percentage yield bonus applied to this patch. Should NOT be used for
    /// global yield bonuses.
    pub yield_bonus_pct: u32,
    /// Percentage XP bonus applied to this patch.
    pub xp_bonus_pct: u32,
    /// Is this patch guaranteed to be disease-free?
    pub disease_free: bool,
}

impl OsrsConfig {
    pub fn load() -> anyhow::Result<Self> {
        let mut s = Config::try_from(&OsrsConfig::default()).unwrap();
        s.merge(File::with_name(CONFIG_FILE_PATH).required(false))?;
        Ok(s.try_into().with_context(|| "Error loading config")?)
    }

    /// Convert a (possibly empty) list of username parts into a username. If
    /// the array has at least one element, the elements will be appended
    /// together with spaces between. If not, then we'll fall back to the
    /// default player defined in the config. If that is not present either,
    /// then return an arg error.
    pub fn get_username(&self, username: &[String]) -> anyhow::Result<String> {
        match (username, &self.default_player) {
            // No arg provided, empty default - error
            (&[], None) => {
                Err(OsrsError::ArgsError("No player given".into()).into())
            }
            // No arg provided, but we have a default - use the default
            (&[], Some(default_player)) => Ok(default_player.clone()),
            // Arg was provided, return that
            (&[_, ..], _) => Ok(username.join(" ")),
        }
    }
}
