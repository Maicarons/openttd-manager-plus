//! Mirror selection and URL translation for download acceleration.
//!
//! Provides a [`MirrorSelector`] that manages a list of mirror sources and
//! selects the best one based on latency testing.

use crate::{Error, Result};
use log::{debug, warn};
use reqwest::Client;
use std::cmp::Ordering;
use std::time::{Duration, Instant};

/// A mirror source used to download files from an alternative location.
#[derive(Debug, Clone)]
pub struct MirrorSource {
    /// Human-readable name for this mirror
    pub name: String,
    /// Base URL of the mirror (e.g. `"https://ghproxy.com/"`)
    pub base_url: String,
    /// Preference weight; lower values are preferred
    pub weight: u32,
    /// Measured or configured latency, if known
    pub latency: Option<Duration>,
}

/// Manages a set of mirror sources and provides selection logic.
pub struct MirrorSelector {
    mirrors: Vec<MirrorSource>,
    client: Client,
}

impl Default for MirrorSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl MirrorSelector {
    /// Create a new `MirrorSelector` with no mirrors.
    ///
    /// Use [`add_mirror`](Self::add_mirror) to populate the list, or rely on
    /// the [`Default`] implementation which adds the standard mirrors.
    pub fn new() -> Self {
        Self {
            mirrors: Vec::new(),
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to build reqwest Client"),
        }
    }

    /// Add a mirror source to the selector.
    pub fn add_mirror(&mut self, mirror: MirrorSource) {
        self.mirrors.push(mirror);
    }

    /// Return the number of registered mirrors.
    pub fn len(&self) -> usize {
        self.mirrors.len()
    }

    /// Returns `true` if no mirrors are registered.
    pub fn is_empty(&self) -> bool {
        self.mirrors.is_empty()
    }

    /// Select the best mirror based on latency testing.
    ///
    /// Latency is tested for all registered mirrors, and the one with the
    /// lowest (weighted) score is returned.  Returns an error if no mirrors
    /// are registered or all are unreachable.
    pub async fn select_best(&self) -> Result<&MirrorSource> {
        if self.mirrors.is_empty() {
            return Err(Error::MirrorUnavailable(
                "No mirrors configured".to_string(),
            ));
        }

        let latencies = self.test_latencies().await;
        if latencies.is_empty() {
            return Err(Error::MirrorUnavailable(
                "All mirrors are unreachable".to_string(),
            ));
        }

        // Return the first mirror from the sorted list (lowest score).
        Ok(latencies[0])
    }

    /// Test latency of all registered mirrors and return them sorted by
    /// (weight, latency).  Unreachable mirrors are excluded from the result.
    ///
    /// Each mirror is probed with a HEAD request to its base URL (or a
    /// well-known endpoint).  Timeouts are treated as unreachable.
    pub async fn test_latencies(&self) -> Vec<&MirrorSource> {
        if self.mirrors.is_empty() {
            return Vec::new();
        }

        let mut results: Vec<(&MirrorSource, Option<Duration>)> = Vec::new();

        for mirror in &self.mirrors {
            let probe_url = self.probe_url(&mirror.base_url);
            let start = Instant::now();

            match self.client.head(&probe_url).send().await {
                Ok(resp) => {
                    let latency = start.elapsed();
                    debug!(
                        "Mirror '{}' latency: {:?} (HTTP {})",
                        mirror.name,
                        latency,
                        resp.status()
                    );
                    results.push((mirror, Some(latency)));
                }
                Err(e) => {
                    warn!("Mirror '{}' unreachable: {}", mirror.name, e);
                    // Do not add to results — unreachable mirrors are excluded.
                }
            }
        }

        // Sort by (weight, latency).
        results.sort_by(|a, b| {
            let weight_cmp = a.0.weight.cmp(&b.0.weight);
            if weight_cmp != Ordering::Equal {
                return weight_cmp;
            }
            match (a.1, b.1) {
                (Some(la), Some(lb)) => la.cmp(&lb),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            }
        });

        results.into_iter().map(|(m, _)| m).collect()
    }

    /// Translate a GitHub URL to a mirror URL.
    ///
    /// For the `direct` mirror the original URL is returned unchanged.  For
    /// prefix-based mirrors (e.g. `ghproxy`) the base URL is prepended.
    ///
    /// # URL translation rules
    ///
    /// * `direct` — returns `original_url` as-is.
    /// * `ghproxy` — returns `https://ghproxy.com/{original_url}`.
    /// * `ghfast` — returns `https://ghfast.net/{original_url}`.
    /// * Any other mirror — returns `{base_url}{original_url}`.
    pub fn translate_url(&self, original_url: &str, mirror: &MirrorSource) -> String {
        match mirror.name.as_str() {
            "direct" => original_url.to_string(),
            "ghproxy" => {
                let base = mirror.base_url.trim_end_matches('/');
                format!("{}/{}", base, original_url)
            }
            "ghfast" => {
                let base = mirror.base_url.trim_end_matches('/');
                format!("{}/{}", base, original_url)
            }
            _other => {
                // Generic prefix-based translation.
                let base = mirror.base_url.trim_end_matches('/');
                format!("{}/{}", base, original_url)
            }
        }
    }

    /// Build a probe URL from the mirror base.
    ///
    /// For well-known mirrors we use a fast health endpoint; otherwise we
    /// probe the base URL itself.
    fn probe_url(&self, base_url: &str) -> String {
        let base = base_url.trim_end_matches('/');
        // Use a lightweight endpoint for well-known GitHub proxy mirrors.
        if base.contains("ghproxy.com") || base.contains("ghfast.net") {
            format!("{}/https://github.com", base)
        } else {
            base.to_string()
        }
    }
}

/// Set up the default mirror sources.
///
/// This is a convenience function that returns the standard mirrors:
///
/// | Name | Base URL | Weight |
/// |------|----------|--------|
/// | `direct` | *(none)* | 10 |
/// | `ghproxy` | `https://ghproxy.com/` | 5 |
/// | `ghfast` | `https://ghfast.net/` | 5 |
pub fn default_mirrors() -> Vec<MirrorSource> {
    vec![
        MirrorSource {
            name: "direct".to_string(),
            base_url: String::new(),
            weight: 10,
            latency: None,
        },
        MirrorSource {
            name: "ghproxy".to_string(),
            base_url: "https://ghproxy.com/".to_string(),
            weight: 5,
            latency: None,
        },
        MirrorSource {
            name: "ghfast".to_string(),
            base_url: "https://ghfast.net/".to_string(),
            weight: 5,
            latency: None,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_mirrors() {
        let mirrors = default_mirrors();
        assert_eq!(mirrors.len(), 3);
        assert_eq!(mirrors[0].name, "direct");
        assert_eq!(mirrors[1].name, "ghproxy");
        assert_eq!(mirrors[2].name, "ghfast");
    }

    #[test]
    fn test_translate_url_direct() {
        let mirrors = default_mirrors();
        let selector = MirrorSelector::new();
        let original = "https://github.com/user/repo/releases/download/v1.0/file.zip";

        let translated = selector.translate_url(original, &mirrors[0]);
        assert_eq!(translated, original);
    }

    #[test]
    fn test_translate_url_ghproxy() {
        let mirrors = default_mirrors();
        let selector = MirrorSelector::new();
        let original = "https://github.com/user/repo/releases/download/v1.0/file.zip";

        let translated = selector.translate_url(original, &mirrors[1]);
        assert_eq!(translated, "https://ghproxy.com/https://github.com/user/repo/releases/download/v1.0/file.zip");
    }

    #[test]
    fn test_translate_url_ghfast() {
        let mirrors = default_mirrors();
        let selector = MirrorSelector::new();
        let original = "https://github.com/user/repo/releases/download/v1.0/file.zip";

        let translated = selector.translate_url(original, &mirrors[2]);
        assert_eq!(translated, "https://ghfast.net/https://github.com/user/repo/releases/download/v1.0/file.zip");
    }

    #[test]
    fn test_empty_selector() {
        let selector = MirrorSelector::new();
        assert!(selector.is_empty());
        assert_eq!(selector.len(), 0);
    }

    #[test]
    fn test_add_mirror() {
        let mut selector = MirrorSelector::new();
        let mirror = MirrorSource {
            name: "test".to_string(),
            base_url: "https://example.com/".to_string(),
            weight: 1,
            latency: None,
        };
        selector.add_mirror(mirror);
        assert_eq!(selector.len(), 1);
        assert!(!selector.is_empty());
    }

    #[test]
    fn test_select_best_no_mirrors() {
        let selector = MirrorSelector::new();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(selector.select_best());
        assert!(result.is_err());
    }
}