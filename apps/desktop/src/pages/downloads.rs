//! Download management page — active downloads, queue, completed.

use dioxus_native::prelude::*;
use crate::state::{AppState, TaskStatus};

/// Human-readable label for a task status.
fn status_label(status: &TaskStatus) -> &'static str {
    match status {
        TaskStatus::Queued => "Queued",
        TaskStatus::Downloading => "Downloading",
        TaskStatus::Paused => "Paused",
        TaskStatus::Completed => "Completed",
        TaskStatus::Failed => "Failed",
        TaskStatus::Cancelled => "Cancelled",
    }
}

/// Colour for a status badge.
fn status_color(status: &TaskStatus) -> &'static str {
    match status {
        TaskStatus::Queued => "#95a5a6",
        TaskStatus::Downloading => "#3498db",
        TaskStatus::Paused => "#f39c12",
        TaskStatus::Completed => "#4a9e4a",
        TaskStatus::Failed => "#e74c3c",
        TaskStatus::Cancelled => "#999",
    }
}

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
pub fn DownloadsPage() -> Element {
    let mut state = use_context::<AppState>();

    let queue = state.download_queue.read().clone();
    let _stats = *state.download_stats.read();

    // Partition by status
    let active: Vec<_> = queue.iter().filter(|t| t.status == TaskStatus::Downloading).collect();
    let queued: Vec<_> = queue.iter().filter(|t| t.status == TaskStatus::Queued).collect();
    let completed: Vec<_> = queue.iter().filter(|t| t.status == TaskStatus::Completed).collect();
    let failed: Vec<_> = queue.iter().filter(|t| t.status == TaskStatus::Failed).collect();

    rsx! {
        div { style: "max-width: 800px; margin: 0 auto;",
            h2 { style: "color: #2c3e50;", "Download Manager" }

            // ── Stats summary ───────────────────────────────────────
            div { style: "display: flex; gap: 12px; margin: 16px 0; flex-wrap: wrap;",
                div { style: "background: white; border-radius: 8px; padding: 12px 16px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    span { style: "font-size: 13px; color: #666;", "Active: " }
                    strong { "{active.len()}" }
                }
                div { style: "background: white; border-radius: 8px; padding: 12px 16px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    span { style: "font-size: 13px; color: #666;", "Queued: " }
                    strong { "{queued.len()}" }
                }
                div { style: "background: white; border-radius: 8px; padding: 12px 16px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    span { style: "font-size: 13px; color: #666;", "Completed: " }
                    strong { "{completed.len()}" }
                }
                div { style: "background: white; border-radius: 8px; padding: 12px 16px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    span { style: "font-size: 13px; color: #666;", "Failed: " }
                    strong { "{failed.len()}" }
                }
            }

            // ── Active downloads ────────────────────────────────────
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Active Downloads" }
                if active.is_empty() {
                    p { style: "color: #999;", "No active downloads." }
                } else {
                    div { style: "display: flex; flex-direction: column; gap: 12px;",
                        {active.iter().map(|task| {
                            let pct = if task.total_bytes > 0 {
                                (task.downloaded_bytes as f64 / task.total_bytes as f64 * 100.0) as u32
                            } else {
                                0
                            };
                            rsx! {
                                div {
                                    key: "{task.id}",
                                    div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 4px;",
                                        span { style: "font-size: 14px; font-weight: 500;", "{task.name}" }
                                        span { style: "font-size: 12px; color: #666;", "{pct}%" }
                                    }
                                    div { style: "background: #e0e0e0; border-radius: 4px; height: 8px; overflow: hidden;",
                                        div { style: "background: #3498db; width: {pct}%; height: 100%; border-radius: 4px; transition: width 0.3s;", }
                                    }
                                    div { style: "display: flex; justify-content: space-between; margin-top: 4px;",
                                        span { style: "font-size: 11px; color: #999;",
                                            "{format_bytes(task.downloaded_bytes)} / {format_bytes(task.total_bytes)}"
                                        }
                                        span { style: "font-size: 11px; color: #999;", "Downloading..." }
                                    }
                                }
                            }
                        })}
                    }
                }
            }

            // ── Queue ───────────────────────────────────────────────
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Queue" }
                if queued.is_empty() {
                    p { style: "color: #999;", "Queue is empty." }
                } else {
                    div { style: "display: flex; flex-direction: column; gap: 8px;",
                        {queued.iter().map(|task| {
                            rsx! {
                                div {
                                    key: "{task.id}",
                                    style: "display: flex; justify-content: space-between; align-items: center;
                                            padding: 8px 0; border-bottom: 1px solid #f0f0f0;",
                                    div {
                                        div { style: "font-size: 14px;", "{task.name}" }
                                        div { style: "font-size: 11px; color: #999;", "{format_bytes(task.total_bytes)}" }
                                    }
                                    span { style: "background: {status_color(&task.status)}; color: white;
                                            font-size: 11px; padding: 2px 8px; border-radius: 3px;",
                                        "{status_label(&task.status)}"
                                    }
                                }
                            }
                        })}
                    }
                }
            }

            // ── Completed ───────────────────────────────────────────
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0; display: flex; justify-content: space-between; align-items: center;",
                    span { "Completed" }
                    if !completed.is_empty() {
                        button {
                            style: "background: #e74c3c; color: white; border: none; padding: 4px 12px;
                                    border-radius: 4px; cursor: pointer; font-size: 12px;",
                            onclick: move |_| {
                                // Clear completed items in a scope so the write lock is dropped
                                // before calling refresh_downloads.
                                {
                                    let mut queue = state.download_queue.write();
                                    queue.retain(|t| t.status != TaskStatus::Completed);
                                }
                                state.refresh_downloads();
                            },
                            "Clear Completed"
                        }
                    }
                }
                if completed.is_empty() {
                    p { style: "color: #999;", "No completed downloads." }
                } else {
                    div { style: "display: flex; flex-direction: column; gap: 8px;",
                        {completed.iter().map(|task| {
                            rsx! {
                                div {
                                    key: "{task.id}",
                                    style: "display: flex; justify-content: space-between; align-items: center;
                                            padding: 8px 0; border-bottom: 1px solid #f0f0f0;",
                                    div {
                                        div { style: "font-size: 14px;", "{task.name}" }
                                        div { style: "font-size: 11px; color: #999;", "{format_bytes(task.total_bytes)}" }
                                    }
                                    span { style: "background: {status_color(&task.status)}; color: white;
                                            font-size: 11px; padding: 2px 8px; border-radius: 3px;",
                                        "{status_label(&task.status)}"
                                    }
                                }
                            }
                        })}
                    }
                }
            }

            // ── Failed ──────────────────────────────────────────────
            if !failed.is_empty() {
                div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    h3 { style: "margin-top: 0; color: #e74c3c;", "Failed" }
                    div { style: "display: flex; flex-direction: column; gap: 8px;",
                        {failed.iter().map(|task| {
                            rsx! {
                                div {
                                    key: "{task.id}",
                                    style: "display: flex; justify-content: space-between; align-items: center;
                                            padding: 8px 0; border-bottom: 1px solid #f0f0f0;",
                                    div {
                                        div { style: "font-size: 14px;", "{task.name}" }
                                        div { style: "font-size: 11px; color: #e74c3c;",
                                            {task.error.as_deref().unwrap_or("Unknown error")}
                                        }
                                    }
                                    span { style: "background: {status_color(&task.status)}; color: white;
                                            font-size: 11px; padding: 2px 8px; border-radius: 3px;",
                                        "{status_label(&task.status)}"
                                    }
                                }
                            }
                        })}
                    }
                }
            }
        }
    }
}