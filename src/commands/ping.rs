use crate::{
    commands::Command, error::OsrsError, utils::context::CommandContext,
};
use async_trait::async_trait;
use std::io::Write;
use structopt::StructOpt;

/// Run a network ping against a world
#[cfg_attr(wasm, allow(unused))]
#[derive(Debug, StructOpt)]
pub struct PingCommand {
    /// The number of the world you want to ping
    world: usize,
    /// The number of pings to run (omit to run forever)
    #[structopt(short, long)]
    count: Option<usize>,
}

#[async_trait(?Send)]
impl<O: Write> Command<O> for PingCommand {
    // Native implementation
    #[cfg(not(wasm))]
    async fn execute(&self, _context: CommandContext<O>) -> anyhow::Result<()>
    where
        O: 'async_trait,
    {
        use std::process;

        if self.world < 301 {
            return Err(OsrsError::ArgsError(
                "Invalid world: Must be at least 301".into(),
            )
            .into());
        }

        let hostname = format!("oldschool{}.runescape.com", self.world - 300);

        let mut cmd = process::Command::new("ping");
        // Arg format depends on system
        if cfg!(target_os = "windows") {
            match self.count {
                // On Windows, "-n -1" means run forever
                None => {
                    cmd.args(&["-n", "-1"]);
                }
                Some(count) => {
                    cmd.args(&["-n", &count.to_string()]);
                }
            };
        } else {
            match self.count {
                // On Linux, it runs forever if you just omit "-c"
                None => {}
                Some(count) => {
                    cmd.args(&["-c", &count.to_string()]);
                }
            }
        };
        let result = cmd.arg(&hostname).spawn();

        // Execute the command
        let mut child = result.expect("ping command failed to start");
        child.wait()?;
        Ok(())
    }

    // Browser implementation
    #[cfg(wasm)]
    async fn execute(&self, _context: CommandContext<O>) -> anyhow::Result<()>
    where
        O: 'async_trait,
    {
        Err(OsrsError::UnsupportedEnvironment(
            "Ping not supported in browser".into(),
        )
        .into())
    }
}
