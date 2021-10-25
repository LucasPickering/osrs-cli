use crate::{
    error::OsrsError,
    utils::farm::{AnimaPlant, Compost, HerbPatch},
};
use anyhow::Context;
use figment::{
    providers::{Format, Json, Serialized},
    Figment,
};
use serde::{Deserialize, Serialize};

/// The path to the file where we store configuration.
pub const CONFIG_FILE_PATH: &str = if cfg!(debug_assertions) {
    "./osrs.json"
} else {
    "~/.config/osrs.json"
};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OsrsConfig {
    /// For commands that take a player name, this player will be used when
    /// none is given.
    pub default_player: Option<String>,
    pub farming: FarmingConfig,
}

/// Configuration relatd to a player's farm patches
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FarmingConfig {
    pub herbs: FarmingHerbsConfig,
}

/// Configuration related to a player's herb patches
///
/// Impls for this type live in [crate::utils::farm].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FarmingHerbsConfig {
    /// The list of herb patches being farmed
    pub patches: Vec<HerbPatch>,
    /// The type of compost being used
    pub compost: Option<Compost>,
    /// Do you have magic secateurs equipped? (10% yield bonus)
    pub magic_secateurs: bool,
    /// Do you have a farming cape equipped? (5% yield bonus)
    pub farming_cape: bool,
    /// Do you have a bottomless bucket? Affects cost of compost per patch
    pub bottomless_bucket: bool,
    /// The type of Anima plant currently alive at the Farming Guild (can
    /// affect disease and yield rates)
    pub anima_plant: Option<AnimaPlant>,
}

impl OsrsConfig {
    /// Load config data from the pre-defined config file path. Any missing
    /// values will be populated with defaults.
    pub fn load() -> anyhow::Result<Self> {
        Figment::from(Serialized::defaults(OsrsConfig::default()))
            .join(Json::file(CONFIG_FILE_PATH))
            .extract()
            .with_context(|| {
                format!("Error loading config from file `{}`", CONFIG_FILE_PATH)
            })
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
