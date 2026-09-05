//! Mod management page

use dioxus_native::prelude::*;

#[component]
pub fn ModsPage() -> Element {
    rsx! {
        div { style: "max-width: 800px; margin: 0 auto;",
            h2 { style: "color: #2c3e50;", "Mod Management" }

            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Online Mod Browser" }
                p { style: "color: #999;", "Mod browser coming soon. (Backend integration pending)" }
            }

            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Installed Mods" }
                p { style: "color: #999;", "No mods installed." }
            }
        }
    }
}