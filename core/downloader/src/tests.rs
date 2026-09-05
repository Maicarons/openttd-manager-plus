//! Integration and unit tests for the downloader crate.

// ---------------------------------------------------------------------------
// Re-imports from each module's own test module are already present via
// `#[cfg(test)] mod tests` blocks inside each source file.  This file
// contains additional cross-module and integration-style tests.
// ---------------------------------------------------------------------------

use crate::engine::*;
use crate::mirror::*;
use crate::verify::*;
use crate::{Error, Result};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

// ---------------------------------------------------------------------------
// DownloadConfig tests
// ---------------------------------------------------------------------------

#[test]
fn test_download_config_default_values() {
    let config = DownloadConfig::default();
    assert_eq!(config.max_concurrent, 4, "max_concurrent should be 4");
    assert_eq!(config.chunk_size, 8192, "chunk_size should be 8192");
    assert_eq!(config.retry_count, 3, "retry_count should be 3");
    assert_eq!(config.timeout_secs, 30, "timeout_secs should be 30");
    assert!(
        config.user_agent.contains("OpenTTD-Manager-Plus"),
        "user_agent should contain 'OpenTTD-Manager-Plus'"
    );
}

#[test]
fn test_download_config_custom() {
    let config = DownloadConfig {
        max_concurrent: 8,
        chunk_size: 16384,
        retry_count: 5,
        timeout_secs: 60,
        user_agent: "TestAgent/1.0".to_string(),
    };
    assert_eq!(config.max_concurrent, 8);
    assert_eq!(config.chunk_size, 16384);
    assert_eq!(config.retry_count, 5);
    assert_eq!(config.timeout_secs, 60);
    assert_eq!(config.user_agent, "TestAgent/1.0");
}

// ---------------------------------------------------------------------------
// DownloadProgress tests
// ---------------------------------------------------------------------------

#[test]
fn test_download_progress_default() {
    let progress = DownloadProgress {
        bytes_downloaded: 0,
        total_bytes: None,
        speed: 0.0,
        elapsed: Duration::from_secs(0),
    };
    assert_eq!(progress.bytes_downloaded, 0);
    assert!(progress.total_bytes.is_none());
    assert_eq!(progress.speed, 0.0);
    assert_eq!(progress.elapsed.as_secs(), 0);
}

#[test]
fn test_download_progress_with_values() {
    let progress = DownloadProgress {
        bytes_downloaded: 1_048_576,
        total_bytes: Some(10_485_760),
        speed: 524_288.0,
        elapsed: Duration::from_secs(2),
    };
    assert_eq!(progress.bytes_downloaded, 1_048_576);
    assert_eq!(progress.total_bytes, Some(10_485_760));
    assert!((progress.speed - 524_288.0).abs() < f64::EPSILON);
    assert_eq!(progress.elapsed.as_secs(), 2);
}

#[test]
fn test_download_progress_clone() {
    let p1 = DownloadProgress {
        bytes_downloaded: 100,
        total_bytes: Some(200),
        speed: 50.0,
        elapsed: Duration::from_secs(1),
    };
    let p2 = p1.clone();
    assert_eq!(p1.bytes_downloaded, p2.bytes_downloaded);
    assert_eq!(p1.total_bytes, p2.total_bytes);
    assert!((p1.speed - p2.speed).abs() < f64::EPSILON);
    assert_eq!(p1.elapsed, p2.elapsed);
}

// ---------------------------------------------------------------------------
// DownloadEngine unit tests (no network)
// ---------------------------------------------------------------------------

#[test]
fn test_download_engine_new() {
    let config = DownloadConfig::default();
    let engine = DownloadEngine::new(config);
    // No direct way to inspect config, but we can test shutdown/reset.
    assert!(!engine.is_shutdown());
}

#[test]
fn test_download_engine_shutdown() {
    let engine = DownloadEngine::new(DownloadConfig::default());
    assert!(!engine.is_shutdown());
    engine.shutdown();
    assert!(engine.is_shutdown());
    engine.reset_shutdown();
    assert!(!engine.is_shutdown());
}

#[tokio::test]
async fn test_download_engine_head_invalid_url() {
    let engine = DownloadEngine::new(DownloadConfig::default());
    let result = engine.head("https://invalid.example.com/nonexistent").await;
    // Should fail with some kind of error
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// MirrorSelector tests
// ---------------------------------------------------------------------------

#[test]
fn test_mirror_selector_default_has_mirrors() {
    let mut selector = MirrorSelector::new();
    for mirror in crate::mirror::default_mirrors() {
        selector.add_mirror(mirror);
    }
    assert_eq!(selector.len(), 3);
}

#[test]
fn test_mirror_selector_empty() {
    let selector = MirrorSelector::new();
    assert!(selector.is_empty());
    assert_eq!(selector.len(), 0);
}

#[test]
fn test_mirror_source_clone() {
    let m1 = MirrorSource {
        name: "test".to_string(),
        base_url: "https://example.com/".to_string(),
        weight: 1,
        latency: Some(Duration::from_millis(50)),
    };
    let m2 = m1.clone();
    assert_eq!(m1.name, m2.name);
    assert_eq!(m1.weight, m2.weight);
    assert_eq!(m1.latency, m2.latency);
}

#[test]
fn test_mirror_source_debug() {
    let m = MirrorSource {
        name: "debug-test".to_string(),
        base_url: "https://example.com/".to_string(),
        weight: 1,
        latency: None,
    };
    let debug_str = format!("{:?}", m);
    assert!(debug_str.contains("debug-test"));
    assert!(debug_str.contains("example.com"));
}

// ---------------------------------------------------------------------------
// URL translation tests
// ---------------------------------------------------------------------------

#[test]
fn test_translate_url_handles_trailing_slash() {
    let mirrors: Vec<MirrorSource> = vec![
        MirrorSource {
            name: "ghproxy".to_string(),
            base_url: "https://ghproxy.com/".to_string(),
            weight: 5,
            latency: None,
        },
    ];
    let selector = MirrorSelector::new();
    let original = "https://github.com/user/repo/releases/download/v1.0/file.zip";
    let translated = selector.translate_url(original, &mirrors[0]);
    assert_eq!(
        translated,
        "https://ghproxy.com/https://github.com/user/repo/releases/download/v1.0/file.zip"
    );
}

#[test]
fn test_translate_url_custom_mirror() {
    let mirrors: Vec<MirrorSource> = vec![
        MirrorSource {
            name: "custom".to_string(),
            base_url: "https://custom-mirror.example.com/".to_string(),
            weight: 1,
            latency: None,
        },
    ];
    let selector = MirrorSelector::new();
    let original = "https://github.com/user/repo/archive/v1.0.tar.gz";
    let translated = selector.translate_url(original, &mirrors[0]);
    assert_eq!(
        translated,
        "https://custom-mirror.example.com/https://github.com/user/repo/archive/v1.0.tar.gz"
    );
}

// ---------------------------------------------------------------------------
// SHA-256 verification tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sha256_empty_file() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    let path = tmp.path().to_path_buf();

    // SHA-256 of empty input
    let hash = sha256_file(&path).await.unwrap();
    assert_eq!(
        hash,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[tokio::test]
async fn test_sha256_large_data() {
    let mut tmp = tempfile::NamedTempFile::new().unwrap();
    let data = vec![0xABu8; 65536]; // 64 KB of repeated bytes
    use std::io::Write;
    tmp.write_all(&data).unwrap();
    let path = tmp.path().to_path_buf();

    let hash = sha256_file(&path).await.unwrap();
    // Just verify it's a 64-character hex string
    assert_eq!(hash.len(), 64);
    assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
}

#[tokio::test]
async fn test_verify_sha256_nonexistent_file() {
    let path = Path::new("/nonexistent/test_file.bin");
    let result = verify_sha256(path, "abc").await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Progress callback tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_progress_callback_invocation() {
    // We can't easily test the async callback with a real HTTP server here,
    // but we can verify the callback type works synchronously.
    let called = Arc::new(AtomicBool::new(false));
    let called_clone = called.clone();

    let cb: ProgressCallback = Arc::new(move |progress: DownloadProgress| {
        called_clone.store(true, Ordering::SeqCst);
        assert!(progress.bytes_downloaded > 0 || progress.speed == 0.0);
    });

    // Invoke the callback manually
    cb(DownloadProgress {
        bytes_downloaded: 1024,
        total_bytes: Some(2048),
        speed: 102400.0,
        elapsed: Duration::from_secs(1),
    });

    assert!(called.load(Ordering::SeqCst));
}

#[tokio::test]
async fn test_progress_callback_does_not_panic_on_drop() {
    let cb: ProgressCallback = Arc::new(move |_progress: DownloadProgress| {
        // No-op
    });

    // Drop the callback without calling it
    drop(cb);
}

// ---------------------------------------------------------------------------
// Zip verification tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_verify_zip_empty_file() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    let path = tmp.path().to_path_buf();

    let result = verify_zip(&path).await;
    assert!(result.is_err());
    // An empty file is not a valid ZIP
}

#[tokio::test]
async fn test_verify_zip_multiple_entries() {
    use std::io::Write;

    let mut tmp = tempfile::NamedTempFile::new().unwrap();
    let mut zip_data = std::io::Cursor::new(Vec::new());
    {
        let mut zip = zip::ZipWriter::new(&mut zip_data);
        zip.start_file::<&str, ()>("file1.txt", zip::write::FileOptions::default())
            .unwrap();
        zip.write_all(b"content1").unwrap();
        zip.start_file::<&str, ()>("dir/file2.txt", zip::write::FileOptions::default())
            .unwrap();
        zip.write_all(b"content2").unwrap();
        zip.finish().unwrap();
    }
    tmp.write_all(zip_data.get_ref()).unwrap();
    let path = tmp.path().to_path_buf();

    let result = verify_zip(&path).await;
    assert!(result.is_ok());
    assert!(result.unwrap());
}

// ---------------------------------------------------------------------------
// Error tests
// ---------------------------------------------------------------------------

#[test]
fn test_error_display_network() {
    // Network error from reqwest is abstract; test other variants.
    let err = Error::ChecksumMismatch {
        expected: "abc".to_string(),
        actual: "def".to_string(),
    };
    let msg = format!("{}", err);
    assert!(msg.contains("abc"));
    assert!(msg.contains("def"));
}

#[test]
fn test_error_display_invalid_url() {
    let err = Error::InvalidUrl("bad://".to_string());
    assert_eq!(format!("{}", err), "Invalid URL: bad://");
}

#[test]
fn test_error_display_mirror_unavailable() {
    let err = Error::MirrorUnavailable("test mirror".to_string());
    assert_eq!(format!("{}", err), "Mirror unavailable: test mirror");
}

#[test]
fn test_error_display_cancelled() {
    let err = Error::Cancelled;
    assert_eq!(format!("{}", err), "Download cancelled");
}

#[test]
fn test_error_display_http_error() {
    let err = Error::HttpError {
        status: reqwest::StatusCode::NOT_FOUND,
        message: "Not Found".to_string(),
    };
    let msg = format!("{}", err);
    assert!(msg.contains("404"));
    assert!(msg.contains("Not Found"));
}

// ---------------------------------------------------------------------------
// Result type tests
// ---------------------------------------------------------------------------

#[test]
fn test_result_type_alias() {
    // Verify the crate's Result type is usable.
    fn returns_result() -> Result<()> {
        Ok(())
    }
    assert!(returns_result().is_ok());
}

// ---------------------------------------------------------------------------
// Mockito-based HTTP tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod mock_http_tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_download_small_file() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test.txt")
            .with_status(200)
            .with_body("Hello, World!")
            .with_header("content-type", "text/plain")
            .create();

        let url = format!("{}/test.txt", server.url());
        let engine = DownloadEngine::new(DownloadConfig::default());
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().to_path_buf();

        let result = engine.download(&url, &path, None).await;
        assert!(result.is_ok());

        mock.assert();
    }

    #[tokio::test]
    async fn test_download_with_progress() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/progress.txt")
            .with_status(200)
            .with_body("x".repeat(1024))
            .create();

        let url = format!("{}/progress.txt", server.url());
        let engine = DownloadEngine::new(DownloadConfig::default());
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().to_path_buf();

        let progress_called = Arc::new(AtomicBool::new(false));
        let pc = progress_called.clone();
        let cb: ProgressCallback = Arc::new(move |p: DownloadProgress| {
            pc.store(true, Ordering::SeqCst);
            assert!(p.bytes_downloaded > 0);
        });

        let result = engine.download(&url, &path, Some(cb)).await;
        assert!(result.is_ok());
        assert!(progress_called.load(Ordering::SeqCst));

        mock.assert();
    }

    #[tokio::test]
    async fn test_head_request() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("HEAD", "/file.bin")
            .with_status(200)
            .with_header("content-length", "1024")
            .create();

        let url = format!("{}/file.bin", server.url());
        let engine = DownloadEngine::new(DownloadConfig::default());

        let result = engine.head(&url).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(1024));

        mock.assert();
    }

    #[tokio::test]
    async fn test_head_request_no_content_length() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("HEAD", "/no-length.bin")
            .with_status(200)
            .create();

        let url = format!("{}/no-length.bin", server.url());
        let engine = DownloadEngine::new(DownloadConfig::default());

        let result = engine.head(&url).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);

        mock.assert();
    }

    #[tokio::test]
    async fn test_download_http_error() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/notfound")
            .with_status(404)
            .create();

        let url = format!("{}/notfound", server.url());
        let engine = DownloadEngine::new(DownloadConfig {
            retry_count: 0,
            ..Default::default()
        });
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().to_path_buf();

        let result = engine.download(&url, &path, None).await;
        assert!(result.is_err());

        mock.assert();
    }

    #[tokio::test]
    async fn test_download_resumable() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/resumable.bin")
            .with_status(200)
            .with_body("HelloWorld")
            .with_header("accept-ranges", "bytes")
            .create();

        let url = format!("{}/resumable.bin", server.url());
        let engine = DownloadEngine::new(DownloadConfig::default());
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().to_path_buf();

        let result = engine.download_resumable(&url, &path, None).await;
        assert!(result.is_ok());

        mock.assert();
    }

    #[tokio::test]
    async fn test_mirror_latency_test() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("HEAD", "/")
            .with_status(200)
            .create();

        let mut selector = MirrorSelector::new();
        selector.add_mirror(MirrorSource {
            name: "test-mirror".to_string(),
            base_url: server.url(),
            weight: 1,
            latency: None,
        });

        let sorted = selector.test_latencies().await;
        assert_eq!(sorted.len(), 1);
        assert_eq!(sorted[0].name, "test-mirror");

        mock.assert();
    }

    #[tokio::test]
    async fn test_mirror_select_best() {
        let mut server = Server::new_async().await;
        let _mock = server
            .mock("HEAD", "/")
            .with_status(200)
            .create();

        let mut selector = MirrorSelector::new();
        selector.add_mirror(MirrorSource {
            name: "test-mirror".to_string(),
            base_url: server.url(),
            weight: 1,
            latency: None,
        });

        let best = selector.select_best().await;
        assert!(best.is_ok());
        assert_eq!(best.unwrap().name, "test-mirror");
    }
}

// ---------------------------------------------------------------------------
// Edge-case tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_verify_sha256_with_whitespace_in_expected() {
    let mut tmp = tempfile::NamedTempFile::new().unwrap();
    use std::io::Write;
    write!(tmp, "hello").unwrap();
    let path = tmp.path().to_path_buf();

    // Expected hash with leading/trailing whitespace
    let hash = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
    let result = verify_sha256(&path, &format!("  {}  ", hash)).await;
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_verify_sha256_uppercase_expected() {
    let mut tmp = tempfile::NamedTempFile::new().unwrap();
    use std::io::Write;
    write!(tmp, "hello").unwrap();
    let path = tmp.path().to_path_buf();

    // Uppercase hex — should be normalized to lowercase
    let hash = "2CF24DBA5FB0A30E26E83B2AC5B9E29E1B161E5C1FA7425E73043362938B9824";
    let result = verify_sha256(&path, hash).await;
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_engine_cancellation() {
    let engine = DownloadEngine::new(DownloadConfig::default());
    engine.shutdown();

    let tmp = tempfile::NamedTempFile::new().unwrap();
    let path = tmp.path().to_path_buf();

    let result = engine
        .download("https://example.com/file.bin", &path, None)
        .await;
    match result {
        Err(Error::Cancelled) => {} // expected
        _ => panic!("Expected Cancelled error, got {:?}", result),
    }
}