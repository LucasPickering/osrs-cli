//! This command is a container for additional subcommands related to farming
//! calculators.

mod herb;

use crate::{
    commands::{calc::farm::herb::CalcFarmHerbCommand, Command, CommandType},
    utils::context::CommandContext,
};
use async_trait::async_trait;
use std::io::Write;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum CalcFarmCommandType {
    Herb(CalcFarmHerbCommand),
}

impl<O: Write> CommandType<O> for CalcFarmCommandType {
    fn command(&self) -> &dyn Command<O> {
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

#[async_trait(?Send)]
impl<O: Write> Command<O> for CalcFarmCommand {
    async fn execute(&self, context: CommandContext<O>) -> anyhow::Result<()>
    where
        O: 'async_trait,
    {
        self.cmd.command().execute(context).await
    }
}
