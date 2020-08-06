use crate::error::{OsrsError, OsrsResult};
use config::{Config, File};
use serde::{Deserialize, Serialize};

/// The path to the file where we store configuration.
pub const CONFIG_FILE_PATH: &str = if cfg!(debug_assertions) {
    "./osrs.json"
} else {
    "~/.config/ors.json"
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OsrsConfig {
    /// For commands that take a player name, this player will be used when
    /// none is given.
    pub default_player: Option<String>,
}

impl OsrsConfig {
    pub fn load() -> OsrsResult<Self> {
        let mut s = Config::new();
        s.merge(File::with_name(CONFIG_FILE_PATH).required(false))?;
        s.try_into().map_err(OsrsError::from)
    }

    /// TODO
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
