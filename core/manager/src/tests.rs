//! Integration tests for the `otmp-core-manager` crate.
//!
//! This module contains tests for version model serialization, version
//! comparison, and fetcher logic.

#[cfg(test)]
mod tests {
    use crate::cache::ManifestCache;
    use crate::version::{DownloadAsset, VersionInfo, VersionSource, VersionType};
    use semver::Version;
    use tempfile::TempDir;

    // ------------------------------------------------------------------
    // Version model serialization / deserialization tests
    // ------------------------------------------------------------------

    #[test]
    fn test_version_source_serde() {
        let sources = vec![
            VersionSource::Official,
            VersionSource::Jgrpp,
            VersionSource::CmClient,
            VersionSource::Custom("test-source".to_string()),
        ];

        for source in &sources {
            let json = serde_json::to_string(source).expect("Failed to serialize VersionSource");
            let deserialized: VersionSource =
                serde_json::from_str(&json).expect("Failed to deserialize VersionSource");
            assert_eq!(*source, deserialized);
        }
    }

    #[test]
    fn test_version_type_serde() {
        let types = vec![
            VersionType::Stable,
            VersionType::ReleaseCandidate,
            VersionType::Beta,
            VersionType::Nightly,
            VersionType::PreRelease,
            VersionType::Custom,
        ];

        for vt in &types {
            let json = serde_json::to_string(vt).expect("Failed to serialize VersionType");
            let deserialized: VersionType =
                serde_json::from_str(&json).expect("Failed to deserialize VersionType");
            assert_eq!(*vt, deserialized);
        }
    }

    #[test]
    fn test_version_info_serde_roundtrip() {
        let info = VersionInfo {
            id: uuid::Uuid::new_v4(),
            source: VersionSource::Official,
            version: Version::new(15, 3, 0),
            version_type: VersionType::Stable,
            name: "stable".to_string(),
            release_date: None,
            downloads: vec![
                DownloadAsset {
                    name: "openttd-15.3-windows-win64.exe".to_string(),
                    url: "https://cdn.openttd.org/openttd-releases/15.3/openttd-15.3-windows-win64.exe".to_string(),
                    platform: "windows-win64".to_string(),
                    size: Some(8916160),
                    checksum: Some("abc123".to_string()),
                    checksum_type: Some("sha256".to_string()),
                },
                DownloadAsset {
                    name: "openttd-15.3-linux-generic-amd64.tar.xz".to_string(),
                    url: "https://cdn.openttd.org/openttd-releases/15.3/openttd-15.3-linux-generic-amd64.tar.xz".to_string(),
                    platform: "linux-generic-amd64".to_string(),
                    size: Some(23046700),
                    checksum: None,
                    checksum_type: None,
                },
            ],
            changelog: Some("https://cdn.openttd.org/openttd-releases/15.3/changelog.md".to_string()),
            is_prerelease: false,
        };

        // Serialize to JSON
        let json = serde_json::to_string_pretty(&info).expect("Failed to serialize VersionInfo");
        assert!(!json.is_empty());

        // Deserialize back
        let deserialized: VersionInfo =
            serde_json::from_str(&json).expect("Failed to deserialize VersionInfo");

        assert_eq!(info.id, deserialized.id);
        assert_eq!(info.source, deserialized.source);
        assert_eq!(info.version, deserialized.version);
        assert_eq!(info.version_type, deserialized.version_type);
        assert_eq!(info.name, deserialized.name);
        assert_eq!(info.downloads.len(), deserialized.downloads.len());
        assert_eq!(info.downloads[0].name, deserialized.downloads[0].name);
        assert_eq!(info.downloads[0].url, deserialized.downloads[0].url);
        assert_eq!(info.changelog, deserialized.changelog);
        assert_eq!(info.is_prerelease, deserialized.is_prerelease);
    }

    #[test]
    fn test_version_info_with_dates() {
        use chrono::NaiveDateTime;

        let date = NaiveDateTime::parse_from_str("2026-04-04T19:57:00", "%Y-%m-%dT%H:%M:%S")
            .expect("Failed to parse date");

        let info = VersionInfo {
            id: uuid::Uuid::new_v4(),
            source: VersionSource::Official,
            version: Version::new(15, 3, 0),
            version_type: VersionType::Stable,
            name: "stable".to_string(),
            release_date: Some(date),
            downloads: vec![],
            changelog: None,
            is_prerelease: false,
        };

        let json = serde_json::to_string(&info).expect("Failed to serialize");
        let deserialized: VersionInfo =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(info.release_date, deserialized.release_date);
    }

    // ------------------------------------------------------------------
    // Version comparison tests
    // ------------------------------------------------------------------

    #[test]
    fn test_version_comparison() {
        let v1 = Version::new(15, 3, 0);
        let v2 = Version::new(16, 0, 0);
        let v3 = Version::new(15, 3, 0);

        assert!(v1 < v2);
        assert!(v2 > v1);
        assert_eq!(v1, v3);
    }

    #[test]
    fn test_version_with_pre_release() {
        let stable = Version::new(1, 0, 0);
        let beta = Version::parse("1.0.0-beta.2").expect("Failed to parse beta version");
        let rc = Version::parse("1.0.0-rc.1").expect("Failed to parse RC version");

        // Pre-release versions compare less than the release
        assert!(beta < stable);
        assert!(rc < stable);
        // RC sorts after beta
        assert!(beta < rc);
    }

    #[test]
    fn test_version_sorting() {
        let mut versions = vec![
            Version::new(1, 0, 0),
            Version::new(0, 1, 0),
            Version::new(2, 0, 0),
            Version::new(1, 5, 0),
        ];

        versions.sort();

        assert_eq!(versions, vec![
            Version::new(0, 1, 0),
            Version::new(1, 0, 0),
            Version::new(1, 5, 0),
            Version::new(2, 0, 0),
        ]);
    }

    // ------------------------------------------------------------------
    // VersionSource display tests
    // ------------------------------------------------------------------

    #[test]
    fn test_version_source_display() {
        assert_eq!(VersionSource::Official.to_string(), "official");
        assert_eq!(VersionSource::Jgrpp.to_string(), "jgrpp");
        assert_eq!(VersionSource::CmClient.to_string(), "cmclient");
        assert_eq!(
            VersionSource::Custom("test".to_string()).to_string(),
            "custom/test"
        );
    }

    // ------------------------------------------------------------------
    // VersionType display tests
    // ------------------------------------------------------------------

    #[test]
    fn test_version_type_display() {
        assert_eq!(VersionType::Stable.to_string(), "stable");
        assert_eq!(VersionType::ReleaseCandidate.to_string(), "release-candidate");
        assert_eq!(VersionType::Beta.to_string(), "beta");
        assert_eq!(VersionType::Nightly.to_string(), "nightly");
        assert_eq!(VersionType::PreRelease.to_string(), "pre-release");
        assert_eq!(VersionType::Custom.to_string(), "custom");
    }

    // ------------------------------------------------------------------
    // Cache integration tests
    // ------------------------------------------------------------------

    #[test]
    fn test_cache_save_load_roundtrip() {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let cache = ManifestCache::new(tmp_dir.path());

        let versions = vec![
            VersionInfo {
                id: uuid::Uuid::new_v4(),
                source: VersionSource::Official,
                version: Version::new(15, 3, 0),
                version_type: VersionType::Stable,
                name: "stable".to_string(),
                release_date: None,
                downloads: vec![],
                changelog: None,
                is_prerelease: false,
            },
        ];

        // Save and load
        cache.save(&versions).expect("Failed to save cache");
        let loaded = cache.load().expect("Failed to load cache");

        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].version, Version::new(15, 3, 0));
        assert_eq!(loaded[0].source, VersionSource::Official);
    }

    #[test]
    fn test_cache_expiry() {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let cache = ManifestCache::with_ttl(tmp_dir.path(), std::time::Duration::from_secs(0));

        let versions = vec![
            VersionInfo {
                id: uuid::Uuid::new_v4(),
                source: VersionSource::Official,
                version: Version::new(15, 3, 0),
                version_type: VersionType::Stable,
                name: "stable".to_string(),
                release_date: None,
                downloads: vec![],
                changelog: None,
                is_prerelease: false,
            },
        ];

        cache.save(&versions).expect("Failed to save cache");
        let loaded = cache.load().expect("Failed to load cache");

        // With TTL=0, the cache should be expired
        assert!(loaded.is_none());
    }

    #[test]
    fn test_cache_invalidate_and_reload() {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let cache = ManifestCache::new(tmp_dir.path());

        let versions = vec![
            VersionInfo {
                id: uuid::Uuid::new_v4(),
                source: VersionSource::Official,
                version: Version::new(15, 3, 0),
                version_type: VersionType::Stable,
                name: "stable".to_string(),
                release_date: None,
                downloads: vec![],
                changelog: None,
                is_prerelease: false,
            },
        ];

        cache.save(&versions).expect("Failed to save cache");
        assert!(cache.is_valid());

        cache.invalidate().expect("Failed to invalidate cache");
        assert!(!cache.is_valid());

        let loaded = cache.load().expect("Failed to load cache");
        assert!(loaded.is_none());
    }

    #[test]
    fn test_cache_with_multiple_versions() {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let cache = ManifestCache::new(tmp_dir.path());

        let versions: Vec<VersionInfo> = (1..=5)
            .map(|i| VersionInfo {
                id: uuid::Uuid::new_v4(),
                source: VersionSource::Official,
                version: Version::new(i, 0, 0),
                version_type: VersionType::Stable,
                name: format!("v{i}"),
                release_date: None,
                downloads: vec![],
                changelog: None,
                is_prerelease: false,
            })
            .collect();

        cache.save(&versions).expect("Failed to save cache");
        let loaded = cache.load().expect("Failed to load cache").unwrap();

        assert_eq!(loaded.len(), 5);
        assert_eq!(loaded[0].version, Version::new(1, 0, 0));
        assert_eq!(loaded[4].version, Version::new(5, 0, 0));
    }

    // ------------------------------------------------------------------
    // DownloadAsset tests
    // ------------------------------------------------------------------

    #[test]
    fn test_download_asset_serde() {
        let asset = DownloadAsset {
            name: "openttd-15.3-windows-win64.exe".to_string(),
            url: "https://cdn.openttd.org/openttd-releases/15.3/openttd-15.3-windows-win64.exe".to_string(),
            platform: "windows-win64".to_string(),
            size: Some(8916160),
            checksum: Some("abc123".to_string()),
            checksum_type: Some("sha256".to_string()),
        };

        let json = serde_json::to_string(&asset).expect("Failed to serialize");
        let deserialized: DownloadAsset =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(asset.name, deserialized.name);
        assert_eq!(asset.url, deserialized.url);
        assert_eq!(asset.platform, deserialized.platform);
        assert_eq!(asset.size, deserialized.size);
        assert_eq!(asset.checksum, deserialized.checksum);
        assert_eq!(asset.checksum_type, deserialized.checksum_type);
    }

    #[test]
    fn test_download_asset_optional_fields() {
        let asset = DownloadAsset {
            name: "openttd-15.3-linux-generic-amd64.tar.xz".to_string(),
            url: "https://cdn.openttd.org/openttd-releases/15.3/openttd-15.3-linux-generic-amd64.tar.xz".to_string(),
            platform: "linux-generic-amd64".to_string(),
            size: None,
            checksum: None,
            checksum_type: None,
        };

        let json = serde_json::to_string(&asset).expect("Failed to serialize");
        let deserialized: DownloadAsset =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(asset.name, deserialized.name);
        assert!(deserialized.size.is_none());
        assert!(deserialized.checksum.is_none());
        assert!(deserialized.checksum_type.is_none());
    }

    // ------------------------------------------------------------------
    // VersionInfo prerelease detection
    // ------------------------------------------------------------------

    #[test]
    fn test_prerelease_flag() {
        let stable = VersionInfo {
            id: uuid::Uuid::new_v4(),
            source: VersionSource::Official,
            version: Version::new(15, 3, 0),
            version_type: VersionType::Stable,
            name: "stable".to_string(),
            release_date: None,
            downloads: vec![],
            changelog: None,
            is_prerelease: false,
        };

        let beta = VersionInfo {
            is_prerelease: true,
            ..stable.clone()
        };

        assert!(!stable.is_prerelease);
        assert!(beta.is_prerelease);
    }
}