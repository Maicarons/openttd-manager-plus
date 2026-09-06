//! OpenTTD Manager Plus - Desktop Application
//!
//! Cross-platform desktop launcher for Windows, Linux, and macOS.
//! Built with Dioxus Native — Blitz/WGPU renderer (no WebView).

mod app;
mod components;
mod pages;
mod state;
mod utils;

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("Failed to initialize logger");
    log::info!("Starting OpenTTD Manager Plus Desktop (Native Renderer)");

    use dioxus_native::Config;
    use dioxus_native::WindowAttributes;
    let cfg = Config::new()
        .with_window_attributes(
            WindowAttributes::default()
                .with_title("OpenTTD Manager Plus")
                .with_inner_size(dioxus_native::LogicalSize::new(1200.0, 800.0))
        );

    dioxus_native::launch_cfg(app::App, Vec::new(), vec![Box::new(cfg)]);
}