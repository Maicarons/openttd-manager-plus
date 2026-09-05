//! Download queue manager
//!
//! Manages multiple download tasks with priority ordering, status tracking,
//! concurrency control, and retry logic.

use std::path::PathBuf;
use std::sync::Arc;
use chrono::{NaiveDateTime, Utc};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::engine::{DownloadConfig, DownloadEngine};
use crate::{Error, Result};

/// Status of a download task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Queued,
    Downloading,
    Paused,
    Completed,
    Failed(String),
    Cancelled,
}

impl TaskStatus {
    pub fn is_terminal(&self) -> bool {
        matches!(self, TaskStatus::Completed | TaskStatus::Failed(_) | TaskStatus::Cancelled)
    }
}

/// Priority level for a download task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Normal
    }
}

/// A single download task in the queue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTask {
    pub id: Uuid,
    pub url: String,
    pub destination: PathBuf,
    pub name: String,
    pub priority: Priority,
    pub status: TaskStatus,
    pub total_bytes: u64,
    pub bytes_downloaded: u64,
    pub speed: f64,
    pub error_message: Option<String>,
    pub created_at: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>,
    pub retry_count: u32,
    pub max_retries: u32,
}

/// Aggregate statistics for the download queue.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct QueueStats {
    pub total: usize,
    pub queued: usize,
    pub downloading: usize,
    pub completed: usize,
    pub failed: usize,
    pub cancelled: usize,
    pub paused: usize,
    pub total_bytes: u64,
    pub downloaded_bytes: u64,
}

/// Callback for task status changes.
pub type TaskCallback = Arc<dyn Fn(&DownloadTask) + Send + Sync>;

struct QueueInner {
    tasks: Vec<DownloadTask>,
    max_concurrent: usize,
    active_count: usize,
    on_change: Option<TaskCallback>,
}

/// Download queue manager.
pub struct DownloadQueue {
    inner: Arc<Mutex<QueueInner>>,
    engine: DownloadEngine,
}

impl DownloadQueue {
    pub fn new(config: DownloadConfig) -> Self {
        Self {
            inner: Arc::new(Mutex::new(QueueInner {
                tasks: Vec::new(),
                max_concurrent: 3,
                active_count: 0,
                on_change: None,
            })),
            engine: DownloadEngine::new(config),
        }
    }

    pub async fn add_task(&self, url: String, destination: PathBuf, name: String, priority: Priority) -> Uuid {
        let id = Uuid::new_v4();
        let task = DownloadTask {
            id,
            url,
            destination,
            name,
            priority,
            status: TaskStatus::Queued,
            total_bytes: 0,
            bytes_downloaded: 0,
            speed: 0.0,
            error_message: None,
            created_at: Utc::now().naive_utc(),
            completed_at: None,
            retry_count: 0,
            max_retries: 3,
        };
        let mut guard = self.inner.lock().await;
        guard.tasks.push(task.clone());
        Self::notify(&guard.on_change, &task);
        info!("Added task {} ({})", task.name, task.id);
        id
    }

    pub async fn remove_task(&self, id: &Uuid) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let len_before = guard.tasks.len();
        guard.tasks.retain(|t| &t.id != id);
        if guard.tasks.len() == len_before {
            return Err(Error::InvalidUrl(format!("Task {} not found", id)));
        }
        Ok(())
    }

    pub async fn pause(&self, id: &Uuid) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let idx = guard.tasks.iter().position(|t| &t.id == id)
            .ok_or_else(|| Error::InvalidUrl(format!("Task {} not found", id)))?;
        let was_downloading = guard.tasks[idx].status == TaskStatus::Downloading;
        if was_downloading {
            guard.tasks[idx].status = TaskStatus::Paused;
            guard.active_count = guard.active_count.saturating_sub(1);
            let snapshot = guard.tasks[idx].clone();
            Self::notify(&guard.on_change, &snapshot);
        }
        Ok(())
    }

    pub async fn resume(&self, id: &Uuid) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let idx = guard.tasks.iter().position(|t| &t.id == id)
            .ok_or_else(|| Error::InvalidUrl(format!("Task {} not found", id)))?;
        if guard.tasks[idx].status == TaskStatus::Paused {
            guard.tasks[idx].status = TaskStatus::Queued;
            let snapshot = guard.tasks[idx].clone();
            Self::notify(&guard.on_change, &snapshot);
        }
        Ok(())
    }

    pub async fn cancel(&self, id: &Uuid) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let idx = guard.tasks.iter().position(|t| &t.id == id)
            .ok_or_else(|| Error::InvalidUrl(format!("Task {} not found", id)))?;
        let is_terminal = guard.tasks[idx].status.is_terminal();
        if !is_terminal {
            if guard.tasks[idx].status == TaskStatus::Downloading {
                guard.active_count = guard.active_count.saturating_sub(1);
            }
            guard.tasks[idx].status = TaskStatus::Cancelled;
            let snapshot = guard.tasks[idx].clone();
            Self::notify(&guard.on_change, &snapshot);
        }
        Ok(())
    }

    pub async fn get_task(&self, id: &Uuid) -> Option<DownloadTask> {
        let guard = self.inner.lock().await;
        guard.tasks.iter().find(|t| &t.id == id).cloned()
    }

    pub async fn list_tasks(&self) -> Vec<DownloadTask> {
        let guard = self.inner.lock().await;
        guard.tasks.clone()
    }

    pub async fn stats(&self) -> QueueStats {
        let guard = self.inner.lock().await;
        let mut stats = QueueStats::default();
        stats.total = guard.tasks.len();
        for task in &guard.tasks {
            stats.total_bytes += task.total_bytes;
            stats.downloaded_bytes += task.bytes_downloaded;
            match task.status {
                TaskStatus::Queued => stats.queued += 1,
                TaskStatus::Downloading => stats.downloading += 1,
                TaskStatus::Completed => stats.completed += 1,
                TaskStatus::Failed(_) => stats.failed += 1,
                TaskStatus::Cancelled => stats.cancelled += 1,
                TaskStatus::Paused => stats.paused += 1,
            }
        }
        stats
    }

    pub async fn clear_completed(&self) {
        let mut guard = self.inner.lock().await;
        guard.tasks.retain(|t| !t.status.is_terminal() && t.status != TaskStatus::Completed);
    }

    pub async fn cancel_all(&self) {
        let mut guard = self.inner.lock().await;
        for task in guard.tasks.iter_mut() {
            if !task.status.is_terminal() {
                task.status = TaskStatus::Cancelled;
            }
        }
        guard.active_count = 0;
    }

    pub async fn retry(&self, id: &Uuid) -> Result<()> {
        let mut guard = self.inner.lock().await;
        let idx = guard.tasks.iter().position(|t| &t.id == id)
            .ok_or_else(|| Error::InvalidUrl(format!("Task {} not found", id)))?;
        let is_failed = matches!(guard.tasks[idx].status, TaskStatus::Failed(_));
        if is_failed {
            guard.tasks[idx].status = TaskStatus::Queued;
            guard.tasks[idx].retry_count = 0;
            guard.tasks[idx].bytes_downloaded = 0;
            guard.tasks[idx].error_message = None;
            let snapshot = guard.tasks[idx].clone();
            Self::notify(&guard.on_change, &snapshot);
        }
        Ok(())
    }

    pub async fn process(&self) {
        let snapshots: Vec<DownloadTask> = {
            let mut guard = self.inner.lock().await;
            let slots = guard.max_concurrent.saturating_sub(guard.active_count);
            if slots == 0 { return; }
            let mut indices: Vec<usize> = (0..guard.tasks.len())
                .filter(|&i| guard.tasks[i].status == TaskStatus::Queued)
                .collect();
            indices.sort_by(|&a, &b| guard.tasks[b].priority.cmp(&guard.tasks[a].priority));
            indices.truncate(slots);
            for &i in &indices {
                guard.tasks[i].status = TaskStatus::Downloading;
                guard.active_count += 1;
            }
            indices.iter().map(|&i| guard.tasks[i].clone()).collect()
        };
        for task in snapshots {
            let engine = self.engine.clone();
            let inner = self.inner.clone();
            tokio::spawn(async move {
                let result = engine.download(&task.url, &task.destination, None).await;
                let mut guard = inner.lock().await;
                guard.active_count = guard.active_count.saturating_sub(1);
                if let Some(t) = guard.tasks.iter_mut().find(|t| t.id == task.id) {
                    match result {
                        Ok(()) => {
                            t.status = TaskStatus::Completed;
                            t.completed_at = Some(Utc::now().naive_utc());
                            info!("Completed task {} ({})", task.name, task.id);
                        }
                        Err(e) => {
                            if t.retry_count < t.max_retries {
                                t.retry_count += 1;
                                t.status = TaskStatus::Queued;
                                warn!("Task {} failed (retry {}/{}): {}", task.name, t.retry_count, t.max_retries, e);
                            } else {
                                t.status = TaskStatus::Failed(e.to_string());
                                t.error_message = Some(e.to_string());
                                error!("Task {} failed: {}", task.name, e);
                            }
                        }
                    }
                }
            });
        }
    }

    pub fn set_on_change(&self, callback: TaskCallback) {
        let inner = self.inner.clone();
        tokio::spawn(async move {
            let mut guard = inner.lock().await;
            guard.on_change = Some(callback);
        });
    }

    fn notify(on_change: &Option<TaskCallback>, task: &DownloadTask) {
        if let Some(cb) = on_change { cb(task); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_add_and_list() {
        let queue = DownloadQueue::new(DownloadConfig::default());
        let id = queue.add_task("url".into(), PathBuf::from("dest"), "test".into(), Priority::Normal).await;
        let tasks = queue.list_tasks().await;
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, id);
    }

    #[tokio::test]
    async fn test_remove() {
        let queue = DownloadQueue::new(DownloadConfig::default());
        let id = queue.add_task("url".into(), PathBuf::from("dest"), "test".into(), Priority::Normal).await;
        queue.remove_task(&id).await.unwrap();
        assert!(queue.list_tasks().await.is_empty());
    }

    #[tokio::test]
    async fn test_cancel() {
        let queue = DownloadQueue::new(DownloadConfig::default());
        let id = queue.add_task("url".into(), PathBuf::from("dest"), "test".into(), Priority::Normal).await;
        queue.cancel(&id).await.unwrap();
        let task = queue.get_task(&id).await.unwrap();
        assert_eq!(task.status, TaskStatus::Cancelled);
    }

    #[tokio::test]
    async fn test_stats() {
        let queue = DownloadQueue::new(DownloadConfig::default());
        queue.add_task("a".into(), PathBuf::from("a"), "A".into(), Priority::Normal).await;
        queue.add_task("b".into(), PathBuf::from("b"), "B".into(), Priority::Normal).await;
        let stats = queue.stats().await;
        assert_eq!(stats.total, 2);
        assert_eq!(stats.queued, 2);
    }

    #[tokio::test]
    async fn test_clear_completed() {
        let queue = DownloadQueue::new(DownloadConfig::default());
        let id = queue.add_task("url".into(), PathBuf::from("dest"), "test".into(), Priority::Normal).await;
        queue.cancel(&id).await.unwrap();
        queue.clear_completed().await;
        assert!(queue.list_tasks().await.is_empty());
    }
}
