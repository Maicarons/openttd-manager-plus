//! Configuration management page — profiles, config editor with presets

use dioxus_native::prelude::*;
use crate::state::AppState;
use otmp_core_config::profile::ConfigProfile;

fn config_profile_label(cp: &ConfigProfile) -> &'static str {
    match cp { ConfigProfile::Isolated => "Isolated", ConfigProfile::Shared { .. } => "Shared", ConfigProfile::Hybrid { .. } => "Hybrid" }
}

/// Config presets for common scenarios
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConfigPreset {
    Default,
    Performance,
    Multiplayer,
    MaxGraphics,
}

impl ConfigPreset {
    fn label(&self) -> &'static str {
        match self {
            ConfigPreset::Default => "Default (Recommended)",
            ConfigPreset::Performance => "High Performance",
            ConfigPreset::Multiplayer => "Multiplayer Optimized",
            ConfigPreset::MaxGraphics => "Maximum Graphics",
        }
    }

    fn generate(&self) -> &'static str {
        match self {
            ConfigPreset::Default => r#"[misc]
resolution = "1920,1080"
display = 0
fullscreen = false
music_volume = 100
effect_volume = 100
gui_scale = 100

[gameplay]
difficulty = "easy"
autosave = "every 3 months"

[network]
max_players = 4
server_port = 3979
"#,
            ConfigPreset::Performance => r#"[misc]
resolution = "1280,720"
display = 0
fullscreen = false
music_volume = 50
effect_volume = 100
gui_scale = 75

[gameplay]
autosave = "every 6 months"
"#,
            ConfigPreset::Multiplayer => r#"[misc]
resolution = "1920,1080"
display = 0
fullscreen = false

[network]
max_players = 15
server_port = 3979
server_name = "OpenTTD Server"
server_password = ""
server_advertise = true

[gameplay]
autosave = "every month"
"#,
            ConfigPreset::MaxGraphics => r#"[misc]
resolution = "3840,2160"
display = 0
fullscreen = true
music_volume = 100
effect_volume = 100
gui_scale = 150
"#,
        }
    }
}

#[component]
pub fn ConfigsPage() -> Element {
    let state = use_context::<AppState>();
    let profiles = state.profiles.read().clone();
    let has_profiles = !profiles.is_empty();

    let mut active_tab = use_signal(|| 0);
    let mut editor_content = use_signal(|| ConfigPreset::Default.generate().to_string());

    let mut state_effect = state.clone();
    use_effect(move || {
        if state_effect.profiles.read().is_empty() {
            state_effect.load_profiles();
        }
    });

    let presets = vec![ConfigPreset::Default, ConfigPreset::Performance, ConfigPreset::Multiplayer, ConfigPreset::MaxGraphics];

    // Pre-compute tab styles
    let tab0_bg = if *active_tab.read() == 0 { "#4a9e4a" } else { "#e0e0e0" };
    let tab0_color = if *active_tab.read() == 0 { "white" } else { "#333" };
    let tab1_bg = if *active_tab.read() == 1 { "#4a9e4a" } else { "#e0e0e0" };
    let tab1_color = if *active_tab.read() == 1 { "white" } else { "#333" };
    let is_tab0 = *active_tab.read() == 0;
    let is_tab1 = *active_tab.read() == 1;

    rsx! {
        div { style: "max-width: 900px; margin: 0 auto;",
            h2 { style: "color: #2c3e50;", "Configuration" }

            // Tab bar
            div { style: "display: flex; gap: 8px; margin: 16px 0;",
                button {
                    style: "background: {tab0_bg}; color: {tab0_color}; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-size: 13px;",
                    onclick: move |_| active_tab.set(0),
                    "Profiles"
                }
                button {
                    style: "background: {tab1_bg}; color: {tab1_color}; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-size: 13px;",
                    onclick: move |_| active_tab.set(1),
                    "Config Editor"
                }
            }

            // Tab 0: Profiles
            if is_tab0 {
                div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    h3 { style: "margin-top: 0;", "Configuration Profiles" }
                    if !has_profiles {
                        p { style: "color: #999;", "No configuration profiles yet." }
                    } else {
                        {profiles.iter().map(|p| {
                            let name = p.name.clone();
                            let is_default = p.is_default;
                            rsx! {
                                div {
                                    key: "{p.id}", style: "padding: 12px 0; border-bottom: 1px solid #eee;",
                                    div { style: "display: flex; justify-content: space-between; align-items: center;",
                                        div {
                                            div { style: "font-weight: 600;", "{name} "
                                                if is_default { span { style: "margin-left: 8px; background: #4a9e4a; color: white; padding: 2px 8px; border-radius: 4px; font-size: 11px;", "Default" } }
                                            }
                                        }
                                        div {
                                            button { style: "background: #3498db; color: white; border: none; padding: 4px 10px; border-radius: 4px; cursor: pointer; font-size: 11px;", "Edit" }
                                            button { style: "margin-left: 4px; background: #e74c3c; color: white; border: none; padding: 4px 10px; border-radius: 4px; cursor: pointer; font-size: 11px;", "Delete" }
                                        }
                                    }
                                }
                            }
                        })}
                    }
                    button { style: "margin-top: 12px; background: #4a9e4a; color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-size: 13px;", "+ New Profile" }
                }
            }

            // Tab 1: Config Editor with Presets
            if is_tab1 {
                // Preset buttons
                div { style: "display: flex; gap: 8px; margin: 16px 0; flex-wrap: wrap;",
                    {presets.into_iter().map(|preset| {
                        let bg = "#e0e0e0"; let color = "#333";
                        let p_label = preset.label();
                        let mut ec = editor_content;
                        rsx! {
                            button {
                                key: "{p_label}",
                                style: "background: {bg}; color: {color}; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-size: 12px;",
                                onclick: move |_| { ec.set(preset.generate().to_string()); },
                                "{p_label}"
                            }
                        }
                    })}
                }

                // Code editor
                div { style: "background: #1e1e2e; border-radius: 8px; overflow: hidden; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    div { style: "background: #2d2d44; padding: 8px 16px; display: flex; justify-content: space-between; align-items: center;",
                        span { style: "color: #aaa; font-size: 12px;", "openttd.cfg" }
                        div { style: "display: flex; gap: 4px;",
                            button { style: "background: #4a9e4a; color: white; border: none; padding: 4px 10px; border-radius: 4px; cursor: pointer; font-size: 11px;", "Save" }
                            button { style: "background: #e74c3c; color: white; border: none; padding: 4px 10px; border-radius: 4px; cursor: pointer; font-size: 11px;", "Reset" }
                        }
                    }
                    textarea {
                        style: "width: 100%; min-height: 400px; background: #1e1e2e; color: #d4d4d4; border: none; padding: 16px; font-family: 'Consolas', 'Courier New', monospace; font-size: 13px; line-height: 1.5; resize: vertical; box-sizing: border-box;",
                        value: "{editor_content}",
                        oninput: move |e| editor_content.set(e.value()),
                    }
                }
                div { style: "margin-top: 12px; font-size: 12px; color: #999;",
                    "Click a preset to load config template. Edit directly in the text area. Press Save to apply."
                }
            }
        }
    }
}