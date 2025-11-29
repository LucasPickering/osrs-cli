use anyhow::Context;
use figment::{
    providers::{Format, Json, Serialized},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OsrsConfig {
    /// For commands that take a player name, this player will be used when
    /// none is given.
    pub default_player: Option<String>,
}

impl OsrsConfig {
    /// Create a base Figment instance that populates the default values.
    /// Further values should be chained on by the caller
    fn figment() -> Figment {
        Figment::from(Serialized::defaults(OsrsConfig::default()))
    }
}

// Native implementation, which stores the config on the file system
#[cfg(not(wasm))]
mod native {
    use super::*;
    use std::{
        fs::OpenOptions,
        path::{Path, PathBuf},
    };

    impl OsrsConfig {
        /// Load config data from the pre-defined config file path. Any missing
        /// values will be populated with defaults.
        pub fn load() -> anyhow::Result<Self> {
            let path = Self::path();
            Self::figment()
                .merge(Json::file(&path))
                .extract()
                .with_context(|| {
                    format!(
                        "Error loading config from file `{}`",
                        path.display()
                    )
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

        /// Get the path to the file where we store configuration
        fn path() -> PathBuf {
            let config_dir = if cfg!(debug_assertions) {
                // In dev mode, always use current dir so we don't pollute the
                // fs
                ".".into()
            } else {
                // Use the OS-defined config directory, if possible. If not
                // available, fall back to the home dir, then finally current
                // dir
                dirs::config_dir()
                    .or_else(dirs::home_dir)
                    .unwrap_or_default()
            };

            config_dir.join("osrs.json")
        }
    }
}

// Wasm implementation, which stores the config in browser local storage
#[cfg(wasm)]
mod wasm {
    use super::*;
    use crate::utils::browser::LocalStorage;

    impl OsrsConfig {
        const STORAGE_KEY: &'static str = "config";

        /// Load config data from browser local storage. Any missing values will
        /// be populated with defaults.
        pub fn load() -> anyhow::Result<Self> {
            let storage = LocalStorage::new()?;
            // If nothing is stored, fall back to empty data
            let stored_data = storage
                .get(Self::STORAGE_KEY)?
                .unwrap_or_else(|| "{}".into());

            Self::figment()
                .merge(Json::string(&stored_data))
                .extract()
                .with_context(|| {
                    format!(
                        "Error loading config from local storage key `{}`. Contents: {}",
                        Self::STORAGE_KEY,
                        stored_data
                    )
                })
        }

        /// Overwrite the current config value in local storage with this
        /// config.
        pub fn save(&self) -> anyhow::Result<()> {
            let storage = LocalStorage::new()?;
            let config = serde_json::to_string(self)?;
            storage.set(Self::STORAGE_KEY, &config).with_context(|| {
                format!(
                    "Error writing config to local storage key `{}`",
                    Self::STORAGE_KEY
                )
            })
        }
    }
}
