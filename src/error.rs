use thiserror::Error;

/// Generic error type for stuff that can go wrong during command execution.
#[derive(Debug, Error)]
pub enum OsrsError {
    #[error("Argument error: {0}")]
    ArgsError(String),

    #[error("Unknown skill: {0}")]
    UnknownSkill(String),

    #[error("Invalid level. Must be between 1 and 127, got: {0}")]
    InvalidLevel(usize),

    /// User attempted an action that isn't supported in this environment. This
    /// error *shouldn't* ever happen. Could be something like attempting an
    /// action in the browser that's only supported natively.
    #[cfg(target_family = "wasm")]
    #[error("Not supported in this environment: {0}")]
    UnsupportedEnvironment(String),

    /// Error on a dynamic cast of a JS type when attempting to get a string.
    /// Indicates an implementation bug.
    #[cfg(target_family = "wasm")]
    #[error("Unexpected type of JavaScript value, expected String")]
    ExpectedString,
}
