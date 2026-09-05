//! OpenTTD Manager Plus - Mobile Application
//!
//! Android & iOS mobile launcher for managing OpenTTD APK/IPA versions.
//! Built with Dioxus Native — Blitz/WGPU renderer (no WebView).

use dioxus_native::prelude::*;

fn main() {
    // Initialize logger
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("Failed to initialize logger");
    log::info!("Starting OpenTTD Manager Plus Mobile (Native Renderer)");

    // Launch the Dioxus native app with Blitz/WGPU rendering
    dioxus_native::launch(app);
}

/// Root application component
fn app() -> Element {
    rsx! {
        // Mobile-optimized layout with bottom navigation
        div {
            style: "display: flex; flex-direction: column; height: 100vh;",
            header {
                style: "background: #2c3e50; color: white; padding: 12px 16px; text-align: center;",
                h1 { style: "font-size: 18px; margin: 0;", "OpenTTD Manager Plus" }
            }
            main {
                style: "flex: 1; padding: 16px; overflow-y: auto;",
                h2 { "版本管理" }
                p { "浏览和安装 OpenTTD 各版本" }
            }
            // Bottom navigation bar
            footer {
                style: "display: flex; background: #34495e; color: white;",
                button { style: "flex: 1; padding: 12px; border: none; background: none; color: white;", "🏠 首页" }
                button { style: "flex: 1; padding: 12px; border: none; background: none; color: white;", "📦 版本" }
                button { style: "flex: 1; padding: 12px; border: none; background: none; color: white;", "⬇️ 下载" }
                button { style: "flex: 1; padding: 12px; border: none; background: none; color: white;", "⚙️ 设置" }
            }
        }
    }
}