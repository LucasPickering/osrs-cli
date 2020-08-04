#![deny(clippy::all, unused)]

use crate::{
    commands::{
        CalcCommandType, CalcOptions, CalcXpCommand, Command, HiscoreCommand,
        HiscoreOptions, PingCommand, PingOptions,
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
enum CommandType {
    Calc(CalcOptions),
    Hiscore(HiscoreOptions),
    Ping(PingOptions),
}

/// Oldschool RuneScape CLI.
/// Bugs/suggestions: https://github.com/LucasPickering/osrs-cli
#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(subcommand)]
    cmd: CommandType,
}

fn run(opt: Options) -> OsrsResult<()> {
    let context = CommandContext::new();
    match opt.cmd {
        CommandType::Calc(CalcOptions {
            cmd: CalcCommandType::Xp(opts),
        }) => CalcXpCommand.execute(&context, &opts),
        CommandType::Hiscore(opts) => HiscoreCommand.execute(&context, &opts),
        CommandType::Ping(opts) => PingCommand.execute(&context, &opts),
    }
}

fn main() {
    let options = Options::from_args();
    let exit_code = match run(options) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("{:#}", err);
            1
        }
    };
    process::exit(exit_code);
}
