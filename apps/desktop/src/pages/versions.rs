//! Version management page — list, filter, and download OpenTTD versions.

use dioxus_native::prelude::*;
use crate::state::AppState;
use otmp_core_manager::version::VersionType;
use otmp_core_manager::version::VersionSource;

fn version_type_label(vt: &VersionType) -> &'static str { match vt { VersionType::Stable=>"Stable", VersionType::ReleaseCandidate=>"RC", VersionType::Beta=>"Beta", VersionType::Nightly=>"Nightly", VersionType::PreRelease=>"Pre-release", VersionType::Custom=>"Custom" } }
fn version_type_color(vt: &VersionType) -> &'static str { match vt { VersionType::Stable=>"#4a9e4a", VersionType::ReleaseCandidate=>"#e67e22", VersionType::Beta=>"#e74c3c", VersionType::Nightly=>"#9b59b6", VersionType::PreRelease=>"#e74c3c", VersionType::Custom=>"#95a5a6" } }
fn source_color(source: &VersionSource) -> &'static str { match source { VersionSource::Official=>"#3498db", VersionSource::Jgrpp=>"#e67e22", VersionSource::CmClient=>"#1abc9c", VersionSource::Custom(_)=>"#95a5a6" } }
fn source_label(source: &VersionSource) -> String { match source { VersionSource::Official=>"Official".into(), VersionSource::Jgrpp=>"JGRPP".into(), VersionSource::CmClient=>"CMClient".into(), VersionSource::Custom(n)=>format!("Custom/{n}") } }

#[component]
pub fn VersionsPage() -> Element {
    let state = use_context::<AppState>();
    let active_filter = state.active_filter.read().clone();
    let loading = *state.loading.read();
    let error = state.error.read().clone();
    let filtered = state.filtered_versions.read().clone();
    let filters = vec!["All", "Official", "JGRPP", "CMClient"];

    let filter_sig = state.active_filter;
    let loading_sig = state.loading;
    let error_sig = state.error;
    let versions_sig = state.versions;
    let filtered_sig = state.filtered_versions;

    use_effect(move || {
        if versions_sig.read().is_empty() {
            let mut l = loading_sig; let mut e = error_sig; let mut v = versions_sig; let mut f = filtered_sig;
            spawn(async move {
                l.set(true); e.set(None);
                use otmp_core_manager::version::VersionInfo;
                let placeholder = vec![
                    VersionInfo { id: uuid::Uuid::new_v4(), source: VersionSource::Official, version: semver::Version::parse("14.1.0").unwrap(), version_type: VersionType::Stable, name: "OpenTTD".into(), release_date: None, downloads: vec![], changelog: None, is_prerelease: false },
                    VersionInfo { id: uuid::Uuid::new_v4(), source: VersionSource::Official, version: semver::Version::parse("14.0.0").unwrap(), version_type: VersionType::Stable, name: "OpenTTD".into(), release_date: None, downloads: vec![], changelog: None, is_prerelease: false },
                    VersionInfo { id: uuid::Uuid::new_v4(), source: VersionSource::Jgrpp, version: semver::Version::parse("0.59.1").unwrap(), version_type: VersionType::Stable, name: "JGRPP".into(), release_date: None, downloads: vec![], changelog: None, is_prerelease: false },
                    VersionInfo { id: uuid::Uuid::new_v4(), source: VersionSource::CmClient, version: semver::Version::parse("1.0.0").unwrap(), version_type: VersionType::Stable, name: "CMClient".into(), release_date: None, downloads: vec![], changelog: None, is_prerelease: false },
                ];
                v.set(placeholder);
                let all = v.read().clone(); f.set(all);
                l.set(false);
            });
        }
    });

    rsx! {
        div { style: "max-width: 900px; margin: 0 auto;",
            h2 { style: "color: #2c3e50;", "Version Management" }
            {error.as_ref().map(|msg| rsx! { div { style: "background: #f8d7da; color: #721c24; border: 1px solid #f5c6cb; border-radius: 8px; padding: 12px 16px; margin: 12px 0; font-size: 13px;", "⚠ {msg}" } })}
            div { style: "display: flex; gap: 8px; margin: 16px 0; flex-wrap: wrap;",
                {filters.iter().map(|f| {
                    let is_active = *f == active_filter; let bg = if is_active { "#4a9e4a" } else { "#e0e0e0" }; let color = if is_active { "white" } else { "#333" };
                    let f_str = f.to_string(); let f_clone = f_str.clone(); let mut f_sig = filter_sig;
                    rsx! { button { key: "{f_str}", style: "background: {bg}; color: {color}; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-size: 13px;", onclick: move |_| f_sig.set(f_clone.clone()), "{f_str}" } }
                })}
                button { style: "background: #3498db; color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-size: 13px; margin-left: auto;",
                    onclick: move |_| { let mut l = loading_sig; let mut e = error_sig; let mut v = versions_sig; spawn(async move { l.set(true); e.set(None); v.set(Vec::new()); l.set(false); }); }, "Refresh"
                }
            }
            if loading { div { style: "text-align: center; padding: 40px 0; color: #666;", p { "Fetching versions..." } } }
            if filtered.is_empty() && !loading { div { style: "background: white; border-radius: 8px; padding: 40px; box-shadow: 0 1px 3px rgba(0,0,0,0.1); text-align: center; color: #999;", p { "No versions found." } } }
            {filtered.iter().map(|v| {
                let source = v.source.clone(); let vt = v.version_type.clone(); let color = source_color(&source); let label = source_label(&source);
                let version = v.version.to_string(); let name = v.name.clone(); let date_str = v.release_date.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default();
                rsx! {
                    div {
                        key: "{v.id}", style: "background: white; border-radius: 8px; padding: 16px 20px; margin: 8px 0; display: flex; align-items: center; justify-content: space-between; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                        div {
                            div { style: "font-weight: 600; font-size: 15px;", "{name} " span { style: "color: #666; font-weight: 400;", "v{version}" } }
                            div { style: "margin-top: 4px; display: flex; gap: 4px; flex-wrap: wrap;",
                                span { style: "background: {color}; color: white; padding: 2px 8px; border-radius: 4px; font-size: 11px;", "{label}" }
                                span { style: "background: {version_type_color(&vt)}; color: white; padding: 2px 8px; border-radius: 4px; font-size: 11px;", "{version_type_label(&vt)}" }
                                if !date_str.is_empty() { span { style: "color: #999; font-size: 11px;", "{date_str}" } }
                            }
                        }
                        div { button { style: "background: #4a9e4a; color: white; border: none; padding: 6px 14px; border-radius: 4px; cursor: pointer; font-size: 12px;", "Install" } button { style: "margin-left: 4px; background: #ecf0f1; color: #333; border: none; padding: 6px 14px; border-radius: 4px; cursor: pointer; font-size: 12px;", "Details" } }
                    }
                }
            })}
        }
    }
}
