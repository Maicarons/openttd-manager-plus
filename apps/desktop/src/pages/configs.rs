//! Configuration management page — profiles, create, edit, delete.

use dioxus_native::prelude::*;
use crate::state::AppState;
use otmp_core_config::profile::ConfigProfile;

fn config_profile_label(cp: &ConfigProfile) -> &'static str {
    match cp { ConfigProfile::Isolated => "Isolated", ConfigProfile::Shared { .. } => "Shared", ConfigProfile::Hybrid { .. } => "Hybrid" }
}

fn config_profile_description(cp: &ConfigProfile) -> String {
    match cp {
        ConfigProfile::Isolated => "Each instance has its own complete config directory.".to_string(),
        ConfigProfile::Shared { shared_dir } => format!("All instances share config at: {}", shared_dir.display()),
        ConfigProfile::Hybrid { shared_dir, independent_items } => {
            let items: Vec<String> = independent_items.iter().map(|i| format!("{i:?}")).collect();
            format!("Shared: {}, independent: {}", shared_dir.display(), items.join(", "))
        }
    }
}

#[component]
pub fn ConfigsPage() -> Element {
    let state = use_context::<AppState>();
    let profiles = state.profiles.read().clone();
    let has_profiles = !profiles.is_empty();

    let _profiles_sig = state.profiles;

    let mut state_effect = state.clone();
    use_effect(move || {
        if state_effect.profiles.read().is_empty() {
            state_effect.load_profiles();
        }
    });

    rsx! {
        div { style: "max-width: 800px; margin: 0 auto;",
            h2 { style: "color: #2c3e50;", "Configuration" }
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Configuration Profiles" }
                if !has_profiles {
                    p { style: "color: #999;", "No configuration profiles created yet." }
                } else {
                    {profiles.iter().map(|p| {
                        let name = p.name.clone();
                        let is_default = p.is_default;
                        let desc = config_profile_description(&p.config);
                        rsx! {
                            div {
                                key: "{p.id}", style: "padding: 12px 0; border-bottom: 1px solid #eee;",
                                div { style: "display: flex; justify-content: space-between; align-items: center;",
                                    div {
                                        div { style: "font-weight: 600;", "{name} "
                                            if is_default { span { style: "margin-left: 8px; background: #4a9e4a; color: white; padding: 2px 8px; border-radius: 4px; font-size: 11px;", "Default" } }
                                            span { style: "margin-left: 8px; background: #ecf0f1; color: #333; padding: 2px 8px; border-radius: 4px; font-size: 11px;", "{config_profile_label(&p.config)}" }
                                        }
                                        div { style: "font-size: 12px; color: #999; margin-top: 4px;", "{desc}" }
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
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "openttd.cfg Editor" }
                p { style: "color: #999;", "Select a profile to edit its configuration file." }
            }
        }
    }
}
