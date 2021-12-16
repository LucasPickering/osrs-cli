mod commands;
mod config;
mod error;
mod utils;

use crate::{
    commands::{
        CalcCommand, Command, CommandType, ConfigCommand, HiscoreCommand,
        PingCommand, PriceCommand, WikiCommand,
    },
    utils::context::CommandContext,
};
use std::io::Write;
use structopt::StructOpt;

/// All top-level CLI commands.
#[derive(Debug, StructOpt)]
enum OsrsCommandType {
    Calc(CalcCommand),
    #[structopt(visible_alias = "cfg")]
    Config(ConfigCommand),
    #[structopt(visible_alias = "hs")]
    Hiscore(HiscoreCommand),
    Ping(PingCommand),
    #[structopt(visible_alias = "ge")]
    Price(PriceCommand),
    Wiki(WikiCommand),
}

impl<O: Write> CommandType<O> for OsrsCommandType {
    fn command(&self) -> &dyn Command<O> {
        match &self {
            Self::Calc(cmd) => cmd,
            Self::Config(cmd) => cmd,
            Self::Hiscore(cmd) => cmd,
            Self::Ping(cmd) => cmd,
            Self::Price(cmd) => cmd,
            Self::Wiki(cmd) => cmd,
        }
    }
}

/// Oldschool RuneScape CLI.
/// Bugs/suggestions: https://github.com/LucasPickering/osrs-cli/issues
#[derive(Debug, StructOpt)]
pub struct OsrsOptions {
    #[structopt(subcommand)]
    cmd: OsrsCommandType,
}

impl OsrsOptions {
    /// Execute the command defined by this options object. This is the main
    /// entrypoint to the program. Callers can customize how the these options
    /// are parsed, then call this function to execute.
    ///
    /// TODO include note about output
    pub async fn run<O: Write>(self, output: O) -> anyhow::Result<()> {
        let context = CommandContext::load(output)?;
        self.cmd.command().execute(context).await
    }
}

/// Public WebAssembly API
#[cfg(wasm)]
mod wasm {
    use super::*;
    use crate::error::OsrsError;
    use wasm_bindgen::prelude::*;

    /// Initialization. This function gets called when the wasm module is loaded
    #[wasm_bindgen(start)]
    pub fn start() {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    /// Wasm entrypoint, parses and executes arguments. Command should be a list
    /// of arguments, where each one is a string. You *must* pass the binary
    /// name as the first argument.
    ///
    /// We can't take in Vec<String> because wasm_bindgen.
    /// Clean up after https://github.com/rustwasm/wasm-bindgen/issues/168
    #[wasm_bindgen(js_name = runCommand)]
    pub async fn run_command(command: Vec<JsValue>) -> String {
        // Replace this with a try block after is stable
        // https://github.com/rust-lang/rust/issues/31436
        async fn helper(command: Vec<JsValue>) -> anyhow::Result<String> {
            // Convert each arg to a string
            let args: Vec<String> = command
                .into_iter()
                .map::<Result<String, OsrsError>, _>(|value| {
                    value.as_string().ok_or(OsrsError::ExpectedString)
                })
                // Pull all results into one
                .collect::<Result<_, _>>()?;

            // Write all output to a buffer, which we'll return to JS
            // TODO figure out how to stream output back to JS
            let mut output = Vec::new();
            let options = OsrsOptions::from_iter_safe(args)?;
            options.run(&mut output).await?;
            Ok(String::from_utf8(output)?)
        }

        match helper(command).await {
            Ok(output) => output,
            // TODO return error here instead and make the caller print it
            Err(err) => format!("{}\n", err),
        }
    }
}
