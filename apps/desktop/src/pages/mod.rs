//! Page definitions and routing

pub mod home;
pub mod versions;
pub mod downloads;
pub mod configs;
pub mod mods;
pub mod settings;

pub use home::HomePage;
pub use versions::VersionsPage;
pub use downloads::DownloadsPage;
pub use configs::ConfigsPage;
pub use mods::ModsPage;
pub use settings::SettingsPage;

/// Enum of all pages in the application
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Page {
    Home,
    Versions,
    Downloads,
    Configs,
    Mods,
    Settings,
}

impl Page {
    /// Get the display title for this page
    pub fn title(&self) -> &'static str {
        match self {
            Page::Home => "Home",
            Page::Versions => "Version Management",
            Page::Downloads => "Download Manager",
            Page::Configs => "Configuration",
            Page::Mods => "Mod Management",
            Page::Settings => "Settings",
        }
    }
}