//! Version management page — list, install, and manage OpenTTD versions

use dioxus_native::prelude::*;

#[component]
pub fn VersionsPage() -> Element {
    let mut active_filter = use_signal(|| "All".to_string());

    let filters = vec!["All", "Official", "JGRPP", "CMClient"];

    rsx! {
        div { style: "max-width: 800px; margin: 0 auto;",
            h2 { style: "color: #2c3e50;", "Version Management" }

            // Filter tabs
            div { style: "display: flex; gap: 8px; margin: 16px 0;",
                {filters.into_iter().map(|f| {
                    let is_active = f == active_filter.read().as_str();
                    let bg = if is_active { "#4a9e4a" } else { "#e0e0e0" };
                    let color = if is_active { "white" } else { "#333" };
                    let f_str = f.to_string();
                    let f_clone = f_str.clone();
                    rsx! {
                        button {
                            key: "{f}",
                            style: "background: {bg}; color: {color}; border: none; padding: 8px 16px;
                                    border-radius: 4px; cursor: pointer; font-size: 13px;",
                            onclick: move |_| active_filter.set(f_clone.clone()),
                            "{f}"
                        }
                    }
                })}
            }

            // Version list
            div { style: "background: white; border-radius: 8px; padding: 20px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Official OpenTTD" }
                p { style: "color: #999;", "Fetching version list... (Backend integration pending)" }
            }
        }
    }
}