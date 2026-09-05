//! Root application component with theme and i18n support

use dioxus_native::prelude::*;
use crate::components::*;
use crate::pages::*;
use crate::state::AppState;
use crate::utils::theme::ThemeManager;
use crate::utils::i18n::I18nManager;

#[component]
pub fn App() -> Element {
    let mut page = use_signal(|| Page::Home);
    let state = AppState::new();
    let theme_manager = ThemeManager::new();
    let i18n_manager = I18nManager::new();

    // Provide contexts
    use_context_provider(|| state.clone());
    use_context_provider(|| theme_manager.clone());
    use_context_provider(|| i18n_manager.clone());

    let theme = *theme_manager.theme.read();
    let locale = *i18n_manager.locale.read();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100vh; font-family: system-ui, sans-serif;
                    background: var(--bg-primary, #f5f5f5); color: var(--text-primary, #1a1a1a);",
            Header {}
            main {
                style: "flex: 1; display: flex; overflow: hidden;",
                Sidebar {
                    current_page: page.read().clone(),
                    on_navigate: move |p| page.set(p),
                }
                section {
                    style: "flex: 1; padding: 24px; overflow-y: auto;
                            background: var(--bg-primary, #f5f5f5);",
                    match page.read().clone() {
                        Page::Home => rsx! { HomePage {} },
                        Page::Versions => rsx! { VersionsPage {} },
                        Page::Downloads => rsx! { DownloadsPage {} },
                        Page::Configs => rsx! { ConfigsPage {} },
                        Page::Mods => rsx! { ModsPage {} },
                        Page::Saves => rsx! { SavesPage {} },
                        Page::Settings => rsx! { SettingsPage {} },
                    }
                }
            }
        }
    }
}