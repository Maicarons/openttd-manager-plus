//! Root application component with theme and i18n support

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

    // Provide contexts
    use_context_provider(|| state.clone());
    use_context_provider(|| theme_manager.clone());
    use_context_provider(|| i18n_manager.clone());

    // Get theme CSS vars
    let css_vars = theme_manager.css_vars();
    let is_dark = matches!(*theme_manager.theme.read(), Theme::Dark);

    // Background color based on theme
    let bg = if is_dark { "#1a1a2e" } else { "#f5f5f5" };
    let text_color = if is_dark { "#e0e0e0" } else { "#1a1a1a" };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100vh; font-family: system-ui, sans-serif;
                    background: {bg}; color: {text_color}; {css_vars}",
            Header {}
            main {
                style: "flex: 1; display: flex; overflow: hidden;",
                Sidebar {
                    current_page: page.read().clone(),
                    on_navigate: move |p| page.set(p),
                }
                section {
                    style: "flex: 1; padding: 24px; overflow-y: auto; background: {bg};",
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