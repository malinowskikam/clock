use std::mem::replace;
use std::thread::sleep;
use std::time::Duration;
use jiff::{Span, Timestamp, Unit, Zoned};
use jiff::tz::TimeZone;
use crate::error::Result;

pub trait Scheduler {
    fn wait_until_next_event(&mut self) -> Result<Zoned>;
    fn should_continue(&mut self) -> bool;
}

pub struct SystemTimeScheduler {
    next_event: Zoned,
    span: Span,
    tz: TimeZone,
    events_n: Option<usize>
}

impl SystemTimeScheduler {
    pub fn new(events_n: Option<usize>, span: Span) -> Result<SystemTimeScheduler> {
        let tz = TimeZone::system();

        // TODO add bounds check
        let span_len_millis = span.total(Unit::Millisecond)? as i64;
        let now_millis = Zoned::now().timestamp().as_millisecond();

        let next_event_millis = now_millis + (span_len_millis - (now_millis % span_len_millis));
        let next_event = Zoned::new(Timestamp::from_millisecond(next_event_millis)?, tz.clone());

        Ok(SystemTimeScheduler {
            next_event,
            span,
            tz,
            events_n
        })
    }
}

impl Scheduler for SystemTimeScheduler {
    fn wait_until_next_event(&mut self) -> Result<Zoned> {
        let now = Zoned::now();
        if now < self.next_event {
            let wait_span = (self.next_event.timestamp() - now.timestamp())
                .total(Unit::Millisecond)?;
            sleep(Duration::from_millis(wait_span as u64));
        }

        let new_next_event = Zoned::new(self.next_event.timestamp() + self.span, self.tz.clone());
        Ok(replace(&mut self.next_event, new_next_event))
    }

    fn should_continue(&mut self) -> bool {
        match &mut self.events_n {
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
}