//! Home page — overview and quick start with reactive data

use dioxus_native::prelude::*;
use crate::state::AppState;

#[component]
pub fn HomePage() -> Element {
    let state = use_context::<AppState>();
    let versions_len = state.versions.read().len();
    let instances_len = state.instances.read().len();
    let stats = *state.download_stats.read();

    // Trigger fetch on first render via signals
    let v = state.versions;
    let l = state.loading;
    let e = state.error;
    let f = state.filtered_versions;
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
                v.set(placeholder);
                let all = v.read().clone();
                f.set(all);
                l.set(false);
            });
        }
    });

    rsx! {
        div { style: "max-width: 800px; margin: 0 auto;",
            h2 { style: "color: #2c3e50;", "Welcome to OpenTTD Manager Plus" }
            p { style: "color: #666; line-height: 1.6;", "Manage OpenTTD versions, configurations, mods, and more." }
            div { style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 16px; margin: 16px 0;",
                div { style: "background: white; border-radius: 8px; padding: 20px; text-align: center; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    h3 { style: "margin: 0; font-size: 28px; color: #4a9e4a;", "{versions_len}" }
                    p { style: "color: #999; margin: 4px 0 0;", "Available Versions" }
                }
                div { style: "background: white; border-radius: 8px; padding: 20px; text-align: center; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    h3 { style: "margin: 0; font-size: 28px;", "{instances_len}" }
                    p { style: "color: #999; margin: 4px 0 0;", "Installed" }
                }
                div { style: "background: white; border-radius: 8px; padding: 20px; text-align: center; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                    h3 { style: "margin: 0; font-size: 28px; color: #4a9e4a;", "{stats.active}" }
                    p { style: "color: #999; margin: 4px 0 0;", "Active Downloads" }
                }
            }
            div { style: "background: white; border-radius: 8px; padding: 20px; margin: 16px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                h3 { style: "margin-top: 0;", "Quick Actions" }
                div { style: "display: flex; gap: 8px; flex-wrap: wrap;",
                    button { style: "background: #4a9e4a; color: white; border: none; padding: 10px 20px; border-radius: 4px; cursor: pointer;", "Browse Versions" }
                    button { style: "background: #3498db; color: white; border: none; padding: 10px 20px; border-radius: 4px; cursor: pointer;", "Download Mods" }
                    button { style: "background: #e67e22; color: white; border: none; padding: 10px 20px; border-radius: 4px; cursor: pointer;", "Import Save" }
                }
            }
        }
    }
}
