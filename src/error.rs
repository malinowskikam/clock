use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClockError {
    #[error("Args parsing error: {0}")]
    ArgsParsingError(#[from] gumdrop::Error),
    #[error("Args validation error: {0}")]
    ArgsValidationError(String),
    #[error("Clock thread panic")]
    ClockPanic,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("DateTime error: {0}")]
    DateTimeError(#[from] jiff::Error),
}

pub type Result<T> = std::result::Result<T, ClockError>;
