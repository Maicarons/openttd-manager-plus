//! Downloads page — mobile download queue

use dioxus_native::prelude::*;
use crate::state::MobileState;

#[component]
pub fn DownloadsPage() -> Element {
    let state = use_context::<MobileState>();
    let downloads = state.downloads.read().clone();

    rsx! {
        div { style: "padding: 8px;",
            h2 { style: "color: #2c3e50; font-size: 20px; margin: 0 0 16px 0;", "Downloads" }

            if downloads.is_empty() {
                div { style: "text-align: center; padding: 40px 0; color: #999;",
                    div { style: "font-size: 48px; margin-bottom: 12px;", "⬇️" }
                    p { "No downloads yet." }
                    p { style: "font-size: 12px;", "Browse versions and tap Download to start." }
                }
            } else {
                {downloads.iter().map(|t| {
                    let name = t.name.clone();
                    let pct = if t.total_bytes > 0 { (t.bytes_downloaded as f64 / t.total_bytes as f64 * 100.0) as u32 } else { 0 };
                    let status = match t.status {
                        otmp_core_downloader::queue::TaskStatus::Downloading => "Downloading",
                        otmp_core_downloader::queue::TaskStatus::Completed => "Completed",
                        otmp_core_downloader::queue::TaskStatus::Failed(_) => "Failed",
                        otmp_core_downloader::queue::TaskStatus::Queued => "Queued",
                        _ => "Pending",
                    };
                    rsx! {
                        div {
                            key: "{t.id}",
                            style: "background: white; border-radius: 12px; padding: 14px; margin: 8px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                            div { style: "display: flex; justify-content: space-between;",
                                span { style: "font-weight: 600;", "{name}" }
                                span { style: "font-size: 12px; color: #4a9e4a;", "{status}" }
                            }
                            div { style: "margin-top: 8px; height: 6px; background: #ecf0f1; border-radius: 3px;",
                                div { style: "height: 100%; width: {pct}%; background: #4a9e4a; border-radius: 3px; transition: width 0.3s;" }
                            }
                            div { style: "margin-top: 4px; font-size: 11px; color: #999; text-align: right;", "{pct}%" }
                        }
                    }
                })}
            }
        }
    }
}