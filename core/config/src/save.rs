//! Save/archive management for OpenTTD.
//!
//! This module provides the [`SaveType`] enum, [`SaveInfo`] metadata struct,
//! and [`SaveManager`] which manages save files, scenarios, and heightmaps
//! on disk.

use std::path::{Path, PathBuf};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Error, Result};

/// Extension mapping for save file types.
const SAVE_EXTENSIONS: &[(&str, SaveType)] = &[
    (".sav", SaveType::SaveGame),
    (".scn", SaveType::Scenario),
    (".ss1", SaveType::Heightmap),
    (".hgt", SaveType::Heightmap),
];

/// Type of save/archive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SaveType {
    /// Regular save game (.sav)
    SaveGame,
    /// Scenario file (.scn)
    Scenario,
    /// Heightmap (.ss1 or .hgt)
    Heightmap,
}

impl SaveType {
    /// Return the file extensions associated with this save type.
    pub fn extensions(&self) -> Vec<&'static str> {
        match self {
            SaveType::SaveGame => vec![".sav"],
            SaveType::Scenario => vec![".scn"],
            SaveType::Heightmap => vec![".ss1", ".hgt"],
        }
    }

    /// Determine the save type from a file extension.
    pub fn from_extension(ext: &str) -> Option<Self> {
        let ext = ext.to_lowercase();
        SAVE_EXTENSIONS
            .iter()
            .find(|(e, _)| *e == &ext)
            .map(|(_, t)| t.clone())
    }
}

/// Metadata about a save/archive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveInfo {
    /// Unique identifier for this save.
    pub id: Uuid,
    /// Human-readable name (derived from the file stem).
    pub name: String,
    /// Full path to the save file on disk.
    pub path: PathBuf,
    /// Type of save (SaveGame, Scenario, or Heightmap).
    pub save_type: SaveType,
    /// File size in bytes.
    pub size: u64,
    /// Last modification timestamp.
    pub modified: NaiveDateTime,
    /// Optional instance this save is associated with.
    pub instance_id: Option<Uuid>,
    /// User-defined tags for categorisation.
    pub tags: Vec<String>,
}

/// Save/archive manager.
///
/// Scans a configured base directory for save files, scenarios, and heightmaps,
/// and provides operations to import, export, backup, restore, rename, delete,
/// and search them.
pub struct SaveManager {
    /// Base directory where save files are stored.
    base_dir: PathBuf,
    /// In-memory collection of save metadata.
    saves: Vec<SaveInfo>,
}

impl SaveManager {
    /// Create a new save manager at the given base directory.
    ///
    /// No scanning is performed until [`scan`](SaveManager::scan) is called.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let manager = SaveManager::new(PathBuf::from("/tmp/otmp/saves"));
    /// ```
    pub fn new(base_dir: PathBuf) -> Self {
        Self {
            base_dir,
            saves: Vec::new(),
        }
    }

    /// Scan the saves directory for save files and populate the internal list.
    ///
    /// This rescans the directory, replacing any previously loaded saves.
    /// Only files with recognised extensions (`.sav`, `.scn`, `.ss1`, `.hgt`)
    /// are included.
    ///
    /// If the directory does not exist, an empty list is returned (no error).
    pub fn scan(&mut self) -> Result<()> {
        self.saves.clear();

        if !self.base_dir.exists() {
            log::info!(
                "Saves directory does not exist at {:?}, starting empty",
                self.base_dir
            );
            return Ok(());
        }

        let mut entries: Vec<_> = std::fs::read_dir(&self.base_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
            .collect();

        // Sort by file name for deterministic order
        entries.sort_by_key(|e| e.file_name());

        let now = chrono::Local::now().naive_local();

        for entry in entries {
            let path = entry.path();
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| format!(".{}", e.to_lowercase()));

            let save_type = ext
                .as_deref()
                .and_then(SaveType::from_extension);

            if let Some(save_type) = save_type {
                let metadata = std::fs::metadata(&path)?;
                let stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let modified = metadata
                    .modified()
                    .ok()
                    .and_then(|t| {
                        let duration = t
                            .duration_since(std::time::UNIX_EPOCH)
                            .ok()?;
                        chrono::DateTime::from_timestamp(
                            duration.as_secs() as i64,
                            duration.subsec_nanos(),
                        )
                        .map(|dt| dt.naive_utc())
                    })
                    .unwrap_or(now);

                self.saves.push(SaveInfo {
                    id: Uuid::new_v4(),
                    name: stem,
                    path,
                    save_type,
                    size: metadata.len(),
                    modified,
                    instance_id: None,
                    tags: Vec::new(),
                });
            }
        }

        log::info!(
            "Scanned {} save file(s) from {:?}",
            self.saves.len(),
            self.base_dir
        );
        Ok(())
    }

    /// List all saves.
    pub fn list(&self) -> &[SaveInfo] {
        &self.saves
    }

    /// List saves by type.
    pub fn list_by_type(&self, save_type: SaveType) -> Vec<&SaveInfo> {
        self.saves
            .iter()
            .filter(|s| s.save_type == save_type)
            .collect()
    }

    /// Get a save by ID.
    pub fn get(&self, id: &Uuid) -> Option<&SaveInfo> {
        self.saves.iter().find(|s| s.id == *id)
    }

    /// Import a save file from an external path into the managed directory.
    ///
    /// The file is copied into the base directory, given a UUID-based filename
    /// to avoid collisions, and added to the internal list.
    ///
    /// # Errors
    ///
    /// Returns an error if the source file cannot be read or the copy fails.
    pub fn import(&mut self, source: &Path, save_type: SaveType) -> Result<SaveInfo> {
        if !source.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Source file does not exist: {}", source.display()),
            )));
        }

        // Determine the extension for the target type
        let exts = save_type.extensions();
        let ext = exts.first().ok_or_else(|| {
            Error::InvalidConfig("No extension defined for save type".to_string())
        })?;

        // Generate a unique filename using UUID
        let id = Uuid::new_v4();
        let filename = format!("{id}{ext}");
        let dest = self.base_dir.join(&filename);

        // Ensure base directory exists
        std::fs::create_dir_all(&self.base_dir)?;

        // Copy the file
        std::fs::copy(source, &dest)?;

        let metadata = std::fs::metadata(&dest)?;
        let modified = metadata
            .modified()
            .ok()
            .and_then(|t| {
                let duration = t.duration_since(std::time::UNIX_EPOCH).ok()?;
                chrono::DateTime::from_timestamp(
                    duration.as_secs() as i64,
                    duration.subsec_nanos(),
                )
                .map(|dt| dt.naive_utc())
            })
            .unwrap_or_else(|| chrono::Local::now().naive_local());

        let save_info = SaveInfo {
            id,
            name: filename.strip_suffix(ext).unwrap_or(&filename).to_string(),
            path: dest,
            save_type,
            size: metadata.len(),
            modified,
            instance_id: None,
            tags: Vec::new(),
        };

        log::info!(
            "Imported save {} ({}) from {:?}",
            save_info.id,
            save_info.name,
            source
        );

        self.saves.push(save_info.clone());
        Ok(save_info)
    }

    /// Export a save to an external path.
    ///
    /// # Errors
    ///
    /// Returns an error if the save ID is not found or the file cannot be copied.
    pub fn export(&self, id: &Uuid, destination: &Path) -> Result<()> {
        let save = self
            .saves
            .iter()
            .find(|s| s.id == *id)
            .ok_or_else(|| Error::SaveNotFound(id.to_string()))?;

        if let Some(parent) = destination.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::copy(&save.path, destination)?;
        log::info!("Exported save {id} to {:?}", destination);
        Ok(())
    }

    /// Delete a save from disk and remove it from the internal list.
    ///
    /// # Errors
    ///
    /// Returns an error if the save ID is not found or the file cannot be deleted.
    pub fn delete(&mut self, id: &Uuid) -> Result<()> {
        let pos = self
            .saves
            .iter()
            .position(|s| s.id == *id)
            .ok_or_else(|| Error::SaveNotFound(id.to_string()))?;

        let save = &self.saves[pos];
        if save.path.exists() {
            std::fs::remove_file(&save.path)?;
        }

        self.saves.remove(pos);
        log::info!("Deleted save {id}");
        Ok(())
    }

    /// Create a backup of a save file by copying it to a backup directory.
    ///
    /// The backup file is named `<save-id>-<original-stem><ext>`.
    ///
    /// # Errors
    ///
    /// Returns an error if the save ID is not found or the backup copy fails.
    pub fn backup(&self, id: &Uuid, backup_dir: &Path) -> Result<PathBuf> {
        let save = self
            .saves
            .iter()
            .find(|s| s.id == *id)
            .ok_or_else(|| Error::SaveNotFound(id.to_string()))?;

        std::fs::create_dir_all(backup_dir)?;

        let ext = save
            .path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("sav");
        let backup_name = format!("{}-{}.{}", save.id, save.name, ext);
        let backup_path = backup_dir.join(&backup_name);

        std::fs::copy(&save.path, &backup_path)?;
        log::info!("Backed up save {id} to {:?}", backup_path);
        Ok(backup_path)
    }

    /// Restore a save from a backup file.
    ///
    /// The backup file is copied into the managed directory with a new UUID,
    /// and added to the internal list.
    ///
    /// # Errors
    ///
    /// Returns an error if the backup file does not exist or the copy fails.
    pub fn restore(&mut self, backup_path: &Path) -> Result<SaveInfo> {
        if !backup_path.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Backup file does not exist: {}", backup_path.display()),
            )));
        }

        let ext = backup_path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| format!(".{}", e.to_lowercase()))
            .unwrap_or_else(|| ".sav".to_string());

        let save_type = SaveType::from_extension(&ext).unwrap_or(SaveType::SaveGame);

        let id = Uuid::new_v4();
        let filename = format!("{id}{ext}");
        let dest = self.base_dir.join(&filename);

        std::fs::create_dir_all(&self.base_dir)?;
        std::fs::copy(backup_path, &dest)?;

        let metadata = std::fs::metadata(&dest)?;
        let modified = metadata
            .modified()
            .ok()
            .and_then(|t| {
                let duration = t.duration_since(std::time::UNIX_EPOCH).ok()?;
                chrono::DateTime::from_timestamp(
                    duration.as_secs() as i64,
                    duration.subsec_nanos(),
                )
                .map(|dt| dt.naive_utc())
            })
            .unwrap_or_else(|| chrono::Local::now().naive_local());

        let save_info = SaveInfo {
            id,
            name: filename.strip_suffix(&ext).unwrap_or(&filename).to_string(),
            path: dest,
            save_type,
            size: metadata.len(),
            modified,
            instance_id: None,
            tags: Vec::new(),
        };

        log::info!("Restored save from backup {:?}, new id {id}", backup_path);
        self.saves.push(save_info.clone());
        Ok(save_info)
    }

    /// Rename a save (updates the in-memory name, not the file on disk).
    ///
    /// # Errors
    ///
    /// Returns an error if the save ID is not found.
    pub fn rename(&mut self, id: &Uuid, new_name: &str) -> Result<()> {
        let save = self
            .saves
            .iter_mut()
            .find(|s| s.id == *id)
            .ok_or_else(|| Error::SaveNotFound(id.to_string()))?;

        save.name = new_name.to_string();
        log::info!("Renamed save {id} to \"{new_name}\"");
        Ok(())
    }

    /// Search saves by name or tag (case-insensitive substring match).
    ///
    /// Returns all saves whose `name` or any `tag` contains the query string.
    pub fn search(&self, query: &str) -> Vec<&SaveInfo> {
        let query_lower = query.to_lowercase();
        self.saves
            .iter()
            .filter(|s| {
                s.name.to_lowercase().contains(&query_lower)
                    || s.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_save_file(dir: &Path, name: &str) -> PathBuf {
        let path = dir.join(name);
        std::fs::write(&path, b"dummy save content").unwrap();
        path
    }

    /// Helper to create a temporary directory with a few save files.
    fn setup_save_dir() -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let save_dir = dir.path().join("saves");
        std::fs::create_dir(&save_dir).unwrap();
        create_save_file(&save_dir, "game1.sav");
        create_save_file(&save_dir, "game2.sav");
        create_save_file(&save_dir, "scenario1.scn");
        create_save_file(&save_dir, "heightmap1.hgt");
        create_save_file(&save_dir, "old_heightmap.ss1");
        // A non-save file that should be ignored
        create_save_file(&save_dir, "readme.txt");
        (dir, save_dir)
    }

    // ── Scan tests ──────────────────────────────────────────────────────

    #[test]
    fn test_scan_detects_all_save_files() {
        let (_dir, save_dir) = setup_save_dir();

        let mut manager = SaveManager::new(save_dir);
        manager.scan().unwrap();

        let saves = manager.list();
        // 5 save files: 2 .sav, 1 .scn, 1 .hgt, 1 .ss1
        assert_eq!(saves.len(), 5);
    }

    #[test]
    fn test_scan_ignores_non_save_files() {
        let (_dir, save_dir) = setup_save_dir();

        let mut manager = SaveManager::new(save_dir);
        manager.scan().unwrap();

        let names: Vec<&str> = manager.list().iter().map(|s| s.name.as_str()).collect();
        assert!(!names.contains(&"readme"));
    }

    #[test]
    fn test_scan_nonexistent_directory() {
        let mut manager = SaveManager::new(PathBuf::from("/tmp/does_not_exist_12345"));
        manager.scan().unwrap();
        assert!(manager.list().is_empty());
    }

    #[test]
    fn test_scan_empty_directory() {
        let dir = tempfile::tempdir().unwrap();
        let save_dir = dir.path().join("empty_saves");
        std::fs::create_dir(&save_dir).unwrap();

        let mut manager = SaveManager::new(save_dir);
        manager.scan().unwrap();
        assert!(manager.list().is_empty());
    }

    #[test]
    fn test_scan_rescans_replace_previous() {
        let dir = tempfile::tempdir().unwrap();
        let save_dir = dir.path().join("saves");
        std::fs::create_dir(&save_dir).unwrap();

        let mut manager = SaveManager::new(save_dir.clone());
        manager.scan().unwrap();
        assert!(manager.list().is_empty());

        // Add a file and rescan
        create_save_file(&save_dir, "game.sav");
        manager.scan().unwrap();
        assert_eq!(manager.list().len(), 1);
    }

    // ── List by type tests ──────────────────────────────────────────────

    #[test]
    fn test_list_by_type() {
        let (_dir, save_dir) = setup_save_dir();

        let mut manager = SaveManager::new(save_dir);
        manager.scan().unwrap();

        let save_games = manager.list_by_type(SaveType::SaveGame);
        assert_eq!(save_games.len(), 2);

        let scenarios = manager.list_by_type(SaveType::Scenario);
        assert_eq!(scenarios.len(), 1);

        let heightmaps = manager.list_by_type(SaveType::Heightmap);
        assert_eq!(heightmaps.len(), 2);
    }

    // ── Get tests ───────────────────────────────────────────────────────

    #[test]
    fn test_get_by_id() {
        let (_dir, save_dir) = setup_save_dir();

        let mut manager = SaveManager::new(save_dir);
        manager.scan().unwrap();

        let first = manager.list().first().unwrap();
        let found = manager.get(&first.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, first.id);
    }

    #[test]
    fn test_get_nonexistent() {
        let manager = SaveManager::new(PathBuf::from("/tmp"));
        assert!(manager.get(&Uuid::new_v4()).is_none());
    }

    // ── Import tests ────────────────────────────────────────────────────

    #[test]
    fn test_import_copies_file_and_adds_metadata() {
        let dir = tempfile::tempdir().unwrap();
        let save_dir = dir.path().join("saves");
        std::fs::create_dir(&save_dir).unwrap();

        let source = dir.path().join("external.sav");
        std::fs::write(&source, b"save data").unwrap();

        let mut manager = SaveManager::new(save_dir.clone());
        let info = manager.import(&source, SaveType::SaveGame).unwrap();

        assert_eq!(info.save_type, SaveType::SaveGame);
        assert!(info.path.exists());
        // The imported file should be in the managed directory
        assert!(info.path.starts_with(&save_dir));
        // The destination file should have the same content
        let imported_content = std::fs::read(&info.path).unwrap();
        assert_eq!(imported_content, b"save data");
        // The manager should have one entry
        assert_eq!(manager.list().len(), 1);
    }

    #[test]
    fn test_import_nonexistent_source() {
        let mut manager = SaveManager::new(PathBuf::from("/tmp"));
        let result = manager.import(
            &PathBuf::from("/tmp/nonexistent_123.sav"),
            SaveType::SaveGame,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_import_determines_type_correctly() {
        let dir = tempfile::tempdir().unwrap();
        let save_dir = dir.path().join("saves");
        std::fs::create_dir(&save_dir).unwrap();

        let source = dir.path().join("test.scn");
        std::fs::write(&source, b"scenario data").unwrap();

        let mut manager = SaveManager::new(save_dir);
        let info = manager.import(&source, SaveType::Scenario).unwrap();
        assert_eq!(info.save_type, SaveType::Scenario);
    }

    // ── Export tests ────────────────────────────────────────────────────

    #[test]
    fn test_export_copies_file_to_destination() {
        let dir = tempfile::tempdir().unwrap();
        let save_dir = dir.path().join("saves");
        std::fs::create_dir(&save_dir).unwrap();
        create_save_file(&save_dir, "game.sav");

        let mut manager = SaveManager::new(save_dir);
        manager.scan().unwrap();

        let id = manager.list()[0].id;
        let dest = dir.path().join("exported.sav");

        manager.export(&id, &dest).unwrap();
        assert!(dest.exists());

        let exported_content = std::fs::read(&dest).unwrap();
        assert_eq!(exported_content, b"dummy save content");
    }

    #[test]
    fn test_export_nonexistent_id() {
        let manager = SaveManager::new(PathBuf::from("/tmp"));
        let result = manager.export(&Uuid::new_v4(), &PathBuf::from("/tmp/out.sav"));
        assert!(result.is_err());
    }

    #[test]
    fn test_export_creates_parent_directories() {
        let dir = tempfile::tempdir().unwrap();
        let save_dir = dir.path().join("saves");
        std::fs::create_dir(&save_dir).unwrap();
        create_save_file(&save_dir, "game.sav");

        let mut manager = SaveManager::new(save_dir);
        manager.scan().unwrap();

        let id = manager.list()[0].id;
        let dest = dir.path().join("subdir/exported.sav");

        manager.export(&id, &dest).unwrap();
        assert!(dest.exists());
    }

    // ── Delete tests ────────────────────────────────────────────────────

    #[test]
    fn test_delete_removes_file_and_metadata() {
        let dir = tempfile::tempdir().unwrap();
        let save_dir = dir.path().join("saves");
        std::fs::create_dir(&save_dir).unwrap();
        create_save_file(&save_dir, "game.sav");

        let mut manager = SaveManager::new(save_dir);
        manager.scan().unwrap();

        assert_eq!(manager.list().len(), 1);
        let id = manager.list()[0].id;
        let path = manager.list()[0].path.clone();

        manager.delete(&id).unwrap();
        assert_eq!(manager.list().len(), 0);
        assert!(!path.exists());
    }

    #[test]
    fn test_delete_nonexistent_id() {
        let mut manager = SaveManager::new(PathBuf::from("/tmp"));
        let result = manager.delete(&Uuid::new_v4());
        assert!(result.is_err());
    }

    // ── Backup tests ────────────────────────────────────────────────────

    #[test]
    fn test_backup_creates_copy_in_backup_dir() {
        let dir = tempfile::tempdir().unwrap();
        let save_dir = dir.path().join("saves");
        let backup_dir = dir.path().join("backups");
        std::fs::create_dir(&save_dir).unwrap();
        create_save_file(&save_dir, "game.sav");

        let mut manager = SaveManager::new(save_dir);
        manager.scan().unwrap();

        let id = manager.list()[0].id;
        let backup_path = manager.backup(&id, &backup_dir).unwrap();

        assert!(backup_path.exists());
        assert!(backup_path.starts_with(&backup_dir));
        // Content should match
        let backup_content = std::fs::read(&backup_path).unwrap();
        assert_eq!(backup_content, b"dummy save content");
    }

    #[test]
    fn test_backup_nonexistent_id() {
        let dir = tempfile::tempdir().unwrap();
        let manager = SaveManager::new(PathBuf::from("/tmp"));
        let result = manager.backup(&Uuid::new_v4(), &dir.path().join("backups"));
        assert!(result.is_err());
    }

    // ── Restore tests ───────────────────────────────────────────────────

    #[test]
    fn test_restore_imports_backup_as_new_save() {
        let dir = tempfile::tempdir().unwrap();
        let save_dir = dir.path().join("saves");
        let backup_dir = dir.path().join("backups");
        std::fs::create_dir(&save_dir).unwrap();
        std::fs::create_dir(&backup_dir).unwrap();

        // Create a backup file manually
        let backup_file = backup_dir.join("backup.sav");
        std::fs::write(&backup_file, b"restored content").unwrap();

        let mut manager = SaveManager::new(save_dir.clone());
        let info = manager.restore(&backup_file).unwrap();

        assert_eq!(info.save_type, SaveType::SaveGame);
        assert!(info.path.exists());
        assert!(info.path.starts_with(&save_dir));
        // The restored content should match
        let content = std::fs::read(&info.path).unwrap();
        assert_eq!(content, b"restored content");
        assert_eq!(manager.list().len(), 1);
    }

    #[test]
    fn test_restore_nonexistent_backup() {
        let mut manager = SaveManager::new(PathBuf::from("/tmp"));
        let result = manager.restore(&PathBuf::from("/tmp/nonexistent_backup.sav"));
        assert!(result.is_err());
    }

    #[test]
    fn test_restore_detects_type_from_extension() {
        let dir = tempfile::tempdir().unwrap();
        let save_dir = dir.path().join("saves");
        let backup_dir = dir.path().join("backups");
        std::fs::create_dir(&save_dir).unwrap();
        std::fs::create_dir(&backup_dir).unwrap();

        let backup_file = backup_dir.join("backup.hgt");
        std::fs::write(&backup_file, b"heightmap data").unwrap();

        let mut manager = SaveManager::new(save_dir);
        let info = manager.restore(&backup_file).unwrap();
        assert_eq!(info.save_type, SaveType::Heightmap);
    }

    // ── Rename tests ────────────────────────────────────────────────────

    #[test]
    fn test_rename_updates_name() {
        let (_dir, save_dir) = setup_save_dir();

        let mut manager = SaveManager::new(save_dir);
        manager.scan().unwrap();

        let id = manager.list()[0].id;
        let old_name = manager.list()[0].name.clone();

        manager.rename(&id, "new_name").unwrap();
        let updated = manager.get(&id).unwrap();
        assert_eq!(updated.name, "new_name");
        assert_ne!(updated.name, old_name);
    }

    #[test]
    fn test_rename_nonexistent_id() {
        let mut manager = SaveManager::new(PathBuf::from("/tmp"));
        let result = manager.rename(&Uuid::new_v4(), "new_name");
        assert!(result.is_err());
    }

    // ── Search tests ────────────────────────────────────────────────────

    #[test]
    fn test_search_by_name() {
        let (_dir, save_dir) = setup_save_dir();

        let mut manager = SaveManager::new(save_dir);
        manager.scan().unwrap();

        // Search for "game" should find game1 and game2
        let results = manager.search("game");
        assert_eq!(results.len(), 2);

        // Search for "scenario" should find scenario1
        let results = manager.search("scenario");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search_by_tag() {
        let (_dir, save_dir) = setup_save_dir();

        let mut manager = SaveManager::new(save_dir);
        manager.scan().unwrap();

        // Add a tag to one save
        manager.list_mut().first_mut().unwrap().tags.push("important".to_string());

        // Search for the tag
        let results = manager.search("important");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search_case_insensitive() {
        let (_dir, save_dir) = setup_save_dir();

        let mut manager = SaveManager::new(save_dir);
        manager.scan().unwrap();

        let results = manager.search("GAME");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_search_no_match() {
        let (_dir, save_dir) = setup_save_dir();

        let mut manager = SaveManager::new(save_dir);
        manager.scan().unwrap();

        let results = manager.search("nonexistent");
        assert!(results.is_empty());
    }

    // ── SaveType tests ──────────────────────────────────────────────────

    #[test]
    fn test_save_type_from_extension() {
        assert_eq!(
            SaveType::from_extension(".sav"),
            Some(SaveType::SaveGame)
        );
        assert_eq!(
            SaveType::from_extension(".scn"),
            Some(SaveType::Scenario)
        );
        assert_eq!(
            SaveType::from_extension(".hgt"),
            Some(SaveType::Heightmap)
        );
        assert_eq!(
            SaveType::from_extension(".ss1"),
            Some(SaveType::Heightmap)
        );
        assert_eq!(SaveType::from_extension(".txt"), None);
        assert_eq!(SaveType::from_extension(".SAV"), Some(SaveType::SaveGame));
    }

    #[test]
    fn test_save_type_extensions() {
        assert_eq!(SaveType::SaveGame.extensions(), vec![".sav"]);
        assert_eq!(SaveType::Scenario.extensions(), vec![".scn"]);
        let mut hgt = SaveType::Heightmap.extensions();
        hgt.sort();
        assert_eq!(hgt, vec![".hgt", ".ss1"]);
    }

    // ── Integration tests ───────────────────────────────────────────────

    #[test]
    fn test_import_scan_export_cycle() {
        let dir = tempfile::tempdir().unwrap();
        let save_dir = dir.path().join("saves");
        std::fs::create_dir(&save_dir).unwrap();

        let source = dir.path().join("external.sav");
        std::fs::write(&source, b"cycle data").unwrap();

        // Import into a manager
        let mut manager = SaveManager::new(save_dir.clone());
        let info = manager.import(&source, SaveType::SaveGame).unwrap();

        // Scan should find the imported file
        let mut scanner = SaveManager::new(save_dir);
        scanner.scan().unwrap();
        assert_eq!(scanner.list().len(), 1);
        assert_eq!(scanner.list()[0].name, info.name);

        // Export to a new location
        let export_dest = dir.path().join("exported.sav");
        manager.export(&info.id, &export_dest).unwrap();
        assert!(export_dest.exists());
        let exported_content = std::fs::read(&export_dest).unwrap();
        assert_eq!(exported_content, b"cycle data");
    }

    #[test]
    fn test_backup_restore_cycle() {
        let dir = tempfile::tempdir().unwrap();
        let save_dir = dir.path().join("saves");
        let backup_dir = dir.path().join("backups");
        std::fs::create_dir(&save_dir).unwrap();
        std::fs::create_dir(&backup_dir).unwrap();
        create_save_file(&save_dir, "important.sav");

        let mut manager = SaveManager::new(save_dir.clone());
        manager.scan().unwrap();

        let id = manager.list()[0].id;

        // Backup
        let backup_path = manager.backup(&id, &backup_dir).unwrap();
        assert!(backup_path.exists());

        // Delete the original
        manager.delete(&id).unwrap();
        assert_eq!(manager.list().len(), 0);

        // Restore from backup
        let restored = manager.restore(&backup_path).unwrap();
        assert_eq!(manager.list().len(), 1);
        assert!(restored.path.exists());
        assert_ne!(restored.id, id); // New ID assigned

        // Content should match
        let content = std::fs::read(&restored.path).unwrap();
        assert_eq!(content, b"dummy save content");
    }

    // ── Helper: list_mut for tests ──────────────────────────────────────

    /// Helper that provides mutable access to the saves list (for test setup).
    impl SaveManager {
        fn list_mut(&mut self) -> &mut Vec<SaveInfo> {
            &mut self.saves
        }
    }
}