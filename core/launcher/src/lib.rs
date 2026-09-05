//! OpenTTD Manager Plus - Game Launcher Core
//!
//! Responsible for launching OpenTTD with the correct
//! version, configuration, and arguments.

pub mod args;
pub mod runner;

#[cfg(test)]
mod tests;

pub use args::ArgsBuilder;
pub use runner::{GameProcess, GameRunner, GameRunnerConfig};

/// Result type alias for the launcher crate
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for the launcher crate
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An I/O error occurred
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// The game process failed to launch
    #[error("Launch failed: {0}")]
    LaunchFailed(String),
    /// The game executable was not found
    #[error("Game not found: {0}")]
    GameNotFound(String),
    /// A process-related error occurred
    #[error("Process error: {0}")]
    Process(String),
    /// An invalid argument was provided
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}