//! CityMania Client (CMClient) version fetcher.
//!
//! Fetches version information from the CityMania Client GitHub releases
//! page at `https://api.github.com/repos/OpenTTD/OpenTTD-cmclient/releases`.
//! Handles pagination via per-page limits and maps GitHub release assets
//! to [`DownloadAsset`] entries.

use async_trait::async_trait;
use log::{debug, info, warn};
use serde::Deserialize;

use crate::source::VersionFetcher;
use crate::version::{DownloadAsset, VersionInfo, VersionSource, VersionType};
use crate::{Error, Result};

/// GitHub API base URL for CMClient releases.
const GITHUB_API_BASE: &str =
    "https://api.github.com/repos/OpenTTD/OpenTTD-cmclient/releases";

/// HTTP client timeout in seconds.
const REQUEST_TIMEOUT_SECS: u64 = 30;

/// Maximum number of releases to fetch per page.
const PER_PAGE: u32 = 100;

/// GitHub API release representation.
#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    name: Option<String>,
    prerelease: bool,
    published_at: Option<String>,
    body: Option<String>,
    assets: Vec<GitHubAsset>,
}

/// GitHub API asset representation.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
    size: Option<i64>,
    content_type: Option<String>,
}

/// Fetcher for CityMania Client releases.
///
/// Uses the GitHub Releases API to retrieve version information for
/// CMClient.  Supports pagination to fetch all available releases.
pub struct CmClientFetcher {
    /// HTTP client used for requests.
    client: reqwest::Client,
}

impl CmClientFetcher {
    /// Creates a new `CmClientFetcher` with a default HTTP client.
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .user_agent("otmp-core-manager/0.1.0")
            .build()
            .expect("Failed to build HTTP client");

        Self { client }
    }

    /// Creates a new `CmClientFetcher` with a custom HTTP client.
    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// Parses a CMClient tag name to extract the semver version.
    ///
    /// Tags are typically in the form `v0.1.0` or `v0.1.0-beta1`.
    fn parse_version(tag_name: &str) -> Result<semver::Version> {
        let version_str = tag_name
            .strip_prefix('v')
            .unwrap_or(tag_name);

        // Normalize pre-release tags: "beta1" -> "beta.1"
        let normalized = version_str
            .replace("-beta", "-beta.")
            .replace("-rc", "-rc.")
            .replace("-alpha", "-alpha.");

        semver::Version::parse(&normalized).map_err(|e| {
            warn!("Failed to parse CMClient version '{tag_name}': {e}");
            Error::Parse(format!("Invalid CMClient version '{tag_name}': {e}"))
        })
    }

    /// Determines the [`VersionType`] from a tag name.
    fn classify_version(tag_name: &str) -> VersionType {
        let lower = tag_name.to_lowercase();
        if lower.contains("-rc") {
            VersionType::ReleaseCandidate
        } else if lower.contains("-beta") {
            VersionType::Beta
        } else if lower.contains("-alpha") {
            VersionType::PreRelease
        } else {
            VersionType::Stable
        }
    }

    /// Extracts a platform identifier from an asset filename.
    fn extract_platform(filename: &str) -> String {
        let lower = filename.to_lowercase();
        if lower.contains("linux") || lower.contains("ubuntu") || lower.ends_with(".tar.xz") {
            if lower.contains("x86_64") || lower.contains("amd64") {
                "linux-amd64".to_string()
            } else if lower.contains("aarch64") || lower.contains("arm64") {
                "linux-aarch64".to_string()
            } else {
                "linux".to_string()
            }
        } else if lower.contains("macos") || lower.contains("osx") || lower.contains("darwin") {
            if lower.contains("universal") {
                "macos-universal".to_string()
            } else {
                "macos".to_string()
            }
        } else if lower.contains("windows") || lower.contains("win") {
            if lower.contains("arm64") {
                "windows-arm64".to_string()
            } else if lower.contains("x86_64") || lower.contains("win64") {
                "windows-win64".to_string()
            } else {
                "windows-win32".to_string()
            }
        } else {
            "unknown".to_string()
        }
    }

    /// Fetches a single page of GitHub releases.
    async fn fetch_page(&self, page: u32) -> Result<Vec<GitHubRelease>> {
        let url = format!("{GITHUB_API_BASE}?per_page={PER_PAGE}&page={page}");
        debug!("Fetching CMClient releases page {page}: {url}");

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .map_err(|e| {
                warn!("Failed to fetch CMClient releases page {page}: {e}");
                Error::Network(e)
            })?;

        if !response.status().is_success() {
            return Err(Error::Parse(format!(
                "GitHub API returned HTTP {} for page {page}",
                response.status()
            )));
        }

        let releases: Vec<GitHubRelease> = response.json().await.map_err(|e| {
            warn!("Failed to parse GitHub releases JSON page {page}: {e}");
            Error::Parse(format!("Invalid GitHub releases JSON: {e}"))
        })?;

        Ok(releases)
    }
}

impl Default for CmClientFetcher {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl VersionFetcher for CmClientFetcher {
    fn source(&self) -> VersionSource {
        VersionSource::CmClient
    }

    fn name(&self) -> &'static str {
        "CityMania Client"
    }

    async fn fetch_versions(&self) -> Result<Vec<VersionInfo>> {
        info!("Fetching CMClient releases from GitHub API");

        let mut all_releases = Vec::new();
        let mut page = 1u32;

        loop {
            let releases = self.fetch_page(page).await?;
            let count = releases.len();
            all_releases.extend(releases);

            if count < PER_PAGE as usize {
                break;
            }
            page += 1;
        }

        info!(
            "Fetched {} total CMClient releases from GitHub",
            all_releases.len()
        );

        let mut versions = Vec::with_capacity(all_releases.len());

        for release in &all_releases {
            let version = Self::parse_version(&release.tag_name)?;
            let version_type = Self::classify_version(&release.tag_name);

            let release_date = release.published_at.as_ref().and_then(|d| {
                chrono::DateTime::parse_from_rfc3339(d)
                    .ok()
                    .map(|dt| dt.naive_utc())
            });

            let mut downloads = Vec::new();

            for asset in &release.assets {
                // Skip source archives
                if asset.name.contains("source") {
                    continue;
                }

                let platform = Self::extract_platform(&asset.name);

                downloads.push(DownloadAsset {
                    name: asset.name.clone(),
                    url: asset.browser_download_url.clone(),
                    platform,
                    size: asset.size.map(|s| s as u64),
                    checksum: None,
                    checksum_type: None,
                });
            }

            let name = release
                .name
                .clone()
                .unwrap_or_else(|| release.tag_name.clone());

            versions.push(VersionInfo {
                id: uuid::Uuid::new_v4(),
                source: VersionSource::CmClient,
                version,
                version_type,
                name,
                release_date,
                downloads,
                changelog: release.body.clone(),
                is_prerelease: release.prerelease,
            });
        }

        info!(
            "Successfully fetched {} CMClient versions",
            versions.len()
        );
        Ok(versions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        assert_eq!(
            CmClientFetcher::parse_version("v0.1.0").unwrap(),
            semver::Version::new(0, 1, 0)
        );
        assert_eq!(
            CmClientFetcher::parse_version("v0.1.0-beta1").unwrap(),
            semver::Version::parse("0.1.0-beta.1").unwrap()
        );
        assert_eq!(
            CmClientFetcher::parse_version("0.1.0").unwrap(),
            semver::Version::new(0, 1, 0)
        );
    }

    #[test]
    fn test_parse_version_invalid() {
        assert!(CmClientFetcher::parse_version("not-a-version").is_err());
    }

    #[test]
    fn test_classify_version() {
        assert_eq!(
            CmClientFetcher::classify_version("v0.1.0"),
            VersionType::Stable
        );
        assert_eq!(
            CmClientFetcher::classify_version("v0.1.0-beta1"),
            VersionType::Beta
        );
        assert_eq!(
            CmClientFetcher::classify_version("v0.1.0-RC1"),
            VersionType::ReleaseCandidate
        );
        assert_eq!(
            CmClientFetcher::classify_version("v0.1.0-alpha1"),
            VersionType::PreRelease
        );
    }

    #[test]
    fn test_extract_platform() {
        assert_eq!(
            CmClientFetcher::extract_platform("openttd-cmclient-0.1.0-windows-win64.zip"),
            "windows-win64"
        );
        assert_eq!(
            CmClientFetcher::extract_platform("openttd-cmclient-0.1.0-linux-amd64.tar.xz"),
            "linux-amd64"
        );
        assert_eq!(
            CmClientFetcher::extract_platform("openttd-cmclient-0.1.0-macos-universal.dmg"),
            "macos-universal"
        );
        assert_eq!(
            CmClientFetcher::extract_platform("openttd-cmclient-0.1.0-windows-arm64.zip"),
            "windows-arm64"
        );
        assert_eq!(
            CmClientFetcher::extract_platform("openttd-cmclient-0.1.0-linux-aarch64.tar.xz"),
            "linux-aarch64"
        );
        assert_eq!(
            CmClientFetcher::extract_platform("openttd-cmclient-0.1.0-source.tar.gz"),
            "unknown"
        );
    }

    #[test]
    fn test_github_api_url_construction() {
        // The fetch_page function constructs URLs like:
        // {GITHUB_API_BASE}?per_page={PER_PAGE}&page={page}
        let page = 1u32;
        let expected = format!(
            "https://api.github.com/repos/OpenTTD/OpenTTD-cmclient/releases?per_page=100&page={page}"
        );
        let actual = format!(
            "{GITHUB_API_BASE}?per_page={PER_PAGE}&page={page}"
        );
        assert_eq!(expected, actual);

        let page = 3u32;
        let expected = format!(
            "https://api.github.com/repos/OpenTTD/OpenTTD-cmclient/releases?per_page=100&page={page}"
        );
        let actual = format!(
            "{GITHUB_API_BASE}?per_page={PER_PAGE}&page={page}"
        );
        assert_eq!(expected, actual);
    }
}