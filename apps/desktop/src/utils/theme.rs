//! Theme management (light/dark)

use dioxus_native::prelude::*;

/// Available themes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

/// Manages application theme state
pub struct ThemeManager {
    pub theme: Signal<Theme>,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            theme: Signal::new(Theme::Light),
        }
    }

    pub fn toggle(&mut self) {
        let current = *self.theme.read();
        self.theme.set(match current {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        });
    }
}