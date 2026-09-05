//! OpenTTD Manager Plus - Configuration Management
//!
//! Manages openttd.cfg, mods, NewGRF, audio, and image library
//! configurations for independent or shared instances.

pub mod config_file;
pub mod instance;
pub mod profile;

/// Result type alias for the config crate
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for the config crate
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An I/O error occurred
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// A parse error occurred
    #[error("Parse error: {0}")]
    Parse(String),

    /// A JSON serialization/deserialization error occurred
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// The specified profile was not found
    #[error("Profile not found: {0}")]
    ProfileNotFound(String),

    /// The specified instance was not found
    #[error("Instance not found: {0}")]
    InstanceNotFound(String),

    /// An instance with the same ID already exists
    #[error("Duplicate instance: {0}")]
    DuplicateInstance(String),

    /// A profile with the same ID already exists
    #[error("Duplicate profile: {0}")]
    DuplicateProfile(String),

    /// Invalid configuration data
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}