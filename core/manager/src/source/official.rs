//! Official OpenTTD version fetcher.
//!
//! Fetches version information from the OpenTTD CDN at
//! `https://cdn.openttd.org/openttd-releases/`.  Uses the `latest.yaml`
//! manifest for the version list and per-version `manifest.yaml` files
//! for download asset details.

use async_trait::async_trait;
use log::{debug, info, warn};
use serde::Deserialize;

use crate::source::VersionFetcher;
use crate::version::{DownloadAsset, VersionInfo, VersionSource, VersionType};
use crate::{Error, Result};

/// Base URL for the OpenTTD releases CDN.
const CDN_BASE: &str = "https://cdn.openttd.org/openttd-releases";

/// HTTP client timeout in seconds.
const REQUEST_TIMEOUT_SECS: u64 = 30;

/// YAML structure of `latest.yaml`.
#[derive(Debug, Deserialize)]
struct LatestYaml {
    latest: Vec<LatestEntry>,
}

/// A single entry in `latest.yaml`.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct LatestEntry {
    version: String,
    name: String,
    category: String,
    date: Option<String>,
}

/// YAML structure of a per-version `manifest.yaml`.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ManifestYaml {
    name: String,
    category: String,
    version: String,
    date: Option<String>,
    changelog: Option<String>,
    base: Option<String>,
    files: Option<Vec<ManifestFile>>,
}

/// A single file entry in a version manifest.
#[derive(Debug, Deserialize)]
struct ManifestFile {
    id: String,
    size: Option<u64>,
    md5sum: Option<String>,
    sha1sum: Option<String>,
    sha256sum: Option<String>,
}

/// Fetcher for official OpenTTD releases.
///
/// Connects to the OpenTTD CDN, retrieves the `latest.yaml` version
/// listing, and fetches per-version manifest files for download links.
pub struct OfficialFetcher {
    /// HTTP client used for requests.
    client: reqwest::Client,
}

impl OfficialFetcher {
    /// Creates a new `OfficialFetcher` with a default HTTP client.
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .user_agent("otmp-core-manager/0.1.0")
            .build()
            .expect("Failed to build HTTP client");

        Self { client }
    }

    /// Creates a new `OfficialFetcher` with a custom HTTP client.
    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// Parses a version string to determine its [`VersionType`].
    fn classify_version(version_str: &str) -> VersionType {
        let lower = version_str.to_lowercase();
        if lower.contains("-rc") || lower.starts_with("rc") {
            VersionType::ReleaseCandidate
        } else if lower.contains("-beta") || lower.starts_with("beta") {
            VersionType::Beta
        } else if lower.contains("-alpha") || lower.starts_with("alpha") {
            VersionType::PreRelease
        } else {
            VersionType::Stable
        }
    }

    /// Extracts the platform identifier from a file name.
    ///
    /// For example, `openttd-15.3-windows-win64.exe` yields `windows-win64`.
    fn extract_platform(filename: &str) -> String {
        // Strip common extensions
        let stem = filename
            .trim_end_matches(".exe")
            .trim_end_matches(".zip")
            .trim_end_matches(".tar.xz")
            .trim_end_matches(".tar.gz")
            .trim_end_matches(".dmg");

        // Try to find the platform part after the version and a hyphen
        // Format: openttd-<version>-<platform>.<ext>
        // or: openttd-<version>-<platform>.tar.xz
        if let Some(pos) = stem.find('-') {
            // Skip the first part (openttd) and version
            let rest = &stem[pos + 1..];
            if let Some(platform_start) = rest.find('-') {
                let platform = &rest[platform_start + 1..];
                if !platform.is_empty() && !platform.contains('-') {
                    return platform.to_string();
                }
                // Handle multi-part platforms like "linux-generic-amd64"
                if let Some(dash_pos) = platform.find('-') {
                    let prefix = &platform[..dash_pos];
                    if prefix == "linux" || prefix == "windows" || prefix == "macos" {
                        return platform.to_string();
                    }
                }
            }
        }

        // Fallback: use the filename stem
        stem.to_string()
    }

    /// Fetches the manifest.yaml for a specific version directory.
    ///
    /// Returns the manifest content as a [`ManifestYaml`], or `None` if
    /// the manifest could not be fetched (e.g. 404).
    async fn fetch_manifest(&self, version_dir: &str) -> Result<Option<ManifestYaml>> {
        let url = format!("{CDN_BASE}/{version_dir}/manifest.yaml");
        debug!("Fetching manifest: {url}");

        let response = self.client.get(&url).send().await.map_err(|e| {
            warn!("Failed to fetch manifest {url}: {e}");
            Error::Network(e)
        })?;

        if !response.status().is_success() {
            debug!("Manifest not found at {url} (HTTP {})", response.status());
            return Ok(None);
        }

        let text = response.text().await.map_err(|e| {
            warn!("Failed to read manifest body from {url}: {e}");
            Error::Parse(format!("Failed to read manifest body: {e}"))
        })?;

        let manifest: ManifestYaml = serde_yaml::from_str(&text).map_err(|e| {
            warn!("Failed to parse manifest YAML from {url}: {e}");
            Error::Parse(format!("Invalid manifest YAML at {url}: {e}"))
        })?;

        Ok(Some(manifest))
    }
}

impl Default for OfficialFetcher {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl VersionFetcher for OfficialFetcher {
    fn source(&self) -> VersionSource {
        VersionSource::Official
    }

    fn name(&self) -> &'static str {
        "Official"
    }

    async fn fetch_versions(&self) -> Result<Vec<VersionInfo>> {
        let latest_url = format!("{CDN_BASE}/latest.yaml");
        debug!("Fetching latest versions from {latest_url}");

        let response = self
            .client
            .get(&latest_url)
            .send()
            .await
            .map_err(|e| {
                warn!("Failed to fetch latest.yaml: {e}");
                Error::Network(e)
            })?;

        if !response.status().is_success() {
            return Err(Error::Parse(format!(
                "Failed to fetch latest.yaml: HTTP {}",
                response.status()
            )));
        }

        let text = response.text().await.map_err(|e| {
            warn!("Failed to read latest.yaml body: {e}");
            Error::Parse(format!("Failed to read response body: {e}"))
        })?;

        let latest: LatestYaml = serde_yaml::from_str(&text).map_err(|e| {
            warn!("Failed to parse latest.yaml: {e}");
            Error::Parse(format!("Invalid latest.yaml: {e}"))
        })?;

        info!("Found {} version entries in latest.yaml", latest.latest.len());

        let mut versions = Vec::with_capacity(latest.latest.len());

        for entry in &latest.latest {
            let version_dir = &entry.version;
            let semver_str = if entry.version.contains('-') {
                // Strip pre-release suffix for semver parsing
                // e.g. "16.0-beta2" -> "16.0-beta.2" for semver
                // But we try the raw string first
                entry.version.clone()
            } else {
                // Ensure 3-part semver: "15.3" -> "15.3.0"
                let parts: Vec<&str> = entry.version.split('.').collect();
                match parts.len() {
                    1 => format!("{}.0.0", entry.version),
                    2 => format!("{}.0", entry.version),
                    _ => entry.version.clone(),
                }
            };

            // Parse version with semver tolerances
            let version = semver::Version::parse(&semver_str).or_else(|_| {
                // Try with pre-release parts normalized
                let normalized = entry
                    .version
                    .replace("-beta", "-beta.")
                    .replace("-rc", "-rc.")
                    .replace("-alpha", "-alpha.");
                semver::Version::parse(&normalized)
            }).map_err(|e| {
                warn!("Failed to parse version '{}': {e}", entry.version);
                Error::Parse(format!("Invalid version '{}': {e}", entry.version))
            })?;

            let version_type = Self::classify_version(&entry.version);

            // Parse the date
            let release_date = entry.date.as_ref().and_then(|d| {
                // Try multiple date formats
                chrono::NaiveDateTime::parse_from_str(d, "%Y-%m-%dT%H:%M:%S%:z")
                    .or_else(|_| chrono::NaiveDateTime::parse_from_str(d, "%Y-%m-%dT%H:%M:%S"))
                    .or_else(|_| {
                        chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d")
                            .map(|d| d.and_hms_opt(0, 0, 0).unwrap())
                    })
                    .ok()
            });

            // Fetch the manifest for download assets
            let manifest = self.fetch_manifest(version_dir).await?;

            let changelog_url = manifest
                .as_ref()
                .and_then(|m| m.changelog.as_ref())
                .map(|cl| format!("{CDN_BASE}/{version_dir}/{cl}"));

            let is_prerelease = version_type != VersionType::Stable;

            let mut downloads = Vec::new();

            if let Some(ref m) = manifest {
                if let Some(ref files) = m.files {
                    // Only include binary files (not source/docs)
                    for file in files {
                        if file.id.contains("source")
                            || file.id.contains("docs-ai")
                            || file.id.contains("docs-gs")
                            || file.id.contains("docs.")
                        {
                            continue;
                        }

                        let platform = Self::extract_platform(&file.id);
                        let url = format!("{CDN_BASE}/{version_dir}/{}", file.id);

                        // Prefer sha256, fall back to md5
                        let (checksum, checksum_type) = if let Some(ref sha256) = file.sha256sum {
                            (Some(sha256.clone()), Some("sha256".to_string()))
                        } else if let Some(ref sha1) = file.sha1sum {
                            (Some(sha1.clone()), Some("sha1".to_string()))
                        } else if let Some(ref md5) = file.md5sum {
                            (Some(md5.clone()), Some("md5".to_string()))
                        } else {
                            (None, None)
                        };

                        downloads.push(DownloadAsset {
                            name: file.id.clone(),
                            url,
                            platform,
                            size: file.size,
                            checksum,
                            checksum_type,
                        });
                    }
                }
            }

            versions.push(VersionInfo {
                id: uuid::Uuid::new_v4(),
                source: VersionSource::Official,
                version,
                version_type,
                name: entry.name.clone(),
                release_date,
                downloads,
                changelog: changelog_url,
                is_prerelease,
            });
        }

        info!(
            "Successfully fetched {} official versions",
            versions.len()
        );
        Ok(versions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_version() {
        assert_eq!(OfficialFetcher::classify_version("15.3"), VersionType::Stable);
        assert_eq!(
            OfficialFetcher::classify_version("15.0-RC1"),
            VersionType::ReleaseCandidate
        );
        assert_eq!(
            OfficialFetcher::classify_version("16.0-beta2"),
            VersionType::Beta
        );
        assert_eq!(
            OfficialFetcher::classify_version("14.0-alpha1"),
            VersionType::PreRelease
        );
    }

    #[test]
    fn test_extract_platform() {
        assert_eq!(
            OfficialFetcher::extract_platform("openttd-15.3-windows-win64.exe"),
            "windows-win64"
        );
        assert_eq!(
            OfficialFetcher::extract_platform("openttd-15.3-linux-generic-amd64.tar.xz"),
            "linux-generic-amd64"
        );
        assert_eq!(
            OfficialFetcher::extract_platform("openttd-15.3-macos-universal.dmg"),
            "macos-universal"
        );
    }
}