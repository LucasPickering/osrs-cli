use crate::{
    commands::{
        config::{get::ConfigGetCommand, set::ConfigSetCommand},
        Command, CommandType,
    },
    error::OsrsResult,
    utils::context::CommandContext,
};
use structopt::StructOpt;

mod get;
mod set;

#[derive(Debug, StructOpt)]
pub enum ConfigCommandType {
    Get(ConfigGetCommand),
    Set(ConfigSetCommand),
}

impl CommandType for ConfigCommandType {
    fn command(&self) -> &dyn Command {
        match &self {
            Self::Get(cmd) => cmd,
            Self::Set(cmd) => cmd,
        }
    }
}

/// TODO
#[derive(Debug, StructOpt)]
pub struct ConfigCommand {
    #[structopt(subcommand)]
    pub cmd: ConfigCommandType,
}

impl Command for ConfigCommand {
    fn execute(&self, context: &CommandContext) -> OsrsResult<()> {
        self.cmd.command().execute(context)
    }
}
