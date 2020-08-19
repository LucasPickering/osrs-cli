use crate::error::{OsrsError, OsrsResult};
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
    /// Do you have magic secateurs? (10% yield bonus)
    pub farming_cape: bool,
    /// Do you have a bottomless bucket?
    pub bottomless_bucket: bool,
    /// Do you have an attas seed planted at the farming guild while
    /// harvesting?
    pub attas_plant: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Compost {
    Normal,
    Supercompost,
    Ultracompost,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HerbPatch {
    pub name: String,
    /// Percentage yield bonus applied to this patch. Should NOT be used for
    /// global yield bonuses.
    pub yield_bonus_pct: u32,
    pub xp_bonus_pct: u32,
    pub disease_free: bool,
}

impl OsrsConfig {
    pub fn load() -> OsrsResult<Self> {
        let mut s = Config::try_from(&OsrsConfig::default()).unwrap();
        s.merge(File::with_name(CONFIG_FILE_PATH).required(false))?;
        s.try_into().map_err(OsrsError::from)
    }

    /// Convert a (possibly empty) list of username parts into a username. If
    /// the array has at least one element, the elements will be appended
    /// together with spaces between. If not, then we'll fall back to the
    /// default player defined in the config. If that is not present either,
    /// then return an arg error.
    pub fn get_username(&self, username: &[String]) -> OsrsResult<String> {
        match (username, &self.default_player) {
            // No arg provided, empty default - error
            (&[], None) => Err(OsrsError::ArgsError("No player given".into())),
            // No arg provided, but we have a default - use the default
            (&[], Some(default_player)) => Ok(default_player.clone()),
            // Arg was provided, return that
            (&[_, ..], _) => Ok(username.join(" ")),
        }
    }
}
