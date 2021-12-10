use crate::config::OsrsConfig;

/// A helper type to encapsulate values that we are likely to use multiple
/// time while executing a command. Centralizes that logic to clean shit up.
pub struct CommandContext {
    config: OsrsConfig,
}

impl CommandContext {
    pub fn load() -> anyhow::Result<CommandContext> {
        let config = OsrsConfig::load()?;

        Ok(CommandContext { config })
    }

    pub fn config(&self) -> &OsrsConfig {
        &self.config
    }
}
