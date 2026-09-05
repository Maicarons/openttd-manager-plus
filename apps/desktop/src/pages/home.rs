//! Home page — overview and quick start

use dioxus_native::prelude::*;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        div { style: "max-width: 800px; margin: 0 auto;",
            h2 { style: "color: #2c3e50;", "Welcome to OpenTTD Manager Plus" }
            p { style: "color: #666; line-height: 1.6;",
                "Manage OpenTTD versions, configurations, mods, and more — all in one place."
            }

            // Quick start card
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Quick Start" }
                p { "Your default version is not set. Install a version to get started." }
                button { style: "background: #4a9e4a; color: white; border: none; padding: 10px 20px; border-radius: 4px; cursor: pointer;",
                    "Browse Versions"
                }
            }

            // Recent versions
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Recent Versions" }
                p { style: "color: #999;", "No versions installed yet." }
            }

            // Download queue
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Download Queue" }
                p { style: "color: #999;", "No active downloads." }
            }
        }
    }
}