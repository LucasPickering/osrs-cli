#![deny(clippy::all, unused)]

use crate::{
    commands::{
        CalcCommand, Command, CommandType, HiscoreCommand, PingCommand,
        WikiCommand,
    },
    error::OsrsResult,
    utils::context::CommandContext,
};
use std::process;
use structopt::StructOpt;

mod commands;
mod error;
mod utils;

#[derive(Debug, StructOpt)]
enum OsrsCommandType {
    Calc(CalcCommand),
    Hiscore(HiscoreCommand),
    Ping(PingCommand),
    Wiki(WikiCommand),
}

impl CommandType for OsrsCommandType {
    fn command(&self) -> &dyn Command {
        match &self {
            Self::Calc(cmd) => cmd,
            Self::Hiscore(cmd) => cmd,
            Self::Ping(cmd) => cmd,
            Self::Wiki(cmd) => cmd,
        }
    }
}

/// Oldschool RuneScape CLI.
/// Bugs/suggestions: https://github.com/LucasPickering/osrs-cli
#[derive(Debug, StructOpt)]
struct OsrsOptions {
    #[structopt(subcommand)]
    cmd: OsrsCommandType,
}

impl Command for OsrsOptions {
    fn execute(&self, context: &CommandContext) -> OsrsResult<()> {
        self.cmd.command().execute(context)
    }
}

fn main() {
    let context = CommandContext::new();
    let options = OsrsOptions::from_args();
    let exit_code = match options.execute(&context) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("{:#}", err);
            1
        }
    };
    process::exit(exit_code);
}
