//! Application-wide i18n support with Dioxus Signal for reactive locale switching.
//! Supports 7 languages: Chinese, English, Japanese, Korean, French, German, Italian.
//! Auto-detects system language on startup.

use dioxus_native::prelude::*;

/// Supported locales
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Locale {
    ZhCn,  // 简体中文
    EnUs,  // English
    JaJp,  // 日本語
    KoKr,  // 한국어
    FrFr,  // Français
    DeDe,  // Deutsch
    ItIt,  // Italiano
}

impl Locale {
    /// All supported locales for iteration
    pub fn all() -> Vec<Locale> {
        vec![Locale::ZhCn, Locale::EnUs, Locale::JaJp, Locale::KoKr, Locale::FrFr, Locale::DeDe, Locale::ItIt]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Locale::ZhCn => "简体中文",
            Locale::EnUs => "English",
            Locale::JaJp => "日本語",
            Locale::KoKr => "한국어",
            Locale::FrFr => "Français",
            Locale::DeDe => "Deutsch",
            Locale::ItIt => "Italiano",
        }
    }

    /// Detect system language from environment variables
    pub fn detect() -> Self {
        // Try common environment variables
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
        // On Windows, try PowerShell
        #[cfg(target_os = "windows")]
        return Locale::ZhCn;
        #[cfg(not(target_os = "windows"))]
        Locale::EnUs
    }
}

/// Reactive i18n manager
#[derive(Clone, Copy)]
pub struct I18nManager {
    pub locale: Signal<Locale>,
}

impl I18nManager {
    pub fn new() -> Self {
        Self {
            locale: Signal::new(Locale::detect()),
        }
    }

    pub fn set_locale(&mut self, locale: Locale) {
        self.locale.set(locale);
    }

    pub fn t(&self, key: &'static str) -> &'static str {
        match *self.locale.read() {
            Locale::ZhCn => self.zh(key),
            Locale::EnUs => self.en(key),
            Locale::JaJp => self.ja(key),
            Locale::KoKr => self.ko(key),
            Locale::FrFr => self.fr(key),
            Locale::DeDe => self.de(key),
            Locale::ItIt => self.it(key),
        }
    }

    fn zh(&self, key: &'static str) -> &'static str {
        match key {
            "app.title" => "OpenTTD Manager Plus",
            "nav.home" => "首页",
            "nav.versions" => "版本管理",
            "nav.downloads" => "下载管理",
            "nav.configs" => "配置管理",
            "nav.mods" => "模组管理",
            "nav.saves" => "存档管理",
            "nav.settings" => "设置",
            "home.welcome" => "欢迎使用 OpenTTD Manager Plus",
            "home.available" => "可用版本",
            "home.installed" => "已安装",
            "home.active_downloads" => "活跃下载",
            "version.management" => "版本管理",
            "version.fetching" => "正在获取版本列表...",
            "version.none" => "暂无版本",
            "version.refresh" => "刷新",
            "version.install" => "安装",
            "version.details" => "详情",
            "download.manager" => "下载管理",
            "download.active" => "活跃下载",
            "download.queued" => "排队中",
            "download.completed" => "已完成",
            "download.failed" => "失败",
            "download.none" => "暂无下载任务",
            "download.clear" => "清除已完成",
            "config.management" => "配置管理",
            "config.profiles" => "配置方案",
            "config.none" => "暂无配置方案",
            "config.new_profile" => "新建方案",
            "config.editor" => "openttd.cfg 编辑器",
            "mods.management" => "模组管理",
            "mods.search" => "搜索模组...",
            "mods.browser" => "在线模组浏览器",
            "mods.installed" => "已安装模组",
            "mods.installed_hint" => "暂无已安装模组",
            "mods.download" => "下载",
            "saves.management" => "存档管理",
            "saves.none" => "暂无存档",
            "settings.title" => "设置",
            "settings.language" => "语言",
            "settings.theme" => "主题",
            "settings.light" => "浅色",
            "settings.dark" => "深色",
            "common.loading" => "加载中...",
            _ => key,
        }
    }

    fn en(&self, key: &'static str) -> &'static str {
        match key {
            "app.title" => "OpenTTD Manager Plus",
            "nav.home" => "Home", "nav.versions" => "Versions", "nav.downloads" => "Downloads",
            "nav.configs" => "Configs", "nav.mods" => "Mods", "nav.saves" => "Saves", "nav.settings" => "Settings",
            "home.welcome" => "Welcome to OpenTTD Manager Plus",
            "home.available" => "Available Versions", "home.installed" => "Installed", "home.active_downloads" => "Active Downloads",
            "version.management" => "Version Management", "version.fetching" => "Fetching versions...",
            "version.none" => "No versions found", "version.refresh" => "Refresh",
            "version.install" => "Install", "version.details" => "Details",
            "download.manager" => "Download Manager", "download.active" => "Active",
            "download.queued" => "Queued", "download.completed" => "Completed", "download.failed" => "Failed",
            "download.none" => "No active downloads", "download.clear" => "Clear Completed",
            "config.management" => "Configuration", "config.profiles" => "Profiles",
            "config.none" => "No profiles", "config.new_profile" => "New Profile", "config.editor" => "Config Editor",
            "mods.management" => "Mod Management", "mods.search" => "Search mods...",
            "mods.browser" => "Online Mod Browser", "mods.installed" => "Installed Mods",
            "mods.installed_hint" => "No mods installed", "mods.download" => "Download",
            "saves.management" => "Save Management", "saves.none" => "No saves",
            "settings.title" => "Settings", "settings.language" => "Language", "settings.theme" => "Theme",
            "settings.light" => "Light", "settings.dark" => "Dark",
            "common.loading" => "Loading...",
            _ => key,
        }
    }

    fn ja(&self, key: &'static str) -> &'static str {
        match key {
            "app.title" => "OpenTTD Manager Plus",
            "nav.home" => "ホーム", "nav.versions" => "バージョン", "nav.downloads" => "ダウンロード",
            "nav.configs" => "設定", "nav.mods" => "Mod", "nav.saves" => "セーブ", "nav.settings" => "設定",
            "home.welcome" => "OpenTTD Manager Plus へようこそ",
            "home.available" => "利用可能", "home.installed" => "インストール済み", "home.active_downloads" => "アクティブ",
            "version.management" => "バージョン管理", "version.fetching" => "バージョン一覧を取得中...",
            "version.none" => "バージョンが見つかりません", "version.refresh" => "更新",
            "version.install" => "インストール", "version.details" => "詳細",
            "download.manager" => "ダウンロード管理", "download.active" => "アクティブ",
            "download.queued" => "待機中", "download.completed" => "完了", "download.failed" => "失敗",
            "download.none" => "ダウンロードなし", "download.clear" => "完了をクリア",
            "config.management" => "設定管理", "config.profiles" => "プロファイル",
            "config.none" => "プロファイルなし", "config.new_profile" => "新規プロファイル", "config.editor" => "設定エディタ",
            "mods.management" => "Mod管理", "mods.search" => "Modを検索...",
            "mods.installed" => "インストール済みMod", "mods.installed_hint" => "Modはありません",
            "mods.download" => "ダウンロード",
            "saves.management" => "セーブ管理", "saves.none" => "セーブはありません",
            "settings.title" => "設定", "settings.language" => "言語", "settings.theme" => "テーマ",
            "settings.light" => "ライト", "settings.dark" => "ダーク",
            "common.loading" => "読み込み中...",
            _ => key,
        }
    }

    fn ko(&self, key: &'static str) -> &'static str {
        match key {
            "app.title" => "OpenTTD Manager Plus",
            "nav.home" => "홈", "nav.versions" => "버전", "nav.downloads" => "다운로드",
            "nav.configs" => "구성", "nav.mods" => "모드", "nav.saves" => "저장", "nav.settings" => "설정",
            "home.welcome" => "OpenTTD Manager Plus에 오신 것을 환영합니다",
            "home.available" => "사용 가능", "home.installed" => "설치됨", "home.active_downloads" => "활성",
            "version.management" => "버전 관리", "version.fetching" => "버전 목록 가져오는 중...",
            "version.none" => "버전을 찾을 수 없음", "version.refresh" => "새로고침",
            "version.install" => "설치", "version.details" => "상세",
            "download.manager" => "다운로드 관리자", "download.active" => "활성",
            "download.queued" => "대기 중", "download.completed" => "완료", "download.failed" => "실패",
            "download.none" => "다운로드 없음", "download.clear" => "완료 지우기",
            "config.management" => "구성 관리", "config.profiles" => "프로필",
            "config.none" => "프로필 없음", "config.new_profile" => "새 프로필", "config.editor" => "구성 편집기",
            "mods.management" => "모드 관리", "mods.search" => "모드 검색...",
            "mods.installed" => "설치된 모드", "mods.installed_hint" => "설치된 모드 없음",
            "mods.download" => "다운로드",
            "saves.management" => "저장 관리", "saves.none" => "저장 없음",
            "settings.title" => "설정", "settings.language" => "언어", "settings.theme" => "테마",
            "settings.light" => "라이트", "settings.dark" => "다크",
            "common.loading" => "로딩 중...",
            _ => key,
        }
    }

    fn fr(&self, key: &'static str) -> &'static str {
        match key {
            "app.title" => "OpenTTD Manager Plus",
            "nav.home" => "Accueil", "nav.versions" => "Versions", "nav.downloads" => "Téléchargements",
            "nav.configs" => "Config", "nav.mods" => "Mods", "nav.saves" => "Sauvegardes", "nav.settings" => "Paramètres",
            "home.welcome" => "Bienvenue sur OpenTTD Manager Plus",
            "home.available" => "Disponibles", "home.installed" => "Installés", "home.active_downloads" => "Actifs",
            "version.management" => "Gestion des versions", "version.fetching" => "Chargement des versions...",
            "version.none" => "Aucune version trouvée", "version.refresh" => "Actualiser",
            "version.install" => "Installer", "version.details" => "Détails",
            "download.manager" => "Gestionnaire de téléchargement", "download.active" => "Actifs",
            "download.queued" => "En attente", "download.completed" => "Terminés", "download.failed" => "Échoués",
            "download.none" => "Aucun téléchargement", "download.clear" => "Effacer terminés",
            "config.management" => "Configuration", "config.profiles" => "Profils",
            "config.none" => "Aucun profil", "config.new_profile" => "Nouveau profil", "config.editor" => "Éditeur de config",
            "mods.management" => "Gestion des mods", "mods.search" => "Rechercher des mods...",
            "mods.installed" => "Mods installés", "mods.installed_hint" => "Aucun mod installé",
            "mods.download" => "Télécharger",
            "saves.management" => "Gestion des sauvegardes", "saves.none" => "Aucune sauvegarde",
            "settings.title" => "Paramètres", "settings.language" => "Langue", "settings.theme" => "Thème",
            "settings.light" => "Clair", "settings.dark" => "Sombre",
            "common.loading" => "Chargement...",
            _ => key,
        }
    }

    fn de(&self, key: &'static str) -> &'static str {
        match key {
            "app.title" => "OpenTTD Manager Plus",
            "nav.home" => "Start", "nav.versions" => "Versionen", "nav.downloads" => "Downloads",
            "nav.configs" => "Konfig", "nav.mods" => "Mods", "nav.saves" => "Spielstände", "nav.settings" => "Einstellungen",
            "home.welcome" => "Willkommen bei OpenTTD Manager Plus",
            "home.available" => "Verfügbar", "home.installed" => "Installiert", "home.active_downloads" => "Aktiv",
            "version.management" => "Versionsverwaltung", "version.fetching" => "Versionen werden geladen...",
            "version.none" => "Keine Versionen gefunden", "version.refresh" => "Aktualisieren",
            "version.install" => "Installieren", "version.details" => "Details",
            "download.manager" => "Download-Manager", "download.active" => "Aktiv",
            "download.queued" => "Wartend", "download.completed" => "Abgeschlossen", "download.failed" => "Fehlgeschlagen",
            "download.none" => "Keine Downloads", "download.clear" => "Abgeschlossene löschen",
            "config.management" => "Konfiguration", "config.profiles" => "Profile",
            "config.none" => "Keine Profile", "config.new_profile" => "Neues Profil", "config.editor" => "Konfig-Editor",
            "mods.management" => "Mod-Verwaltung", "mods.search" => "Mods suchen...",
            "mods.installed" => "Installierte Mods", "mods.installed_hint" => "Keine Mods installiert",
            "mods.download" => "Herunterladen",
            "saves.management" => "Spielstandverwaltung", "saves.none" => "Keine Spielstände",
            "settings.title" => "Einstellungen", "settings.language" => "Sprache", "settings.theme" => "Design",
            "settings.light" => "Hell", "settings.dark" => "Dunkel",
            "common.loading" => "Laden...",
            _ => key,
        }
    }

    fn it(&self, key: &'static str) -> &'static str {
        match key {
            "app.title" => "OpenTTD Manager Plus",
            "nav.home" => "Home", "nav.versions" => "Versioni", "nav.downloads" => "Download",
            "nav.configs" => "Config", "nav.mods" => "Mod", "nav.saves" => "Salvataggi", "nav.settings" => "Impostazioni",
            "home.welcome" => "Benvenuto in OpenTTD Manager Plus",
            "home.available" => "Disponibili", "home.installed" => "Installati", "home.active_downloads" => "Attivi",
            "version.management" => "Gestione versioni", "version.fetching" => "Caricamento versioni...",
            "version.none" => "Nessuna versione trovata", "version.refresh" => "Aggiorna",
            "version.install" => "Installa", "version.details" => "Dettagli",
            "download.manager" => "Gestione download", "download.active" => "Attivi",
            "download.queued" => "In coda", "download.completed" => "Completati", "download.failed" => "Falliti",
            "download.none" => "Nessun download", "download.clear" => "Pulisci completati",
            "config.management" => "Configurazione", "config.profiles" => "Profili",
            "config.none" => "Nessun profilo", "config.new_profile" => "Nuovo profilo", "config.editor" => "Editor config",
            "mods.management" => "Gestione mod", "mods.search" => "Cerca mod...",
            "mods.installed" => "Mod installati", "mods.installed_hint" => "Nessun mod installato",
            "mods.download" => "Scarica",
            "saves.management" => "Gestione salvataggi", "saves.none" => "Nessun salvataggio",
            "settings.title" => "Impostazioni", "settings.language" => "Lingua", "settings.theme" => "Tema",
            "settings.light" => "Chiaro", "settings.dark" => "Scuro",
            "common.loading" => "Caricamento...",
            _ => key,
        }
    }
}