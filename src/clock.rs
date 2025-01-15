use crate::args::Args;
use jiff::{Span};
use std::thread::{spawn, JoinHandle};
use crate::error::ClockError;
use crate::output::Output;
use crate::scheduler::{Scheduler, SystemTimeScheduler};

pub fn start_clock(args: Args) -> JoinHandle<Result<(), ClockError>> {
    spawn(move || {
        let format = if let Some(format) = args.format {
            format
        } else {
            "%H:%M:%S".to_string()
        };

        let mut output = Output::try_from_args(&args.output)?;

        let span = Span::new().minutes(1);
        let mut scheduler = SystemTimeScheduler::new(args.number, span)?;

        start_clock_loop(&format, &mut output, &mut scheduler)?;
        Ok(())
    })
}

fn start_clock_loop(format: &str, output: &mut Output, scheduler: &mut dyn Scheduler) -> Result<(), ClockError> {
    while scheduler.should_continue() {
        let next_event = scheduler.wait_until_next_event()?;
        output.write(&next_event.strftime(format).to_string())?;
    }
    Ok(())
}
