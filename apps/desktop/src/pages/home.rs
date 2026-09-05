//! Home page — overview and quick start.
//!
//! Shows installed versions, mod counts, and download queue status.

use dioxus_native::prelude::*;
use crate::state::AppState;

/// Format bytes to a human-readable string.
fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

#[component]
pub fn HomePage() -> Element {
    let state = use_context::<AppState>();

    // Read all values before the template (no `let` inside rsx!)
    let versions_len = state.versions.read().len();
    let filtered_len = state.filtered_versions.read().len();
    let instances_len = state.instances.read().len();
    let has_instances = instances_len > 0;
    let stats = *state.download_stats.read();
    let error = state.error.read().clone();
    let downloads_active = stats.active;
    let downloads_queued = stats.queued;
    let mods_available = 0usize;
    let overall_pct = if stats.total_bytes > 0 {
        ((stats.downloaded_bytes as f64) / (stats.total_bytes as f64) * 100.0) as u32
    } else {
        0
    };

    rsx! {
        div { style: "max-width: 900px; margin: 0 auto;",
            h2 { style: "color: #2c3e50;", "Welcome to OpenTTD Manager Plus" }
            p { style: "color: #666; line-height: 1.6;",
                "Manage OpenTTD versions, configurations, mods, and more — all in one place."
            }

            // Error banner
            {error.as_ref().map(|msg| rsx! {
                div { style: "background: #f8d7da; color: #721c24; border: 1px solid #f5c6cb;
                              border-radius: 8px; padding: 12px 16px; margin: 12px 0; font-size: 13px;",
                    "⚠ {msg}"
                }
            })}

            // Stats cards
            div { style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(180px, 1fr)); gap: 12px; margin: 16px 0;",
                div { style: "background: white; border-radius: 8px; padding: 16px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    div { style: "font-size: 28px; font-weight: 700; color: #2c3e50;", "{versions_len}" }
                    div { style: "font-size: 13px; color: #666;", "Available Versions" }
                }
                div { style: "background: white; border-radius: 8px; padding: 16px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    div { style: "font-size: 28px; font-weight: 700; color: #2c3e50;", "{instances_len}" }
                    div { style: "font-size: 13px; color: #666;", "Installed Instances" }
                }
                div { style: "background: white; border-radius: 8px; padding: 16px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    div { style: "font-size: 28px; font-weight: 700; color: #2c3e50;", "{mods_available}" }
                    div { style: "font-size: 13px; color: #666;", "Available Mods" }
                }
                div { style: "background: white; border-radius: 8px; padding: 16px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    div { style: "font-size: 28px; font-weight: 700; color: #2c3e50;", "{downloads_active}" }
                    div { style: "font-size: 13px; color: #666;", "Active Downloads" }
                }
            }

            // Quick start card
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Quick Start" }
                if has_instances {
                    p { "Your game is ready to launch. Select an instance from the Versions page to get started." }
                    button { style: "background: #4a9e4a; color: white; border: none; padding: 10px 20px; border-radius: 4px; cursor: pointer; font-size: 14px;",
                        "Launch Game"
                    }
                } else {
                    p { "You haven't installed any versions yet. Browse the available versions to get started." }
                    button { style: "background: #4a9e4a; color: white; border: none; padding: 10px 20px; border-radius: 4px; cursor: pointer; font-size: 14px;",
                        "Browse Versions"
                    }
                }
            }

            // Recent versions
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Recent Versions" }
                if versions_len == 0 {
                    p { style: "color: #999;", "No versions fetched yet. Go to the Versions page to fetch the list." }
                } else {
                    p { style: "color: #666; font-size: 13px;",
                        "Showing {filtered_len} of {versions_len} versions (use the filter tabs on the Versions page)."
                    }
                }
            }

            // Download queue status
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Download Queue" }
                if downloads_active == 0 && downloads_queued == 0 {
                    p { style: "color: #999;", "No active downloads." }
                } else {
                    p { style: "color: #666; font-size: 13px;",
                        "{downloads_active} active, {downloads_queued} queued"
                    }
                    if stats.total_bytes > 0 {
                        div { style: "margin-top: 8px;",
                            div { style: "font-size: 12px; color: #666; margin-bottom: 4px;",
                                "Overall: {format_bytes(stats.downloaded_bytes)} / {format_bytes(stats.total_bytes)} ({overall_pct}%)"
                            }
                            div { style: "background: #e0e0e0; border-radius: 4px; height: 8px; overflow: hidden;",
                                div { style: "background: #4a9e4a; width: {overall_pct}%; height: 100%; border-radius: 4px; transition: width 0.3s;", }
                            }
                        }
                    }
                }
            }
        }
    }
}