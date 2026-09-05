//! Parse and generate `openttd.cfg` configuration files.
//!
//! This module provides the [`ConfigFile`] struct which can parse, modify,
//! and write OpenTTD configuration files. The format is a simple INI-like
//! structure:
//!
//! ```text
//! # Comment
//! [section]
//! key = value
//! [misc]
//! resolution = 1920,1080
//! display = 0
//! ```

use std::fmt;
use std::path::Path;

use crate::{Error, Result};

/// A parsed OpenTTD configuration file, consisting of an ordered list of entries.
///
/// Entries are stored in insertion order and may optionally belong to a section.
/// Comments and blank lines are not preserved during round-trip serialization.
#[derive(Debug, Clone)]
pub struct ConfigFile {
    /// Ordered list of configuration entries.
    entries: Vec<ConfigEntry>,
}

/// A single configuration entry from an `openttd.cfg` file.
#[derive(Debug, Clone)]
pub struct ConfigEntry {
    /// Optional section header. `None` for entries before any `[section]` line.
    pub section: Option<String>,
    /// Configuration key name.
    pub key: String,
    /// Configuration value string.
    pub value: String,
}

impl ConfigFile {
    /// Create a new empty `ConfigFile`.
    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Parse an `openttd.cfg` file from the given filesystem path.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or if the content is not
    /// valid UTF-8.
    pub fn parse(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::parse_str(&content)
    }

    /// Parse an `openttd.cfg` from a string.
    ///
    /// # Errors
    ///
    /// Returns an error if the content contains invalid lines that cannot be
    /// parsed.
    pub fn parse_str(content: &str) -> Result<Self> {
        let mut entries = Vec::new();
        let mut current_section: Option<String> = None;

        for (line_num, raw_line) in content.lines().enumerate() {
            let line = raw_line.trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Section header
            if line.starts_with('[') {
                let section = line
                    .strip_prefix('[')
                    .and_then(|s| s.strip_suffix(']'))
                    .map(|s| s.trim().to_string())
                    .ok_or_else(|| {
                        Error::Parse(format!(
                            "Malformed section header at line {}: {raw_line}",
                            line_num + 1,
                        ))
                    })?;
                current_section = Some(section);
                continue;
            }

            // Key-value pair
            if let Some(eq_pos) = line.find('=') {
                // Check for escaped `==` or malformed key
                if eq_pos == 0 {
                    return Err(Error::Parse(format!(
                        "Missing key before '=' at line {}: {raw_line}",
                        line_num + 1,
                    )));
                }
                let key = line[..eq_pos].trim().to_string();
                let value = line[eq_pos + 1..].trim().to_string();
                entries.push(ConfigEntry {
                    section: current_section.clone(),
                    key,
                    value,
                });
            } else {
                return Err(Error::Parse(format!(
                    "Line does not contain a key-value pair or section header: {}",
                    line_num + 1,
                )));
            }
        }

        Ok(Self { entries })
    }

    /// Write the configuration to a file at the given path.
    ///
    /// Creates the parent directory if it does not exist.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    pub fn write(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = self.to_string();
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Get a configuration value by key, optionally within a section.
    ///
    /// Returns the value of the *last* matching entry (most recently set).
    /// If `section` is `None`, matches entries in any section as well as
    /// section-less entries.
    pub fn get(&self, key: &str, section: Option<&str>) -> Option<&str> {
        self.entries
            .iter()
            .rev()
            .find(|e| {
                e.key == key
                    && match section {
                        Some(s) => e.section.as_deref() == Some(s),
                        None => true,
                    }
            })
            .map(|e| e.value.as_str())
    }

    /// Set a configuration value.
    ///
    /// If an entry with the same key (and section, if provided) already exists,
    /// its value is updated in place. Otherwise, a new entry is appended.
    ///
    /// When `section` is `Some`, only entries within that section are considered
    /// for update. When `section` is `None`, only entries without a section are
    /// considered.
    pub fn set(&mut self, key: &str, value: &str, section: Option<&str>) {
        let section_owned = section.map(|s| s.to_string());
        // Try to find an existing entry to update
        if let Some(entry) = self
            .entries
            .iter_mut()
            .rev()
            .find(|e| e.key == key && e.section.as_deref() == section)
        {
            entry.value = value.to_string();
            return;
        }
        // Append a new entry
        self.entries.push(ConfigEntry {
            section: section_owned,
            key: key.to_string(),
            value: value.to_string(),
        });
    }

    /// Remove *all* entries matching the given key, optionally within a section.
    ///
    /// When `section` is `Some`, only entries within that section are removed.
    /// When `section` is `None`, all entries with the given key are removed
    /// regardless of section.
    pub fn remove(&mut self, key: &str, section: Option<&str>) {
        self.entries.retain(|e| {
            if e.key != key {
                return true;
            }
            match section {
                Some(s) => e.section.as_deref() != Some(s),
                None => false,
            }
        });
    }

    /// Generate a default `openttd.cfg` suitable for a modern OpenTTD version.
    ///
    /// The generated configuration includes sensible defaults for graphics,
    /// sound, music, and display settings.
    pub fn generate_default() -> Self {
        let mut cfg = Self::new();
        cfg.set("player_name", "Player", None);
        cfg.set("player_company", "0", None);
        cfg.set("resolution", "1920,1080", Some("misc"));
        cfg.set("fullscreen", "false", Some("misc"));
        cfg.set("display", "0", Some("misc"));
        cfg.set("music_volume", "127", Some("misc"));
        cfg.set("effect_volume", "127", Some("misc"));
        cfg.set("sfx_volume", "127", Some("misc"));
        cfg.set("last_newgrf_count", "0", None);
        // Basic game settings
        cfg.set("language", "english", Some("gameopt"));
        cfg.set("currency", "GBP", Some("gameopt"));
        cfg.set("road_side", "left", Some("gameopt"));
        cfg.set("town_name", "english", Some("gameopt"));
        cfg.set("autosave", "monthly", Some("gameopt"));
        // Network defaults
        cfg.set("max_players", "15", Some("network"));
        cfg.set("server_port", "3979", Some("network"));
        cfg.set("server_ip", "0.0.0.0", Some("network"));
        cfg.set("server_password", "", Some("network"));
        cfg.set("rcon_password", "", Some("network"));
        cfg
    }
}

impl fmt::Display for ConfigFile {
    /// Serialize the config file back to its text representation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut last_section: Option<&str> = None;

        for entry in &self.entries {
            let section = entry.section.as_deref();
            if section != last_section {
                if let Some(s) = section {
                    if last_section.is_some() {
                        writeln!(f)?;
                    }
                    writeln!(f, "[{s}]")?;
                }
                last_section = section;
            }
            writeln!(f, "{} = {}", entry.key, entry.value)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let content = r#"
# This is a comment
[section]
key = value
[misc]
resolution = 1920,1080
display = 0
"#;
        let cfg = ConfigFile::parse_str(content).unwrap();
        assert_eq!(cfg.entries.len(), 3);
        assert_eq!(cfg.entries[0].key, "key");
        assert_eq!(cfg.entries[0].value, "value");
        assert_eq!(cfg.entries[0].section.as_deref(), Some("section"));
        assert_eq!(cfg.entries[1].key, "resolution");
        assert_eq!(cfg.entries[1].value, "1920,1080");
        assert_eq!(cfg.entries[2].key, "display");
        assert_eq!(cfg.entries[2].value, "0");
    }

    #[test]
    fn test_parse_empty() {
        let cfg = ConfigFile::parse_str("").unwrap();
        assert_eq!(cfg.entries.len(), 0);
    }

    #[test]
    fn test_parse_only_comments() {
        let cfg = ConfigFile::parse_str("# just a comment\n# another").unwrap();
        assert_eq!(cfg.entries.len(), 0);
    }

    #[test]
    fn test_get() {
        let content = "[section]\nkey = value\nkey = other\n";
        let cfg = ConfigFile::parse_str(content).unwrap();
        // Should return the last value
        assert_eq!(cfg.get("key", Some("section")), Some("other"));
    }

    #[test]
    fn test_get_nonexistent() {
        let cfg = ConfigFile::parse_str("key = value").unwrap();
        assert_eq!(cfg.get("nope", None), None);
    }

    #[test]
    fn test_get_without_section() {
        let cfg = ConfigFile::parse_str("key = value\n[misc]\nkey = other").unwrap();
        // Without section filter, returns the last entry regardless of section
        assert_eq!(cfg.get("key", None), Some("other"));
        // With section filter
        assert_eq!(cfg.get("key", Some("misc")), Some("other"));
    }

    #[test]
    fn test_set_new() {
        let mut cfg = ConfigFile::new();
        cfg.set("key", "value", None);
        assert_eq!(cfg.entries.len(), 1);
        assert_eq!(cfg.get("key", None), Some("value"));
    }

    #[test]
    fn test_set_update_existing() {
        let mut cfg = ConfigFile::new();
        cfg.set("key", "old", None);
        cfg.set("key", "new", None);
        assert_eq!(cfg.entries.len(), 1);
        assert_eq!(cfg.get("key", None), Some("new"));
    }

    #[test]
    fn test_set_with_section() {
        let mut cfg = ConfigFile::new();
        cfg.set("key", "no_section", None);
        cfg.set("key", "in_section", Some("sec"));
        // Two entries: one without section, one in section
        assert_eq!(cfg.entries.len(), 2);
        assert_eq!(cfg.get("key", None), Some("in_section"));
        assert_eq!(cfg.get("key", Some("sec")), Some("in_section"));
        assert_eq!(cfg.get("key", Some("other")), None);
    }

    #[test]
    fn test_remove() {
        let mut cfg = ConfigFile::new();
        cfg.set("key", "value", None);
        cfg.set("other", "keep", None);
        cfg.remove("key", None);
        assert_eq!(cfg.entries.len(), 1);
        assert_eq!(cfg.entries[0].key, "other");
    }

    #[test]
    fn test_remove_with_section() {
        let mut cfg = ConfigFile::new();
        cfg.set("key", "no_section", None);
        cfg.set("key", "in_section", Some("sec"));
        cfg.remove("key", Some("sec"));
        assert_eq!(cfg.entries.len(), 1);
        assert_eq!(cfg.entries[0].section, None);
    }

    #[test]
    fn test_roundtrip() {
        let original = "[section]\nkey = value\n\n[misc]\nresolution = 1920,1080\n";
        let cfg = ConfigFile::parse_str(original).unwrap();
        let output = cfg.to_string();
        // Parse again and verify
        let cfg2 = ConfigFile::parse_str(&output).unwrap();
        assert_eq!(cfg2.get("key", Some("section")), Some("value"));
        assert_eq!(cfg2.get("resolution", Some("misc")), Some("1920,1080"));
    }

    #[test]
    fn test_write_read_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("openttd.cfg");
        let mut cfg = ConfigFile::new();
        cfg.set("key", "value", None);
        cfg.set("resolution", "800,600", Some("misc"));
        cfg.write(&path).unwrap();

        let loaded = ConfigFile::parse(&path).unwrap();
        assert_eq!(loaded.get("key", None), Some("value"));
        assert_eq!(loaded.get("resolution", Some("misc")), Some("800,600"));
    }

    #[test]
    fn test_generate_default() {
        let cfg = ConfigFile::generate_default();
        // Should have several entries
        assert!(!cfg.entries.is_empty());
        // Check some expected defaults
        assert_eq!(cfg.get("player_name", None), Some("Player"));
        assert_eq!(cfg.get("resolution", Some("misc")), Some("1920,1080"));
        assert_eq!(cfg.get("server_port", Some("network")), Some("3979"));
        // Should be parseable
        let serialized = cfg.to_string();
        let reparsed = ConfigFile::parse_str(&serialized).unwrap();
        assert_eq!(reparsed.entries.len(), cfg.entries.len());
    }

    #[test]
    fn test_parse_malformed_section() {
        let result = ConfigFile::parse_str("[bad section\nkey = value");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_key() {
        let result = ConfigFile::parse_str("= value");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_malformed_line() {
        let result = ConfigFile::parse_str("just some text");
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_default_produces_valid_parseable_config() {
        let cfg = ConfigFile::generate_default();
        let serialized = cfg.to_string();
        // Should not be empty
        assert!(!serialized.is_empty());
        // Should parse without error
        let reparsed = ConfigFile::parse_str(&serialized).unwrap();
        assert_eq!(reparsed.entries.len(), cfg.entries.len());
        // Verify key structure - all keys should be non-empty
        for entry in &reparsed.entries {
            assert!(!entry.key.is_empty());
        }
    }
}