use crate::{
    commands::Command, error::OsrsResult, utils::context::CommandContext,
};
use structopt::StructOpt;

/// Get the current value of all config options.
#[derive(Debug, StructOpt)]
pub struct ConfigGetCommand {}

impl Command for ConfigGetCommand {
    fn execute(&self, context: &CommandContext) -> OsrsResult<()> {
        // This serialization shouldn't ever fail, so if it does we want to
        // panic
        println!(
            "{}",
            serde_json::to_string_pretty(context.config()).unwrap()
        );
        Ok(())
    }
}
