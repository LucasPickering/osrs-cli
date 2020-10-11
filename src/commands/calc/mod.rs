//! This command is a container for additional subcommands related to making
//! calculations.

mod drop;
mod xp;

use crate::{
    commands::{
        calc::{drop::CalcDropCommand, xp::CalcXpCommand},
        Command, CommandType,
    },
    utils::context::CommandContext,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum CalcCommandType {
    Drop(CalcDropCommand),
    Xp(CalcXpCommand),
}

impl CommandType for CalcCommandType {
    fn command(&self) -> &dyn Command {
        match &self {
            Self::Drop(cmd) => cmd,
            Self::Xp(cmd) => cmd,
        }
    }
}

/// Calculators!
#[derive(Debug, StructOpt)]
pub struct CalcCommand {
    #[structopt(subcommand)]
    pub cmd: CalcCommandType,
}

impl Command for CalcCommand {
    fn execute(&self, context: &CommandContext) -> anyhow::Result<()> {
        self.cmd.command().execute(context)
    }
}
