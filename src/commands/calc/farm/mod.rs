//! This command is a container for additional subcommands related to farming
//! calculators.

mod herb;

use crate::{
    commands::{calc::farm::herb::CalcFarmHerbCommand, Command, CommandType},
    utils::context::CommandContext,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum CalcFarmCommandType {
    Herb(CalcFarmHerbCommand),
}

impl CommandType for CalcFarmCommandType {
    fn command(&self) -> &dyn Command {
        match &self {
            Self::Herb(cmd) => cmd,
        }
    }
}

/// Calculators related to farming
#[derive(Debug, StructOpt)]
pub struct CalcFarmCommand {
    #[structopt(subcommand)]
    pub cmd: CalcFarmCommandType,
}

impl Command for CalcFarmCommand {
    fn execute(&self, context: &CommandContext) -> anyhow::Result<()> {
        self.cmd.command().execute(context)
    }
}
