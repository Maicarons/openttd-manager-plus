//! Root application component

use dioxus_native::prelude::*;
use crate::components::*;
use crate::pages::*;
use crate::state::AppState;

#[component]
pub fn App() -> Element {
    let mut page = use_signal(|| Page::Home);
    let mut state = AppState::new();

    // Provide AppState to all child components
    use_context_provider(|| state.clone());

    // Load instances and profiles on startup
    use_effect(move || {
        state.load_instances();
        state.load_profiles();
    });

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100vh; font-family: system-ui, sans-serif;",
            Header {}
            main {
                style: "flex: 1; display: flex; overflow: hidden;",
                Sidebar {
                    current_page: page.read().clone(),
                    on_navigate: move |p| page.set(p),
                }
                section {
                    style: "flex: 1; padding: 24px; overflow-y: auto; background: #f5f5f5;",
                    match page.read().clone() {
                        Page::Home => rsx! { HomePage {} },
                        Page::Versions => rsx! { VersionsPage {} },
                        Page::Downloads => rsx! { DownloadsPage {} },
                        Page::Configs => rsx! { ConfigsPage {} },
                        Page::Mods => rsx! { ModsPage {} },
                        Page::Settings => rsx! { SettingsPage {} },
                    }
                }
            }
        }
    }
}