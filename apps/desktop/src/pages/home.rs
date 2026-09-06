//! Home page — dashboard with stats cards and quick actions.

use dioxus_native::prelude::*;
use crate::state::AppState;

#[component]
pub fn HomePage() -> Element {
    let state = use_context::<AppState>();
    let versions_len = state.versions.read().len();
    let instances_len = state.instances.read().len();
    let stats = *state.download_stats.read();
    let v = state.versions; let l = state.loading; let e = state.error; let f = state.filtered_versions;

    use_effect(move || {
        if v.read().is_empty() {
            let mut l = l; let mut e = e; let mut v = v; let mut f = f;
            spawn(async move {
                l.set(true); e.set(None);
                use otmp_core_manager::version::VersionInfo;
                use otmp_core_manager::version::VersionSource;
                use otmp_core_manager::version::VersionType;
                let placeholder = vec![
                    VersionInfo{id: uuid::Uuid::new_v4(), source: VersionSource::Official, version: semver::Version::parse("14.1.0").unwrap(), version_type: VersionType::Stable, name: "OpenTTD".into(), release_date: None, downloads: vec![], changelog: None, is_prerelease: false},
                    VersionInfo{id: uuid::Uuid::new_v4(), source: VersionSource::Jgrpp, version: semver::Version::parse("0.59.1").unwrap(), version_type: VersionType::Stable, name: "JGRPP".into(), release_date: None, downloads: vec![], changelog: None, is_prerelease: false},
                ];
                v.set(placeholder); let all = v.read().clone(); f.set(all);
                l.set(false);
            });
        }
    });

    rsx! {
        div { style: "max-width: 900px; margin: 0 auto;",
            h1 { style: "font-size: 24px; font-weight: 700; color: #0f172a; margin: 0 0 4px 0;", "Dashboard" }
            p { style: "font-size: 14px; color: #64748b; margin: 0 0 24px 0;", "Manage OpenTTD versions, configurations, mods, and more." }

            div { style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px; margin-bottom: 24px;",
                StatCard { value: "{versions_len}", label: "Available Versions", color: "#4ade80" }
                StatCard { value: "{instances_len}", label: "Installed", color: "#60a5fa" }
                StatCard { value: "{stats.active}", label: "Active Downloads", color: "#f472b6" }
            }

            div { style: "background: white; border-radius: 12px; padding: 24px; box-shadow: 0 1px 3px rgba(0,0,0,0.06), 0 1px 2px rgba(0,0,0,0.04);",
                h2 { style: "font-size: 16px; font-weight: 600; color: #0f172a; margin: 0 0 16px 0;", "Quick Actions" }
                div { style: "display: flex; gap: 12px; flex-wrap: wrap;",
                    button { style: "background: #0f172a; color: white; border: none; padding: 10px 20px; border-radius: 8px; font-size: 13px; font-weight: 500; cursor: pointer;", "Browse Versions" }
                    button { style: "background: #0f172a; color: white; border: none; padding: 10px 20px; border-radius: 8px; font-size: 13px; font-weight: 500; cursor: pointer;", "Download Mods" }
                    button { style: "background: white; color: #0f172a; border: 1px solid #e2e8f0; padding: 10px 20px; border-radius: 8px; font-size: 13px; font-weight: 500; cursor: pointer;", "Import Save" }
                }
            }
        }
    }
}

#[component]
fn StatCard(value: String, label: String, color: String) -> Element {
    rsx! {
        div { style: "background: white; border-radius: 12px; padding: 20px; box-shadow: 0 1px 3px rgba(0,0,0,0.06), 0 1px 2px rgba(0,0,0,0.04);",
            div { style: "font-size: 32px; font-weight: 700; color: {color}; margin-bottom: 4px;", "{value}" }
            div { style: "font-size: 13px; color: #64748b;", "{label}" }
        }
    }
}
