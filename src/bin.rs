#![deny(clippy::all)]
#![cfg_attr(nightly, feature(backtrace))]

use osrs_cli::OsrsOptions;
use std::{io, process};
use structopt::StructOpt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let options = OsrsOptions::from_args();
    let exit_code = match options.run(io::stdout()).await {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("{:#}", err);

            // Only use backtraces on nightly
            #[cfg(nightly)]
            {
                // print a backtrace if available
                use std::backtrace::BacktraceStatus;
                let bt = err.backtrace();
                if bt.status() == BacktraceStatus::Captured {
                    eprintln!("{}", bt);
                }
            }

            1
        }
    };
    process::exit(exit_code);
}
