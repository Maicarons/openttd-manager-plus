//! Mod management page — browse mods from BaNaNaS, search, and manage local mods

use dioxus_native::prelude::*;
use otmp_core_manager::source::bananas::BananasFetcher;
use otmp_core_manager::version::ModType;

fn mod_type_label(mt: &ModType) -> &'static str {
    match mt {
        ModType::NewGRF => "NewGRF",
        ModType::AI => "AI",
        ModType::GameScript => "GameScript",
        ModType::MusicSet => "Music",
    }
}

#[component]
pub fn ModsPage() -> Element {
    let mut active_tab = use_signal(|| ModType::NewGRF);
    let mut search_query = use_signal(|| String::new());
    let mods = use_signal(|| Vec::<otmp_core_manager::version::ModInfo>::new());
    let loading = use_signal(|| false);

    let tabs = vec![ModType::NewGRF, ModType::AI, ModType::GameScript, ModType::MusicSet];
    let current_tab = active_tab.read().clone();
    let query = search_query.read().clone();

    let loaded_mods = mods.read().clone();
    let filtered: Vec<_> = if query.is_empty() {
        loaded_mods.clone()
    } else {
        let q = query.to_lowercase();
        loaded_mods.into_iter()
            .filter(|m| m.name.to_lowercase().contains(&q) || m.author.to_lowercase().contains(&q))
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
                    let mut l2 = loading;
                    let mut m2 = mods;
                    rsx! {
                        button {
                            key: "{mod_type_label(&t)}",
                            style: "background: {bg}; color: {color}; border: none; padding: 8px 16px;
                                    border-radius: 4px; cursor: pointer; font-size: 13px; transition: background 0.2s;",
                            onclick: move |_| {
                                active_tab.set(t);
                                l2.set(true);
                                let fetcher = BananasFetcher::new();
                                let mt = t;
                                spawn(async move {
                                    match fetcher.fetch_mods(mt).await {
                                        Ok(items) => { m2.set(items); }
                                        Err(e) => { log::error!("Failed to fetch mods: {e}"); }
                                    }
                                    l2.set(false);
                                });
                            },
                            "{mod_type_label(&t)}"
                        }
                    }
                })}
            }

            // Search bar
            div { style: "margin: 12px 0;",
                input {
                    style: "width: 100%; padding: 8px 12px; border: 1px solid #ccc; border-radius: 4px; font-size: 14px; box-sizing: border-box;",
                    placeholder: "Search mods by name or author...",
                    value: "{search_query}",
                    oninput: move |e| search_query.set(e.value()),
                }
            }

            // Online mod browser
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Online Mod Browser — {mod_type_label(&current_tab)}" }

                if *loading.read() {
                    p { style: "color: #999;", "Loading mods from BaNaNaS..." }
                } else if filtered.is_empty() {
                    p { style: "color: #999;", "No mods found. Click a tab above to browse." }
                } else {
                    div { style: "display: flex; flex-direction: column; gap: 8px;",
                        {filtered.iter().map(|mod_info| {
                            let name = mod_info.name.clone();
                            let desc = mod_info.description.clone();
                            let author = mod_info.author.clone();
                            let version = mod_info.version.clone();
                            let filesize = mod_info.filesize.map(|s| {
                                if s > 1024*1024 { format!("{:.1} MB", s as f64 / (1024.0*1024.0)) }
                                else { format!("{:.1} KB", s as f64 / 1024.0) }
                            }).unwrap_or_default();
                            rsx! {
                                div {
                                    key: "{mod_info.id}",
                                    style: "display: flex; justify-content: space-between; align-items: center; padding: 12px; border-radius: 6px; background: #fafafa; border: 1px solid #eee;",
                                    div { style: "flex: 1;",
                                        div { style: "font-weight: 600; font-size: 14px;", "{name}" }
                                        div { style: "font-size: 12px; color: #666;", "{desc}" }
                                        div { style: "font-size: 11px; color: #999; margin-top: 2px;", "by {author} · v{version} {filesize}" }
                                    }
                                    button {
                                        style: "background: #4a9e4a; color: white; border: none; padding: 6px 14px; border-radius: 4px; cursor: pointer; font-size: 12px;",
                                        onclick: move |_| { log::info!("Download mod: {name}"); },
                                        "Download"
                                    }
                                }
                            }
                        })}
                    }
                }
            }

            // Installed mods
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Installed Mods" }
                p { style: "color: #999;", "No mods installed yet." }
            }
        }
    }
}