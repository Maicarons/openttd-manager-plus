//! Versions page — mobile version list and APK download

use dioxus_native::prelude::*;
use crate::state::{MobileState, ApkManager, ApkVersion};

#[component]
pub fn VersionsPage() -> Element {
    let state = use_context::<MobileState>();
    let apk_manager = use_context_provider(|| ApkManager::new());
    let apk_versions = ApkManager::get_apk_versions();

    rsx! {
        div { style: "padding: 8px;",
            h2 { style: "color: #2c3e50; font-size: 20px; margin: 0 0 16px 0;", "OpenTTD Versions" }

            // APK Downloads section
            div { style: "margin-bottom: 16px;",
                h3 { style: "font-size: 16px; color: #2c3e50; margin: 0 0 8px 0;", "Android APK Downloads" }
                p { style: "font-size: 12px; color: #999; margin: 0 0 12px 0;", "Download and install OpenTTD APK files directly on your device." }

                {apk_versions.into_iter().map(|apk| {
                    let version = apk.version.clone();
                    let url = apk.url.clone();
                    let size_str = apk.size.map(|s| format!("{:.0} MB", s as f64 / 1_000_000.0)).unwrap_or_default();
                    let date = apk.release_date.clone().unwrap_or_default();
                    let is_apk = url.ends_with(".apk");

                    rsx! {
                        div {
                            key: "{version}",
                            style: "background: white; border-radius: 12px; padding: 14px; margin: 8px 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                            div { style: "display: flex; justify-content: space-between; align-items: center;",
                                div {
                                    div { style: "font-weight: 600; font-size: 15px;", "OpenTTD v{version}" }
                                    div { style: "font-size: 12px; color: #999; margin-top: 2px;",
                                        if is_apk { "📱 APK" } else { "📦 AAB" }
                                        " • {size_str} • {date}"
                                    }
                                }
                                div { style: "display: flex; gap: 6px;",
                                    button {
                                        style: "background: #4a9e4a; color: white; border: none; padding: 8px 14px; border-radius: 8px; font-size: 12px; cursor: pointer;",
                                        onclick: move |_| {
                                            log::info!("Downloading APK v{} from {}", version, url);
                                        },
                                        "Download"
                                    }
                                    if is_apk {
                                        button {
                                            style: "background: #3498db; color: white; border: none; padding: 8px 14px; border-radius: 8px; font-size: 12px; cursor: pointer;",
                                            "Install"
                                        }
                                    }
                                }
                            }
                        }
                    }
                })}
            }

            // OS version info
            div { style: "background: #e8f4f8; border-radius: 12px; padding: 14px; margin: 16px 0;",
                div { style: "font-size: 13px; color: #2c3e50;",
                    "💡 APK files can be installed directly. AAB files require additional processing or Google Play."
                }
            }
        }
    }
}