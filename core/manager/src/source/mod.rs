//! Version source abstractions and fetchers.
//!
//! This module provides the [`VersionFetcher`] trait, which defines a
//! common interface for fetching version information from different
//! sources (official OpenTTD CDN, JGRPP GitHub releases, etc.).

use async_trait::async_trait;

use crate::version::{VersionInfo, VersionSource};
use crate::Result;

/// Trait for fetching version information from a source.
///
/// Implementors provide version lists from their respective sources,
/// such as the official OpenTTD CDN or the JGRPP GitHub releases page.
#[async_trait]
pub trait VersionFetcher: Send + Sync {
    /// Returns the [`VersionSource`] variant this fetcher handles.
    fn source(&self) -> VersionSource;

    /// Fetches the list of available versions from this source.
    ///
    /// Returns a vector of [`VersionInfo`] structs, or an error if the
    /// fetch or parsing fails.
    async fn fetch_versions(&self) -> Result<Vec<VersionInfo>>;

    /// Returns a human-readable name for this fetcher (e.g. `"Official"`).
    fn name(&self) -> &'static str;
}

pub mod bananas;
pub mod cmclient;
pub mod official;
pub mod jgrpp;