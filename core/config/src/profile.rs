//! Configuration profile management.
//!
//! This module provides the [`ConfigProfile`] enum which defines how each
//! instance's configuration is shared or isolated, and the [`ProfileManager`]
//! which manages named profiles that can be applied to instances.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Error, Result};

const PROFILES_FILE: &str = "profiles.json";

/// Defines how configuration is shared or isolated across instances.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConfigProfile {
    /// Each instance has its own complete config directory.
    Isolated,
    /// All instances share a single config directory.
    Shared {
        /// Path to the shared configuration directory.
        shared_dir: PathBuf,
    },
    /// Hybrid: some resources are shared, others are independent per instance.
    Hybrid {
        /// Path to the shared configuration directory.
        shared_dir: PathBuf,
        /// List of config items that are kept independent per instance.
        independent_items: Vec<ConfigItem>,
    },
}

/// Which config items can be independent in [`ConfigProfile::Hybrid`] mode.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConfigItem {
    /// The main `openttd.cfg` file
    ConfigFile,
    /// NewGRF files
    NewGRF,
    /// Music files
    Music,
    /// Data files
    Data,
    /// Save games
    Saves,
    /// Scenario files
    Scenario,
    /// Screenshots
    Screenshots,
}

/// A named configuration profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// Unique identifier for this profile.
    pub id: Uuid,
    /// Human-readable name for this profile.
    pub name: String,
    /// The profile configuration (isolated, shared, or hybrid).
    pub config: ConfigProfile,
    /// Whether this profile is the default.
    #[serde(default)]
    pub is_default: bool,
}

/// Manages a collection of configuration profiles, persisted as JSON on disk.
///
/// Profiles are stored in a single `profiles.json` file within the configured
/// `base_dir`. At most one profile may be marked as default.
pub struct ProfileManager {
    /// Base directory where `profiles.json` is stored.
    base_dir: PathBuf,
    /// In-memory collection of profiles.
    profiles: Vec<Profile>,
}

impl ProfileManager {
    /// Create a new profile manager at the given base directory.
    ///
    /// No data is loaded from disk until [`load`](ProfileManager::load) is called.
    pub fn new(base_dir: PathBuf) -> Self {
        Self {
            base_dir,
            profiles: Vec::new(),
        }
    }

    /// Return the path to the profiles JSON file.
    fn profiles_path(&self) -> PathBuf {
        self.base_dir.join(PROFILES_FILE)
    }

    /// Load all profiles from the JSON file on disk.
    ///
    /// If the file does not exist, an empty collection is loaded (no error).
    pub fn load(&mut self) -> Result<()> {
        let path = self.profiles_path();
        if !path.exists() {
            self.profiles = Vec::new();
            log::info!("No profiles file found at {:?}, starting fresh", path);
            return Ok(());
        }
        let data = std::fs::read_to_string(&path)?;
        self.profiles = serde_json::from_str(&data)?;
        log::info!("Loaded {} profile(s) from {:?}", self.profiles.len(), path);
        Ok(())
    }

    /// Save all profiles to the JSON file on disk.
    ///
    /// Creates the parent directory if it does not exist.
    pub fn save(&self) -> Result<()> {
        let path = self.profiles_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_string_pretty(&self.profiles)?;
        std::fs::write(&path, data)?;
        log::info!("Saved {} profile(s) to {:?}", self.profiles.len(), path);
        Ok(())
    }

    /// Register a new profile.
    ///
    /// Returns an error if a profile with the same ID already exists.
    pub fn add(&mut self, profile: Profile) -> Result<()> {
        if self.profiles.iter().any(|p| p.id == profile.id) {
            return Err(Error::DuplicateProfile(profile.id.to_string()));
        }
        // If the added profile is marked as default, un-mark any existing default.
        if profile.is_default {
            for p in &mut self.profiles {
                p.is_default = false;
            }
        }
        self.profiles.push(profile);
        log::info!("Added profile");
        Ok(())
    }

    /// Remove a profile by its ID.
    ///
    /// Returns an error if no profile with the given ID exists.
    /// If the removed profile was the default, there will be no default after removal.
    pub fn remove(&mut self, id: &Uuid) -> Result<()> {
        let len_before = self.profiles.len();
        self.profiles.retain(|p| &p.id != id);
        if self.profiles.len() == len_before {
            return Err(Error::ProfileNotFound(id.to_string()));
        }
        log::info!("Removed profile {id}");
        Ok(())
    }

    /// Get an immutable reference to a profile by ID.
    pub fn get(&self, id: &Uuid) -> Option<&Profile> {
        self.profiles.iter().find(|p| &p.id == id)
    }

    /// List all registered profiles.
    pub fn list(&self) -> &[Profile] {
        &self.profiles
    }

    /// Get the default profile, if one is set.
    pub fn get_default(&self) -> Option<&Profile> {
        self.profiles.iter().find(|p| p.is_default)
    }

    /// Set the profile with the given ID as the default.
    ///
    /// Any previously default profile is unmarked. Returns an error if no
    /// profile with the given ID exists.
    pub fn set_default(&mut self, id: &Uuid) -> Result<()> {
        if !self.profiles.iter().any(|p| &p.id == id) {
            return Err(Error::ProfileNotFound(id.to_string()));
        }
        for p in &mut self.profiles {
            p.is_default = &p.id == id;
        }
        log::info!("Set default profile {id}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn make_profile(name: &str) -> Profile {
        Profile {
            id: Uuid::new_v4(),
            name: name.to_string(),
            config: ConfigProfile::Isolated,
            is_default: false,
        }
    }

    #[test]
    fn test_add_and_list() {
        let mut pm = ProfileManager::new(PathBuf::from("/tmp/otmp_test"));
        pm.add(make_profile("test_add")).unwrap();
        assert_eq!(pm.list().len(), 1);
    }

    #[test]
    fn test_add_duplicate() {
        let mut pm = ProfileManager::new(PathBuf::from("/tmp/otmp_test"));
        let mut p = make_profile("dup");
        p.id = Uuid::from_u128(42);
        pm.add(p.clone()).unwrap();
        let result = pm.add(p);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove() {
        let mut pm = ProfileManager::new(PathBuf::from("/tmp/otmp_test"));
        let p = make_profile("to_remove");
        let id = p.id;
        pm.add(p).unwrap();
        assert_eq!(pm.list().len(), 1);
        pm.remove(&id).unwrap();
        assert_eq!(pm.list().len(), 0);
    }

    #[test]
    fn test_remove_not_found() {
        let mut pm = ProfileManager::new(PathBuf::from("/tmp/otmp_test"));
        let result = pm.remove(&Uuid::from_u128(999));
        assert!(result.is_err());
    }

    #[test]
    fn test_get() {
        let mut pm = ProfileManager::new(PathBuf::from("/tmp/otmp_test"));
        let p = make_profile("get_me");
        let id = p.id;
        pm.add(p).unwrap();
        let found = pm.get(&id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "get_me");
    }

    #[test]
    fn test_get_nonexistent() {
        let pm = ProfileManager::new(PathBuf::from("/tmp/otmp_test"));
        assert!(pm.get(&Uuid::from_u128(888)).is_none());
    }

    #[test]
    fn test_get_default() {
        let mut pm = ProfileManager::new(PathBuf::from("/tmp/otmp_test"));
        assert!(pm.get_default().is_none());
        let mut p = make_profile("default_one");
        p.id = Uuid::from_u128(1);
        p.is_default = true;
        pm.add(p).unwrap();
        let def = pm.get_default();
        assert!(def.is_some());
        assert_eq!(def.unwrap().name, "default_one");
    }

    #[test]
    fn test_set_default() {
        let mut pm = ProfileManager::new(PathBuf::from("/tmp/otmp_test"));
        let p1 = make_profile("first");
        let p2 = make_profile("second");
        let id1 = p1.id;
        let id2 = p2.id;
        pm.add(p1).unwrap();
        pm.add(p2).unwrap();
        pm.set_default(&id1).unwrap();
        assert_eq!(pm.get_default().unwrap().id, id1);
        pm.set_default(&id2).unwrap();
        assert_eq!(pm.get_default().unwrap().id, id2);
        // First should no longer be default
        let first = pm.get(&id1).unwrap();
        assert!(!first.is_default);
    }

    #[test]
    fn test_set_default_not_found() {
        let mut pm = ProfileManager::new(PathBuf::from("/tmp/otmp_test"));
        let result = pm.set_default(&Uuid::from_u128(999));
        assert!(result.is_err());
    }

    #[test]
    fn test_add_default_unsets_previous() {
        let mut pm = ProfileManager::new(PathBuf::from("/tmp/otmp_test"));
        let mut p1 = make_profile("first");
        p1.is_default = true;
        pm.add(p1).unwrap();
        let mut p2 = make_profile("second");
        p2.is_default = true;
        pm.add(p2).unwrap();
        // Only the second should be default
        assert_eq!(pm.get_default().unwrap().name, "second");
        assert!(!pm.list().iter().any(|p| p.name == "first" && p.is_default));
    }

    #[test]
    fn test_load_save_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let mut pm = ProfileManager::new(dir.path().to_path_buf());
        let mut p = make_profile("roundtrip");
        p.is_default = true;
        let id = p.id;
        pm.add(p).unwrap();
        pm.save().unwrap();

        let mut loaded = ProfileManager::new(dir.path().to_path_buf());
        loaded.load().unwrap();
        assert_eq!(loaded.list().len(), 1);
        let loaded_p = loaded.get(&id).unwrap();
        assert_eq!(loaded_p.name, "roundtrip");
        assert!(loaded_p.is_default);
    }

    #[test]
    fn test_load_missing_file() {
        let dir = tempfile::tempdir().unwrap();
        let mut pm = ProfileManager::new(dir.path().to_path_buf());
        pm.load().unwrap();
        assert_eq!(pm.list().len(), 0);
    }
}