//! Mobile application root component with bottom navigation

use dioxus_native::prelude::*;
use crate::components::*;
use crate::pages::*;
use crate::state::MobileState;

/// Pages in the mobile app
#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Home,
    Versions,
    Downloads,
    Settings,
}

#[component]
pub fn App() -> Element {
    let mut page = use_signal(|| Page::Home);
    let state = MobileState::new();

    // Provide mobile state
    use_context_provider(|| state.clone());

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100vh; font-family: system-ui, sans-serif; background: #f5f5f5;",
            // Header
            header {
                style: "background: #2c3e50; color: white; padding: 12px 16px; text-align: center;",
                h1 { style: "font-size: 18px; margin: 0;", "OpenTTD Manager Plus" }
            }
            // Content area
            main {
                style: "flex: 1; overflow-y: auto; padding: 12px;",
                match page.read().clone() {
                    Page::Home => rsx! { HomePage {} },
                    Page::Versions => rsx! { VersionsPage {} },
                    Page::Downloads => rsx! { DownloadsPage {} },
                    Page::Settings => rsx! { SettingsPage {} },
                }
            }
            // Bottom navigation
            BottomNav {
                current_page: page.read().clone(),
                on_navigate: move |p| page.set(p),
            }
        }
    }
}