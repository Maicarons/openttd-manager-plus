//! Mobile pages — Home, Versions, Downloads, Settings

pub mod home;
pub mod versions;
pub mod downloads;
pub mod settings;

pub use home::HomePage;
pub use versions::VersionsPage;
pub use downloads::DownloadsPage;
pub use settings::SettingsPage;