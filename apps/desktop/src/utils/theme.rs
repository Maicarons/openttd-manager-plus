//! Theme management with Dioxus Signal for reactive light/dark switching.
//! Applies CSS variables to the root element for theming.

use dioxus_native::prelude::*;

/// Available themes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

/// Reactive theme manager
#[derive(Clone, Copy)]
pub struct ThemeManager {
    pub theme: Signal<Theme>,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            theme: Signal::new(Theme::Light),
        }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme.set(theme);
    }

    /// Get CSS variables string for the current theme
    pub fn css_vars(&self) -> &'static str {
        match *self.theme.read() {
            Theme::Light => "",
            Theme::Dark => "--bg-primary: #1a1a2e; --bg-secondary: #16213e; --bg-card: #1e2a45; --text-primary: #e0e0e0; --text-secondary: #999; --border-color: #2a2a4a; --bg-input: #1e2a45; --sidebar-bg: #1a1a2e; --header-bg: #16213e;",
        }
    }
}