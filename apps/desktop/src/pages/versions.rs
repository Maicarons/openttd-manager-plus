//! Versions page — list, filter, and manage OpenTTD versions.

use dioxus_native::prelude::*;
use crate::state::AppState;
use otmp_core_manager::version::VersionType;
use otmp_core_manager::version::VersionSource;

fn source_label(s: &VersionSource) -> &'static str { match s { VersionSource::Official=>"Official", VersionSource::Jgrpp=>"JGRPP", VersionSource::CmClient=>"CMClient", VersionSource::Custom(_)=>"Custom" } }
fn source_badge_color(s: &VersionSource) -> &'static str { match s { VersionSource::Official=>"#3b82f6", VersionSource::Jgrpp=>"#f59e0b", VersionSource::CmClient=>"#8b5cf6", VersionSource::Custom(_)=>"#64748b" } }

#[component]
pub fn VersionsPage() -> Element {
    let state = use_context::<AppState>();
    let active_filter = state.active_filter.read().clone();
    let loading = *state.loading.read();
    let error = state.error.read().clone();
    let filtered = state.filtered_versions.read().clone();
    let filters = vec!["All", "Official", "JGRPP", "CMClient"];

    let filter_sig = state.active_filter; let loading_sig = state.loading; let error_sig = state.error;
    let versions_sig = state.versions; let filtered_sig = state.filtered_versions;

    use_effect(move || {
        if versions_sig.read().is_empty() {
            let mut l = loading_sig; let mut e = error_sig; let mut v = versions_sig; let mut f = filtered_sig;
            spawn(async move {
                l.set(true); e.set(None);
                use otmp_core_manager::version::VersionInfo;
                let placeholder = vec![
                    VersionInfo{id: uuid::Uuid::new_v4(), source: VersionSource::Official, version: semver::Version::parse("14.1.0").unwrap(), version_type: VersionType::Stable, name: "OpenTTD".into(), release_date: None, downloads: vec![], changelog: None, is_prerelease: false},
                    VersionInfo{id: uuid::Uuid::new_v4(), source: VersionSource::Official, version: semver::Version::parse("14.0.0").unwrap(), version_type: VersionType::Stable, name: "OpenTTD".into(), release_date: None, downloads: vec![], changelog: None, is_prerelease: false},
                    VersionInfo{id: uuid::Uuid::new_v4(), source: VersionSource::Jgrpp, version: semver::Version::parse("0.59.1").unwrap(), version_type: VersionType::Stable, name: "JGRPP".into(), release_date: None, downloads: vec![], changelog: None, is_prerelease: false},
                    VersionInfo{id: uuid::Uuid::new_v4(), source: VersionSource::CmClient, version: semver::Version::parse("1.0.0").unwrap(), version_type: VersionType::Stable, name: "CMClient".into(), release_date: None, downloads: vec![], changelog: None, is_prerelease: false},
                ];
                v.set(placeholder); let all = v.read().clone(); f.set(all);
                l.set(false);
            });
        }
    });

    rsx! {
        div { style: "max-width: 900px; margin: 0 auto;",
            div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px;",
                h1 { style: "font-size: 24px; font-weight: 700; color: #0f172a; margin: 0;", "Versions" }
                button { style: "background: #0f172a; color: white; border: none; padding: 8px 16px; border-radius: 8px; font-size: 13px; font-weight: 500; cursor: pointer; display: flex; align-items: center; gap: 6px;",
                    onclick: move |_| { let mut l = loading_sig; let mut e = error_sig; let mut v = versions_sig; spawn(async move { l.set(true); e.set(None); v.set(Vec::new()); l.set(false); }); },
                    svg { width: "16", height: "16", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", path { d: "M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" } }
                    "Refresh"
                }
            }

            {error.as_ref().map(|msg| rsx! { div { style: "background: #fef2f2; color: #991b1b; border: 1px solid #fecaca; border-radius: 8px; padding: 12px 16px; margin-bottom: 16px; font-size: 13px;", "⚠ {msg}" } })}

            // Filter tabs
            div { style: "display: flex; gap: 4px; margin-bottom: 20px; background: #f1f5f9; padding: 4px; border-radius: 8px; width: fit-content;",
                {filters.iter().map(|f| {
                    let is_active = *f == active_filter;
                    let bg = if is_active { "white" } else { "transparent" };
                    let shadow = if is_active { "0 1px 2px rgba(0,0,0,0.05)" } else { "none" };
                    let f_str = f.to_string(); let f_clone = f_str.clone(); let mut f_sig = filter_sig;
                    rsx! {
                        button {
                            key: "{f_str}",
                            style: "background: {bg}; color: #0f172a; border: none; padding: 6px 14px; border-radius: 6px; font-size: 13px; font-weight: 500; cursor: pointer; box-shadow: {shadow}; transition: background 0.15s;",
                            onclick: move |_| f_sig.set(f_clone.clone()),
                            "{f_str}"
                        }
                    }
                })}
            }

            if loading {
                div { style: "text-align: center; padding: 60px 0; color: #64748b;", div { style: "font-size: 14px;", "Loading versions..." } }
            } else if filtered.is_empty() {
                div { style: "text-align: center; padding: 60px 0; color: #64748b;", div { style: "font-size: 14px;", "No versions found." } }
            } else {
                div { style: "display: flex; flex-direction: column; gap: 8px;",
                    {filtered.iter().map(|v| {
                        let source = v.source.clone(); let color = source_badge_color(&source); let label = source_label(&source);
                        let version = v.version.to_string(); let name = v.name.clone();
                        let date_str = v.release_date.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default();
                        rsx! {
                            div {
                                key: "{v.id}",
                                style: "background: white; border-radius: 10px; padding: 16px 20px; display: flex; align-items: center; justify-content: space-between; box-shadow: 0 1px 3px rgba(0,0,0,0.06); transition: box-shadow 0.15s; cursor: pointer;",
                                div {
                                    div { style: "display: flex; align-items: center; gap: 8px;",
                                        span { style: "font-weight: 600; font-size: 14px; color: #0f172a;", "{name}" }
                                        span { style: "font-size: 13px; color: #64748b;", "v{version}" }
                                        span { style: "background: {color}; color: white; padding: 2px 8px; border-radius: 4px; font-size: 11px; font-weight: 500;", "{label}" }
                                        if !date_str.is_empty() { span { style: "font-size: 12px; color: #94a3b8;", "{date_str}" } }
                                    }
                                }
                                div { style: "display: flex; gap: 6px;",
                                    button { style: "background: #0f172a; color: white; border: none; padding: 6px 14px; border-radius: 6px; font-size: 12px; font-weight: 500; cursor: pointer;", "Install" }
                                    button { style: "background: white; color: #0f172a; border: 1px solid #e2e8f0; padding: 6px 14px; border-radius: 6px; font-size: 12px; cursor: pointer;", "Details" }
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}
