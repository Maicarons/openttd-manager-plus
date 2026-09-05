//! Core download engine with streaming download, resume support,
//! progress tracking, and retry logic.

use crate::{Error, Result};
use log::{debug, info, warn};
use reqwest::Client;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;

/// Download progress callback
pub type ProgressCallback = Box<dyn Fn(DownloadProgress) + Send + 'static>;

/// Download progress information reported to the callback.
#[derive(Debug, Clone)]
pub struct DownloadProgress {
    /// Number of bytes downloaded so far
    pub bytes_downloaded: u64,
    /// Total bytes to download, if known
    pub total_bytes: Option<u64>,
    /// Current download speed in bytes per second
    pub speed: f64,
    /// Time elapsed since download started
    pub elapsed: Duration,
}

/// Download engine configuration.
#[derive(Debug, Clone)]
pub struct DownloadConfig {
    /// Maximum number of concurrent downloads
    pub max_concurrent: usize,
    /// Chunk size in bytes for streaming reads
    pub chunk_size: usize,
    /// Number of retry attempts on failure
    pub retry_count: u32,
    /// Timeout in seconds for the HTTP request
    pub timeout_secs: u64,
    /// User-Agent header value for HTTP requests
    pub user_agent: String,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 4,
            chunk_size: 8192,
            retry_count: 3,
            timeout_secs: 30,
            user_agent: format!(
                "OpenTTD-Manager-Plus/{}",
                env!("CARGO_PKG_VERSION", "CARGO_PKG_VERSION not set")
            ),
        }
    }
}

/// Core download engine providing streaming downloads with progress
/// tracking, resume support, and retry logic.
pub struct DownloadEngine {
    config: DownloadConfig,
    client: Client,
    shutdown: Arc<AtomicBool>,
}

impl DownloadEngine {
    /// Create a new `DownloadEngine` with the given configuration.
    pub fn new(config: DownloadConfig) -> Self {
        let timeout = Duration::from_secs(config.timeout_secs);
        let client = Client::builder()
            .user_agent(&config.user_agent)
            .timeout(timeout)
            .build()
            .expect("Failed to build reqwest Client");

        Self {
            config,
            client,
            shutdown: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Signal the engine to cancel any in-progress downloads.
    pub fn shutdown(&self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }

    /// Check if the engine has been shutdown.
    pub fn is_shutdown(&self) -> bool {
        self.shutdown.load(Ordering::SeqCst)
    }

    /// Reset the shutdown flag so new downloads can proceed.
    pub fn reset_shutdown(&self) {
        self.shutdown.store(false, Ordering::SeqCst);
    }

    /// Download a file from `url` to `destination` with optional progress
    /// reporting.  Does not attempt resume — overwrites the destination if
    /// it exists.
    pub async fn download(
        &self,
        url: &str,
        destination: &Path,
        progress: Option<ProgressCallback>,
    ) -> Result<()> {
        self.download_with_retry(url, destination, progress, false).await
    }

    /// Download a file from `url` to `destination` with resume support.
    ///
    /// If `destination` already exists and the server supports `Range`
    /// requests, the download resumes from where it left off.
    pub async fn download_resumable(
        &self,
        url: &str,
        destination: &Path,
        progress: Option<ProgressCallback>,
    ) -> Result<()> {
        self.download_with_retry(url, destination, progress, true).await
    }

    /// Perform a `HEAD` request to check if a remote file exists and return
    /// its content-length, if the server provides it.
    pub async fn head(&self, url: &str) -> Result<Option<u64>> {
        let response = self.client.head(url).send().await?;
        if !response.status().is_success() {
            return Err(Error::HttpError {
                status: response.status(),
                message: format!("HEAD request failed for {}", url),
            });
        }
        let size = response
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());
        Ok(size)
    }

    // ------------------------------------------------------------------
    // Internal helpers
    // ------------------------------------------------------------------

    /// Core download loop with optional retry and resume.
    async fn download_with_retry(
        &self,
        url: &str,
        destination: &Path,
        progress: Option<ProgressCallback>,
        resumable: bool,
    ) -> Result<()> {
        let mut last_error = None;

        for attempt in 0..=self.config.retry_count {
            if self.shutdown.load(Ordering::SeqCst) {
                return Err(Error::Cancelled);
            }

            match self
                .download_inner(url, destination, &progress, resumable, attempt)
                .await
            {
                Ok(()) => return Ok(()),
                Err(e) => {
                    warn!("Download attempt {} failed: {}", attempt + 1, e);
                    last_error = Some(e);
                    if attempt < self.config.retry_count {
                        let delay = Duration::from_secs(1u64 << attempt); // exponential backoff
                        sleep(delay).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or(Error::Cancelled))
    }

    /// Single-shot download attempt (may resume if `resumable` is true and
    /// a partial file exists).
    async fn download_inner(
        &self,
        url: &str,
        destination: &Path,
        progress: &Option<ProgressCallback>,
        resumable: bool,
        _attempt: u32,
    ) -> Result<()> {
        let start = Instant::now();
        let mut bytes_downloaded: u64 = 0;
        let mut existing_len: u64 = 0;

        // If resumable, check for an existing partial file.
        let (range_header, file_mode) = if resumable && destination.exists() {
            let meta = fs::metadata(destination).await?;
            existing_len = meta.len();
            if existing_len > 0 {
                debug!(
                    "Resuming download of {} from byte {}",
                    url, existing_len
                );
                bytes_downloaded = existing_len;
                (
                    Some(format!("bytes={}-", existing_len)),
                    OpenOptions::new().append(true).open(destination).await?,
                )
            } else {
                (None, fs::File::create(destination).await?)
            }
        } else {
            (None, fs::File::create(destination).await?)
        };

        // Build the request.
        let mut req = self.client.get(url);
        if let Some(range) = &range_header {
            req = req.header(reqwest::header::RANGE, range);
        }
        let response = req.send().await?;

        // For resumable downloads, a 206 means the server honoured the range.
        // If we expected a range but got 200, fall back to full download.
        let total_bytes = if range_header.is_some() && response.status() == reqwest::StatusCode::PARTIAL_CONTENT
        {
            // Content-Length in a 206 response is the remaining bytes.
            let remaining = response
                .headers()
                .get(reqwest::header::CONTENT_LENGTH)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(0);
            Some(existing_len + remaining)
        } else {
            // Normal download (full content).
            response
                .headers()
                .get(reqwest::header::CONTENT_LENGTH)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
        };

        let status = response.status();
        if !status.is_success() {
            return Err(Error::HttpError {
                status,
                message: format!("HTTP {} for {}", status, url),
            });
        }

        // Stream the response body to file.
        let mut file = file_mode;
        let mut stream = response.bytes_stream();
        let mut last_report = Instant::now();
        let mut last_bytes = bytes_downloaded;

        use futures_util::StreamExt;
        while let Some(chunk_result) = stream.next().await {
            if self.shutdown.load(Ordering::SeqCst) {
                return Err(Error::Cancelled);
            }

            let chunk = chunk_result?;
            file.write_all(&chunk).await?;
            bytes_downloaded += chunk.len() as u64;

            // Report progress (throttled to avoid excessive CPU).
            let now = Instant::now();
            if now.duration_since(last_report) >= Duration::from_millis(100) {
                let elapsed = now.duration_since(last_report);
                let delta = bytes_downloaded - last_bytes;
                let speed = if elapsed.as_secs_f64() > 0.0 {
                    delta as f64 / elapsed.as_secs_f64()
                } else {
                    0.0
                };

                if let Some(ref cb) = progress {
                    cb(DownloadProgress {
                        bytes_downloaded,
                        total_bytes,
                        speed,
                        elapsed: now.duration_since(start),
                    });
                }

                last_report = now;
                last_bytes = bytes_downloaded;
            }
        }

        // Final progress report.
        let total_elapsed = start.elapsed();
        let avg_speed = if total_elapsed.as_secs_f64() > 0.0 {
            bytes_downloaded as f64 / total_elapsed.as_secs_f64()
        } else {
            0.0
        };
        if let Some(ref cb) = progress {
            cb(DownloadProgress {
                bytes_downloaded,
                total_bytes,
                speed: avg_speed,
                elapsed: total_elapsed,
            });
        }

        info!(
            "Downloaded {} -> {} ({:.2} MB in {:.2}s)",
            url,
            destination.display(),
            bytes_downloaded as f64 / 1_048_576.0,
            total_elapsed.as_secs_f64(),
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = DownloadConfig::default();
        assert_eq!(config.max_concurrent, 4);
        assert_eq!(config.chunk_size, 8192);
        assert_eq!(config.retry_count, 3);
        assert_eq!(config.timeout_secs, 30);
        assert!(config.user_agent.contains("OpenTTD-Manager-Plus"));
    }

    #[test]
    fn test_progress_struct() {
        let p = DownloadProgress {
            bytes_downloaded: 1024,
            total_bytes: Some(2048),
            speed: 51200.0,
            elapsed: Duration::from_secs(2),
        };
        assert_eq!(p.bytes_downloaded, 1024);
        assert_eq!(p.total_bytes, Some(2048));
        assert!(p.speed > 0.0);
    }
}