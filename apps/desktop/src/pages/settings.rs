//! Settings page — language dropdown, theme toggle, about.

use dioxus_native::prelude::*;
use crate::utils::theme::ThemeManager;
use crate::utils::theme::Theme;
use crate::utils::i18n::I18nManager;
use crate::utils::i18n::Locale;

#[component]
pub fn SettingsPage() -> Element {
    let mut theme_mgr = use_context::<ThemeManager>();
    let mut i18n_mgr = use_context::<I18nManager>();
    let current_theme = *theme_mgr.theme.read();
    let current_locale = *i18n_mgr.locale.read();
    let locales = Locale::all();

    rsx! {
        div { style: "max-width: 700px; margin: 0 auto;",
            h1 { style: "font-size: 24px; font-weight: 700; color: #0f172a; margin: 0 0 24px 0;", "Settings" }

            // General
            div { style: "background: white; border-radius: 12px; padding: 24px; margin-bottom: 16px; box-shadow: 0 1px 3px rgba(0,0,0,0.06);",
                h2 { style: "font-size: 15px; font-weight: 600; color: #0f172a; margin: 0 0 16px 0;", "General" }
                div { style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 0;",
                    span { style: "font-size: 14px; color: #0f172a;", "Language" }
                    select {
                        style: "padding: 6px 32px 6px 12px; border: 1px solid #e2e8f0; border-radius: 6px; font-size: 13px; background: white; color: #0f172a; appearance: none; cursor: pointer;",
                        onchange: move |e| {
                            match e.value().as_str() {
                                "zh" => i18n_mgr.set_locale(Locale::ZhCn), "en" => i18n_mgr.set_locale(Locale::EnUs),
                                "ja" => i18n_mgr.set_locale(Locale::JaJp), "ko" => i18n_mgr.set_locale(Locale::KoKr),
                                "fr" => i18n_mgr.set_locale(Locale::FrFr), "de" => i18n_mgr.set_locale(Locale::DeDe),
                                "it" => i18n_mgr.set_locale(Locale::ItIt), _ => {}
                            }
                        },
                        {locales.iter().map(|l| {
                            let code = match l { Locale::ZhCn=>"zh", Locale::EnUs=>"en", Locale::JaJp=>"ja", Locale::KoKr=>"ko", Locale::FrFr=>"fr", Locale::DeDe=>"de", Locale::ItIt=>"it" };
                            let selected = if *l == current_locale { "selected" } else { "" };
                            rsx! { option { key: "{code}", value: "{code}", selected: "{selected}", "{l.label()}" } }
                        })}
                    }
                }
                div { style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 0;",
                    span { style: "font-size: 14px; color: #0f172a;", "Theme" }
                    div { style: "display: flex; gap: 8px;",
                        ThemeButton { active: current_theme == Theme::Light, label: "Light", onclick: move |_| theme_mgr.set_theme(Theme::Light) }
                        ThemeButton { active: current_theme == Theme::Dark, label: "Dark", onclick: move |_| theme_mgr.set_theme(Theme::Dark) }
                    }
                }
            }

            // About
            div { style: "background: white; border-radius: 12px; padding: 24px; box-shadow: 0 1px 3px rgba(0,0,0,0.06);",
                h2 { style: "font-size: 15px; font-weight: 600; color: #0f172a; margin: 0 0 16px 0;", "About" }
                SettingRow { label: "Version", value: "0.1.0" }
                SettingRow { label: "License", value: "AGPL-3.0" }
                SettingRow { label: "Renderer", value: "Dioxus Native (Blitz/WGPU)" }
                SettingRow { label: "Platform", value: "Windows / Linux / macOS / Android / iOS" }
            }
        }
    }
}

#[component]
fn ThemeButton(active: bool, label: String, onclick: EventHandler) -> Element {
    let bg = if active { "#0f172a" } else { "white" };
    let color = if active { "white" } else { "#0f172a" };
    let border = if active { "1px solid #0f172a" } else { "1px solid #e2e8f0" };
    rsx! {
        button { style: "background: {bg}; color: {color}; border: {border}; padding: 6px 14px; border-radius: 6px; font-size: 13px; cursor: pointer;", onclick: move |_| onclick.call(()), "{label}" }
    }
}

#[component]
fn SettingRow(label: String, value: String) -> Element {
    rsx! {
        div { style: "display: flex; justify-content: space-between; padding: 6px 0; font-size: 13px;",
            span { style: "color: #64748b;", "{label}" }
            span { style: "color: #0f172a;", "{value}" }
        }
    }
}
