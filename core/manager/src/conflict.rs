//! Mod conflict detection — detect conflicts between NewGRF, AI, and GameScript mods.
//!
//! Conflicts can occur when multiple mods modify the same game element,
//! depend on incompatible versions, or have overlapping functionality.

use std::collections::HashMap;
use crate::version::ModInfo;

/// Severity of a mod conflict
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConflictSeverity {
    /// The conflict will likely cause a crash or broken gameplay
    Critical,
    /// The conflict may cause unexpected behavior
    Warning,
    /// Informational note about mod interaction
    Info,
}

/// Type of mod conflict detected
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConflictType {
    /// Two mods define the same (graphics) ID
    DuplicateId {
        id: String,
        mods: Vec<String>,
    },
    /// A mod depends on another that is not installed
    MissingDependency {
        mod_name: String,
        missing_dep: String,
    },
    /// Two mods are known to be incompatible
    KnownIncompatible {
        mod_a: String,
        mod_b: String,
        reason: String,
    },
    /// Version mismatch — a mod requires a different version of another
    VersionMismatch {
        mod_name: String,
        depends_on: String,
        required_version: String,
        installed_version: String,
    },
    /// Multiple mods that modify the same game mechanic
    OverlappingFunction {
        mechanic: String,
        mods: Vec<String>,
    },
}

/// A detected conflict between mods
#[derive(Debug, Clone)]
pub struct ModConflict {
    pub severity: ConflictSeverity,
    pub conflict_type: ConflictType,
    pub description: String,
}

/// Result of a conflict scan
#[derive(Debug, Clone)]
pub struct ConflictReport {
    pub conflicts: Vec<ModConflict>,
    pub total_mods: usize,
    pub critical_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
}

/// Known incompatibilities between mods
/// Format: (mod_a_pattern, mod_b_pattern, reason)
static KNOWN_INCOMPATIBILITIES: &[(&str, &str, &str)] = &[
    ("OpenGFX", "zBase", "Both replace base graphics set"),
    ("OpenSFX", "zBaseSFX", "Both replace base sound set"),
    ("OpenMSX", "zBaseMusic", "Both replace base music set"),
    ("FIRST", "FIRST2", "FIRST v2 is a complete replacement for v1"),
    ("av8", "Av8", "Duplicate aircraft sets (case variant)"),
    ("North American City Set", "American City Set", "Both modify town building generation"),
    ("Total Town Replacement", "City Growth Manager", "Both modify town growth mechanics"),
];

/// Known dependency requirements
/// Format: (mod_name, depends_on)
static KNOWN_DEPENDENCIES: &[(&str, &str)] = &[
    ("FIRST", "OpenGFX"),
    ("av8", "OpenGFX"),
    ("City Growth Manager", "OpenGFX"),
];

/// Mod conflict detector
pub struct ConflictDetector;

impl ConflictDetector {
    /// Scan a list of mods for conflicts
    pub fn scan(mods: &[ModInfo]) -> ConflictReport {
        let mut conflicts = Vec::new();
        let mut critical = 0;
        let mut warnings = 0;
        let mut info = 0;

        // Check for duplicate IDs
        let mut id_map: HashMap<&str, Vec<&str>> = HashMap::new();
        for m in mods {
            id_map.entry(m.id.as_str()).or_default().push(m.name.as_str());
        }
        for (id, names) in &id_map {
            if names.len() > 1 {
                let mod_names: Vec<String> = names.iter().map(|n| n.to_string()).collect();
                let desc = format!("Multiple mods share ID '{}': {}", id, mod_names.join(", "));
                conflicts.push(ModConflict {
                    severity: ConflictSeverity::Critical,
                    conflict_type: ConflictType::DuplicateId {
                        id: id.to_string(),
                        mods: mod_names.clone(),
                    },
                    description: desc,
                });
                critical += 1;
            }
        }

        // Check for known incompatibilities
        for (pat_a, pat_b, reason) in KNOWN_INCOMPATIBILITIES {
            let has_a = mods.iter().any(|m| m.name.contains(pat_a));
            let has_b = mods.iter().any(|m| m.name.contains(pat_b));
            if has_a && has_b {
                let desc = format!("Incompatible mods: {} and {} — {}", pat_a, pat_b, reason);
                conflicts.push(ModConflict {
                    severity: ConflictSeverity::Critical,
                    conflict_type: ConflictType::KnownIncompatible {
                        mod_a: pat_a.to_string(),
                        mod_b: pat_b.to_string(),
                        reason: reason.to_string(),
                    },
                    description: desc,
                });
                critical += 1;
            }
        }

        // Check for missing dependencies
        for (mod_name, dep) in KNOWN_DEPENDENCIES {
            let has_mod = mods.iter().any(|m| m.name.contains(mod_name));
            let has_dep = mods.iter().any(|m| m.name.contains(dep));
            if has_mod && !has_dep {
                let desc = format!("{} requires {} which is not installed", mod_name, dep);
                conflicts.push(ModConflict {
                    severity: ConflictSeverity::Warning,
                    conflict_type: ConflictType::MissingDependency {
                        mod_name: mod_name.to_string(),
                        missing_dep: dep.to_string(),
                    },
                    description: desc,
                });
                warnings += 1;
            }
        }

        // Check for overlapping categories
        let mut category_map: HashMap<&str, Vec<&str>> = HashMap::new();
        for m in mods {
            if let Some(ref cat) = m.category {
                category_map.entry(cat).or_default().push(m.name.as_str());
            }
        }
        for (cat, names) in &category_map {
            if names.len() > 2 {
                let mod_names: Vec<String> = names.iter().map(|n| n.to_string()).collect();
                let desc = format!("Multiple mods in category '{}': {}", cat, mod_names.join(", "));
                conflicts.push(ModConflict {
                    severity: ConflictSeverity::Info,
                    conflict_type: ConflictType::OverlappingFunction {
                        mechanic: cat.to_string(),
                        mods: mod_names,
                    },
                    description: desc,
                });
                info += 1;
            }
        }

        ConflictReport {
            conflicts,
            total_mods: mods.len(),
            critical_count: critical,
            warning_count: warnings,
            info_count: info,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::version::ModType;

    fn make_mod(id: &str, name: &str, category: Option<&str>) -> ModInfo {
        ModInfo {
            id: id.to_string(),
            name: name.to_string(),
            version: "1.0.0".to_string(),
            mod_type: ModType::NewGRF,
            author: "Test".to_string(),
            description: "".to_string(),
            url: None,
            category: category.map(|c| c.to_string()),
            compatibility: None,
            download_url: None,
            filesize: None,
            created_at: None,
            updated_at: None,
        }
    }

    #[test]
    fn test_no_conflicts() {
        let mods = vec![
            make_mod("a", "Mod A", Some("transport")),
            make_mod("b", "Mod B", Some("industry")),
        ];
        let report = ConflictDetector::scan(&mods);
        assert_eq!(report.conflicts.len(), 0);
    }

    #[test]
    fn test_duplicate_id() {
        let mods = vec![
            make_mod("same", "Mod A", None),
            make_mod("same", "Mod B", None),
        ];
        let report = ConflictDetector::scan(&mods);
        assert!(report.critical_count > 0);
        assert!(report.conflicts.iter().any(|c| matches!(c.conflict_type, ConflictType::DuplicateId { .. })));
    }

    #[test]
    fn test_known_incompatibility() {
        let mods = vec![
            make_mod("1", "OpenGFX Base", None),
            make_mod("2", "zBase Graphics", None),
        ];
        let report = ConflictDetector::scan(&mods);
        assert!(report.critical_count > 0);
    }

    #[test]
    fn test_missing_dependency() {
        let mods = vec![
            make_mod("1", "FIRST Industry", None),
            // OpenGFX is missing
        ];
        let report = ConflictDetector::scan(&mods);
        assert!(report.warning_count > 0);
    }

    #[test]
    fn test_overlapping_category() {
        let mods = vec![
            make_mod("a", "Mod A", Some("transport")),
            make_mod("b", "Mod B", Some("transport")),
            make_mod("c", "Mod C", Some("transport")),
        ];
        let report = ConflictDetector::scan(&mods);
        assert!(report.info_count > 0);
    }

    #[test]
    fn test_report_counts() {
        let mods = vec![
            make_mod("same", "Mod A", Some("transport")),
            make_mod("same", "Mod B", Some("transport")),
            make_mod("c", "FIRST Industry", None),
        ];
        let report = ConflictDetector::scan(&mods);
        assert_eq!(report.total_mods, 3);
        assert!(report.critical_count > 0);
    }
}