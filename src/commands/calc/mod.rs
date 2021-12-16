//! This command is a container for additional subcommands related to making
//! calculations.

mod drop;
mod stew;
mod xp;

use crate::{
    commands::{
        calc::{
            drop::CalcDropCommand, stew::CalcStewCommand, xp::CalcXpCommand,
        },
        Command, CommandType,
    },
    utils::context::CommandContext,
};
use async_trait::async_trait;
use std::io::Write;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum CalcCommandType {
    Drop(CalcDropCommand),
    Stew(CalcStewCommand),
    Xp(CalcXpCommand),
}

impl<O: Write> CommandType<O> for CalcCommandType {
    fn command(&self) -> &dyn Command<O> {
        match &self {
            Self::Drop(cmd) => cmd,
            Self::Stew(cmd) => cmd,
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

#[async_trait(?Send)]
impl<O: Write> Command<O> for CalcCommand {
    async fn execute(&self, context: CommandContext<O>) -> anyhow::Result<()>
    where
        O: 'async_trait,
    {
        self.cmd.command().execute(context).await
    }
}
