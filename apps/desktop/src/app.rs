//! Root application component with theme support and global context providers.

use dioxus_native::prelude::*;
use crate::components::*;
use crate::pages::*;
use crate::state::AppState;
use crate::utils::theme::ThemeManager;
use crate::utils::theme::Theme;
use crate::utils::i18n::I18nManager;

#[component]
pub fn App() -> Element {
    let mut page = use_signal(|| Page::Home);
    let state = AppState::new();
    let theme_manager = ThemeManager::new();
    let i18n_manager = I18nManager::new();

    use_context_provider(|| state.clone());
    use_context_provider(|| theme_manager.clone());
    use_context_provider(|| i18n_manager.clone());

    let is_dark = matches!(*theme_manager.theme.read(), Theme::Dark);
    let bg_primary = if is_dark { "#0f172a" } else { "#f8fafc" };
    let _bg_sidebar = if is_dark { "#1e293b" } else { "#1e293b" };
    let text_primary = if is_dark { "#e2e8f0" } else { "#0f172a" };
    let _text_sidebar = "#e2e8f0";

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100vh; font-family: 'Segoe UI', system-ui, -apple-system, sans-serif;
                    background: {bg_primary}; color: {text_primary}; transition: background 0.2s, color 0.2s;",
            Header {}
            main {
                style: "flex: 1; display: flex; overflow: hidden;",
                Sidebar {
                    current_page: page.read().clone(),
                    on_navigate: move |p| page.set(p),
                }
                section {
                    style: "flex: 1; padding: 32px; overflow-y: auto; background: {bg_primary};",
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
