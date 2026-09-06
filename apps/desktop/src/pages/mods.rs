//! Mods page — browse from BaNaNaS, search, and manage local mods.

use dioxus_native::prelude::*;
use otmp_core_manager::source::bananas::BananasFetcher;
use otmp_core_manager::version::ModType;

fn mod_type_label(mt: &ModType) -> &'static str { match mt { ModType::NewGRF=>"NewGRF", ModType::AI=>"AI", ModType::GameScript=>"GameScript", ModType::MusicSet=>"Music" } }

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
    let filtered: Vec<_> = if query.is_empty() { loaded_mods.clone() } else {
        let q = query.to_lowercase();
        loaded_mods.into_iter().filter(|m| m.name.to_lowercase().contains(&q) || m.author.to_lowercase().contains(&q)).collect()
    };

    rsx! {
        div { style: "max-width: 900px; margin: 0 auto;",
            h1 { style: "font-size: 24px; font-weight: 700; color: #0f172a; margin: 0 0 24px 0;", "Mods" }

            // Search bar
            div { style: "margin-bottom: 16px;",
                input {
                    style: "width: 100%; padding: 10px 14px; border: 1px solid #e2e8f0; border-radius: 8px; font-size: 14px; box-sizing: border-box; outline: none; background: white; color: #0f172a;",
                    placeholder: "Search mods by name or author...",
                    value: "{search_query}",
                    oninput: move |e| search_query.set(e.value()),
                }
            }

            // Type tabs
            div { style: "display: flex; gap: 4px; margin-bottom: 20px; background: #f1f5f9; padding: 4px; border-radius: 8px; width: fit-content;",
                {tabs.into_iter().map(|t| {
                    let is_active = t == current_tab;
                    let mut l = loading; let mut m = mods;
                    let bg = if is_active { "white" } else { "transparent" };
                    let shadow = if is_active { "0 1px 2px rgba(0,0,0,0.05)" } else { "none" };
                    rsx! {
                        button {
                            key: "{mod_type_label(&t)}",
                            style: "background: {bg}; color: #0f172a; border: none; padding: 6px 14px; border-radius: 6px; font-size: 13px; font-weight: 500; cursor: pointer; box-shadow: {shadow};",
                            onclick: move |_| {
                                active_tab.set(t); l.set(true);
                                let fetcher = BananasFetcher::new();
                                spawn(async move {
                                    match fetcher.fetch_mods(t).await {
                                        Ok(items) => { m.set(items); }
                                        Err(e) => { log::error!("Failed to fetch mods: {e}"); }
                                    }
                                    l.set(false);
                                });
                            },
                            "{mod_type_label(&t)}"
                        }
                    }
                })}
            }

            if *loading.read() {
                div { style: "text-align: center; padding: 60px 0; color: #64748b;", div { style: "font-size: 14px;", "Loading mods from BaNaNaS..." } }
            } else if filtered.is_empty() {
                div { style: "background: white; border-radius: 12px; padding: 40px; text-align: center; color: #64748b; box-shadow: 0 1px 3px rgba(0,0,0,0.06);",
                    div { style: "font-size: 14px;", "No mods found. Click a tab above to browse." }
                }
            } else {
                div { style: "display: flex; flex-direction: column; gap: 8px;",
                    {filtered.iter().map(|mod_info| {
                        let name = mod_info.name.clone(); let desc = mod_info.description.clone();
                        let author = mod_info.author.clone(); let version = mod_info.version.clone();
                        let filesize = mod_info.filesize.map(|s| if s > 1024*1024 { format!("{:.1} MB", s as f64 / (1024.0*1024.0)) } else { format!("{:.1} KB", s as f64 / 1024.0) }).unwrap_or_default();
                        rsx! {
                            div {
                                key: "{mod_info.id}", style: "background: white; border-radius: 10px; padding: 16px 20px; display: flex; justify-content: space-between; align-items: center; box-shadow: 0 1px 3px rgba(0,0,0,0.06);",
                                div { style: "flex: 1;",
                                    div { style: "font-weight: 600; font-size: 14px; color: #0f172a;", "{name}" }
                                    div { style: "font-size: 13px; color: #64748b; margin-top: 2px;", "{desc}" }
                                    div { style: "font-size: 12px; color: #94a3b8; margin-top: 4px;", "by {author} · v{version} {filesize}" }
                                }
                                button { style: "background: #0f172a; color: white; border: none; padding: 6px 14px; border-radius: 6px; font-size: 12px; font-weight: 500; cursor: pointer;", "Download" }
                            }
                        }
                    })}
                }
            }
        }
    }
}
