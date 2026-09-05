//! Settings page with language/theme switching and download configuration

use dioxus_native::prelude::*;
use crate::state::AppState;
use crate::utils::theme::ThemeManager;
use crate::utils::theme::Theme;
use crate::utils::i18n::I18nManager;
use crate::utils::i18n::Locale;

#[component]
pub fn SettingsPage() -> Element {
    let state = use_context::<AppState>();
    let mut theme_mgr = use_context::<ThemeManager>();
    let mut i18n_mgr = use_context::<I18nManager>();
    let current_theme = *theme_mgr.theme.read();
    let current_locale = *i18n_mgr.locale.read();

    // Pre-compute button styles
    let zh_bg = if current_locale == Locale::ZhCn { "#4a9e4a" } else { "#e0e0e0" };
    let zh_color = if current_locale == Locale::ZhCn { "white" } else { "#333" };
    let en_bg = if current_locale == Locale::EnUs { "#4a9e4a" } else { "#e0e0e0" };
    let en_color = if current_locale == Locale::EnUs { "white" } else { "#333" };
    let light_bg = if current_theme == Theme::Light { "#4a9e4a" } else { "#e0e0e0" };
    let light_color = if current_theme == Theme::Light { "white" } else { "#333" };
    let dark_bg = if current_theme == Theme::Dark { "#4a9e4a" } else { "#e0e0e0" };
    let dark_color = if current_theme == Theme::Dark { "white" } else { "#333" };

    rsx! {
        div { style: "max-width: 800px; margin: 0 auto;",
            h2 { style: "color: #2c3e50;", "Settings" }

            // General section
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "General" }

                // Language selector
                div { style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 0;",
                    span { "Language" }
                    div { style: "display: flex; gap: 8px;",
                        button {
                            style: "background: {zh_bg}; color: {zh_color}; border: none; padding: 6px 12px; border-radius: 4px; cursor: pointer; font-size: 12px;",
                            onclick: move |_| i18n_mgr.set_locale(Locale::ZhCn),
                            "简体中文"
                        }
                        button {
                            style: "background: {en_bg}; color: {en_color}; border: none; padding: 6px 12px; border-radius: 4px; cursor: pointer; font-size: 12px;",
                            onclick: move |_| i18n_mgr.set_locale(Locale::EnUs),
                            "English"
                        }
                    }
                }

                // Theme selector
                div { style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 0;",
                    span { "Theme" }
                    div { style: "display: flex; gap: 8px;",
                        button {
                            style: "background: {light_bg}; color: {light_color}; border: none; padding: 6px 12px; border-radius: 4px; cursor: pointer; font-size: 12px;",
                            onclick: move |_| theme_mgr.set_theme(Theme::Light),
                            "Light"
                        }
                        button {
                            style: "background: {dark_bg}; color: {dark_color}; border: none; padding: 6px 12px; border-radius: 4px; cursor: pointer; font-size: 12px;",
                            onclick: move |_| theme_mgr.set_theme(Theme::Dark),
                            "Dark"
                        }
                    }
                }
            }

            // Download section
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Download" }
                div { style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 0;",
                    span { "Mirror Source" }
                    span { style: "color: #999;", "Auto (Fastest)" }
                }
                div { style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 0;",
                    span { "Max Concurrent Downloads" }
                    span { style: "color: #999;", "3" }
                }
            }

            // About section
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "About" }
                div { style: "display: flex; justify-content: space-between; padding: 4px 0;", span { "Version" }, span { "0.1.0" } }
                div { style: "display: flex; justify-content: space-between; padding: 4px 0;", span { "License" }, span { "AGPL-3.0" } }
                div { style: "display: flex; justify-content: space-between; padding: 4px 0;", span { "Renderer" }, span { "Dioxus Native (Blitz/WGPU)" } }
            }
        }
    }
}