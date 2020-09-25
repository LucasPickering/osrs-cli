//! This command is a container for additional subcommands related to making
//! calculations.

mod prob;
mod xp;

use crate::{
    commands::{
        calc::{prob::CalcProbCommand, xp::CalcXpCommand},
        Command, CommandType,
    },
    utils::context::CommandContext,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum CalcCommandType {
    Prob(CalcProbCommand),
    Xp(CalcXpCommand),
}

impl CommandType for CalcCommandType {
    fn command(&self) -> &dyn Command {
        match &self {
            Self::Prob(cmd) => cmd,
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
