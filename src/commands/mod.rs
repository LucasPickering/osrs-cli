mod calc;
mod hiscore;
mod ping;

use crate::{error::OsrsResult, utils::context::CommandContext};
pub use calc::*;
pub use hiscore::*;
pub use ping::*;
use structopt::StructOpt;

/// One subcommand for the CLI. Each subcommand has its own CLI arg structure
/// and functionality.
pub trait Command {
    /// The struct that defines the CLI args that this subcommand takes.
    type Options: StructOpt;

    /// Run the command with the given input options.
    fn execute(
        &self,
        context: &CommandContext,
        options: &Self::Options,
    ) -> OsrsResult<()>;
}
