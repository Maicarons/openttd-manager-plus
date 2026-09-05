//! OpenTTD Manager Plus - Mobile Application
//!
//! Android & iOS mobile launcher for managing OpenTTD versions.
//! Built with Dioxus Native — Blitz/WGPU renderer (no WebView).

mod app;
mod components;
mod pages;
mod state;

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("Failed to initialize logger");
    log::info!("Starting OpenTTD Manager Plus Mobile (Native Renderer)");
    dioxus_native::launch(app::App);
}