//! Application state management using Dioxus signals.

use dioxus_native::prelude::*;
use otmp_core_manager::source::official::OfficialFetcher;
use otmp_core_manager::source::jgrpp::JgrppFetcher;
use otmp_core_manager::source::cmclient::CmClientFetcher;
use otmp_core_manager::source::VersionFetcher;
use otmp_core_manager::version::{VersionInfo, VersionSource};
use otmp_core_config::instance::{Instance, InstanceManager};
use otmp_core_config::profile::Profile;
use otmp_core_config::profile::ProfileManager;
use otmp_core_config::save::{SaveInfo, SaveManager};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Priority { #[default] Normal, Low, High, Critical }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus { Queued, Downloading, Paused, Completed, Failed, Cancelled }

#[derive(Debug, Clone)]
pub struct DownloadTask {
    pub id: String, pub name: String, pub url: String, pub destination: PathBuf,
    pub total_bytes: u64, pub downloaded_bytes: u64, pub status: TaskStatus,
    pub priority: Priority, pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct QueueStats { pub active: usize, pub queued: usize, pub completed: usize, pub failed: usize, pub total_bytes: u64, pub downloaded_bytes: u64 }

#[derive(Clone)]
pub struct AppState {
    pub versions: Signal<Vec<VersionInfo>>,
    pub filtered_versions: Signal<Vec<VersionInfo>>,
    pub active_filter: Signal<String>,
    pub loading: Signal<bool>,
    pub error: Signal<Option<String>>,
    pub instances: Signal<Vec<Instance>>,
    pub profiles: Signal<Vec<Profile>>,
    pub saves: Signal<Vec<SaveInfo>>,
    pub download_queue: Signal<Vec<DownloadTask>>,
    pub download_stats: Signal<QueueStats>,
    pub data_dir: Signal<PathBuf>,
}

impl AppState {
    pub fn new() -> Self {
        let data_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).join("otmp-data");
        Self {
            versions: Signal::new(Vec::new()),
            filtered_versions: Signal::new(Vec::new()),
            active_filter: Signal::new("All".to_string()),
            loading: Signal::new(false),
            error: Signal::new(None),
            instances: Signal::new(Vec::new()),
            profiles: Signal::new(Vec::new()),
            saves: Signal::new(Vec::new()),
            download_queue: Signal::new(Vec::new()),
            download_stats: Signal::new(QueueStats::default()),
            data_dir: Signal::new(data_dir),
        }
    }

    /// Fetch versions from all configured sources.
    pub async fn fetch_versions(&self) {
        let mut loading = self.loading;
        let mut error = self.error;
        let mut versions = self.versions;

        loading.set(true);
        error.set(None);

        let mut all_versions: Vec<VersionInfo> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        // Official OpenTTD
        let official = OfficialFetcher::new();
        match official.fetch_versions().await {
            Ok(v) => { log::info!("Fetched {} official versions", v.len()); all_versions.extend(v); }
            Err(e) => errors.push(format!("Official: {e}")),
        }

        // JGRPP
        let jgrpp = JgrppFetcher::new();
        match jgrpp.fetch_versions().await {
            Ok(v) => { log::info!("Fetched {} JGRPP versions", v.len()); all_versions.extend(v); }
            Err(e) => errors.push(format!("JGRPP: {e}")),
        }

        // CMClient
        let cmclient = CmClientFetcher::new();
        match cmclient.fetch_versions().await {
            Ok(v) => { log::info!("Fetched {} CMClient versions", v.len()); all_versions.extend(v); }
            Err(e) => errors.push(format!("CMClient: {e}")),
        }

        all_versions.sort_by(|a, b| b.version.cmp(&a.version));
        versions.set(all_versions);
        if !errors.is_empty() {
            error.set(Some(errors.join("; ")));
        }
        loading.set(false);
    }

    pub fn apply_filter(&mut self) {
        let filter = self.active_filter.read().clone();
        let v = self.versions.read().clone();
        let filtered = match filter.as_str() {
            "All" => v,
            "Official" => v.into_iter().filter(|i| i.source == VersionSource::Official).collect(),
            "JGRPP" => v.into_iter().filter(|i| i.source == VersionSource::Jgrpp).collect(),
            "CMClient" => v.into_iter().filter(|i| i.source == VersionSource::CmClient).collect(),
            _ => v,
        };
        self.filtered_versions.set(filtered);
    }

    pub fn set_filter(&mut self, filter: &str) {
        self.active_filter.set(filter.to_string());
        self.apply_filter();
    }

    pub fn load_instances(&mut self) {
        let mut instances = self.instances;
        let data_dir = self.data_dir.read().clone();
        let mut manager = InstanceManager::new(data_dir);
        if let Err(e) = manager.load() { log::error!("Failed to load instances: {e}"); }
        instances.set(manager.list().to_vec());
    }

    pub fn load_profiles(&mut self) {
        let mut profiles = self.profiles;
        let data_dir = self.data_dir.read().clone();
        let mut manager = ProfileManager::new(data_dir);
        if let Err(e) = manager.load() { log::error!("Failed to load profiles: {e}"); }
        profiles.set(manager.list().to_vec());
    }

    pub fn load_saves(&mut self) {
        let mut saves = self.saves;
        let data_dir = self.data_dir.read().clone();
        let mut manager = SaveManager::new(data_dir);
        if let Err(e) = manager.scan() { log::error!("Failed to scan saves: {e}"); }
        saves.set(manager.list().to_vec());
    }

    pub fn refresh_downloads(&mut self) {
        let queue = self.download_queue.read().clone();
        let mut stats = QueueStats::default();
        for task in &queue {
            stats.total_bytes += task.total_bytes;
            stats.downloaded_bytes += task.downloaded_bytes;
            match task.status {
                TaskStatus::Downloading => stats.active += 1,
                TaskStatus::Queued => stats.queued += 1,
                TaskStatus::Completed => stats.completed += 1,
                TaskStatus::Failed => stats.failed += 1,
                _ => {}
            }
        }
        self.download_stats.set(stats);
    }
}