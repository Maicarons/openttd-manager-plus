//! Instance management for OpenTTD.
//!
//! This module provides the [`Instance`] struct representing an installed
//! OpenTTD version with its configuration, and the [`InstanceManager`] which
//! manages persistence and lifecycle of instances.

use std::path::PathBuf;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Error, Result};

const INSTANCES_FILE: &str = "instances.json";

/// An OpenTTD instance — represents one installed version with its configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    /// Unique identifier for this instance
    pub id: Uuid,
    /// Human-readable name for this instance
    pub name: String,
    /// Version string (e.g. "13.4", "JGRPP 0.53.1")
    pub version: String,
    /// Source of the instance: "official", "jgrpp", "cmclient", or "custom"
    pub source: String,
    /// Path where the OpenTTD binaries are installed
    pub install_path: PathBuf,
    /// Path to the configuration / user data directory
    pub config_path: PathBuf,
    /// Timestamp when this instance was created
    pub created_at: NaiveDateTime,
    /// Timestamp of the last time this instance was launched, if any
    pub last_played: Option<NaiveDateTime>,
    /// `true` if the instance has its own independent config directory,
    /// `false` if it shares a common config directory
    pub is_isolated: bool,
    /// Optional free-form notes about this instance
    pub notes: Option<String>,
}

/// Manages a collection of OpenTTD instances, persisted as JSON on disk.
///
/// The manager stores instances in a single `instances.json` file within the
/// configured `base_dir`. All mutating operations are performed in memory and
/// must be explicitly saved via [`save`](InstanceManager::save) to persist.
pub struct InstanceManager {
    /// Base directory where `instances.json` is stored
    base_dir: PathBuf,
    /// In-memory collection of instances
    instances: Vec<Instance>,
}

impl InstanceManager {
    /// Create a new instance manager at the given base directory.
    ///
    /// No data is loaded from disk until [`load`](InstanceManager::load) is called.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let manager = InstanceManager::new(PathBuf::from("/tmp/otmp"));
    /// ```
    pub fn new(base_dir: PathBuf) -> Self {
        Self {
            base_dir,
            instances: Vec::new(),
        }
    }

    /// Return the path to the instances JSON file.
    fn instances_path(&self) -> PathBuf {
        self.base_dir.join(INSTANCES_FILE)
    }

    /// Load all instances from the JSON file on disk.
    ///
    /// If the file does not exist, an empty collection is loaded (no error).
    pub fn load(&mut self) -> Result<()> {
        let path = self.instances_path();
        if !path.exists() {
            self.instances = Vec::new();
            log::info!("No instances file found at {:?}, starting fresh", path);
            return Ok(());
        }
        let data = std::fs::read_to_string(&path)?;
        self.instances = serde_json::from_str(&data)?;
        log::info!("Loaded {} instance(s) from {:?}", self.instances.len(), path);
        Ok(())
    }

    /// Save all instances to the JSON file on disk.
    ///
    /// Creates the parent directory if it does not exist.
    pub fn save(&self) -> Result<()> {
        let path = self.instances_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_string_pretty(&self.instances)?;
        std::fs::write(&path, data)?;
        log::info!("Saved {} instance(s) to {:?}", self.instances.len(), path);
        Ok(())
    }

    /// Register a new instance.
    ///
    /// Returns an error if an instance with the same ID already exists.
    pub fn add(&mut self, instance: Instance) -> Result<()> {
        if self.instances.iter().any(|i| i.id == instance.id) {
            return Err(Error::DuplicateInstance(instance.id.to_string()));
        }
        self.instances.push(instance);
        log::info!("Added instance");
        Ok(())
    }

    /// Remove an instance by its ID.
    ///
    /// Returns an error if no instance with the given ID exists.
    pub fn remove(&mut self, id: &Uuid) -> Result<()> {
        let len_before = self.instances.len();
        self.instances.retain(|i| &i.id != id);
        if self.instances.len() == len_before {
            return Err(Error::InstanceNotFound(id.to_string()));
        }
        log::info!("Removed instance {id}");
        Ok(())
    }

    /// Get an immutable reference to an instance by ID.
    pub fn get(&self, id: &Uuid) -> Option<&Instance> {
        self.instances.iter().find(|i| &i.id == id)
    }

    /// List all registered instances.
    pub fn list(&self) -> &[Instance] {
        &self.instances
    }

    /// List instances whose `source` matches the given string.
    pub fn list_by_source(&self, source: &str) -> Vec<&Instance> {
        self.instances.iter().filter(|i| i.source == source).collect()
    }

    /// Update the `last_played` timestamp for the instance with the given ID
    /// to the current local time.
    ///
    /// Returns an error if no instance with the given ID exists.
    pub fn record_play(&mut self, id: &Uuid) -> Result<()> {
        let instance = self
            .instances
            .iter_mut()
            .find(|i| &i.id == id)
            .ok_or_else(|| Error::InstanceNotFound(id.to_string()))?;
        instance.last_played = Some(chrono::Local::now().naive_local());
        log::info!("Recorded play for instance {id}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn make_instance(name: &str) -> Instance {
        Instance {
            id: Uuid::new_v4(),
            name: name.to_string(),
            version: "13.4".to_string(),
            source: "official".to_string(),
            install_path: PathBuf::from("/tmp/otmp/instances").join(name),
            config_path: PathBuf::from("/tmp/otmp/config").join(name),
            created_at: chrono::Local::now().naive_local(),
            last_played: None,
            is_isolated: true,
            notes: None,
        }
    }

    #[test]
    fn test_add_and_list() {
        let mut manager = InstanceManager::new(PathBuf::from("/tmp/otmp_test"));
        let inst = make_instance("test_add");
        manager.add(inst).unwrap();
        assert_eq!(manager.list().len(), 1);
    }

    #[test]
    fn test_add_duplicate() {
        let mut manager = InstanceManager::new(PathBuf::from("/tmp/otmp_test"));
        let mut inst = make_instance("dup");
        inst.id = Uuid::from_u128(42);
        manager.add(inst.clone()).unwrap();
        let result = manager.add(inst);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove() {
        let mut manager = InstanceManager::new(PathBuf::from("/tmp/otmp_test"));
        let inst = make_instance("to_remove");
        let id = inst.id;
        manager.add(inst).unwrap();
        assert_eq!(manager.list().len(), 1);
        manager.remove(&id).unwrap();
        assert_eq!(manager.list().len(), 0);
    }

    #[test]
    fn test_remove_not_found() {
        let mut manager = InstanceManager::new(PathBuf::from("/tmp/otmp_test"));
        let result = manager.remove(&Uuid::from_u128(999));
        assert!(result.is_err());
    }

    #[test]
    fn test_get() {
        let mut manager = InstanceManager::new(PathBuf::from("/tmp/otmp_test"));
        let inst = make_instance("get_me");
        let id = inst.id;
        manager.add(inst).unwrap();
        let found = manager.get(&id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "get_me");
    }

    #[test]
    fn test_get_nonexistent() {
        let manager = InstanceManager::new(PathBuf::from("/tmp/otmp_test"));
        assert!(manager.get(&Uuid::from_u128(888)).is_none());
    }

    #[test]
    fn test_list_by_source() {
        let mut manager = InstanceManager::new(PathBuf::from("/tmp/otmp_test"));
        let mut inst = make_instance("official_one");
        inst.source = "official".to_string();
        manager.add(inst).unwrap();
        let mut inst2 = make_instance("jgrpp_one");
        inst2.source = "jgrpp".to_string();
        manager.add(inst2).unwrap();
        let official = manager.list_by_source("official");
        assert_eq!(official.len(), 1);
        let jgrpp = manager.list_by_source("jgrpp");
        assert_eq!(jgrpp.len(), 1);
        let custom = manager.list_by_source("custom");
        assert_eq!(custom.len(), 0);
    }

    #[test]
    fn test_record_play() {
        let mut manager = InstanceManager::new(PathBuf::from("/tmp/otmp_test"));
        let mut inst = make_instance("play_me");
        inst.last_played = None;
        let id = inst.id;
        manager.add(inst).unwrap();
        manager.record_play(&id).unwrap();
        let found = manager.get(&id).unwrap();
        assert!(found.last_played.is_some());
    }

    #[test]
    fn test_record_play_not_found() {
        let mut manager = InstanceManager::new(PathBuf::from("/tmp/otmp_test"));
        let result = manager.record_play(&Uuid::from_u128(777));
        assert!(result.is_err());
    }

    #[test]
    fn test_load_save_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let mut manager = InstanceManager::new(dir.path().to_path_buf());
        let inst = make_instance("roundtrip");
        let id = inst.id;
        manager.add(inst).unwrap();
        manager.save().unwrap();

        // Reload into a fresh manager
        let mut loaded = InstanceManager::new(dir.path().to_path_buf());
        loaded.load().unwrap();
        assert_eq!(loaded.list().len(), 1);
        assert!(loaded.get(&id).is_some());
    }

    #[test]
    fn test_load_missing_file() {
        let dir = tempfile::tempdir().unwrap();
        let mut manager = InstanceManager::new(dir.path().to_path_buf());
        // Should not error when file doesn't exist
        manager.load().unwrap();
        assert_eq!(manager.list().len(), 0);
    }
}