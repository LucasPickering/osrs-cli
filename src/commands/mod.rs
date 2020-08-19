mod calc;
mod config;
mod hiscore;
mod ping;
mod wiki;

pub use self::config::*;
use crate::utils::context::CommandContext;
pub use calc::*;
pub use hiscore::*;
pub use ping::*;
pub use wiki::*;

/// An enum that defines a list of subcommands.
pub trait CommandType {
    /// Get the command out of this wrapper variant.
    fn command(&self) -> &dyn Command;
}

/// One subcommand for the CLI. Each command should also implement `StructOpt`,
/// so that it can collect its own CLI args.
pub trait Command {
    /// Run the command with the given input options.
    fn execute(&self, context: &CommandContext) -> anyhow::Result<()>;
}
