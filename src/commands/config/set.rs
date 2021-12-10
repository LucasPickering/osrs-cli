use crate::{
    commands::Command, config::OsrsConfig, utils::context::CommandContext,
};
use figment::{providers::Serialized, Figment};
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
        // Update the given field in the config
        let current_cfg_value = context.config();
        let new_cfg_value: OsrsConfig =
            Figment::from(Serialized::defaults(current_cfg_value))
                .merge((&self.key, self.value.as_str()))
                .extract()?;

        // If the user didn't make any changes, then don't do anything. This
        // is mostly to prevent a success message when they put in a bogus key.
        if &new_cfg_value != current_cfg_value {
            new_cfg_value.save()?;
            println!("Set {} = {}", self.key, self.value);
        } else {
            println!(
                "No changes. \
                Try `osrs config get` to see available keys & current settings."
            )
        }

        Ok(())
    }
}
