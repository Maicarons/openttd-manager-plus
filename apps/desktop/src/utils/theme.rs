//! Theme management with Dioxus Signal for reactive light/dark switching.
//! Provides CSS variable generation for theming.

use dioxus_native::prelude::*;

/// Available themes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn label(&self) -> &'static str {
        match self {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
        }
    }

    /// Generate CSS variable overrides for the current theme
    pub fn css_vars(&self) -> &'static str {
        match self {
            Theme::Light => "",
            Theme::Dark => r#"
                --bg-primary: #1a1a2e;
                --bg-secondary: #16213e;
                --bg-card: #1e2a45;
                --text-primary: #e0e0e0;
                --text-secondary: #999999;
                --border-color: #2a2a4a;
                --bg-input: #1e2a45;
            "#,
        }
    }
}

/// Reactive theme manager
#[derive(Clone, Copy)]
pub struct ThemeManager {
    pub theme: Signal<Theme>,
}

impl ThemeManager {
    pub fn new() -> Self {
        // Default to light theme
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

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme.set(theme);
    }
}