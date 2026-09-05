//! Custom version import functionality.
//!
//! This module allows importing custom OpenTTD versions from various sources:
//! URLs, local files, and existing installations.  It provides version
//! detection from filenames, platform detection, and download/copy
//! operations.

use std::path::{Path, PathBuf};
use std::process::Command;

use log::{debug, info, warn};
use semver::Version;
use uuid::Uuid;

use crate::version::{DownloadAsset, VersionInfo, VersionSource, VersionType};
use crate::{Error, Result};

/// How to import a custom version.
///
/// Each variant describes a different origin for the version data.
#[derive(Debug, Clone)]
pub enum ImportSource {
    /// Download from a URL.
    Url(String),
    /// Import from a local file (zip, tar.gz, dmg, apk).
    LocalFile(PathBuf),
    /// Import from an existing OpenTTD installation directory.
    ExistingInstall(PathBuf),
}

/// Result of importing a custom version.
///
/// Contains the detected [`VersionInfo`], the local path where the
/// version data resides, and any non-fatal warnings that arose during
/// import.
#[derive(Debug, Clone)]
pub struct ImportResult {
    /// The detected version metadata.
    pub version: VersionInfo,
    /// Local filesystem path to the imported content.
    pub local_path: PathBuf,
    /// Non-fatal warnings encountered during import (e.g. undetected
    /// version).
    pub warnings: Vec<String>,
}

/// Custom version importer.
///
/// Downloads, copies, or detects OpenTTD versions from user-supplied
/// sources and produces [`VersionInfo`] metadata for each.
#[derive(Debug, Clone)]
pub struct CustomImporter {
    /// Directory for caching downloaded files.
    cache_dir: PathBuf,
    /// Directory for storing imported instances.
    instances_dir: PathBuf,
}

/// Known source prefixes for version detection.
const SOURCE_PREFIXES: &[&str] = &["openttd-", "jgrpp-", "cmclient-"];

/// Known platform identifiers embedded in distribution filenames.
const PLATFORM_IDS: &[&str] = &[
    "windows-win64",
    "windows-win32",
    "linux-generic-amd64",
    "linux-generic-i386",
    "macos-universal",
    "macos-x86_64",
    "macos-arm64",
    "source",
    "android",
];

impl CustomImporter {
    /// Creates a new `CustomImporter` with the given cache and instances
    /// directories.
    ///
    /// Both directories are created if they do not exist.
    pub fn new<P1: AsRef<Path>, P2: AsRef<Path>>(
        cache_dir: P1,
        instances_dir: P2,
    ) -> Self {
        let cache_dir = cache_dir.as_ref().to_path_buf();
        let instances_dir = instances_dir.as_ref().to_path_buf();

        let _ = std::fs::create_dir_all(&cache_dir);
        let _ = std::fs::create_dir_all(&instances_dir);

        Self {
            cache_dir,
            instances_dir,
        }
    }

    /// Import a version from any [`ImportSource`].
    ///
    /// Dispatches to the appropriate private method based on the source
    /// variant.
    pub async fn import(&self, source: ImportSource) -> Result<ImportResult> {
        match source {
            ImportSource::Url(url) => self.import_from_url(&url).await,
            ImportSource::LocalFile(path) => self.import_from_file(&path).await,
            ImportSource::ExistingInstall(path) => self.import_from_install(&path).await,
        }
    }

    /// Import a version by downloading from a URL.
    ///
    /// The file is saved to the cache directory and version metadata is
    /// inferred from the filename.
    async fn import_from_url(&self, url: &str) -> Result<ImportResult> {
        info!("Importing from URL: {}", url);

        // Extract filename from the URL's last path segment
        let filename = url
            .split('/')
            .last()
            .filter(|s| !s.is_empty())
            .unwrap_or("downloaded.tar.gz");

        let cache_path = self.cache_dir.join(filename);

        // Download the file
        debug!("Downloading {} to {:?}", url, cache_path);
        let response = reqwest::get(url).await.map_err(Error::Network)?;

        let bytes = response.bytes().await.map_err(Error::Network)?;

        tokio::fs::write(&cache_path, &bytes)
            .await
            .map_err(Error::Io)?;

        // Detect version and platform from the filename
        let version_str = Self::detect_version_from_path(&cache_path);
        let platform = Self::detect_platform_from_filename(filename);

        let version = Self::build_version_info(
            &cache_path,
            version_str.as_deref(),
            platform.as_deref(),
            url,
            "custom-download",
        );

        let mut warnings = Vec::new();
        if version_str.is_none() {
            warnings.push(format!(
                "Could not detect version from filename '{}'. Using '0.0.0' as fallback.",
                filename
            ));
        }

        info!(
            "Downloaded custom version '{}' from {}",
            version.version, url
        );

        Ok(ImportResult {
            version,
            local_path: cache_path,
            warnings,
        })
    }

    /// Import a version from a local file.
    ///
    /// The file is copied into the instances directory and version
    /// metadata is inferred from the filename.
    async fn import_from_file(&self, path: &PathBuf) -> Result<ImportResult> {
        info!("Importing from file: {:?}", path);

        if !path.exists() {
            return Err(Error::VersionNotFound(format!(
                "File not found: {}",
                path.display()
            )));
        }

        // Determine destination filename
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        let dest_path = self.instances_dir.join(filename);

        // Copy file to instances directory
        tokio::fs::copy(path, &dest_path)
            .await
            .map_err(Error::Io)?;

        // Detect version and platform from the filename
        let version_str = Self::detect_version_from_path(path);
        let platform = Self::detect_platform_from_filename(filename);

        let version = Self::build_version_info(
            &dest_path,
            version_str.as_deref(),
            platform.as_deref(),
            &format!("file://{}", path.display()),
            "custom-file",
        );

        let mut warnings = Vec::new();
        if version_str.is_none() {
            warnings.push(format!(
                "Could not detect version from filename '{}'. Using '0.0.0' as fallback.",
                filename
            ));
        }

        info!(
            "Imported custom version '{}' from {:?}",
            version.version, path
        );

        Ok(ImportResult {
            version,
            local_path: dest_path,
            warnings,
        })
    }

    /// Import a version from an existing OpenTTD installation directory.
    ///
    /// Looks for the `openttd` (or `openttd.exe` on Windows) executable
    /// and attempts to extract the version string from `--version`
    /// output.
    async fn import_from_install(&self, path: &PathBuf) -> Result<ImportResult> {
        info!("Importing from install directory: {:?}", path);

        if !path.exists() || !path.is_dir() {
            return Err(Error::VersionNotFound(format!(
                "Installation directory not found: {}",
                path.display()
            )));
        }

        // Look for the openttd executable
        let exe_name = if cfg!(windows) {
            "openttd.exe"
        } else {
            "openttd"
        };
        let exe_path = path.join(exe_name);

        if !exe_path.exists() {
            return Err(Error::VersionNotFound(format!(
                "OpenTTD executable not found in {}",
                path.display()
            )));
        }

        // Try to get version from the executable
        let version_str = Self::get_version_from_executable(&exe_path);

        let mut warnings = Vec::new();

        let version = if let Some(ref ver_str) = version_str {
            let semver = Self::parse_version_string(ver_str).unwrap_or_else(|| {
                warn!("Could not parse version string '{}' as semver", ver_str);
                Version::new(0, 0, 0)
            });

            let version_type = Self::classify_version(ver_str);
            let is_prerelease = matches!(
                version_type,
                VersionType::PreRelease | VersionType::Beta | VersionType::Nightly
            );

            VersionInfo {
                id: Uuid::new_v4(),
                source: VersionSource::Custom("existing-install".to_string()),
                version: semver,
                version_type,
                name: format!("openttd-{}", ver_str),
                release_date: Some(chrono::Utc::now().naive_utc()),
                downloads: vec![],
                changelog: None,
                is_prerelease,
            }
        } else {
            warnings.push(
                "Could not detect version from installation. Using '0.0.0' as fallback."
                    .to_string(),
            );

            VersionInfo {
                id: Uuid::new_v4(),
                source: VersionSource::Custom("existing-install".to_string()),
                version: Version::new(0, 0, 0),
                version_type: VersionType::Custom,
                name: "openttd-0.0.0".to_string(),
                release_date: Some(chrono::Utc::now().naive_utc()),
                downloads: vec![],
                changelog: None,
                is_prerelease: true,
            }
        };

        info!(
            "Imported custom version '{}' from installation at {:?}",
            version.version, path
        );

        Ok(ImportResult {
            version,
            local_path: path.clone(),
            warnings,
        })
    }

    /// Detect version from filename or directory name.
    ///
    /// Looks for patterns like `openttd-14.1`, `jgrpp-0.59.1`,
    /// `cmclient-1.0` in the file or directory name.  Falls back to
    /// scanning for any dotted numeric version pattern.
    fn detect_version_from_path(path: &Path) -> Option<String> {
        let filename = path.file_name()?.to_str()?;
        // Strip compound tar extensions (.tar.xz, .tar.gz, .tar.bz2) first,
        // then the final single extension to get the clean base name.
        let name = if let Some(stem) = filename
            .strip_suffix(".tar.xz")
            .or_else(|| filename.strip_suffix(".tar.gz"))
            .or_else(|| filename.strip_suffix(".tar.bz2"))
            .or_else(|| filename.strip_suffix(".tar"))
        {
            stem
        } else {
            // Strip the last extension (e.g. .zip, .dmg, .apk, .exe)
            path.file_stem()?.to_str()?
        };

        // Try each known source prefix
        for prefix in SOURCE_PREFIXES {
            if let Some(rest) = name.strip_prefix(prefix) {
                // Scan segments delimited by '-' for the first version-like token
                if let Some(ver) = rest.split('-').find(|s| Self::looks_like_version(s)) {
                    return Some(ver.to_string());
                }
                // If no dash-delimited version found, scan the whole remainder
                if Self::looks_like_version(rest) {
                    return Some(rest.to_string());
                }
            }
        }

        // Fallback: scan for any dotted numeric pattern in the name
        Self::find_version_in_string(name)
    }

    /// Detect platform from a filename.
    ///
    /// Checks for known platform identifiers embedded in the filename
    /// (e.g. `windows-win64`, `linux-generic-amd64`, `macos-universal`).
    fn detect_platform_from_filename(name: &str) -> Option<String> {
        PLATFORM_IDS
            .iter()
            .find(|&&pid| name.contains(pid))
            .map(|&s| s.to_string())
    }

    // ------------------------------------------------------------------
    // Internal helpers
    // ------------------------------------------------------------------

    /// Returns `true` if the string looks like a version number.
    ///
    /// A version-like string must contain at least one digit, have at
    /// least one dot, be no longer than 20 characters, and consist only
    /// of alphanumeric characters, dots, and hyphens.
    fn looks_like_version(s: &str) -> bool {
        let has_digit = s.chars().any(|c| c.is_ascii_digit());
        let has_dot = s.contains('.');
        let not_too_long = s.len() <= 20;
        let valid_chars = s
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-');

        has_digit && has_dot && not_too_long && valid_chars
    }

    /// Scan a string for any dotted numeric version pattern (e.g.
    /// `14.1`, `0.59.1`, `1.0.0-beta.2`).
    fn find_version_in_string(s: &str) -> Option<String> {
        let chars: Vec<char> = s.chars().collect();
        let mut best: Option<String> = None;
        let mut best_len: usize = 0;

        let mut i = 0;
        while i < chars.len() {
            if chars[i].is_ascii_digit() {
                let start = i;
                let mut got_dot = false;
                while i < chars.len()
                    && (chars[i].is_ascii_digit()
                        || chars[i] == '.'
                        || (chars[i] == '-' && got_dot))
                {
                    if chars[i] == '.' {
                        got_dot = true;
                    }
                    i += 1;
                }
                if got_dot {
                    let candidate: String = chars[start..i].iter().collect();
                    // Only match if it starts with a digit
                    if candidate.starts_with(|c: char| c.is_ascii_digit())
                        && candidate.len() > best_len
                    {
                        best = Some(candidate.clone());
                        best_len = candidate.len();
                    }
                }
            } else {
                i += 1;
            }
        }

        best
    }

    /// Parse a version string to [`semver::Version`].
    ///
    /// Tries direct parsing first, then appends `.0` for two-component
    /// versions, and finally attempts to extract only the numeric parts.
    fn parse_version_string(s: &str) -> Option<Version> {
        // Try direct parse first
        if let Ok(v) = Version::parse(s) {
            return Some(v);
        }

        // Try with .0 appended (e.g. "14.1" -> "14.1.0")
        if let Ok(v) = Version::parse(&format!("{}.0", s)) {
            return Some(v);
        }

        // Try to extract just the numeric dotted parts
        let numeric: String = s
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '.')
            .collect();
        if !numeric.is_empty() {
            if let Ok(v) = Version::parse(&numeric) {
                return Some(v);
            }
            if let Ok(v) = Version::parse(&format!("{}.0", numeric)) {
                return Some(v);
            }
        }

        None
    }

    /// Classify a version string to determine the release type.
    fn classify_version(s: &str) -> VersionType {
        let lower = s.to_lowercase();
        if lower.contains("rc") || lower.contains("release-candidate") {
            VersionType::ReleaseCandidate
        } else if lower.contains("beta") {
            VersionType::Beta
        } else if lower.contains("alpha") || lower.contains("pre") || lower.contains("dev") {
            VersionType::PreRelease
        } else if lower.contains("nightly") {
            VersionType::Nightly
        } else {
            VersionType::Custom
        }
    }

    /// Get the version string from an OpenTTD executable by running
    /// `--version`.
    fn get_version_from_executable(exe_path: &Path) -> Option<String> {
        let output = Command::new(exe_path).arg("--version").output().ok()?;

        if !output.status.success() {
            return None;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let combined = format!("{}{}", stdout, stderr);

        // Scan each word for a version-like string
        for line in combined.lines() {
            for word in line.split_whitespace() {
                if Self::looks_like_version(word) {
                    return Some(word.to_string());
                }
            }
        }

        None
    }

    /// Build a [`VersionInfo`] from detected metadata.
    fn build_version_info(
        path: &Path,
        version_str: Option<&str>,
        platform: Option<&str>,
        url: &str,
        source_name: &str,
    ) -> VersionInfo {
        let (semver, version_type, name) = if let Some(vs) = version_str {
            let semver = Self::parse_version_string(vs).unwrap_or_else(|| {
                warn!("Could not parse version '{}' as semver; using 0.0.0", vs);
                Version::new(0, 0, 0)
            });
            let vt = Self::classify_version(vs);
            let name = format!("openttd-{}", vs);
            (semver, vt, name)
        } else {
            (Version::new(0, 0, 0), VersionType::Custom, "openttd-0.0.0".to_string())
        };

        let is_prerelease = matches!(
            version_type,
            VersionType::PreRelease
                | VersionType::Beta
                | VersionType::Nightly
                | VersionType::Custom
        );

        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let size = std::fs::metadata(path).ok().map(|m| m.len());

        let asset = DownloadAsset {
            name: filename,
            url: url.to_string(),
            platform: platform.unwrap_or("unknown").to_string(),
            size,
            checksum: None,
            checksum_type: None,
        };

        VersionInfo {
            id: Uuid::new_v4(),
            source: VersionSource::Custom(source_name.to_string()),
            version: semver,
            version_type,
            name,
            release_date: Some(chrono::Utc::now().naive_utc()),
            downloads: vec![asset],
            changelog: None,
            is_prerelease,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    // ------------------------------------------------------------------
    // Version detection tests
    // ------------------------------------------------------------------

    #[test]
    fn test_detect_version_official() {
        let cases = [
            ("openttd-14.1-windows-win64.zip", Some("14.1")),
            ("openttd-14.1-linux-generic-amd64.tar.xz", Some("14.1")),
            ("openttd-12.2-macos-universal.dmg", Some("12.2")),
            ("openttd-14.1-source.tar.xz", Some("14.1")),
            ("openttd-14.1.tar.xz", Some("14.1")),
        ];

        for (filename, expected) in &cases {
            let path = Path::new(filename);
            let result = CustomImporter::detect_version_from_path(path);
            assert_eq!(
                result.as_deref(),
                *expected,
                "Failed for filename: {}",
                filename
            );
        }
    }

    #[test]
    fn test_detect_version_jgrpp() {
        let cases = [
            ("jgrpp-0.59.1-windows-win64.zip", Some("0.59.1")),
            ("jgrpp-0.59.1-linux-generic-amd64.tar.xz", Some("0.59.1")),
            ("jgrpp-0.59.1-source.tar.xz", Some("0.59.1")),
        ];

        for (filename, expected) in &cases {
            let path = Path::new(filename);
            let result = CustomImporter::detect_version_from_path(path);
            assert_eq!(
                result.as_deref(),
                *expected,
                "Failed for filename: {}",
                filename
            );
        }
    }

    #[test]
    fn test_detect_version_cmclient() {
        let cases = [
            ("cmclient-1.0-windows-win64.zip", Some("1.0")),
            ("cmclient-1.0-linux-generic-amd64.tar.xz", Some("1.0")),
            ("cmclient-1.0.0-windows-win64.zip", Some("1.0.0")),
        ];

        for (filename, expected) in &cases {
            let path = Path::new(filename);
            let result = CustomImporter::detect_version_from_path(path);
            assert_eq!(
                result.as_deref(),
                *expected,
                "Failed for filename: {}",
                filename
            );
        }
    }

    #[test]
    fn test_detect_version_no_match() {
        let cases = [
            "random-file.zip",
            "openttd.tar.xz",
            "jgrpp.tar.xz",
        ];

        for filename in &cases {
            let path = Path::new(filename);
            let result = CustomImporter::detect_version_from_path(path);
            assert!(result.is_none(), "Expected None for filename: {}", filename);
        }
    }

    #[test]
    fn test_detect_version_semver_full() {
        let cases = [
            ("openttd-1.0.0-windows-win64.zip", Some("1.0.0")),
            ("openttd-2.3.4-linux.tar.xz", Some("2.3.4")),
            ("jgrpp-0.60.0-windows-win64.zip", Some("0.60.0")),
        ];

        for (filename, expected) in &cases {
            let path = Path::new(filename);
            let result = CustomImporter::detect_version_from_path(path);
            assert_eq!(
                result.as_deref(),
                *expected,
                "Failed for filename: {}",
                filename
            );
        }
    }

    // ------------------------------------------------------------------
    // Platform detection tests
    // ------------------------------------------------------------------

    #[test]
    fn test_detect_platform() {
        let cases = [
            ("openttd-14.1-windows-win64.zip", Some("windows-win64")),
            ("openttd-14.1-windows-win32.zip", Some("windows-win32")),
            ("openttd-14.1-linux-generic-amd64.tar.xz", Some("linux-generic-amd64")),
            ("openttd-14.1-linux-generic-i386.tar.xz", Some("linux-generic-i386")),
            ("openttd-14.1-macos-universal.dmg", Some("macos-universal")),
            ("openttd-14.1-macos-x86_64.dmg", Some("macos-x86_64")),
            ("openttd-14.1-macos-arm64.dmg", Some("macos-arm64")),
            ("openttd-14.1-source.tar.xz", Some("source")),
            ("openttd-14.1-android.apk", Some("android")),
            ("openttd-14.1.tar.xz", None),
        ];

        for (filename, expected) in &cases {
            let result = CustomImporter::detect_platform_from_filename(filename);
            assert_eq!(
                result.as_deref(),
                *expected,
                "Failed for filename: {}",
                filename
            );
        }
    }

    // ------------------------------------------------------------------
    // ImportSource tests
    // ------------------------------------------------------------------

    #[test]
    fn test_import_source_url() {
        let source = ImportSource::Url("https://cdn.openttd.org/openttd-14.1-windows-win64.zip".to_string());
        match source {
            ImportSource::Url(url) => {
                assert!(url.contains("openttd-14.1"));
            }
            _ => panic!("Expected Url variant"),
        }
    }

    #[test]
    fn test_import_source_local_file() {
        let source = ImportSource::LocalFile(PathBuf::from("openttd-14.1-windows-win64.zip"));
        match source {
            ImportSource::LocalFile(path) => {
                assert_eq!(path, PathBuf::from("openttd-14.1-windows-win64.zip"));
            }
            _ => panic!("Expected LocalFile variant"),
        }
    }

    #[test]
    fn test_import_source_existing_install() {
        let source = ImportSource::ExistingInstall(PathBuf::from("C:\\Program Files\\OpenTTD"));
        match source {
            ImportSource::ExistingInstall(path) => {
                assert_eq!(path, PathBuf::from("C:\\Program Files\\OpenTTD"));
            }
            _ => panic!("Expected ExistingInstall variant"),
        }
    }

    // ------------------------------------------------------------------
    // Version string parsing tests
    // ------------------------------------------------------------------

    #[test]
    fn test_parse_version_string() {
        assert_eq!(
            CustomImporter::parse_version_string("14.1"),
            Some(Version::new(14, 1, 0))
        );
        assert_eq!(
            CustomImporter::parse_version_string("14.1.0"),
            Some(Version::new(14, 1, 0))
        );
        assert_eq!(
            CustomImporter::parse_version_string("0.59.1"),
            Some(Version::new(0, 59, 1))
        );
        assert_eq!(
            CustomImporter::parse_version_string("1.0"),
            Some(Version::new(1, 0, 0))
        );
        assert_eq!(
            CustomImporter::parse_version_string("1.0.0"),
            Some(Version::new(1, 0, 0))
        );
        assert_eq!(
            CustomImporter::parse_version_string("1.0.0-beta.2"),
            Some(Version::parse("1.0.0-beta.2").unwrap())
        );
        assert!(CustomImporter::parse_version_string("not-a-version").is_none());
    }

    // ------------------------------------------------------------------
    // Classify version tests
    // ------------------------------------------------------------------

    #[test]
    fn test_classify_version() {
        assert_eq!(
            CustomImporter::classify_version("14.1"),
            VersionType::Custom
        );
        assert_eq!(
            CustomImporter::classify_version("14.1-rc1"),
            VersionType::ReleaseCandidate
        );
        assert_eq!(
            CustomImporter::classify_version("14.1-beta.2"),
            VersionType::Beta
        );
        assert_eq!(
            CustomImporter::classify_version("14.1-alpha.1"),
            VersionType::PreRelease
        );
        assert_eq!(
            CustomImporter::classify_version("14.1-pre"),
            VersionType::PreRelease
        );
        assert_eq!(
            CustomImporter::classify_version("14.1-dev"),
            VersionType::PreRelease
        );
        assert_eq!(
            CustomImporter::classify_version("14.1-nightly"),
            VersionType::Nightly
        );
        assert_eq!(
            CustomImporter::classify_version("14.1-release-candidate"),
            VersionType::ReleaseCandidate
        );
    }

    // ------------------------------------------------------------------
    // looks_like_version tests
    // ------------------------------------------------------------------

    #[test]
    fn test_looks_like_version() {
        assert!(CustomImporter::looks_like_version("14.1"));
        assert!(CustomImporter::looks_like_version("0.59.1"));
        assert!(CustomImporter::looks_like_version("1.0.0"));
        assert!(CustomImporter::looks_like_version("1.0.0-beta.2"));
        assert!(!CustomImporter::looks_like_version("openttd"));
        assert!(!CustomImporter::looks_like_version("windows-win64"));
        assert!(!CustomImporter::looks_like_version(""));
        assert!(!CustomImporter::looks_like_version("abc"));
    }

    // ------------------------------------------------------------------
    // ImportResult tests
    // ------------------------------------------------------------------

    #[test]
    fn test_import_result_creation() {
        let version = VersionInfo {
            id: Uuid::new_v4(),
            source: VersionSource::Custom("test".to_string()),
            version: Version::new(14, 1, 0),
            version_type: VersionType::Custom,
            name: "openttd-14.1".to_string(),
            release_date: None,
            downloads: vec![],
            changelog: None,
            is_prerelease: true,
        };

        let result = ImportResult {
            version: version.clone(),
            local_path: PathBuf::from("openttd-14.1-windows-win64.zip"),
            warnings: vec!["Test warning".to_string()],
        };

        assert_eq!(result.version.version, Version::new(14, 1, 0));
        assert_eq!(result.local_path, PathBuf::from("openttd-14.1-windows-win64.zip"));
        assert_eq!(result.warnings.len(), 1);
        assert_eq!(result.warnings[0], "Test warning");
    }

    // ------------------------------------------------------------------
    // CustomImporter creation tests
    // ------------------------------------------------------------------

    #[test]
    fn test_custom_importer_new() {
        let tmp = tempfile::TempDir::new().expect("Failed to create temp dir");
        let cache_dir = tmp.path().join("cache");
        let instances_dir = tmp.path().join("instances");

        let importer = CustomImporter::new(&cache_dir, &instances_dir);

        // Verify directories were created
        assert!(cache_dir.exists());
        assert!(instances_dir.exists());

        // Verify structure
        assert_eq!(importer.cache_dir, cache_dir);
        assert_eq!(importer.instances_dir, instances_dir);
    }
}