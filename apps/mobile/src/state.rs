//! Mobile app state management

use dioxus_native::prelude::*;
use otmp_core_manager::version::VersionInfo;
use otmp_core_downloader::queue::{DownloadTask, Priority, TaskStatus};
use std::path::PathBuf;

/// Mobile app state with reactive signals
#[derive(Clone)]
pub struct MobileState {
    pub versions: Signal<Vec<VersionInfo>>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub downloads: Signal<Vec<DownloadTask>>,
    pub data_dir: Signal<PathBuf>,
}

impl MobileState {
    pub fn new() -> Self {
        Self {
            versions: Signal::new(Vec::new()),
            loading: Signal::new(false),
            error: Signal::new(None),
            downloads: Signal::new(Vec::new()),
            data_dir: Signal::new(
                std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).join("otmp-data")
            ),
        }
    }
}

/// APK version info for Android downloads
#[derive(Debug, Clone)]
pub struct ApkVersion {
    pub version: String,
    pub url: String,
    pub size: Option<u64>,
    pub release_date: Option<String>,
}

/// APK download manager
#[derive(Clone)]
pub struct ApkManager {
    pub apk_versions: Signal<Vec<ApkVersion>>,
}

impl ApkManager {
    pub fn new() -> Self {
        Self {
            apk_versions: Signal::new(Vec::new()),
        }
    }

    /// Get available APK versions from OpenTTD GitHub
    pub fn get_apk_versions() -> Vec<ApkVersion> {
        vec![
            ApkVersion {
                version: "14.1".to_string(),
                url: "https://github.com/OpenTTD/OpenTTD/releases/download/14.1/OpenTTD-14.1-android.aab".to_string(),
                size: Some(45_000_000),
                release_date: Some("2024-06-01".to_string()),
            },
            ApkVersion {
                version: "14.0".to_string(),
                url: "https://github.com/OpenTTD/OpenTTD/releases/download/14.0/OpenTTD-14.0-android.aab".to_string(),
                size: Some(44_000_000),
                release_date: Some("2024-03-15".to_string()),
            },
            ApkVersion {
                version: "13.4".to_string(),
                url: "https://github.com/OpenTTD/OpenTTD/releases/download/13.4/OpenTTD-13.4-android.apk".to_string(),
                size: Some(42_000_000),
                release_date: Some("2023-11-01".to_string()),
            },
        ]
    }
}