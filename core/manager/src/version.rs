//! Version types for OpenTTD version management.
//!
//! This module defines the core types used to represent OpenTTD versions,
//! their sources, release types, and downloadable assets.

use chrono::NaiveDateTime;
use semver::Version;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Source of an OpenTTD version build.
///
/// Indicates where the version originates from — the official OpenTTD
/// releases, JGR's patch pack, the CityMania Client project, or a
/// custom/user-defined source.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VersionSource {
    /// Official OpenTTD releases from cdn.openttd.org.
    Official,
    /// JGR's Patch Pack releases from GitHub.
    Jgrpp,
    /// CityMania Client releases.
    CmClient,
    /// A custom/user-defined source, identified by name.
    Custom(String),
}

impl std::fmt::Display for VersionSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionSource::Official => write!(f, "official"),
            VersionSource::Jgrpp => write!(f, "jgrpp"),
            VersionSource::CmClient => write!(f, "cmclient"),
            VersionSource::Custom(name) => write!(f, "custom/{name}"),
        }
    }
}

/// Type of release for an OpenTTD version.
///
/// Categorises a version along the release maturity spectrum, from
/// nightly builds through to stable releases.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VersionType {
    /// Full stable release.
    Stable,
    /// Release candidate (pre-release for final testing).
    ReleaseCandidate,
    /// Beta release (feature-complete but still testing).
    Beta,
    /// Nightly/automated build.
    Nightly,
    /// Pre-release or alpha-quality build.
    PreRelease,
    /// Custom or unclassified release type.
    Custom,
}

impl std::fmt::Display for VersionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionType::Stable => write!(f, "stable"),
            VersionType::ReleaseCandidate => write!(f, "release-candidate"),
            VersionType::Beta => write!(f, "beta"),
            VersionType::Nightly => write!(f, "nightly"),
            VersionType::PreRelease => write!(f, "pre-release"),
            VersionType::Custom => write!(f, "custom"),
        }
    }
}

/// A downloadable asset file for a specific version and platform.
///
/// Each asset represents a single file (e.g. a Windows installer, a
/// macOS DMG, a Linux tarball) that can be downloaded and installed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadAsset {
    /// Display name of the asset file (e.g. `openttd-15.3-windows-win64.exe`).
    pub name: String,
    /// Download URL for the asset.
    pub url: String,
    /// Target platform identifier (e.g. `windows-win64`, `linux-generic-amd64`).
    pub platform: String,
    /// File size in bytes, if known.
    pub size: Option<u64>,
    /// Checksum value for integrity verification, if available.
    pub checksum: Option<String>,
    /// Type of checksum algorithm (e.g. `sha256`, `md5`) — `None` implies `sha256`.
    pub checksum_type: Option<String>,
}

/// Full version information for an OpenTTD release.
///
/// This is the primary data structure that aggregates all metadata
/// about a single version, including its source, version identifier,
/// release type, download assets, and changelog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    /// Unique identifier for this version record.
    pub id: Uuid,
    /// Source of the version (official, JGRPP, etc.).
    pub source: VersionSource,
    /// Semantic version number.
    pub version: Version,
    /// Type of release (stable, beta, RC, etc.).
    pub version_type: VersionType,
    /// Human-readable name/title for this release.
    pub name: String,
    /// Release date in UTC, if known.
    pub release_date: Option<NaiveDateTime>,
    /// Available download assets for this version.
    pub downloads: Vec<DownloadAsset>,
    /// URL or text of the changelog, if available.
    pub changelog: Option<String>,
    /// Whether this is a pre-release version.
    pub is_prerelease: bool,
}

/// Types of mods available on BaNaNaS.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModType {
    /// NewGRF graphics / gameplay mods.
    NewGRF,
    /// AI (computer opponent) scripts.
    AI,
    /// GameScripts (gameplay modification scripts).
    GameScript,
    /// Music replacement sets.
    MusicSet,
}

impl std::fmt::Display for ModType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModType::NewGRF => write!(f, "newgrf"),
            ModType::AI => write!(f, "ai"),
            ModType::GameScript => write!(f, "gamescript"),
            ModType::MusicSet => write!(f, "music"),
        }
    }
}

/// A mod/item from the BaNaNaS API.
///
/// Represents a downloadable content item available on OpenTTD's
/// online content service, including its metadata, version, and
/// download URL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModInfo {
    /// Unique content identifier (e.g. `"424f4f54"`).
    pub id: String,
    /// Display name of the mod.
    pub name: String,
    /// Version string (e.g. `"1.0.0"`).
    pub version: String,
    /// The type of mod (NewGRF, AI, GameScript, or MusicSet).
    pub mod_type: ModType,
    /// Author/uploader name.
    pub author: String,
    /// Short description of the mod.
    pub description: String,
    /// URL to the mod's web page, if available.
    pub url: Option<String>,
    /// Category label (e.g. `"transport"`, `"industry"`), if available.
    pub category: Option<String>,
    /// Compatibility info (e.g. `"1.10.0"` or `"any"`), if available.
    pub compatibility: Option<String>,
    /// Direct download URL for the mod file, if available.
    pub download_url: Option<String>,
    /// File size in bytes, if known.
    pub filesize: Option<u64>,
    /// Date the mod was first published, if known.
    pub created_at: Option<chrono::NaiveDateTime>,
    /// Date the mod was last updated, if known.
    pub updated_at: Option<chrono::NaiveDateTime>,
}