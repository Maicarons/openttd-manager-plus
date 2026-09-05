//! Security hardening utilities — input validation, path sanitization, audit.

use std::path::{Path, PathBuf};
use url::Url;

/// Security error types
#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Path traversal detected: {0}")]
    PathTraversal(String),
    #[error("Invalid URL scheme: {0}")]
    InvalidScheme(String),
    #[error("URL contains embedded credentials")]
    EmbeddedCredentials,
    #[error("File size exceeds maximum: {size} > {max}")]
    FileSizeExceeded { size: u64, max: u64 },
    #[error("Invalid file extension: {0}")]
    InvalidExtension(String),
    #[error("Suspicious filename: {0}")]
    SuspiciousFilename(String),
}

/// Path sanitizer to prevent directory traversal attacks
pub struct PathSanitizer {
    base_dir: PathBuf,
}

impl PathSanitizer {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    /// Sanitize a user-provided path, ensuring it stays within base_dir
    pub fn sanitize(&self, user_path: &Path) -> Result<PathBuf, SecurityError> {
        let canonical_base = self.base_dir.canonicalize()
            .map_err(|_| SecurityError::PathTraversal("Cannot resolve base directory".into()))?;

        let resolved = if user_path.is_absolute() {
            user_path.to_path_buf()
        } else {
            canonical_base.join(user_path)
        };

        let canonical = resolved.canonicalize()
            .map_err(|_| SecurityError::PathTraversal(
                format!("Cannot resolve path: {}", user_path.display())
            ))?;

        if !canonical.starts_with(&canonical_base) {
            return Err(SecurityError::PathTraversal(
                format!("Path {} escapes base directory", user_path.display())
            ));
        }

        Ok(canonical)
    }

    /// Check if a filename has a suspicious pattern
    pub fn is_suspicious_filename(name: &str) -> bool {
        let suspicious = ["..", "~", "$", "`", "|", ">", "<", "&", ";"];
        suspicious.iter().any(|&s| name.contains(s))
    }
}

/// URL validator for download sources
pub struct UrlValidator;

impl UrlValidator {
    /// Validate a URL for downloading
    pub fn validate(url_str: &str) -> Result<Url, SecurityError> {
        let url = Url::parse(url_str)
            .map_err(|_| SecurityError::InvalidScheme("Malformed URL".into()))?;

        // Only allow http/https
        match url.scheme() {
            "http" | "https" => {}
            scheme => return Err(SecurityError::InvalidScheme(scheme.into())),
        }

        // Reject embedded credentials
        if url.username() != "" || url.password().is_some() {
            return Err(SecurityError::EmbeddedCredentials);
        }

        Ok(url)
    }

    /// Validate that a URL points to a GitHub release
    pub fn is_github_release(url: &str) -> bool {
        url.contains("github.com") && url.contains("/releases/")
    }
}

/// File extension validator for OpenTTD files
pub struct FileValidator;

impl FileValidator {
    /// Allowed file extensions for OpenTTD-related files
    const ALLOWED_EXTENSIONS: &'static [&'static str] = &[
        "zip", "tar.gz", "tar.bz2", "7z", "dmg", "exe", "appimage",
        "apk", "aab", "ipa",
        "sav", "scn", "ss1", "hgt",
        "grf", "nut",
        "cfg", "txt",
    ];

    /// Validate a file extension
    pub fn validate_extension(path: &Path) -> Result<(), SecurityError> {
        let name = path.to_string_lossy();
        let has_valid = Self::ALLOWED_EXTENSIONS.iter().any(|ext| {
            name.ends_with(&format!(".{}", ext)) || name.ends_with(&format!(".{}", ext.to_uppercase()))
        });
        if has_valid {
            Ok(())
        } else {
            Err(SecurityError::InvalidExtension(
                path.extension().map(|e| e.to_string_lossy().into_owned()).unwrap_or_default()
            ))
        }
    }

    /// Maximum file size for downloads (500 MB)
    pub const MAX_FILE_SIZE: u64 = 500 * 1024 * 1024;

    /// Validate file size
    pub fn validate_size(size: u64) -> Result<(), SecurityError> {
        if size > Self::MAX_FILE_SIZE {
            Err(SecurityError::FileSizeExceeded { size, max: Self::MAX_FILE_SIZE })
        } else {
            Ok(())
        }
    }
}

/// Dependency audit tracker
pub struct DependencyAudit;

impl DependencyAudit {
    /// Check a list of dependency names for known vulnerabilities
    /// (placeholder — in production this would integrate with cargo-audit)
    pub fn check_vulnerabilities() -> Vec<String> {
        // In production, run `cargo audit` and parse output
        Vec::new()
    }

    /// Log dependency information
    pub fn log_dependencies() {
        log::info!("OpenTTD Manager Plus dependencies:");
        log::info!("  - dioxus-native {}", env!("CARGO_PKG_VERSION"));
        log::info!("  - reqwest, tokio, serde, etc.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_path_sanitizer_accepts_valid_path() {
        let base = std::env::temp_dir().join("otmp_test");
        std::fs::create_dir_all(&base).ok();
        let test_file = base.join("test.txt");
        std::fs::write(&test_file, b"test").ok();

        let sanitizer = PathSanitizer::new(base.clone());
        let result = sanitizer.sanitize(Path::new("test.txt"));
        assert!(result.is_ok());

        std::fs::remove_dir_all(base).ok();
    }

    #[test]
    fn test_url_validator_rejects_bad_scheme() {
        let result = UrlValidator::validate("ftp://example.com/file.zip");
        assert!(result.is_err());
    }

    #[test]
    fn test_url_validator_accepts_https() {
        let result = UrlValidator::validate("https://github.com/OpenTTD/OpenTTD/releases/download/14.1/openttd-14.1.zip");
        assert!(result.is_ok());
    }

    #[test]
    fn test_url_validator_rejects_credentials() {
        let result = UrlValidator::validate("https://user:pass@example.com/file.zip");
        assert!(result.is_err());
    }

    #[test]
    fn test_file_extension_validation() {
        assert!(FileValidator::validate_extension(Path::new("save.sav")).is_ok());
        assert!(FileValidator::validate_extension(Path::new("mod.grf")).is_ok());
        assert!(FileValidator::validate_extension(Path::new("file.apk")).is_ok());
        assert!(FileValidator::validate_extension(Path::new("file.exe")).is_ok());
        assert!(FileValidator::validate_extension(Path::new("file.unknown")).is_err());
    }

    #[test]
    fn test_suspicious_filename() {
        assert!(PathSanitizer::is_suspicious_filename("../../etc/passwd"));
        assert!(!PathSanitizer::is_suspicious_filename("openttd-14.1.zip"));
    }

    #[test]
    fn test_github_release_detection() {
        assert!(UrlValidator::is_github_release("https://github.com/OpenTTD/OpenTTD/releases/download/14.1/openttd-14.1.zip"));
        assert!(!UrlValidator::is_github_release("https://example.com/file.zip"));
    }

    #[test]
    fn test_file_size_validation() {
        assert!(FileValidator::validate_size(100_000_000).is_ok());
        assert!(FileValidator::validate_size(600_000_000).is_err());
    }
}