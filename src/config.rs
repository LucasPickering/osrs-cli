use crate::error::OsrsError;
use config::{Config, File};
use serde::{Deserialize, Serialize};

/// The path to the file where we store configuration.
pub const CONFIG_FILE_PATH: &str = if cfg!(debug_assertions) {
    "./osrs.json"
} else {
    "~/.config/ors.json"
};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct OsrsConfig {
    /// For commands that take a player name, this player will be used when
    /// none is given.
    pub default_player: Option<String>,
}

impl OsrsConfig {
    pub fn load() -> anyhow::Result<Self> {
        let mut s = Config::try_from(&OsrsConfig::default()).unwrap();
        s.merge(File::with_name(CONFIG_FILE_PATH).required(false))?;
        Ok(s.try_into()?)
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
