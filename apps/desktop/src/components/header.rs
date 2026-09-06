//! Header component — top bar with app title and version.

use dioxus_native::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            style: "display: flex; align-items: center; justify-content: space-between;
                    background: #1e293b; color: #e2e8f0; padding: 0 24px; height: 48px;
                    border-bottom: 1px solid #334155; flex-shrink: 0;",
            div { style: "display: flex; align-items: center; gap: 12px;",
                svg { width: "22", height: "22", view_box: "0 0 24 24", fill: "none", stroke: "#4ade80", stroke_width: "2",
                    circle { cx: "12", cy: "12", r: "10" }
                    path { d: "M12 6v6l4 2" }
                }
                span { style: "font-size: 15px; font-weight: 600; letter-spacing: -0.01em;", "OpenTTD Manager Plus" }
            }
            span { style: "font-size: 12px; color: #94a3b8;", "v0.1.0" }
        }
    }
}
