//! Save management page — browse, import, export, and manage saves

use dioxus_native::prelude::*;
use crate::state::AppState;

#[component]
pub fn SavesPage() -> Element {
    let state = use_context::<AppState>();
    let saves = state.saves.read().clone();
    let has_saves = !saves.is_empty();

    let mut state_effect = state.clone();
    use_effect(move || {
        state_effect.load_saves();
    });

    rsx! {
        div { style: "max-width: 800px; margin: 0 auto;",
            h2 { style: "color: #2c3e50;", "Save Management" }

            // Stats bar
            div { style: "display: flex; gap: 12px; margin: 16px 0;",
                div { style: "background: white; border-radius: 8px; padding: 12px 20px; flex: 1; text-align: center; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    span { style: "display: block; font-size: 24px; font-weight: bold; color: #4a9e4a;", "{saves.len()}" }
                    span { style: "font-size: 12px; color: #999;", "Total Saves" }
                }
                div { style: "background: white; border-radius: 8px; padding: 12px 20px; flex: 1; text-align: center; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    span { style: "display: block; font-size: 24px; font-weight: bold;", "{saves.iter().filter(|s| s.save_type == otmp_core_config::save::SaveType::SaveGame).count()}" }
                    span { style: "font-size: 12px; color: #999;", "Save Games" }
                }
                div { style: "background: white; border-radius: 8px; padding: 12px 20px; flex: 1; text-align: center; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    span { style: "display: block; font-size: 24px; font-weight: bold;", "{saves.iter().filter(|s| s.save_type == otmp_core_config::save::SaveType::Scenario).count()}" }
                    span { style: "font-size: 12px; color: #999;", "Scenarios" }
                }
            }

            // Save list
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Saves & Scenarios" }
                if !has_saves {
                    p { style: "color: #999;", "No saves found. Import a save file to get started." }
                    button { style: "background: #4a9e4a; color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-size: 13px;", "+ Import Save" }
                } else {
                    {saves.iter().map(|s| {
                        let name = s.name.clone();
                        let size = if s.size > 1024*1024 {
                            format!("{:.1} MB", s.size as f64 / (1024.0*1024.0))
                        } else {
                            format!("{:.1} KB", s.size as f64 / 1024.0)
                        };
                        let type_label = match s.save_type {
                            otmp_core_config::save::SaveType::SaveGame => "Save",
                            otmp_core_config::save::SaveType::Scenario => "Scenario",
                            otmp_core_config::save::SaveType::Heightmap => "Heightmap",
                        };
                        let date = s.modified.format("%Y-%m-%d").to_string();
                        rsx! {
                            div {
                                key: "{s.id}",
                                style: "display: flex; justify-content: space-between; align-items: center; padding: 10px 0; border-bottom: 1px solid #eee;",
                                div {
                                    div { style: "font-weight: 600; font-size: 14px;", "{name}" }
                                    div { style: "font-size: 11px; color: #999; margin-top: 2px;", "{type_label} · {size} · {date}" }
                                }
                                div {
                                    button { style: "background: #3498db; color: white; border: none; padding: 4px 10px; border-radius: 4px; cursor: pointer; font-size: 11px;", "Export" }
                                    button { style: "margin-left: 4px; background: #e74c3c; color: white; border: none; padding: 4px 10px; border-radius: 4px; cursor: pointer; font-size: 11px;", "Delete" }
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}