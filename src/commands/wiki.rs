use crate::{
    commands::Command, error::OsrsResult, utils::context::CommandContext,
};
use structopt::StructOpt;

/// Search for a term on the wiki and open the results in a browser
#[derive(Debug, StructOpt)]
pub struct WikiOptions {
    /// The search query to run
    query: Vec<String>,
}

pub struct WikiCommand;

impl Command for WikiCommand {
    type Options = WikiOptions;

    fn execute(
        &self,
        _context: &CommandContext,
        options: &Self::Options,
    ) -> OsrsResult<()> {
        open::that_in_background(format!(
            "https://oldschool.runescape.wiki/?search={}",
            options.query.join(" ")
        ));
        Ok(())
    }
}
