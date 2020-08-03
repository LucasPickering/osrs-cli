#![deny(clippy::all, unused)]

use crate::commands::{Command, HiscoreCommand, HiscoreOptions};
use std::process;
use structopt::StructOpt;

mod commands;
mod utils;

#[derive(Debug, StructOpt)]
enum CommandType {
    Hiscore(HiscoreOptions),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "osrs")]
struct Options {
    #[structopt(subcommand)]
    cmd: CommandType,
}

fn run(opt: Options) -> anyhow::Result<()> {
    match opt.cmd {
        CommandType::Hiscore(opts) => HiscoreCommand.execute(&opts),
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
