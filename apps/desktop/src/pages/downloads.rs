//! Downloads page — download manager with progress bars.

use dioxus_native::prelude::*;
use crate::state::AppState;

#[component]
pub fn DownloadsPage() -> Element {
    let state = use_context::<AppState>();
    let queue = state.download_queue.read().clone();
    let stats = *state.download_stats.read();

    rsx! {
        div { style: "max-width: 900px; margin: 0 auto;",
            h1 { style: "font-size: 24px; font-weight: 700; color: #0f172a; margin: 0 0 24px 0;", "Downloads" }

            div { style: "display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px; margin-bottom: 24px;",
                MiniStat { value: "{stats.active}", label: "Active", color: "#3b82f6" }
                MiniStat { value: "{stats.queued}", label: "Queued", color: "#f59e0b" }
                MiniStat { value: "{stats.completed}", label: "Completed", color: "#4ade80" }
                MiniStat { value: "{stats.failed}", label: "Failed", color: "#ef4444" }
            }

            if queue.is_empty() {
                div { style: "text-align: center; padding: 60px 0; color: #64748b;",
                    svg { width: "48", height: "48", view_box: "0 0 24 24", fill: "none", stroke: "#cbd5e1", stroke_width: "1.5",
                        path { d: "M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" }
                    }
                    div { style: "font-size: 14px;", "No downloads yet" }
                }
            } else {
                div { style: "display: flex; flex-direction: column; gap: 8px;",
                    {queue.iter().map(|t| {
                        let name = t.name.clone();
                        let pct = if t.total_bytes > 0 { (t.downloaded_bytes as f64 / t.total_bytes as f64 * 100.0) as u32 } else { 0 };
                        let status_color = match t.status {
                            crate::state::TaskStatus::Downloading => "#3b82f6",
                            crate::state::TaskStatus::Completed => "#4ade80",
                            crate::state::TaskStatus::Failed => "#ef4444",
                            _ => "#94a3b8",
                        };
                        rsx! {
                            div {
                                key: "{t.id}", style: "background: white; border-radius: 10px; padding: 16px 20px; box-shadow: 0 1px 3px rgba(0,0,0,0.06);",
                                div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;",
                                    span { style: "font-weight: 500; font-size: 14px; color: #0f172a;", "{name}" }
                                    span { style: "font-size: 12px; color: {status_color}; font-weight: 500;", "{pct}%" }
                                }
                                div { style: "height: 4px; background: #f1f5f9; border-radius: 2px; overflow: hidden;",
                                    div { style: "height: 100%; width: {pct}%; background: {status_color}; border-radius: 2px; transition: width 0.3s;" }
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}

#[component]
fn MiniStat(value: String, label: String, color: String) -> Element {
    rsx! {
        div { style: "background: white; border-radius: 10px; padding: 14px; box-shadow: 0 1px 3px rgba(0,0,0,0.06);",
            div { style: "font-size: 24px; font-weight: 700; color: {color};", "{value}" }
            div { style: "font-size: 12px; color: #64748b; margin-top: 2px;", "{label}" }
        }
    }
}

