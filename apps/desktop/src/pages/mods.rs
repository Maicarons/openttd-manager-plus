//! Mod management page — browse and search mods by type.

use dioxus_native::prelude::*;

/// Mod type enum for the tab bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModType {
    NewGRF,
    AI,
    GameScript,
    Music,
}

impl ModType {
    fn label(&self) -> &'static str {
        match self {
            ModType::NewGRF => "NewGRF",
            ModType::AI => "AI",
            ModType::GameScript => "GameScript",
            ModType::Music => "Music",
        }
    }
}

/// Placeholder mod info for the UI.
struct ModPlaceholder {
    name: &'static str,
    description: &'static str,
    author: &'static str,
    version: &'static str,
}

/// Placeholder mods per type.
fn placeholder_mods(mod_type: ModType) -> Vec<ModPlaceholder> {
    match mod_type {
        ModType::NewGRF => vec![
            ModPlaceholder { name: "OpenGFX+", description: "Enhanced base graphics", author: "OpenTTD Team", version: "0.6.0" },
            ModPlaceholder { name: "FIRST", description: "Industry replacement set", author: "FooBar", version: "2.1.0" },
            ModPlaceholder { name: "av8", description: "Aircraft set", author: "McZapkie", version: "1.5.0" },
            ModPlaceholder { name: "North American City Set", description: "Building set for NA cities", author: "Quast65", version: "1.2.0" },
        ],
        ModType::AI => vec![
            ModPlaceholder { name: "SimpleAI", description: "A basic competitor AI", author: "OpenTTD", version: "1.3" },
            ModPlaceholder { name: "AdmiralAI", description: "Competitive sea-faring AI", author: "Admiral", version: "2.0" },
        ],
        ModType::GameScript => vec![
            ModPlaceholder { name: "City Growth Manager", description: "Manage city growth", author: "krinn", version: "1.0" },
            ModPlaceholder { name: "Cargo Distribution", description: "Distribute cargo goals", author: "frosch", version: "2.0" },
        ],
        ModType::Music => vec![
            ModPlaceholder { name: "OpenMSX", description: "Original OpenTTD music", author: "OpenTTD", version: "1.0" },
            ModPlaceholder { name: "GM Music Set", description: "General MIDI music", author: "Various", version: "1.0" },
        ],
    }
}

#[component]
pub fn ModsPage() -> Element {
    let mut active_tab = use_signal(|| ModType::NewGRF);
    let mut search_query = use_signal(|| String::new());

    let tabs = vec![ModType::NewGRF, ModType::AI, ModType::GameScript, ModType::Music];
    let current_tab = *active_tab.read();
    let query = search_query.read().clone();

    let mods = placeholder_mods(current_tab);
    let filtered: Vec<&ModPlaceholder> = if query.is_empty() {
        mods.iter().collect()
    } else {
        let q = query.to_lowercase();
        mods.iter()
            .filter(|m| m.name.to_lowercase().contains(&q) || m.description.to_lowercase().contains(&q))
            .collect()
    };

    rsx! {
        div { style: "max-width: 800px; margin: 0 auto;",
            h2 { style: "color: #2c3e50;", "Mod Management" }

            // Mod type tabs
            div { style: "display: flex; gap: 8px; margin: 16px 0;",
                {tabs.into_iter().map(|t| {
                    let is_active = t == current_tab;
                    let bg = if is_active { "#4a9e4a" } else { "#e0e0e0" };
                    let color = if is_active { "white" } else { "#333" };
                    rsx! {
                        button {
                            key: "{t.label()}",
                            style: "background: {bg}; color: {color}; border: none; padding: 8px 16px;
                                    border-radius: 4px; cursor: pointer; font-size: 13px;
                                    transition: background 0.2s;",
                            onclick: move |_| active_tab.set(t),
                            "{t.label()}"
                        }
                    }
                })}
            }

            // Search bar (non-functional placeholder)
            div { style: "margin: 12px 0;",
                input {
                    style: "width: 100%; padding: 8px 12px; border: 1px solid #ccc; border-radius: 4px;
                            font-size: 14px; box-sizing: border-box;",
                    placeholder: "Search mods... (placeholder)",
                    value: "{search_query}",
                    oninput: move |e| search_query.set(e.value()),
                }
            }

            // Online mod browser section
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Online Mod Browser — {current_tab.label()}" }

                if filtered.is_empty() {
                    p { style: "color: #999;", "No mods found matching your search." }
                } else {
                    div { style: "display: flex; flex-direction: column; gap: 8px;",
                        {filtered.iter().map(|m| {
                            let name = m.name;
                            let desc = m.description;
                            let author = m.author;
                            let version = m.version;
                            rsx! {
                                div {
                                    key: "{name}",
                                    style: "display: flex; justify-content: space-between; align-items: center;
                                            padding: 12px; border-radius: 6px; background: #fafafa;
                                            border: 1px solid #eee;",
                                    div { style: "flex: 1;",
                                        div { style: "font-weight: 600; font-size: 14px;", "{name}" }
                                        div { style: "font-size: 12px; color: #666;", "{desc}" }
                                        div { style: "font-size: 11px; color: #999; margin-top: 2px;",
                                            "by {author} · v{version}"
                                        }
                                    }
                                    button {
                                        style: "background: #4a9e4a; color: white; border: none; padding: 6px 14px;
                                                border-radius: 4px; cursor: pointer; font-size: 12px;",
                                        onclick: move |_| {
                                            log::info!("Download mod: {name}");
                                        },
                                        "Download"
                                    }
                                }
                            }
                        })}
                    }
                }
            }

            // Installed mods section
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Installed Mods" }
                p { style: "color: #999;", "No mods installed yet." }
            }
        }
    }
}