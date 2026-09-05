//! Mobile components — bottom navigation bar

use dioxus_native::prelude::*;
use crate::app::Page;

#[component]
pub fn BottomNav(current_page: Page, on_navigate: EventHandler<Page>) -> Element {
    let items = vec![
        (Page::Home, "🏠", "Home"),
        (Page::Versions, "📦", "Versions"),
        (Page::Downloads, "⬇️", "Downloads"),
        (Page::Settings, "⚙️", "Settings"),
    ];

    rsx! {
        footer {
            style: "display: flex; background: #34495e; border-top: 1px solid #2c3e50;",
            {items.into_iter().map(|(p, icon, label)| {
                let is_active = p == current_page;
                let bg = if is_active { "rgba(255,255,255,0.15)" } else { "transparent" };
                let p_clone = p.clone();
                rsx! {
                    button {
                        key: "{label}",
                        style: "flex: 1; padding: 10px 4px; border: none; background: {bg}; color: white;
                                font-size: 11px; cursor: pointer; display: flex; flex-direction: column; align-items: center; gap: 2px;",
                        onclick: move |_| on_navigate.call(p_clone.clone()),
                        span { style: "font-size: 18px;", "{icon}" }
                        span { "{label}" }
                    }
                }
            })}
        }
    }
}