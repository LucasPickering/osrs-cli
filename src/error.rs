use thiserror::Error;

/// Generic error type for anything that can go wrong during command execution.
#[derive(Debug, Error)]
pub enum OsrsError {
    #[error("Argument error: {0}")]
    ArgsError(String),
    #[error("Unknown skill: {0}")]
    UnknownSkill(String),
    #[error("Invalid level. Must be between 1 and 127, got: {0}")]
    InvalidLevel(usize),

    // wrapped errors
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Csv(#[from] csv::Error),
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
}

pub type OsrsResult<T> = Result<T, OsrsError>;
