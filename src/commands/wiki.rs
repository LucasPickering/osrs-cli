use crate::{commands::Command, utils::context::CommandContext};
use structopt::StructOpt;

/// Search for a term on the wiki and open the results in a browser
#[derive(Debug, StructOpt)]
pub struct WikiCommand {
    /// The search query to run
    query: Vec<String>,
}

impl Command for WikiCommand {
    fn execute(&self, _context: &CommandContext) -> anyhow::Result<()> {
        open::that_in_background(format!(
            "https://oldschool.runescape.wiki/?search={}",
            self.query.join(" ")
        ));
        Ok(())
    }
}
