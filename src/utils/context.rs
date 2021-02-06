use crate::config::OsrsConfig;
use num_format::{SystemLocale, ToFormattedString};

/// A helper type to encapsulate values that we are likely to use multiple
/// time while executing a command. Centralizes that logic to clean shit up.
pub struct CommandContext {
    config: OsrsConfig,
    locale: SystemLocale,
}

impl CommandContext {
    pub fn load() -> anyhow::Result<CommandContext> {
        let config = OsrsConfig::load()?;
        let locale = SystemLocale::default()?;
        Ok(CommandContext { config, locale })
    }

    pub fn config(&self) -> &OsrsConfig {
        &self.config
    }

    /// Format the given number. The formatting will be based on locale.
    pub fn fmt_num<T: ToFormattedString>(&self, num: &T) -> String {
        num.to_formatted_string(&self.locale)
    }
}
