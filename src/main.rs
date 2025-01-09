mod args;

use std::process::{exit, Command};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use args::Args;
use gumdrop::Options;

fn main() {
    let args = Args::parse_args_default_or_exit();

    if args.version {
        handle_version();
    }

    let mut events_n = args.events_n;

    let mut format_command = Command::new("date");
    let mut timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut next_minute = timestamp + (60 - (timestamp % 60));

    while should_continue(&mut events_n) {
        timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if timestamp < next_minute {
            sleep(Duration::from_secs(next_minute - timestamp));
        }

        format_command
            .args(["--date".to_string(), format!("@{}", next_minute)])
            .status()
            .unwrap();
        next_minute = next_minute + 60;




    }
}

/// The program should continue if the events_n option is unspecified or if it's specified
/// and positive.
///
/// # Arguments
///
/// * `events_n`: Current events_n Option. Will be decremented if specified.
///
/// returns: ``true`` if the program should continue
fn should_continue(events_n: &mut Option<usize>) -> bool {
    match events_n {
        Some(n) => {
            if *n > 0 {
                *n -= 1;
                true
            } else {
                false
            }
        },
        None => true,
    }
}

/// Handle the ``--version`` argument. The program should print the version info and exit.
fn handle_version() -> ! {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    exit(0)
}
