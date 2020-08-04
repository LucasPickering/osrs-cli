#![deny(clippy::all, unused)]

use crate::{
    commands::{
        CalcCommandType, CalcOptions, CalcXpCommand, Command, HiscoreCommand,
        HiscoreOptions,
    },
    utils::context::CommandContext,
};
use std::process;
use structopt::StructOpt;

mod commands;
mod utils;

#[derive(Debug, StructOpt)]
enum CommandType {
    Hiscore(HiscoreOptions),
    Calc(CalcOptions),
}

/// Oldschool RuneScape CLI.
/// Bugs/suggestions: https://github.com/LucasPickering/osrs-cli
#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(subcommand)]
    cmd: CommandType,
}

fn run(opt: Options) -> anyhow::Result<()> {
    let context = CommandContext::new();
    match opt.cmd {
        CommandType::Hiscore(opts) => HiscoreCommand.execute(&context, &opts),
        CommandType::Calc(CalcOptions {
            cmd: CalcCommandType::Xp(opts),
        }) => CalcXpCommand.execute(&context, &opts),
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
