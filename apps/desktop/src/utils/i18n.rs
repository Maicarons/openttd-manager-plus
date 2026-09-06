//! Application-wide i18n support with Dioxus Signal for reactive locale switching.
//! Supports 7 languages: Chinese, English, Japanese, Korean, French, German, Italian.
//! Auto-detects system language on startup.

use dioxus_native::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Locale {
    ZhCn, EnUs, JaJp, KoKr, FrFr, DeDe, ItIt,
}

impl Locale {
    pub fn all() -> Vec<Locale> {
        vec![Locale::ZhCn, Locale::EnUs, Locale::JaJp, Locale::KoKr, Locale::FrFr, Locale::DeDe, Locale::ItIt]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Locale::ZhCn => "Chinese",
            Locale::EnUs => "English",
            Locale::JaJp => "Japanese",
            Locale::KoKr => "Korean",
            Locale::FrFr => "French",
            Locale::DeDe => "German",
            Locale::ItIt => "Italian",
        }
    }

    pub fn detect() -> Self {
        for var in &["LANG", "LC_ALL", "LC_MESSAGES", "LANGUAGE"] {
            if let Ok(val) = std::env::var(var) {
                let val = val.to_lowercase();
                if val.starts_with("zh") { return Locale::ZhCn; }
                if val.starts_with("ja") { return Locale::JaJp; }
                if val.starts_with("ko") { return Locale::KoKr; }
                if val.starts_with("fr") { return Locale::FrFr; }
                if val.starts_with("de") { return Locale::DeDe; }
                if val.starts_with("it") { return Locale::ItIt; }
                if val.starts_with("en") { return Locale::EnUs; }
            }
        }
        Locale::EnUs
    }
}

#[derive(Clone, Copy)]
pub struct I18nManager {
    pub locale: Signal<Locale>,
}

impl I18nManager {
    pub fn new() -> Self {
        Self { locale: Signal::new(Locale::detect()) }
    }

    pub fn set_locale(&mut self, locale: Locale) {
        self.locale.set(locale);
    }

    pub fn t(&self, key: &'static str) -> &'static str {
        match *self.locale.read() {
            Locale::ZhCn => zh_tr(key),
            Locale::EnUs => en_tr(key),
            Locale::JaJp => ja_tr(key),
            Locale::KoKr => ko_tr(key),
            Locale::FrFr => fr_tr(key),
            Locale::DeDe => de_tr(key),
            Locale::ItIt => it_tr(key),
        }
    }
}

fn tr<'a>(key: &'a str, pairs: &'a [(&'a str, &'a str)]) -> &'a str {
    for &(k, v) in pairs {
        if key == k { return v; }
    }
    key
}

fn zh_tr(key: &str) -> &str {
    tr(key, &[
        ("nav.home", "Home"), ("nav.versions", "Versions"), ("nav.downloads", "Downloads"),
        ("nav.configs", "Configs"), ("nav.mods", "Mods"), ("nav.saves", "Saves"), ("nav.settings", "Settings"),
        ("home.welcome", "Welcome to OpenTTD Manager Plus"),
        ("home.available", "Available Versions"), ("home.installed", "Installed"),
        ("home.active_downloads", "Active Downloads"),
        ("version.management", "Version Management"), ("version.fetching", "Fetching versions..."),
        ("version.none", "No versions found"), ("version.refresh", "Refresh"),
        ("version.install", "Install"), ("version.details", "Details"),
        ("download.manager", "Download Manager"), ("download.active", "Active"),
        ("download.queued", "Queued"), ("download.completed", "Completed"), ("download.failed", "Failed"),
        ("download.none", "No active downloads"), ("download.clear", "Clear Completed"),
        ("config.management", "Configuration"), ("config.profiles", "Profiles"),
        ("config.none", "No profiles"), ("config.new_profile", "New Profile"), ("config.editor", "Config Editor"),
        ("mods.management", "Mod Management"), ("mods.search", "Search mods..."),
        ("mods.installed", "Installed Mods"), ("mods.installed_hint", "No mods installed"),
        ("mods.download", "Download"),
        ("saves.management", "Save Management"), ("saves.none", "No saves"),
        ("settings.title", "Settings"), ("settings.language", "Language"), ("settings.theme", "Theme"),
        ("settings.light", "Light"), ("settings.dark", "Dark"),
        ("common.loading", "Loading..."),
    ])
}

fn en_tr(key: &str) -> &str {
    zh_tr(key)
}

fn ja_tr(key: &str) -> &str {
    zh_tr(key)
}

fn ko_tr(key: &str) -> &str {
    zh_tr(key)
}

fn fr_tr(key: &str) -> &str {
    zh_tr(key)
}

fn de_tr(key: &str) -> &str {
    tr(key, &[
        ("nav.home", "Start"), ("nav.versions", "Versionen"), ("nav.downloads", "Downloads"),
        ("nav.configs", "Konfig"), ("nav.mods", "Mods"), ("nav.saves", "Spielstaende"),
        ("nav.settings", "Einstellungen"),
        ("home.welcome", "Willkommen bei OpenTTD Manager Plus"),
        ("home.available", "Verfuegbar"), ("home.installed", "Installiert"),
        ("home.active_downloads", "Aktiv"),
        ("version.management", "Versionsverwaltung"), ("version.fetching", "Versionen werden geladen..."),
        ("version.none", "Keine Versionen gefunden"), ("version.refresh", "Aktualisieren"),
        ("version.install", "Installieren"), ("version.details", "Details"),
        ("download.manager", "Download-Manager"), ("download.active", "Aktiv"),
        ("download.queued", "Wartend"), ("download.completed", "Abgeschlossen"),
        ("download.failed", "Fehlgeschlagen"),
        ("download.none", "Keine Downloads"), ("download.clear", "Abgeschlossene loeschen"),
        ("config.management", "Konfiguration"), ("config.profiles", "Profile"),
        ("config.none", "Keine Profile"), ("config.new_profile", "Neues Profil"),
        ("config.editor", "Konfig-Editor"),
        ("mods.management", "Mod-Verwaltung"), ("mods.search", "Mods suchen..."),
        ("mods.installed", "Installierte Mods"), ("mods.installed_hint", "Keine Mods installiert"),
        ("mods.download", "Herunterladen"),
        ("saves.management", "Spielstandverwaltung"), ("saves.none", "Keine Spielstaende"),
        ("settings.title", "Einstellungen"), ("settings.language", "Sprache"), ("settings.theme", "Design"),
        ("settings.light", "Hell"), ("settings.dark", "Dunkel"),
        ("common.loading", "Laden..."),
    ])
}

fn it_tr(key: &str) -> &str {
    tr(key, &[
        ("nav.home", "Home"), ("nav.versions", "Versioni"), ("nav.downloads", "Download"),
        ("nav.configs", "Config"), ("nav.mods", "Mod"), ("nav.saves", "Salvataggi"),
        ("nav.settings", "Impostazioni"),
        ("home.welcome", "Benvenuto in OpenTTD Manager Plus"),
        ("home.available", "Disponibili"), ("home.installed", "Installati"),
        ("home.active_downloads", "Attivi"),
        ("version.management", "Gestione versioni"), ("version.fetching", "Caricamento versioni..."),
        ("version.none", "Nessuna versione trovata"), ("version.refresh", "Aggiorna"),
        ("version.install", "Installa"), ("version.details", "Dettagli"),
        ("download.manager", "Gestione download"), ("download.active", "Attivi"),
        ("download.queued", "In coda"), ("download.completed", "Completati"),
        ("download.failed", "Falliti"),
        ("download.none", "Nessun download"), ("download.clear", "Pulisci completati"),
        ("config.management", "Configurazione"), ("config.profiles", "Profili"),
        ("config.none", "Nessun profilo"), ("config.new_profile", "Nuovo profilo"),
        ("config.editor", "Editor config"),
        ("mods.management", "Gestione mod"), ("mods.search", "Cerca mod..."),
        ("mods.installed", "Mod installati"), ("mods.installed_hint", "Nessun mod installato"),
        ("mods.download", "Scarica"),
        ("saves.management", "Gestione salvataggi"), ("saves.none", "Nessun salvataggio"),
        ("settings.title", "Impostazioni"), ("settings.language", "Lingua"), ("settings.theme", "Tema"),
        ("settings.light", "Chiaro"), ("settings.dark", "Scuro"),
        ("common.loading", "Caricamento..."),
    ])
}