//! OpenTTD Manager Plus - Version Management Core
//!
//! This crate provides version management for OpenTTD, including
//! official releases, JGRPP, CityMania Client, and custom sources.
//!
//! ## Modules
//!
//! - [`version`] — Core version types ([`VersionInfo`], [`VersionSource`], etc.)
//! - [`source`] — Version source fetchers ([`VersionFetcher`] trait, official, JGRPP)
//! - [`cache`] — Local manifest caching ([`ManifestCache`])

pub mod cache;
pub mod conflict;
pub mod import;
pub mod plugin;
pub mod server;
pub mod source;
pub mod version;

#[cfg(test)]
mod tests;

/// Result type alias for the manager crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for the manager crate.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// A network or HTTP request error.
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// An I/O error (e.g. cache file read/write).
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// A parse/serialization error (e.g. YAML, JSON, or version string).
    #[error("Parse error: {0}")]
    Parse(String),

    /// A version was not found in the source.
    #[error("Version not found: {0}")]
    VersionNotFound(String),

    /// A JSON serialization or deserialization error.
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// A YAML deserialization error.
    #[error("YAML parse error: {0}")]
    Yaml(String),

    /// A semver version parse error.
    #[error("Version parse error: {0}")]
    Semver(#[from] semver::Error),
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Error::Yaml(err.to_string())
    }
}