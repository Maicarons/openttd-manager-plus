//! Saves page — browse, import, export, and manage saves.

use dioxus_native::prelude::*;
use crate::state::AppState;

#[component]
pub fn SavesPage() -> Element {
    let state = use_context::<AppState>();
    let saves = state.saves.read().clone();
    let has_saves = !saves.is_empty();

    let mut state_effect = state.clone();
    use_effect(move || { state_effect.load_saves(); });

    rsx! {
        div { style: "max-width: 900px; margin: 0 auto;",
            div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px;",
                h1 { style: "font-size: 24px; font-weight: 700; color: #0f172a; margin: 0;", "Saves" }
                button { style: "background: #0f172a; color: white; border: none; padding: 8px 16px; border-radius: 8px; font-size: 13px; font-weight: 500; cursor: pointer;", "+ Import Save" }
            }

            if !has_saves {
                div { style: "text-align: center; padding: 60px 0; color: #64748b;",
                    svg { width: "48", height: "48", view_box: "0 0 24 24", fill: "none", stroke: "#cbd5e1", stroke_width: "1.5",
                        path { d: "M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4" }
                    }
                    div { style: "font-size: 14px;", "No saves found." }
                    div { style: "font-size: 13px; margin-top: 4px;", "Import a save file to get started." }
                }
            } else {
                div { style: "display: flex; flex-direction: column; gap: 8px;",
                    {saves.iter().map(|s| {
                        let name = s.name.clone();
                        let size = if s.size > 1024*1024 { format!("{:.1} MB", s.size as f64/(1024.0*1024.0)) } else { format!("{:.1} KB", s.size as f64/1024.0) };
                        let type_label = match s.save_type { otmp_core_config::save::SaveType::SaveGame => "Save", otmp_core_config::save::SaveType::Scenario => "Scenario", otmp_core_config::save::SaveType::Heightmap => "Heightmap" };
                        let date = s.modified.format("%Y-%m-%d").to_string();
                        rsx! {
                            div { key: "{s.id}", style: "background: white; border-radius: 10px; padding: 16px 20px; display: flex; justify-content: space-between; align-items: center; box-shadow: 0 1px 3px rgba(0,0,0,0.06);",
                                div {
                                    div { style: "font-weight: 500; font-size: 14px; color: #0f172a;", "{name}" }
                                    div { style: "font-size: 12px; color: #64748b; margin-top: 2px;", "{type_label} · {size} · {date}" }
                                }
                                div { style: "display: flex; gap: 6px;",
                                    button { style: "background: white; color: #0f172a; border: 1px solid #e2e8f0; padding: 4px 10px; border-radius: 6px; font-size: 12px; cursor: pointer;", "Export" }
                                    button { style: "background: white; color: #ef4444; border: 1px solid #fecaca; padding: 4px 10px; border-radius: 6px; font-size: 12px; cursor: pointer;", "Delete" }
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}
