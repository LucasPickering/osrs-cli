use crate::config::OsrsConfig;
use prettytable::Table;
use std::{fmt::Arguments, io::Write};

/// A helper type to encapsulate values that we are likely to use multiple
/// time while executing a command. Centralizes that logic to clean shit up.
pub struct CommandContext<O: Write> {
    pub config: OsrsConfig,
    /// Output that we send to the user. On native platforms this will
    /// generally be stdout, on others (e.g. web) it could be a byte vector or
    /// similar. This is passed from the caller, so they get to decide what we
    /// do with the output.
    pub output: O,
}

impl<O: Write> CommandContext<O> {
    /// Load initial context from given output. Config will be loaded
    /// automatically from disk/browser storage.
    pub fn load(output: O) -> anyhow::Result<Self> {
        let config = OsrsConfig::load()?;

        Ok(CommandContext { config, output })
    }

    pub fn config(&self) -> &OsrsConfig {
        &self.config
    }

    /// Print data to output, followed by a newline
    pub fn println(&mut self, data: &str) -> anyhow::Result<()> {
        self.output.write_all(data.as_bytes())?;
        self.output.write_all("\n".as_bytes())?;
        Ok(())
    }

    /// print a formatted string to output, followed by a newline
    pub fn println_fmt(&mut self, fmt: Arguments<'_>) -> anyhow::Result<()> {
        self.output.write_fmt(fmt)?;
        self.output.write_all("\n".as_bytes())?;
        Ok(())
    }

    /// Print a pretty table to output
    pub fn print_table(&mut self, table: &Table) -> anyhow::Result<()> {
        // TODO fix colors
        table.print(&mut self.output)?;
        Ok(())
    }
}
