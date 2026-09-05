//! File verification utilities.
//!
//! Provides SHA-256 checksum verification and ZIP integrity checking.

use crate::{Error, Result};
use log::{debug, info, warn};
use sha2::{Digest, Sha256};
use std::path::Path;
use tokio::fs;

/// Verify a file's SHA-256 checksum against an expected hex string.
///
/// Returns `Ok(true)` if the checksums match, `Ok(false)` if they differ,
/// or an `Err` if the file could not be read.
pub async fn verify_sha256(path: &Path, expected: &str) -> Result<bool> {
    let expected = expected.trim().to_lowercase();
    let actual = sha256_file(path).await?;

    if actual == expected {
        info!("SHA-256 match for {}", path.display());
        Ok(true)
    } else {
        warn!(
            "SHA-256 mismatch for {}: expected {}, got {}",
            path.display(),
            expected,
            actual
        );
        Err(Error::ChecksumMismatch {
            expected,
            actual,
        })
    }
}

/// Compute the SHA-256 hex digest of a file.
///
/// Reads the entire file into memory to hash it.  For large files consider
/// streaming the file through a buffered reader.
pub async fn sha256_file(path: &Path) -> Result<String> {
    let contents = fs::read(path).await?;
    let mut hasher = Sha256::new();
    hasher.update(&contents);
    let hash = hasher.finalize();
    let hex = format!("{:x}", hash);
    debug!("SHA-256 for {}: {}", path.display(), hex);
    Ok(hex)
}

/// Verify that a file is a valid ZIP archive by attempting to read its
/// central directory.
///
/// Returns `Ok(true)` if the file parses as a valid ZIP, or an `Err` if the
/// file is not a valid ZIP or cannot be read.
pub async fn verify_zip(path: &Path) -> Result<bool> {
    let file = fs::File::open(path).await?;
    let mut reader = tokio::io::BufReader::new(file);

    // Use a synchronous approach: read the file into memory and parse with
    // the `zip` crate.  For large files this may be expensive, but for
    // typical download verification it is acceptable.
    let mut data = Vec::new();
    tokio::io::AsyncReadExt::read_to_end(&mut reader, &mut data).await?;

    match zip::ZipArchive::new(std::io::Cursor::new(&data)) {
        Ok(_) => {
            info!("ZIP integrity check passed for {}", path.display());
            Ok(true)
        }
        Err(e) => {
            warn!(
                "ZIP integrity check failed for {}: {}",
                path.display(),
                e
            );
            Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid ZIP archive: {}", e),
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_sha256_file() {
        let mut tmp = NamedTempFile::new().unwrap();
        write!(tmp, "hello world").unwrap();
        let path = tmp.path().to_path_buf();

        // SHA-256 of "hello world" (no newline)
        let hash = sha256_file(&path).await.unwrap();
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[tokio::test]
    async fn test_verify_sha256_match() {
        let mut tmp = NamedTempFile::new().unwrap();
        write!(tmp, "test data").unwrap();
        let path = tmp.path().to_path_buf();

        let hash = sha256_file(&path).await.unwrap();
        let result = verify_sha256(&path, &hash).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_verify_sha256_mismatch() {
        let mut tmp = NamedTempFile::new().unwrap();
        write!(tmp, "test data").unwrap();
        let path = tmp.path().to_path_buf();

        let result = verify_sha256(
            &path,
            "0000000000000000000000000000000000000000000000000000000000000000",
        )
        .await;
        assert!(result.is_err());
        match result {
            Err(Error::ChecksumMismatch { expected, actual }) => {
                assert_eq!(
                    expected,
                    "0000000000000000000000000000000000000000000000000000000000000000"
                );
                assert!(!actual.is_empty());
            }
            _ => panic!("Expected ChecksumMismatch error"),
        }
    }

    #[tokio::test]
    async fn test_verify_zip_valid() {
        // Create a minimal valid ZIP file.
        let mut tmp = NamedTempFile::new().unwrap();
        let mut zip_data = std::io::Cursor::new(Vec::new());
        {
            let mut zip = zip::ZipWriter::new(&mut zip_data);
            zip.start_file::<&str, ()>("test.txt", zip::write::FileOptions::default())
                .unwrap();
            zip.write_all(b"hello").unwrap();
            zip.finish().unwrap();
        }
        tmp.write_all(zip_data.get_ref()).unwrap();
        let path = tmp.path().to_path_buf();

        let result = verify_zip(&path).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_verify_zip_invalid() {
        let mut tmp = NamedTempFile::new().unwrap();
        write!(tmp, "not a zip file").unwrap();
        let path = tmp.path().to_path_buf();

        let result = verify_zip(&path).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_verify_zip_nonexistent() {
        let path = Path::new("/nonexistent/file.zip");
        let result = verify_zip(path).await;
        assert!(result.is_err());
    }
}