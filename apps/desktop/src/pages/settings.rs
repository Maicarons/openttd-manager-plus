//! Settings page — app configuration

use dioxus_native::prelude::*;

#[component]
pub fn SettingsPage() -> Element {
    rsx! {
        div { style: "max-width: 800px; margin: 0 auto;",
            h2 { style: "color: #2c3e50;", "Settings" }

            // General
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "General" }
                div { style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 0;",
                    span { "Language" }
                    span { style: "color: #999;", "简体中文 (Chinese)" }
                }
                div { style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 0;",
                    span { "Theme" }
                    span { style: "color: #999;", "Light" }
                }
            }

            // Download
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Download" }
                div { style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 0;",
                    span { "Mirror Source" }
                    span { style: "color: #999;", "Auto (Fastest)" }
                }
                div { style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 0;",
                    span { "Max Concurrent Downloads" }
                    span { style: "color: #999;", "3" }
                }
            }

            // About
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "About" }
                div { style: "display: flex; justify-content: space-between; padding: 4px 0;", span { "Version" }, span { "0.1.0" } }
                div { style: "display: flex; justify-content: space-between; padding: 4px 0;", span { "License" }, span { "AGPL-3.0" } }
                div { style: "display: flex; justify-content: space-between; padding: 4px 0;", span { "Renderer" }, span { "Dioxus Native (Blitz/WGPU)" } }
            }
        }
    }
}