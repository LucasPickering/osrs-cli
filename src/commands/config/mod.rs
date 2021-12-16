use crate::{
    commands::{
        config::{get::ConfigGetCommand, set::ConfigSetCommand},
        Command, CommandType,
    },
    utils::context::CommandContext,
};
use async_trait::async_trait;
use std::io::Write;
use structopt::StructOpt;

mod get;
mod set;

#[derive(Debug, StructOpt)]
pub enum ConfigCommandType {
    Get(ConfigGetCommand),
    Set(ConfigSetCommand),
}

impl<O: Write> CommandType<O> for ConfigCommandType {
    fn command(&self) -> &dyn Command<O> {
        match &self {
            Self::Get(cmd) => cmd,
            Self::Set(cmd) => cmd,
        }
    }
}

/// Get and set configuration values.
#[derive(Debug, StructOpt)]
pub struct ConfigCommand {
    #[structopt(subcommand)]
    pub cmd: ConfigCommandType,
}

#[async_trait(?Send)]
impl<O: Write> Command<O> for ConfigCommand {
    async fn execute(&self, context: CommandContext<O>) -> anyhow::Result<()>
    where
        O: 'async_trait,
    {
        self.cmd.command().execute(context).await
    }
}
