use crate::{
    commands::Command,
    utils::{context::CommandContext, http},
};
use async_trait::async_trait;
use std::io::Write;
use structopt::StructOpt;

/// Search for a term on the wiki and open the results in a browser
#[derive(Debug, StructOpt)]
pub struct WikiCommand {
    /// The search query to run
    query: Vec<String>,
}

#[async_trait(?Send)]
impl<O: Write> Command<O> for WikiCommand {
    // Native implementation
    #[cfg(not(target_family = "wasm"))]
    async fn execute(&self, _context: CommandContext<O>) -> anyhow::Result<()>
    where
        O: 'async_trait,
    {
        open::that(self.url())?;
        Ok(())
    }

    // Browser implementation
    #[cfg(target_family = "wasm")]
    async fn execute(&self, _context: CommandContext<O>) -> anyhow::Result<()>
    where
        O: 'async_trait,
    {
        use crate::utils::browser;

        // Open in a new tab
        browser::window()?
            .open_with_url_and_target(
                &self.url(),
                // Set target="..." to a descriptive value, so if the user
                // searches the same value twice, the browser will re-use the
                // tab.
                // https://developer.mozilla.org/en-US/docs/Web/API/Window/open#do_not_use_target_blank
                &format!("wiki-{}", self.query()),
            )
            .map_err(browser::js_to_anyhow)?; // Don't think this can fail
        Ok(())
    }
}

impl WikiCommand {
    fn query(&self) -> String {
        self.query.join(" ")
    }

    /// Get the wiki search URL
    fn url(&self) -> String {
        http::url(
            "https://oldschool.runescape.wiki/",
            &[("search", &self.query())],
        )
    }
}
