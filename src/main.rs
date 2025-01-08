
use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    let mut format_command = Command::new("date");
    let mut timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut next_minute = timestamp + (60 - (timestamp % 60));

    loop {
        timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        if timestamp < next_minute {
            sleep(Duration::from_secs(next_minute - timestamp));
        }

        format_command.args(["--date".to_string(), format!("@{}", next_minute)]).status().unwrap();
        next_minute = next_minute + 60;

    }
}
