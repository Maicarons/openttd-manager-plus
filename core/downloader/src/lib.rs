//! OpenTTD Manager Plus - Download Engine
//!
//! Multi-threaded download engine with mirror acceleration,
//! checksum verification, and progress reporting.

/// Re-export public modules
pub mod engine;
pub mod mirror;
pub mod verify;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

/// Result type alias for the downloader crate
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for the downloader crate
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },

    #[error("Download cancelled")]
    Cancelled,

    #[error("Mirror unavailable: {0}")]
    MirrorUnavailable(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("HTTP error: {status} {message}")]
    HttpError { status: reqwest::StatusCode, message: String },
}