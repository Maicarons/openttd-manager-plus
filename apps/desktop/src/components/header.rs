//! Top header bar component

use dioxus_native::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            style: "display: flex; align-items: center; justify-content: space-between;
                    background: #2c3e50; color: white; padding: 8px 24px;",
            h1 { style: "font-size: 18px; margin: 0;", "OpenTTD Manager Plus" }
            div {
                style: "display: flex; align-items: center; gap: 12px;",
                span { style: "font-size: 12px; opacity: 0.7;", "v0.1.0" }
            }
        }
    }
}