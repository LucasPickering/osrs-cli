use crate::{
    commands::Command,
    config::{OsrsConfig, CONFIG_FILE_PATH},
    utils::context::CommandContext,
};
use config::Config;
use std::fs::OpenOptions;
use structopt::StructOpt;

/// Set a configuration value
#[derive(Debug, StructOpt)]
pub struct ConfigSetCommand {
    /// The key for the config field to set
    pub key: String,
    /// The new value to use for the field
    pub value: String,
}

impl Command for ConfigSetCommand {
    fn execute(&self, context: &CommandContext) -> anyhow::Result<()> {
        let current_cfg_value = context.config();
        let mut cfg = Config::try_from(current_cfg_value)?;
        cfg.set(&self.key, self.value.as_str())?;
        let new_cfg_value: OsrsConfig = cfg.try_into()?;

        // If the user didn't make any changes, then don't do anything. This
        // is mostly to prevent a success message when they put in a bogus key.
        if &new_cfg_value != current_cfg_value {
            // Write the new cfg value to the cfg file
            let file = OpenOptions::new()
                .read(false)
                .write(true)
                .create(true)
                .open(CONFIG_FILE_PATH)?;
            serde_json::to_writer_pretty(&file, &new_cfg_value)?;

            println!("Set {} = {}", self.key, self.value);
        } else {
            println!("No changes")
        }

        Ok(())
    }
}
