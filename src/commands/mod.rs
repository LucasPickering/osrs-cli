use structopt::StructOpt;

mod hiscore;

pub use hiscore::*;

/// One subcommand for the CLI. Each subcommand has its own CLI arg structure
/// and functionality.
pub trait Command {
    /// The struct that defines the CLI args that this subcommand takes.
    type Options: StructOpt;

    /// Run the command with the given input options.
    fn execute(&self, options: &Self::Options) -> anyhow::Result<()>;
}
