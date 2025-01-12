use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClockError {
    #[error("Args parsing error: {0}")]
    ArgsParsingError(#[from] gumdrop::Error),
    #[error("Args validation error: {0}")]
    ArgsValidationError(String),
}
