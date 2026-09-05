//! Local manifest cache for version lists.
//!
//! Caches fetched version information to disk as JSON files, reducing
//! network requests on subsequent runs.  Cache entries are invalidated
//! based on a configurable time-to-live (TTL, default 30 minutes).

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use log::{debug, info, warn};

use crate::version::VersionInfo;
use crate::Result;

/// Default TTL for cache entries: 30 minutes.
const DEFAULT_TTL_MINUTES: u64 = 30;

/// File name for the cache file.
const CACHE_FILE_NAME: &str = "version_cache.json";

/// A cached manifest entry.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CacheEntry {
    /// Timestamp (UNIX epoch seconds) when this entry was cached.
    cached_at: u64,
    /// The cached version list.
    versions: Vec<VersionInfo>,
}

/// Cache for version manifest data.
///
/// Stores version lists from various sources to disk and provides
/// TTL-based invalidation.
#[derive(Debug, Clone)]
pub struct ManifestCache {
    /// Directory where the cache file is stored.
    cache_dir: PathBuf,
    /// Time-to-live for cache entries.
    ttl: Duration,
}

impl ManifestCache {
    /// Creates a new `ManifestCache` with the given cache directory and
    /// default TTL (30 minutes).
    ///
    /// The cache directory is created if it does not exist.
    pub fn new<P: AsRef<Path>>(cache_dir: P) -> Self {
        let cache_dir = cache_dir.as_ref().to_path_buf();
        let _ = std::fs::create_dir_all(&cache_dir);

        Self {
            cache_dir,
            ttl: Duration::from_secs(DEFAULT_TTL_MINUTES * 60),
        }
    }

    /// Creates a new `ManifestCache` with a custom TTL.
    ///
    /// The cache directory is created if it does not exist.
    pub fn with_ttl<P: AsRef<Path>>(cache_dir: P, ttl: Duration) -> Self {
        let cache_dir = cache_dir.as_ref().to_path_buf();
        let _ = std::fs::create_dir_all(&cache_dir);

        Self { cache_dir, ttl }
    }

    /// Returns the path to the cache file.
    fn cache_path(&self) -> PathBuf {
        self.cache_dir.join(CACHE_FILE_NAME)
    }

    /// Loads cached versions from disk.
    ///
    /// Returns `None` if the cache file does not exist, is corrupt, or
    /// has expired (according to the configured TTL).
    pub fn load(&self) -> Result<Option<Vec<VersionInfo>>> {
        let cache_path = self.cache_path();

        if !cache_path.exists() {
            debug!("Cache file not found at {:?}", cache_path);
            return Ok(None);
        }

        // Check if the cache is still valid based on file modification time
        if !self.is_valid() {
            debug!("Cache has expired, ignoring cached file");
            return Ok(None);
        }

        let data = std::fs::read_to_string(&cache_path).map_err(|e| {
            warn!("Failed to read cache file {:?}: {e}", cache_path);
            e
        })?;

        let entry: CacheEntry = serde_json::from_str(&data).map_err(|e| {
            warn!("Failed to parse cache file {:?}: {e}", cache_path);
            e
        })?;

        // Check TTL based on the stored timestamp
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        if now.saturating_sub(entry.cached_at) > self.ttl.as_secs() {
            debug!("Cache entry has expired (cached at {}, now {})", entry.cached_at, now);
            return Ok(None);
        }

        info!("Loaded {} cached versions from {:?}", entry.versions.len(), cache_path);
        Ok(Some(entry.versions))
    }

    /// Saves version data to the cache file.
    ///
    /// Overwrites any existing cache file.
    pub fn save(&self, versions: &[VersionInfo]) -> Result<()> {
        let cache_path = self.cache_path();

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let entry = CacheEntry {
            cached_at: now,
            versions: versions.to_vec(),
        };

        let data = serde_json::to_string_pretty(&entry).map_err(|e| {
            warn!("Failed to serialize cache data: {e}");
            e
        })?;

        std::fs::write(&cache_path, &data).map_err(|e| {
            warn!("Failed to write cache file {:?}: {e}", cache_path);
            e
        })?;

        info!(
            "Saved {} versions to cache at {:?}",
            versions.len(),
            cache_path
        );
        Ok(())
    }

    /// Checks whether the cache file exists and has not expired.
    ///
    /// Expiration is based on the file's modification time compared to
    /// the configured TTL.
    pub fn is_valid(&self) -> bool {
        let cache_path = self.cache_path();

        if !cache_path.exists() {
            return false;
        }

        let metadata = match std::fs::metadata(&cache_path) {
            Ok(m) => m,
            Err(_) => return false,
        };

        let modified = match metadata.modified() {
            Ok(t) => t,
            Err(_) => return false,
        };

        let elapsed = match SystemTime::now().duration_since(modified) {
            Ok(d) => d,
            Err(_) => return false,
        };

        let valid = elapsed < self.ttl;
        if !valid {
            debug!(
                "Cache expired: modified {:?} ago, TTL is {:?}",
                elapsed, self.ttl
            );
        }
        valid
    }

    /// Invalidates (clears) the cache by deleting the cache file.
    pub fn invalidate(&self) -> Result<()> {
        let cache_path = self.cache_path();

        if cache_path.exists() {
            std::fs::remove_file(&cache_path).map_err(|e| {
                warn!("Failed to remove cache file {:?}: {e}", cache_path);
                e
            })?;
            info!("Cache invalidated at {:?}", cache_path);
        }

        Ok(())
    }

    /// Returns the configured TTL.
    pub fn ttl(&self) -> Duration {
        self.ttl
    }

    /// Sets a new TTL for the cache.
    pub fn set_ttl(&mut self, ttl: Duration) {
        self.ttl = ttl;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::version::{DownloadAsset, VersionSource, VersionType};
    use semver::Version;
    use tempfile::TempDir;

    /// Helper to create a sample [`VersionInfo`] for testing.
    fn sample_version(name: &str) -> VersionInfo {
        VersionInfo {
            id: uuid::Uuid::new_v4(),
            source: VersionSource::Official,
            version: Version::parse(name).unwrap_or(Version::new(1, 0, 0)),
            version_type: VersionType::Stable,
            name: name.to_string(),
            release_date: None,
            downloads: vec![DownloadAsset {
                name: format!("openttd-{name}-windows-win64.zip"),
                url: format!("https://cdn.openttd.org/openttd-releases/{name}/openttd-{name}-windows-win64.zip"),
                platform: "windows-win64".to_string(),
                size: Some(1024),
                checksum: Some("abc123".to_string()),
                checksum_type: Some("sha256".to_string()),
            }],
            changelog: None,
            is_prerelease: false,
        }
    }

    #[test]
    fn test_save_and_load() {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let cache = ManifestCache::new(tmp_dir.path());

        let versions = vec![sample_version("1.0.0"), sample_version("2.0.0")];

        // Save
        cache.save(&versions).expect("Failed to save cache");

        // Load
        let loaded = cache
            .load()
            .expect("Failed to load cache")
            .expect("Cache should have data");

        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].name, "1.0.0");
        assert_eq!(loaded[1].name, "2.0.0");
    }

    #[test]
    fn test_cache_invalidation() {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let cache = ManifestCache::with_ttl(tmp_dir.path(), Duration::from_secs(0));

        let versions = vec![sample_version("1.0.0")];
        cache.save(&versions).expect("Failed to save cache");

        // TTL is 0, so cache should be invalid
        assert!(!cache.is_valid());

        // Load should return None
        let loaded = cache.load().expect("Failed to load cache");
        assert!(loaded.is_none(), "Cache should be invalid due to TTL");
    }

    #[test]
    fn test_cache_validity() {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let cache = ManifestCache::with_ttl(tmp_dir.path(), Duration::from_secs(3600));

        let versions = vec![sample_version("1.0.0")];
        cache.save(&versions).expect("Failed to save cache");

        // Should be valid
        assert!(cache.is_valid());
    }

    #[test]
    fn test_invalidate_explicit() {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let cache = ManifestCache::new(tmp_dir.path());

        let versions = vec![sample_version("1.0.0")];
        cache.save(&versions).expect("Failed to save cache");
        assert!(cache.is_valid());

        cache.invalidate().expect("Failed to invalidate cache");
        assert!(!cache.is_valid());
    }

    #[test]
    fn test_cache_missing() {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let cache = ManifestCache::new(tmp_dir.path());

        let loaded = cache.load().expect("Failed to load cache");
        assert!(loaded.is_none());
    }

    #[test]
    fn test_ttl_accessors() {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut cache = ManifestCache::new(tmp_dir.path());

        assert_eq!(cache.ttl(), Duration::from_secs(DEFAULT_TTL_MINUTES * 60));

        cache.set_ttl(Duration::from_secs(60));
        assert_eq!(cache.ttl(), Duration::from_secs(60));
    }
}