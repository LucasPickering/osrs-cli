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
}
