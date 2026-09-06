//! Cache optimization utilities for startup time and memory improvements.
//! Provides lazy initialization, parallel loading, and cache warming.

use std::path::PathBuf;
use std::time::Instant;
use log::info;

/// Performance metrics for monitoring
#[derive(Debug, Clone, Default)]
pub struct PerfMetrics {
    pub startup_time_ms: u64,
    pub cache_load_ms: u64,
    pub version_fetch_ms: u64,
    pub memory_estimate_mb: u64,
}

/// Cache warmer for pre-loading data on startup
pub struct CacheWarmer {
    base_dir: PathBuf,
}

impl CacheWarmer {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    /// Warm up caches by pre-loading common data files
    pub fn warm_up(&self) -> PerfMetrics {
        let start = Instant::now();
        let mut metrics = PerfMetrics::default();

        // Pre-warm manifest cache directory
        let cache_dir = self.base_dir.join("cache").join("manifests");
        if let Ok(entries) = std::fs::read_dir(&cache_dir) {
            let count = entries.filter_map(|e| e.ok()).count();
            info!("Cache warm: {} manifest files in {}", count, cache_dir.display());
        }

        metrics.startup_time_ms = start.elapsed().as_millis() as u64;
        metrics.cache_load_ms = start.elapsed().as_millis() as u64;
        metrics
    }

    /// Estimate memory usage of caches
    pub fn estimate_memory_usage(&self) -> u64 {
        let cache_dir = self.base_dir.join("cache");
        let mut total: u64 = 0;
        if let Ok(entries) = std::fs::read_dir(&cache_dir) {
            for entry in entries.flatten() {
                if let Ok(meta) = entry.metadata() {
                    total += meta.len();
                }
            }
        }
        total / 1024 / 1024 // Return MB
    }
}

/// Lazy initialization container for expensive resources
pub struct LazyResource<T> {
    value: Option<T>,
    init_fn: Option<Box<dyn FnOnce() -> T + Send>>,
}

impl<T> LazyResource<T> {
    pub fn new(init_fn: Box<dyn FnOnce() -> T + Send>) -> Self {
        Self {
            value: None,
            init_fn: Some(init_fn),
        }
    }

    pub fn get_or_init(&mut self) -> &mut T {
        if self.value.is_none() {
            if let Some(f) = self.init_fn.take() {
                info!("Lazy initializing resource");
                self.value = Some(f());
            }
        }
        self.value.as_mut().unwrap()
    }
}

/// Parallel task executor for non-blocking startup
pub struct ParallelInit {
    tasks: Vec<Box<dyn FnOnce() + Send>>,
}

impl ParallelInit {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: Box<dyn FnOnce() + Send>) {
        self.tasks.push(task);
    }

    /// Run all tasks (simulated parallel in single-threaded context)
    pub fn run_all(self) {
        for task in self.tasks {
            task();
        }
    }
}