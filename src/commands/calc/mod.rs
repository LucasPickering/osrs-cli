//! This command is a container for additional subcommands related to making
//! calculations.

mod xp;

use structopt::StructOpt;
pub use xp::*;

#[derive(Debug, StructOpt)]
pub enum CalcCommandType {
    Xp(CalcXpOptions),
}

/// Calculators!
#[derive(Debug, StructOpt)]
pub struct CalcOptions {
    #[structopt(subcommand)]
    pub cmd: CalcCommandType,
}
