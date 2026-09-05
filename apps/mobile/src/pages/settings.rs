//! Settings page — mobile settings

use dioxus_native::prelude::*;

#[component]
pub fn SettingsPage() -> Element {
    rsx! {
        div { style: "padding: 8px;",
            h2 { style: "color: #2c3e50; font-size: 20px; margin: 0 0 16px 0;", "Settings" }

            div { style: "background: white; border-radius: 12px; padding: 14px; margin: 8px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "font-size: 14px; margin: 0 0 8px 0;", "General" }
                div { style: "display: flex; justify-content: space-between; padding: 8px 0; font-size: 14px; border-bottom: 1px solid #eee;",
                    span { "Language" }
                    span { style: "color: #999;", "简体中文" }
                }
                div { style: "display: flex; justify-content: space-between; padding: 8px 0; font-size: 14px;",
                    span { "Theme" }
                    span { style: "color: #999;", "Light" }
                }
            }

            div { style: "background: white; border-radius: 12px; padding: 14px; margin: 8px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "font-size: 14px; margin: 0 0 8px 0;", "Downloads" }
                div { style: "display: flex; justify-content: space-between; padding: 8px 0; font-size: 14px; border-bottom: 1px solid #eee;",
                    span { "Auto-install APK" }
                    span { style: "color: #999;", "On" }
                }
                div { style: "display: flex; justify-content: space-between; padding: 8px 0; font-size: 14px;",
                    span { "Download over WiFi only" }
                    span { style: "color: #999;", "Off" }
                }
            }

            div { style: "background: white; border-radius: 12px; padding: 14px; margin: 8px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "font-size: 14px; margin: 0 0 8px 0;", "About" }
                div { style: "font-size: 13px; padding: 4px 0;", "Version: 0.1.0" }
                div { style: "font-size: 13px; padding: 4px 0;", "License: AGPL-3.0" }
                div { style: "font-size: 13px; padding: 4px 0;", "Platform: Android / iOS" }
            }
        }
    }
}