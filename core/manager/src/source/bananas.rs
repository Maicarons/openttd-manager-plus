//! BaNaNaS mod metadata fetcher.
//!
//! Fetches mod metadata from OpenTTD's online content service at
//! `https://bananas.openttd.org/api`.  Supports listing mods by type
//! (NewGRF, AI, GameScript, MusicSet), searching by name or author,
//! and resolving download URLs for specific mod versions.
//!
//! The BaNaNaS API returns paginated JSON results with the structure:
//! ```json
//! { "results": [...], "count": N, "next": "..." }
//! ```

use log::{debug, info, warn};
use serde::Deserialize;

use crate::version::{ModInfo, ModType};
use crate::{Error, Result};

/// Base URL for the BaNaNaS API.
const BANANAS_API_BASE: &str = "https://bananas.openttd.org/api";

/// HTTP client timeout in seconds.
const REQUEST_TIMEOUT_SECS: u64 = 30;

/// Default number of items per page.
const DEFAULT_LIMIT: u32 = 100;

/// BaNaNaS API paginated response.
#[derive(Debug, Deserialize)]
struct BananasResponse {
    /// The list of results for the current page.
    results: Vec<BananasItem>,
    /// Total number of items across all pages.
    #[allow(dead_code)]
    count: Option<u64>,
    /// URL for the next page, if any.
    next: Option<String>,
    /// Previous page URL, if any.
    #[allow(dead_code)]
    previous: Option<String>,
}

/// A single item from the BaNaNaS API response.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct BananasItem {
    /// Unique content identifier (e.g. `"424f4f54"`).
    id: Option<String>,
    /// Display name of the mod.
    name: Option<String>,
    /// Version string (e.g. `"1.0.0"`).
    version: Option<String>,
    /// Author/uploader name.
    author: Option<String>,
    /// Short description.
    description: Option<String>,
    /// URL string for the mod's web page.
    url: Option<String>,
    /// Category label (e.g. `"transport"`, `"industry"`).
    category: Option<String>,
    /// Compatibility info (e.g. `"1.10.0"` or `"any"`).
    compatibility: Option<String>,
    /// Direct download URL for the file.
    download_url: Option<String>,
    /// File size in bytes.
    filesize: Option<u64>,
    /// ISO 8601 creation timestamp.
    created_at: Option<String>,
    /// ISO 8601 last-updated timestamp.
    updated_at: Option<String>,
}

/// Fetcher for BaNaNaS mod metadata.
///
/// Connects to the OpenTTD BaNaNaS content API to retrieve mod
/// listings, search for mods, and resolve download URLs.
pub struct BananasFetcher {
    /// HTTP client used for requests.
    client: reqwest::Client,
}

impl BananasFetcher {
    /// Creates a new `BananasFetcher` with a default HTTP client.
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .user_agent("otmp-core-manager/0.1.0")
            .build()
            .expect("Failed to build HTTP client");

        Self { client }
    }

    /// Creates a new `BananasFetcher` with a custom HTTP client.
    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// Returns the API path segment for a given [`ModType`].
    fn mod_type_path(mod_type: &ModType) -> &'static str {
        match mod_type {
            ModType::NewGRF => "newgrf",
            ModType::AI => "ai",
            ModType::GameScript => "gs",
            ModType::MusicSet => "music",
        }
    }

    /// Fetches mods of a specific type.
    ///
    /// Iterates over all pages of the BaNaNaS API to collect every
    /// available mod of the given type.
    pub async fn fetch_mods(&self, mod_type: ModType) -> Result<Vec<ModInfo>> {
        let path = Self::mod_type_path(&mod_type);
        let first_url = format!("{BANANAS_API_BASE}/{path}?limit={DEFAULT_LIMIT}&offset=0");

        info!("Fetching BaNaNaS {mod_type} mods from {path}");

        let mut all_items = Vec::new();
        let mut next_url: Option<String> = Some(first_url);

        while let Some(url) = next_url.take() {
            debug!("Fetching BaNaNaS page: {url}");

            let response = self.client.get(&url).send().await.map_err(|e| {
                warn!("Failed to fetch BaNaNaS page {url}: {e}");
                Error::Network(e)
            })?;

            if !response.status().is_success() {
                return Err(Error::Parse(format!(
                    "BaNaNaS API returned HTTP {} for {url}",
                    response.status()
                )));
            }

            let page: BananasResponse = response.json().await.map_err(|e| {
                warn!("Failed to parse BaNaNaS response from {url}: {e}");
                Error::Parse(format!("Invalid BaNaNaS response JSON: {e}"))
            })?;

            let count = page.results.len();
            debug!("Parsed {count} items from BaNaNaS page");

            for item in page.results {
                all_items.push(Self::convert_item(item, mod_type.clone()));
            }

            // Follow the next page link if present
            if let Some(ref next) = page.next {
                if !next.is_empty() {
                    next_url = Some(next.clone());
                }
            }
        }

        info!(
            "Fetched {} total BaNaNaS {mod_type} mods",
            all_items.len()
        );

        Ok(all_items)
    }

    /// Searches mods by name or author.
    ///
    /// If `mod_type` is `Some`, the search is scoped to that type only.
    /// Otherwise, all mod types are searched.
    pub async fn search(&self, query: &str, mod_type: Option<ModType>) -> Result<Vec<ModInfo>> {
        let encoded_query: String = urlencoding(query);
        let mod_types = match mod_type {
            Some(mt) => vec![mt],
            None => vec![
                ModType::NewGRF,
                ModType::AI,
                ModType::GameScript,
                ModType::MusicSet,
            ],
        };

        let mut results = Vec::new();

        for mt in mod_types {
            let path = Self::mod_type_path(&mt);
            let url = format!(
                "{BANANAS_API_BASE}/{path}?limit={DEFAULT_LIMIT}&offset=0&search={encoded_query}"
            );

            debug!("Searching BaNaNaS {mt} mods for '{query}': {url}");

            let response = self.client.get(&url).send().await.map_err(|e| {
                warn!("Failed to search BaNaNaS {mt} mods: {e}");
                Error::Network(e)
            })?;

            if !response.status().is_success() {
                warn!("BaNaNaS search returned HTTP {} for {url}", response.status());
                continue;
            }

            let page: BananasResponse = response.json().await.map_err(|e| {
                warn!("Failed to parse BaNaNaS search response for {mt}: {e}");
                Error::Parse(format!("Invalid BaNaNaS search response JSON: {e}"))
            })?;

            for item in page.results {
                results.push(Self::convert_item(item, mt.clone()));
            }
        }

        info!("Search for '{query}' returned {} total results", results.len());
        Ok(results)
    }

    /// Gets the download URL for a specific mod version.
    ///
    /// Searches the BaNaNaS API for the given mod id and version,
    /// returning the download URL if a match is found.
    pub async fn get_download_url(&self, mod_id: &str, version: &str) -> Result<String> {
        // Search all mod types for the given id
        let mod_types = vec![
            ModType::NewGRF,
            ModType::AI,
            ModType::GameScript,
            ModType::MusicSet,
        ];

        for mt in mod_types {
            let path = Self::mod_type_path(&mt);
            let url = format!(
                "{BANANAS_API_BASE}/{path}?limit={DEFAULT_LIMIT}&offset=0&search={mod_id}"
            );

            debug!("Searching for download URL in {mt} mods: {url}");

            let response = match self.client.get(&url).send().await {
                Ok(r) => r,
                Err(e) => {
                    warn!("Failed to query BaNaNaS for mod {mod_id}: {e}");
                    continue;
                }
            };

            if !response.status().is_success() {
                continue;
            }

            let page: BananasResponse = match response.json().await {
                Ok(p) => p,
                Err(e) => {
                    warn!("Failed to parse BaNaNaS response for mod {mod_id}: {e}");
                    continue;
                }
            };

            for item in page.results {
                let item_id = item.id.as_deref().unwrap_or("");
                let item_version = item.version.as_deref().unwrap_or("");

                if item_id == mod_id && item_version == version {
                    if let Some(download_url) = item.download_url {
                        // If the URL is relative, prepend the API base
                        if download_url.starts_with('/') {
                            return Ok(format!("{BANANAS_API_BASE}{download_url}"));
                        }
                        return Ok(download_url);
                    }
                }
            }
        }

        Err(Error::VersionNotFound(format!(
            "Mod '{mod_id}' version '{version}' not found on BaNaNaS"
        )))
    }

    /// Converts a raw API item into a [`ModInfo`].
    fn convert_item(item: BananasItem, mod_type: ModType) -> ModInfo {
        let created_at = item.created_at.as_deref().and_then(|s| {
            chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.fZ")
                .or_else(|_| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S"))
                .or_else(|_| {
                    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                        .map(|d| d.and_hms_opt(0, 0, 0).unwrap())
                })
                .ok()
        });

        let updated_at = item.updated_at.as_deref().and_then(|s| {
            chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.fZ")
                .or_else(|_| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S"))
                .or_else(|_| {
                    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                        .map(|d| d.and_hms_opt(0, 0, 0).unwrap())
                })
                .ok()
        });

        let download_url = item.download_url.as_ref().map(|url| {
            if url.starts_with('/') {
                format!("{BANANAS_API_BASE}{url}")
            } else {
                url.clone()
            }
        });

        ModInfo {
            id: item.id.unwrap_or_default(),
            name: item.name.unwrap_or_default(),
            version: item.version.unwrap_or_default(),
            mod_type,
            author: item.author.unwrap_or_default(),
            description: item.description.unwrap_or_default(),
            url: item.url,
            category: item.category,
            compatibility: item.compatibility,
            download_url,
            filesize: item.filesize,
            created_at,
            updated_at,
        }
    }
}

impl Default for BananasFetcher {
    fn default() -> Self {
        Self::new()
    }
}

/// URL-encodes a string for use in query parameters.
fn urlencoding(input: &str) -> String {
    // Simple URL encoding for search queries
    let mut result = String::with_capacity(input.len());
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            b' ' => {
                result.push_str("%20");
            }
            _ => {
                result.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_type_path() {
        assert_eq!(BananasFetcher::mod_type_path(&ModType::NewGRF), "newgrf");
        assert_eq!(BananasFetcher::mod_type_path(&ModType::AI), "ai");
        assert_eq!(BananasFetcher::mod_type_path(&ModType::GameScript), "gs");
        assert_eq!(BananasFetcher::mod_type_path(&ModType::MusicSet), "music");
    }

    #[test]
    fn test_search_query_construction() {
        // For a search on NewGRF mods with query "station", the URL should be:
        // https://bananas.openttd.org/api/newgrf?limit=100&offset=0&search=station
        let query = "station";
        let encoded = urlencoding(query);
        let mt = ModType::NewGRF;
        let path = BananasFetcher::mod_type_path(&mt);
        let url = format!(
            "{BANANAS_API_BASE}/{path}?limit={DEFAULT_LIMIT}&offset=0&search={encoded}"
        );
        let expected = "https://bananas.openttd.org/api/newgrf?limit=100&offset=0&search=station";
        assert_eq!(url, expected);

        // Query with spaces should be URL-encoded
        let query = "urban transit";
        let encoded = urlencoding(query);
        let url = format!(
            "{BANANAS_API_BASE}/{path}?limit={DEFAULT_LIMIT}&offset=0&search={encoded}"
        );
        let expected = "https://bananas.openttd.org/api/newgrf?limit=100&offset=0&search=urban%20transit";
        assert_eq!(url, expected);
    }

    #[test]
    fn test_url_encoding() {
        assert_eq!(urlencoding("station"), "station");
        assert_eq!(urlencoding("urban transit"), "urban%20transit");
        assert_eq!(urlencoding("a+b"), "a%2Bb");
        assert_eq!(urlencoding("hello_world"), "hello_world");
    }

    #[test]
    fn test_convert_item_basic() {
        let item = BananasItem {
            id: Some("424f4f54".to_string()),
            name: Some("Test Mod".to_string()),
            version: Some("1.0.0".to_string()),
            author: Some("TestAuthor".to_string()),
            description: Some("A test mod".to_string()),
            url: Some("https://example.com/mod".to_string()),
            category: Some("transport".to_string()),
            compatibility: Some("1.10.0".to_string()),
            download_url: Some("/api/newgrf/download/424f4f54".to_string()),
            filesize: Some(12345),
            created_at: Some("2024-01-15T10:30:00Z".to_string()),
            updated_at: None,
        };

        let mod_info = BananasFetcher::convert_item(item, ModType::NewGRF);

        assert_eq!(mod_info.id, "424f4f54");
        assert_eq!(mod_info.name, "Test Mod");
        assert_eq!(mod_info.version, "1.0.0");
        assert_eq!(mod_info.mod_type, ModType::NewGRF);
        assert_eq!(mod_info.author, "TestAuthor");
        assert_eq!(mod_info.description, "A test mod");
        assert_eq!(mod_info.url, Some("https://example.com/mod".to_string()));
        assert_eq!(mod_info.category, Some("transport".to_string()));
        assert_eq!(mod_info.compatibility, Some("1.10.0".to_string()));
        assert_eq!(
            mod_info.download_url,
            Some("https://bananas.openttd.org/api/api/newgrf/download/424f4f54".to_string())
        );
        assert_eq!(mod_info.filesize, Some(12345));
        assert!(mod_info.created_at.is_some());
        assert!(mod_info.updated_at.is_none());
    }

    #[test]
    fn test_convert_item_absolute_url() {
        // If the download URL is already absolute, it should not be prefixed
        let item = BananasItem {
            id: Some("abc123".to_string()),
            name: Some("Absolute Mod".to_string()),
            version: Some("2.0.0".to_string()),
            author: Some("Author".to_string()),
            description: Some("Desc".to_string()),
            url: None,
            category: None,
            compatibility: None,
            download_url: Some("https://cdn.bananas.openttd.org/file/abc123.tar.gz".to_string()),
            filesize: None,
            created_at: None,
            updated_at: None,
        };

        let mod_info = BananasFetcher::convert_item(item, ModType::AI);

        assert_eq!(
            mod_info.download_url,
            Some("https://cdn.bananas.openttd.org/file/abc123.tar.gz".to_string())
        );
    }

    #[test]
    fn test_mod_type_display() {
        assert_eq!(ModType::NewGRF.to_string(), "newgrf");
        assert_eq!(ModType::AI.to_string(), "ai");
        assert_eq!(ModType::GameScript.to_string(), "gamescript");
        assert_eq!(ModType::MusicSet.to_string(), "music");
    }
}