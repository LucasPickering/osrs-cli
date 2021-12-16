use crate::{commands::Command, utils::context::CommandContext};
use async_trait::async_trait;
use std::io::Write;
use structopt::StructOpt;

/// Get the current configuration values.
#[derive(Debug, StructOpt)]
pub struct ConfigGetCommand {}

#[async_trait(?Send)]
impl<O: Write> Command<O> for ConfigGetCommand {
    async fn execute(
        &self,
        mut context: CommandContext<O>,
    ) -> anyhow::Result<()>
    where
        O: 'async_trait,
    {
        // This serialization shouldn't ever fail, so if it does we want to
        // panic
        context.println(
            &serde_json::to_string_pretty(context.config()).unwrap(),
        )?;
        Ok(())
    }
}
