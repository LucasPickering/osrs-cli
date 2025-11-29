#![deny(clippy::all)]
#![cfg_attr(nightly, feature(backtrace))]

#[cfg(not(target_family = "wasm"))]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use osrs_cli::OsrsOptions;
    use std::{io, process};
    use structopt::StructOpt;

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

#[cfg(target_family = "wasm")]
fn main() {
    // Delete after https://github.com/rust-lang/cargo/issues/3138
    println!("Bin not supported on Wasm");
}
