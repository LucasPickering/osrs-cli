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
use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
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
    /// Get the path to the file where we store configuration
    pub fn path() -> PathBuf {
        let config_dir = if cfg!(debug_assertions) {
            // In dev mode, always use current dir so we don't pollute the fs
            ".".into()
        } else {
            // Use the OS-defined config directory, if possible. If not
            // available, fall back to the home dir, then finally current dir
            dirs::config_dir()
                .or_else(dirs::home_dir)
                .unwrap_or_else(PathBuf::new)
        };

        config_dir.join("osrs.json")
    }

    /// Load config data from the pre-defined config file path. Any missing
    /// values will be populated with defaults.
    pub fn load() -> anyhow::Result<Self> {
        let path = Self::path();
        Figment::from(Serialized::defaults(OsrsConfig::default()))
            .merge(Json::file(&path))
            .extract()
            .with_context(|| {
                format!("Error loading config from file `{}`", path.display())
            })
    }

    /// Overwrite the current config file with this value
    pub fn save(&self) -> anyhow::Result<()> {
        fn write_config(
            path: &Path,
            new_cfg_value: &OsrsConfig,
        ) -> anyhow::Result<()> {
            let file = OpenOptions::new()
                .read(false)
                .write(true)
                .create(true)
                .truncate(true) // Overwrite
                .open(path)?;
            serde_json::to_writer_pretty(&file, new_cfg_value)?;
            Ok(())
        }

        let path = Self::path();
        write_config(&path, self).with_context(|| {
            format!("Error writing config to file `{}`", path.display())
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
