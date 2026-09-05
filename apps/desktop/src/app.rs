//! Root application component

use dioxus_native::prelude::*;
use crate::components::*;
use crate::pages::*;

#[component]
pub fn App() -> Element {
    let mut page = use_signal(|| Page::Home);

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