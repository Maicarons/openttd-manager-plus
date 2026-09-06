//! Sidebar navigation with SVG icons (no emoji).

use dioxus_native::prelude::*;
use crate::pages::Page;

#[component]
pub fn Sidebar(current_page: Page, on_navigate: EventHandler<Page>) -> Element {
    #[derive(Clone)]
    struct NavItem { page: Page, label: &'static str, svg: &'static str }

    let items = vec![
        NavItem { page: Page::Home, label: "Home", svg: r#"<path d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/>"# },
        NavItem { page: Page::Versions, label: "Versions", svg: r#"<path d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"/>"# },
        NavItem { page: Page::Downloads, label: "Downloads", svg: r#"<path d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"/>"# },
        NavItem { page: Page::Configs, label: "Configs", svg: r#"<path d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/><path d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>"# },
        NavItem { page: Page::Mods, label: "Mods", svg: r#"<path d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>"# },
        NavItem { page: Page::Saves, label: "Saves", svg: r#"<path d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4"/>"# },
        NavItem { page: Page::Settings, label: "Settings", svg: r#"<path d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"/>"# },
    ];

    rsx! {
        nav {
            style: "width: 200px; background: #1e293b; color: #e2e8f0;
                    padding: 8px 0; display: flex; flex-direction: column; flex-shrink: 0;
                    border-right: 1px solid #334155;",
            div { style: "flex: 1;",
                {items.into_iter().map(|item| {
                    let is_active = item.page == current_page;
                    let bg = if is_active { "rgba(74,222,128,0.15)" } else { "transparent" };
                    let border = if is_active { "2px solid #4ade80" } else { "2px solid transparent" };
                    let fw = if is_active { 600 } else { 400 };
                    let p = item.page.clone();
                    let stroke_color = if is_active { "#4ade80" } else { "#94a3b8" };
                    rsx! {
                        div {
                            key: "{item.label}",
                            style: "display: flex; align-items: center; gap: 10px; padding: 10px 16px; margin: 2px 8px; cursor: pointer; border-radius: 6px; background: {bg}; border-left: {border}; font-size: 13px; font-weight: {fw}; transition: background 0.15s, border-color 0.15s;",
                            onclick: move |_| on_navigate.call(p.clone()),
                            svg { width: "18", height: "18", view_box: "0 0 24 24", fill: "none", stroke: "{stroke_color}", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", dangerous_inner_html: "{item.svg}" }
                            span { "{item.label}" }
                        }
                    }
                })}
            }
        }
    }
}
