mod args;
mod clock;
mod error;

use crate::clock::start_clock;
use args::Args;
use gumdrop::Options;
use std::process::exit;
use crate::error::ClockError;
use crate::error::ClockError::ClockPanic;

fn main() -> Result<(), ClockError> {
    let args = match Args::parse_and_validate() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    if args.version {
        handle_version();
    } else if args.help {
        handle_help(args);
    } else {
        let handle = start_clock(args);
        handle.join().map_err(|e| ClockPanic)??;
    }

    Ok(())
}

/// Handle the ``--version`` argument. The program should print the version info and exit.
fn handle_version() -> ! {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    exit(0)
}

/// Handle the ``--version`` argument. The program should print the version info and exit.
fn handle_help(args: Args) -> ! {
    println!(
        "Usage: {} [OPTIONS]\n\n{}",
        env!("CARGO_PKG_NAME"),
        args.self_usage()
    );
    exit(0)
}
