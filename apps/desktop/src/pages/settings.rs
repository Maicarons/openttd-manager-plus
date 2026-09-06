//! Settings page with language dropdown and theme switching

use dioxus_native::prelude::*;
use crate::state::AppState;
use crate::utils::theme::ThemeManager;
use crate::utils::theme::Theme;
use crate::utils::i18n::I18nManager;
use crate::utils::i18n::Locale;

#[component]
pub fn SettingsPage() -> Element {
    let _state = use_context::<AppState>();
    let mut theme_mgr = use_context::<ThemeManager>();
    let mut i18n_mgr = use_context::<I18nManager>();
    let current_theme = *theme_mgr.theme.read();
    let current_locale = *i18n_mgr.locale.read();

    let locales = Locale::all();
    let light_bg = if current_theme == Theme::Light { "#4a9e4a" } else { "#e0e0e0" };
    let light_color = if current_theme == Theme::Light { "white" } else { "#333" };
    let dark_bg = if current_theme == Theme::Dark { "#4a9e4a" } else { "#e0e0e0" };
    let dark_color = if current_theme == Theme::Dark { "white" } else { "#333" };

    rsx! {
        div { style: "max-width: 800px; margin: 0 auto;",
            h2 { style: "color: #2c3e50;", "Settings" }

            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "General" }

                // Language dropdown
                div { style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 0;",
                    span { "Language" }
                    select {
                        style: "padding: 6px 12px; border: 1px solid #ccc; border-radius: 4px; font-size: 13px; background: white;",
                        onchange: move |e| {
                            let val = e.value();
                            if val == "zh" { i18n_mgr.set_locale(Locale::ZhCn); }
                            else if val == "en" { i18n_mgr.set_locale(Locale::EnUs); }
                            else if val == "ja" { i18n_mgr.set_locale(Locale::JaJp); }
                            else if val == "ko" { i18n_mgr.set_locale(Locale::KoKr); }
                            else if val == "fr" { i18n_mgr.set_locale(Locale::FrFr); }
                            else if val == "de" { i18n_mgr.set_locale(Locale::DeDe); }
                            else if val == "it" { i18n_mgr.set_locale(Locale::ItIt); }
                        },
                        {locales.iter().map(|l| {
                            let code = match l { Locale::ZhCn=>"zh", Locale::EnUs=>"en", Locale::JaJp=>"ja", Locale::KoKr=>"ko", Locale::FrFr=>"fr", Locale::DeDe=>"de", Locale::ItIt=>"it" };
                            let label = l.label();
                            let selected = if *l == current_locale { "selected" } else { "" };
                            rsx! {
                                option { key: "{code}", value: "{code}", selected: "{selected}", "{label}" }
                            }
                        })}
                    }
                }

                // Theme toggle
                div { style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 0;",
                    span { "Theme" }
                    div { style: "display: flex; gap: 8px;",
                        button { style: "background: {light_bg}; color: {light_color}; border: none; padding: 6px 12px; border-radius: 4px; cursor: pointer; font-size: 12px;",
                            onclick: move |_| theme_mgr.set_theme(Theme::Light), "Light"
                        }
                        button { style: "background: {dark_bg}; color: {dark_color}; border: none; padding: 6px 12px; border-radius: 4px; cursor: pointer; font-size: 12px;",
                            onclick: move |_| theme_mgr.set_theme(Theme::Dark), "Dark"
                        }
                    }
                }
            }

            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "About" }
                div { style: "display: flex; justify-content: space-between; padding: 4px 0;", span { "Version" }, span { "0.1.0" } }
                div { style: "display: flex; justify-content: space-between; padding: 4px 0;", span { "License" }, span { "AGPL-3.0" } }
                div { style: "display: flex; justify-content: space-between; padding: 4px 0;", span { "Renderer" }, span { "Dioxus Native (Blitz/WGPU)" } }
            }
        }
    }
}
