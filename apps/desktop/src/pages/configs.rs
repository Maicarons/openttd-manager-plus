//! Configuration page — profiles and config editor.

use dioxus_native::prelude::*;
use crate::state::AppState;

#[component]
pub fn ConfigsPage() -> Element {
    let state = use_context::<AppState>();
    let profiles = state.profiles.read().clone();
    let has_profiles = !profiles.is_empty();
    let mut active_tab = use_signal(|| 0);
    let mut editor_content = use_signal(|| r#"[misc]
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
"#.to_string());

    let mut state_effect = state.clone();
    use_effect(move || { if state_effect.profiles.read().is_empty() { state_effect.load_profiles(); } });

    let tab0 = *active_tab.read() == 0;
    let tab1 = *active_tab.read() == 1;
    let tab0_bg = if tab0 { "white" } else { "transparent" };
    let tab0_shadow = if tab0 { "0 1px 2px rgba(0,0,0,0.05)" } else { "none" };
    let tab1_bg = if tab1 { "white" } else { "transparent" };
    let tab1_shadow = if tab1 { "0 1px 2px rgba(0,0,0,0.05)" } else { "none" };

    rsx! {
        div { style: "max-width: 900px; margin: 0 auto;",
            h1 { style: "font-size: 24px; font-weight: 700; color: #0f172a; margin: 0 0 24px 0;", "Configuration" }

            div { style: "display: flex; gap: 4px; margin-bottom: 20px; background: #f1f5f9; padding: 4px; border-radius: 8px; width: fit-content;",
                button { style: "background: {tab0_bg}; color: #0f172a; border: none; padding: 6px 14px; border-radius: 6px; font-size: 13px; font-weight: 500; cursor: pointer; box-shadow: {tab0_shadow};", onclick: move |_| active_tab.set(0), "Profiles" }
                button { style: "background: {tab1_bg}; color: #0f172a; border: none; padding: 6px 14px; border-radius: 6px; font-size: 13px; font-weight: 500; cursor: pointer; box-shadow: {tab1_shadow};", onclick: move |_| active_tab.set(1), "Config Editor" }
            }

            if tab0 {
                div { style: "background: white; border-radius: 12px; padding: 24px; box-shadow: 0 1px 3px rgba(0,0,0,0.06);",
                    h2 { style: "font-size: 15px; font-weight: 600; color: #0f172a; margin: 0 0 16px 0;", "Configuration Profiles" }
                    if !has_profiles { p { style: "color: #64748b; font-size: 14px;", "No configuration profiles yet." } }
                    else {
                        {profiles.iter().map(|p| {
                            let name = p.name.clone(); let is_default = p.is_default;
                            rsx! {
                                div { key: "{p.id}", style: "display: flex; justify-content: space-between; align-items: center; padding: 10px 0; border-bottom: 1px solid #f1f5f9;",
                                    div { span { style: "font-weight: 500; font-size: 14px;", "{name} " } if is_default { span { style: "background: #dcfce7; color: #166534; padding: 2px 8px; border-radius: 4px; font-size: 11px;", "Default" } } }
                                    div { button { style: "background: white; color: #0f172a; border: 1px solid #e2e8f0; padding: 4px 10px; border-radius: 6px; font-size: 12px; cursor: pointer;", "Edit" } " " button { style: "background: white; color: #ef4444; border: 1px solid #fecaca; padding: 4px 10px; border-radius: 6px; font-size: 12px; cursor: pointer;", "Delete" } }
                                }
                            }
                        })}
                    }
                    button { style: "margin-top: 12px; background: #0f172a; color: white; border: none; padding: 8px 16px; border-radius: 8px; font-size: 13px; font-weight: 500; cursor: pointer;", "+ New Profile" }
                }
            }

            if tab1 {
                div { style: "background: #1e1e2e; border-radius: 10px; overflow: hidden; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    div { style: "background: #2d2d44; padding: 10px 16px; display: flex; justify-content: space-between; align-items: center;",
                        span { style: "color: #94a3b8; font-size: 12px; font-family: monospace;", "openttd.cfg" }
                        div { style: "display: flex; gap: 6px;",
                            button { style: "background: #4ade80; color: #0f172a; border: none; padding: 4px 10px; border-radius: 4px; font-size: 11px; font-weight: 600; cursor: pointer;", "Save" }
                            button { style: "background: #334155; color: #e2e8f0; border: none; padding: 4px 10px; border-radius: 4px; font-size: 11px; cursor: pointer;", "Reset" }
                        }
                    }
                    textarea {
                        style: "width: 100%; min-height: 400px; background: #1e1e2e; color: #d4d4d4; border: none; padding: 16px; font-family: 'Consolas','Courier New',monospace; font-size: 13px; line-height: 1.5; resize: vertical; box-sizing: border-box; outline: none;",
                        value: "{editor_content}",
                        oninput: move |e| editor_content.set(e.value()),
                    }
                }
                div { style: "margin-top: 12px; font-size: 12px; color: #64748b;", "Edit your openttd.cfg configuration directly. Changes take effect on next game launch." }
            }
        }
    }
}
