use crate::{
    commands::{
        config::{
            get::ConfigGetCommand, set::ConfigSetCommand,
            set_herb::ConfigSetHerbCommand,
        },
        Command, CommandType,
    },
    utils::context::CommandContext,
};
use structopt::StructOpt;

mod get;
mod set;
mod set_herb;

#[derive(Debug, StructOpt)]
pub enum ConfigCommandType {
    Get(ConfigGetCommand),
    Set(ConfigSetCommand),
    SetHerb(ConfigSetHerbCommand),
}

impl CommandType for ConfigCommandType {
    fn command(&self) -> &dyn Command {
        match &self {
            Self::Get(cmd) => cmd,
            Self::Set(cmd) => cmd,
            Self::SetHerb(cmd) => cmd,
        }
    }
}

/// Get and set configuration values.
#[derive(Debug, StructOpt)]
pub struct ConfigCommand {
    #[structopt(subcommand)]
    pub cmd: ConfigCommandType,
}

impl Command for ConfigCommand {
    fn execute(&self, context: &CommandContext) -> anyhow::Result<()> {
        self.cmd.command().execute(context)
    }
}
