use crate::config::OsrsConfig;
use num_format::{SystemLocale, ToFormattedString};
use reqwest::blocking::Client;

/// A helper type to encapsulate values that we are likely to use multiple
/// time while executing a command. Centralizes that logic to clean shit up.
pub struct CommandContext {
    config: OsrsConfig,
    locale: SystemLocale,
    http_client: Client,
}

impl CommandContext {
    pub fn load() -> anyhow::Result<CommandContext> {
        let config = OsrsConfig::load()?;
        let locale = SystemLocale::default()?;
        let http_client = reqwest::blocking::Client::new();
        Ok(CommandContext {
            config,
            locale,
            http_client,
        })
    }

    pub fn config(&self) -> &OsrsConfig {
        &self.config
    }

    pub fn http_client(&self) -> &Client {
        &self.http_client
    }

    /// Format the given number. The formatting will be based on locale.
    pub fn fmt_num<T: ToFormattedString>(&self, num: &T) -> String {
        num.to_formatted_string(&self.locale)
    }
}
