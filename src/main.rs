#![deny(clippy::all)]
#![cfg_attr(nightly, feature(backtrace))]

use crate::{
    commands::{
        CalcCommand, Command, CommandType, HiscoreCommand, PingCommand,
        WikiCommand,
    },
    utils::context::CommandContext,
};
use commands::ConfigCommand;
use std::process;
use structopt::StructOpt;

mod commands;
mod config;
mod error;
mod utils;

/// All top-level CLI commands.
#[derive(Debug, StructOpt)]
enum OsrsCommandType {
    Calc(CalcCommand),
    Config(ConfigCommand),
    Hiscore(HiscoreCommand),
    Ping(PingCommand),
    Wiki(WikiCommand),
}

impl CommandType for OsrsCommandType {
    fn command(&self) -> &dyn Command {
        match &self {
            Self::Calc(cmd) => cmd,
            Self::Config(cmd) => cmd,
            Self::Hiscore(cmd) => cmd,
            Self::Ping(cmd) => cmd,
            Self::Wiki(cmd) => cmd,
        }
    }
}

/// Oldschool RuneScape CLI.
/// Bugs/suggestions: https://github.com/LucasPickering/osrs-cli/issues
#[derive(Debug, StructOpt)]
struct OsrsOptions {
    #[structopt(subcommand)]
    cmd: OsrsCommandType,
}

impl Command for OsrsOptions {
    fn execute(&self, context: &CommandContext) -> anyhow::Result<()> {
        self.cmd.command().execute(context)
    }
}

fn run() -> anyhow::Result<()> {
    let context = CommandContext::load()?;
    let options = OsrsOptions::from_args();
    options.execute(&context)
}

fn main() {
    let exit_code = match run() {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("{:#}", err);

            // Only use backtraces on nightly
            #[cfg(nightly)]
            {
                // print a backtrace if available
                use std::backtrace::BacktraceStatus;
                let bt = err.backtrace();
                if bt.status() == BacktraceStatus::Captured {
                    eprintln!("{}", bt);
                }
            }

            1
        }
    };
    process::exit(exit_code);
}
