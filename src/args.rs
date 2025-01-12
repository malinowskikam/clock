use crate::error::ClockError;
use gumdrop::Options;
use std::env::args;

#[derive(Options)]
pub struct Args {
    #[options(help = "number of events")]
    pub number: Option<usize>,

    #[options(help = "output format")]
    pub format: Option<String>,

    #[options(help = "output file path")]
    pub output: Option<String>,

    #[options(help = "print help message")]
    pub help: bool,

    #[options(help = "print version")]
    pub version: bool,
}

impl Args {
    pub fn parse_and_validate() -> Result<Self, ClockError> {
        let args = args().skip(1).collect::<Vec<_>>();
        let parsed = Self::parse_args_default(&args)?;
        parsed.validate()?;
        Ok(parsed)
    }

    fn validate(&self) -> Result<(), ClockError> {
        Ok(())
    }
}
