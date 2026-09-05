//! Simple i18n support (Chinese / English)

/// Supported locales
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Locale {
    ZhCn,
    EnUs,
}

impl Locale {
    /// Get the current locale (defaults to Chinese)
    pub fn current() -> Self {
        Locale::ZhCn
    }

    /// Translate a key to the current locale
    pub fn translate(&self, key: &'static str) -> &'static str {
        match self {
            Locale::ZhCn => self.zh(key),
            Locale::EnUs => self.en(key),
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
            "nav.settings" => "设置",
            "home.welcome" => "欢迎使用 OpenTTD Manager Plus",
            "home.quick_start" => "快速启动",
            "home.no_versions" => "暂未安装任何版本",
            "version.fetching" => "正在获取版本列表...",
            "download.none" => "暂无下载任务",
            "config.none" => "暂无配置方案",
            "mods.browser" => "在线模组浏览器",
            _ => key,
        }
    }

    fn en(&self, key: &'static str) -> &'static str {
        match key {
            "app.title" => "OpenTTD Manager Plus",
            "nav.home" => "Home",
            "nav.versions" => "Versions",
            "nav.downloads" => "Downloads",
            "nav.configs" => "Configs",
            "nav.mods" => "Mods",
            "nav.settings" => "Settings",
            "home.welcome" => "Welcome to OpenTTD Manager Plus",
            "home.quick_start" => "Quick Start",
            "home.no_versions" => "No versions installed yet",
            "version.fetching" => "Fetching version list...",
            "download.none" => "No active downloads",
            "config.none" => "No configuration profiles",
            "mods.browser" => "Online Mod Browser",
            _ => key,
        }
    }
}

/// Quick translation helper
pub fn t(key: &'static str) -> &'static str {
    Locale::current().translate(key)
}