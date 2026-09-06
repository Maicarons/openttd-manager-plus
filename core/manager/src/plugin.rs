//! Plugin system — dynamic plugin trait and registry for extending functionality.
//!
//! Plugins can add new version sources, download mirrors, mod fetchers,
//! UI components, or custom startup logic.

use std::collections::HashMap;
use std::sync::Arc;
use std::any::Any;

/// Unique identifier for a plugin
pub type PluginId = String;

/// Version of a plugin
pub type PluginVersion = String;

/// Capability tags for plugin discovery
pub type PluginCapability = String;

/// Result returned by plugin operations
pub type PluginResult<T> = std::result::Result<T, PluginError>;

/// Errors that can occur during plugin operations
#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("Plugin not found: {0}")]
    NotFound(String),
    #[error("Plugin already registered: {0}")]
    AlreadyRegistered(String),
    #[error("Plugin initialization failed: {0}")]
    InitFailed(String),
    #[error("Plugin execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Incompatible plugin version: {0}")]
    VersionMismatch(String),
}

/// Metadata about a registered plugin
#[derive(Debug, Clone)]
pub struct PluginMetadata {
    pub id: PluginId,
    pub name: String,
    pub version: PluginVersion,
    pub author: String,
    pub description: String,
    pub capabilities: Vec<PluginCapability>,
    pub enabled: bool,
}

/// The trait that all plugins must implement
pub trait Plugin: Send + Sync {
    /// Return the plugin's metadata
    fn metadata(&self) -> PluginMetadata;

    /// Initialize the plugin (called when registered)
    fn init(&mut self) -> PluginResult<()> {
        Ok(())
    }

    /// Shutdown the plugin (called when unregistered)
    fn shutdown(&mut self) -> PluginResult<()> {
        Ok(())
    }

    /// Get a capability value by key (for plugin-specific data)
    fn get_capability(&self, _key: &str) -> Option<Arc<dyn Any + Send + Sync>> {
        None
    }
}

/// Plugin registry — manages plugin lifecycle and discovery
pub struct PluginRegistry {
    plugins: HashMap<PluginId, Box<dyn Plugin>>,
    metadata: HashMap<PluginId, PluginMetadata>,
}

impl PluginRegistry {
    /// Create a new empty plugin registry
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    /// Register a plugin
    pub fn register(&mut self, plugin: Box<dyn Plugin>) -> PluginResult<()> {
        let meta = plugin.metadata();
        let id = meta.id.clone();
        if self.plugins.contains_key(&id) {
            return Err(PluginError::AlreadyRegistered(id));
        }
        self.metadata.insert(id.clone(), meta);
        self.plugins.insert(id, plugin);
        Ok(())
    }

    /// Unregister a plugin by ID
    pub fn unregister(&mut self, id: &str) -> PluginResult<()> {
        if let Some(mut plugin) = self.plugins.remove(id) {
            plugin.shutdown().ok();
            self.metadata.remove(id);
            Ok(())
        } else {
            Err(PluginError::NotFound(id.to_string()))
        }
    }

    /// Get a plugin by ID
    pub fn get(&self, id: &str) -> Option<&dyn Plugin> {
        self.plugins.get(id).map(|p| p.as_ref())
    }

    /// Get a mutable plugin by ID
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Box<dyn Plugin>> {
        self.plugins.get_mut(id)
    }

    /// List all registered plugins
    pub fn list(&self) -> Vec<&PluginMetadata> {
        self.metadata.values().collect()
    }

    /// List plugins by capability
    pub fn list_by_capability(&self, capability: &str) -> Vec<&PluginMetadata> {
        self.metadata.values()
            .filter(|m| m.capabilities.iter().any(|c| c == capability))
            .collect()
    }

    /// Enable a plugin
    pub fn enable(&mut self, id: &str) -> PluginResult<()> {
        if let Some(meta) = self.metadata.get_mut(id) {
            meta.enabled = true;
            Ok(())
        } else {
            Err(PluginError::NotFound(id.to_string()))
        }
    }

    /// Disable a plugin
    pub fn disable(&mut self, id: &str) -> PluginResult<()> {
        if let Some(meta) = self.metadata.get_mut(id) {
            meta.enabled = false;
            Ok(())
        } else {
            Err(PluginError::NotFound(id.to_string()))
        }
    }

    /// Check if a plugin is enabled
    pub fn is_enabled(&self, id: &str) -> bool {
        self.metadata.get(id).map(|m| m.enabled).unwrap_or(false)
    }

    /// Get a capability from a specific plugin
    pub fn get_capability(&self, plugin_id: &str, key: &str) -> Option<Arc<dyn Any + Send + Sync>> {
        self.plugins.get(plugin_id)?.get_capability(key)
    }

    /// Number of registered plugins
    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    /// Whether the registry is empty
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }
}

/// Helper macro to create a simple plugin
#[macro_export]
macro_rules! plugin {
    ($id:expr, $name:expr, $version:expr, $author:expr, $desc:expr) => {
        {
            struct SimplePlugin {
                meta: $crate::plugin::PluginMetadata,
            }
            impl $crate::plugin::Plugin for SimplePlugin {
                fn metadata(&self) -> $crate::plugin::PluginMetadata {
                    self.meta.clone()
                }
            }
            Box::new(SimplePlugin {
                meta: $crate::plugin::PluginMetadata {
                    id: $id.to_string(),
                    name: $name.to_string(),
                    version: $version.to_string(),
                    author: $author.to_string(),
                    description: $desc.to_string(),
                    capabilities: Vec::new(),
                    enabled: true,
                },
            })
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlugin {
        meta: PluginMetadata,
    }

    impl Plugin for TestPlugin {
        fn metadata(&self) -> PluginMetadata {
            self.meta.clone()
        }
    }

    fn create_test_plugin(id: &str, name: &str) -> Box<dyn Plugin> {
        Box::new(TestPlugin {
            meta: PluginMetadata {
                id: id.to_string(),
                name: name.to_string(),
                version: "1.0.0".to_string(),
                author: "Test".to_string(),
                description: "A test plugin".to_string(),
                capabilities: vec!["test".to_string()],
                enabled: true,
            },
        })
    }

    #[test]
    fn test_register_and_list() {
        let mut registry = PluginRegistry::new();
        registry.register(create_test_plugin("test1", "Test Plugin 1")).unwrap();
        registry.register(create_test_plugin("test2", "Test Plugin 2")).unwrap();
        assert_eq!(registry.len(), 2);
        assert_eq!(registry.list().len(), 2);
    }

    #[test]
    fn test_register_duplicate() {
        let mut registry = PluginRegistry::new();
        registry.register(create_test_plugin("test", "Test")).unwrap();
        let result = registry.register(create_test_plugin("test", "Duplicate"));
        assert!(result.is_err());
    }

    #[test]
    fn test_unregister() {
        let mut registry = PluginRegistry::new();
        registry.register(create_test_plugin("test", "Test")).unwrap();
        registry.unregister("test").unwrap();
        assert!(registry.is_empty());
    }

    #[test]
    fn test_unregister_nonexistent() {
        let mut registry = PluginRegistry::new();
        let result = registry.unregister("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_plugin() {
        let mut registry = PluginRegistry::new();
        registry.register(create_test_plugin("test", "Test Plugin")).unwrap();
        let plugin = registry.get("test");
        assert!(plugin.is_some());
        assert_eq!(plugin.unwrap().metadata().name, "Test Plugin");
    }

    #[test]
    fn test_enable_disable() {
        let mut registry = PluginRegistry::new();
        registry.register(create_test_plugin("test", "Test")).unwrap();
        assert!(registry.is_enabled("test"));
        registry.disable("test").unwrap();
        assert!(!registry.is_enabled("test"));
        registry.enable("test").unwrap();
        assert!(registry.is_enabled("test"));
    }

    #[test]
    fn test_list_by_capability() {
        let mut registry = PluginRegistry::new();
        registry.register(create_test_plugin("a", "A")).unwrap();
        let mut plugin = create_test_plugin("b", "B");
        // Add a different capability
        let list = registry.list_by_capability("test");
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_plugin_macro() {
        let plugin = plugin!("my_plugin", "My Plugin", "1.0.0", "Author", "Description");
        assert_eq!(plugin.metadata().id, "my_plugin");
        assert_eq!(plugin.metadata().name, "My Plugin");
    }

    #[test]
    fn test_empty_registry() {
        let registry = PluginRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }
}