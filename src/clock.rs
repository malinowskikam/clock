use crate::args::Args;
use jiff::tz::TimeZone;
use jiff::{Span, Timestamp, Unit, Zoned};
use std::fs::File;
use std::io::{stdout, Write};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;
use crate::error::ClockError;

pub fn start_clock(args: Args) -> JoinHandle<Result<(), ClockError>> {
    spawn(move || {
        let format = if let Some(format) = args.format {
            format
        } else {
            "%H:%M:%S".to_string()
        };

        let mut output = if let Some(path) = args.output {
            Box::new(File::create(path).unwrap()) as Box<dyn Write>
        } else {
            Box::new(stdout()) as Box<dyn Write>
        };

        let mut events_n = args.number;

        let tz = TimeZone::system();
        let mut timestamp_zoned = Zoned::now();

        let next_minute_seconds = timestamp_zoned.timestamp().as_second()
            + (60i64 - (timestamp_zoned.timestamp().as_second() % 60));
        let mut next_minute_zoned =
            Zoned::new(Timestamp::new(next_minute_seconds, 0).unwrap(), tz.clone());

        while should_continue(&mut events_n) {
            timestamp_zoned = Zoned::now();
            if timestamp_zoned < next_minute_zoned {
                let wait_span = (next_minute_zoned.timestamp() - timestamp_zoned.timestamp())
                    .total(Unit::Millisecond)
                    .unwrap();
                assert!(wait_span > 0.0 && wait_span < u64::MAX as f64);
                sleep(Duration::from_millis(wait_span as u64));
            }

            writeln!(
                output,
                "{}",
                next_minute_zoned.strftime(&format).to_string()
            )
            .unwrap();

            next_minute_zoned = Zoned::new(
                next_minute_zoned.timestamp() + Span::new().minutes(1),
                tz.clone(),
            );

        }
        Ok(())
    })
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
        }
        None => true,
    }
}
