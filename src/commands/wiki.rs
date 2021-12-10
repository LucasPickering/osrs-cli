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
        open::that(format!(
            "https://oldschool.runescape.wiki/?search={}",
            urlencoding::encode(&self.query.join(" "))
        ))?;
        Ok(())
    }
}
