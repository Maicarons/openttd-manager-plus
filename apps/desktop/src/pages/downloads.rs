//! Download management page — active downloads, queue, history

use dioxus_native::prelude::*;

#[component]
pub fn DownloadsPage() -> Element {
    rsx! {
        div { style: "max-width: 800px; margin: 0 auto;",
            h2 { style: "color: #2c3e50;", "Download Manager" }

            // Active downloads
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Active Downloads" }
                p { style: "color: #999;", "No active downloads." }
            }

            // Download queue
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Queue" }
                p { style: "color: #999;", "Queue is empty." }
            }

            // Completed
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Completed" }
                p { style: "color: #999;", "No completed downloads." }
            }
        }
    }
}