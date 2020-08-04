use crate::{
    commands::Command,
    error::{OsrsError, OsrsResult},
    utils::context::CommandContext,
};
use std::process;
use structopt::StructOpt;

/// Run a network ping against a world
#[derive(Debug, StructOpt)]
pub struct PingOptions {
    /// The number of the world you want to ping
    world: usize,
    /// The number of pings to run (omit to run forever)
    #[structopt(short, long)]
    count: Option<usize>,
}

pub struct PingCommand;

impl Command for PingCommand {
    type Options = PingOptions;

    fn execute(
        &self,
        _context: &CommandContext,
        options: &Self::Options,
    ) -> OsrsResult<()> {
        if options.world < 301 {
            return Err(OsrsError::ArgsError(
                "Invalid world: Must be at least 301".into(),
            ));
        }

        let hostname =
            format!("oldschool{}.runescape.com", options.world - 300);

        // Arg format depends on system
        let result = if cfg!(target_os = "windows") {
            let mut cmd = process::Command::new("ping");
            match options.count {
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
            match options.count {
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
