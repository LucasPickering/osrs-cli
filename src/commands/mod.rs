mod calc;
mod config;
mod hiscore;
mod ping;
mod price;
mod wiki;

pub use self::config::*;
pub use calc::*;
pub use hiscore::*;
pub use ping::*;
pub use price::*;
pub use wiki::*;

use crate::utils::context::CommandContext;

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
