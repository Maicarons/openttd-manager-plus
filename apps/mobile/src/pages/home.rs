//! Home page — mobile dashboard

use dioxus_native::prelude::*;
use crate::state::MobileState;

#[component]
pub fn HomePage() -> Element {
    let state = use_context::<MobileState>();
    let versions_len = state.versions.read().len();
    let downloads_len = state.downloads.read().len();
    let active = state.downloads.read().iter().filter(|t| t.status == otmp_core_downloader::queue::TaskStatus::Downloading).count();

    rsx! {
        div { style: "padding: 8px;",
            h2 { style: "color: #2c3e50; font-size: 20px; margin: 0 0 16px 0;", "OpenTTD Manager Plus" }
            p { style: "color: #666; margin: 0 0 20px 0;", "Manage OpenTTD on your mobile device." }

            // Stats cards
            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div { style: "background: white; border-radius: 12px; padding: 16px; text-align: center; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    div { style: "font-size: 28px; font-weight: bold; color: #4a9e4a;", "{versions_len}" }
                    div { style: "font-size: 12px; color: #999; margin-top: 4px;", "Available Versions" }
                }
                div { style: "background: white; border-radius: 12px; padding: 16px; text-align: center; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    div { style: "font-size: 28px; font-weight: bold;", "{active}" }
                    div { style: "font-size: 12px; color: #999; margin-top: 4px;", "Active Downloads" }
                }
                div { style: "background: white; border-radius: 12px; padding: 16px; text-align: center; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    div { style: "font-size: 28px; font-weight: bold;", "{downloads_len}" }
                    div { style: "font-size: 12px; color: #999; margin-top: 4px;", "Total Downloads" }
                }
                div { style: "background: #4a9e4a; border-radius: 12px; padding: 16px; text-align: center; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    div { style: "font-size: 28px; font-weight: bold; color: white;", "1" }
                    div { style: "font-size: 12px; color: rgba(255,255,255,0.8); margin-top: 4px;", "Platforms" }
                }
            }

            // Quick actions
            div { style: "margin-top: 20px;",
                h3 { style: "font-size: 16px; color: #2c3e50; margin: 0 0 12px 0;", "Quick Actions" }
                div { style: "display: flex; flex-direction: column; gap: 8px;",
                    button { style: "background: white; border: 1px solid #ddd; border-radius: 10px; padding: 14px 16px; text-align: left; font-size: 14px; cursor: pointer; width: 100%;",
                        "📦 Browse OpenTTD Versions"
                    }
                    button { style: "background: white; border: 1px solid #ddd; border-radius: 10px; padding: 14px 16px; text-align: left; font-size: 14px; cursor: pointer; width: 100%;",
                        "⬇️ Download APK"
                    }
                }
            }
        }
    }
}