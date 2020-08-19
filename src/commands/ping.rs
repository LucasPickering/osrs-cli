use crate::{
    commands::Command, error::OsrsError, utils::context::CommandContext,
};
use std::process;
use structopt::StructOpt;

/// Run a network ping against a world
#[derive(Debug, StructOpt)]
pub struct PingCommand {
    /// The number of the world you want to ping
    world: usize,
    /// The number of pings to run (omit to run forever)
    #[structopt(short, long)]
    count: Option<usize>,
}

impl Command for PingCommand {
    fn execute(&self, _context: &CommandContext) -> anyhow::Result<()> {
        if self.world < 301 {
            return Err(OsrsError::ArgsError(
                "Invalid world: Must be at least 301".into(),
            )
            .into());
        }

        let hostname = format!("oldschool{}.runescape.com", self.world - 300);

        // Arg format depends on system
        let result = if cfg!(target_os = "windows") {
            let mut cmd = process::Command::new("ping");
            match self.count {
                // On Windows, "-n -1" means run forever
                None => {
                    cmd.args(&["-n", "-1"]);
                }
                Some(count) => {
                    cmd.args(&["-n", &count.to_string()]);
                }
            };
            cmd.arg(&hostname).spawn()
        } else {
            let mut cmd = process::Command::new("ping");
            match self.count {
                // On Linux, it runs forever if you just omit "-c"
                None => {}
                Some(count) => {
                    cmd.args(&["-c", &count.to_string()]);
                }
            }
            cmd.arg(&hostname).spawn()
        };

        // Execute the command
        let mut child = result.expect("ping command failed to start");
        child.wait()?;
        Ok(())
    }
}
