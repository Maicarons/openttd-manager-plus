//! Sidebar navigation component

use dioxus_native::prelude::*;
use crate::pages::Page;

#[component]
pub fn Sidebar(current_page: Page, on_navigate: EventHandler<Page>) -> Element {
    let nav_items = vec![
        (Page::Home, "🏠", "Home"),
        (Page::Versions, "📦", "Versions"),
        (Page::Downloads, "⬇️", "Downloads"),
        (Page::Configs, "🔧", "Configs"),
        (Page::Mods, "🎨", "Mods"),
        (Page::Settings, "⚙️", "Settings"),
    ];

    rsx! {
        nav {
            style: "width: 200px; background: #34495e; color: white; padding: 16px 0;",
            ul { style: "list-style: none; padding: 0; margin: 0;",
                {nav_items.into_iter().map(|(page, icon, label)| {
                    let is_active = page == current_page;
                    let bg = if is_active { "rgba(255,255,255,0.15)" } else { "transparent" };
                    let p = page.clone();
                    rsx! {
                        li {
                            key: "{label}",
                            style: "padding: 10px 20px; cursor: pointer; background: {bg};
                                    font-size: 14px; transition: background 0.2s;",
                            onclick: move |_| on_navigate.call(p.clone()),
                            "{icon} {label}"
                        }
                    }
                })}
            }
        }
    }
}