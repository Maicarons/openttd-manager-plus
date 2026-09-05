//! Application-wide i18n support with Dioxus Signal for reactive locale switching.
//! Supports Chinese (Simplified) and English.

use dioxus_native::prelude::*;

/// Supported locales
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Locale {
    ZhCn,
    EnUs,
}

impl Locale {
    pub fn label(&self) -> &'static str {
        match self {
            Locale::ZhCn => "简体中文",
            Locale::EnUs => "English",
        }
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
            locale: Signal::new(Locale::ZhCn),
        }
    }

    pub fn set_locale(&mut self, locale: Locale) {
        self.locale.set(locale);
    }

    pub fn t(&self, key: &'static str) -> &'static str {
        match *self.locale.read() {
            Locale::ZhCn => self.zh(key),
            Locale::EnUs => self.en(key),
        }
    }

    fn zh(&self, key: &'static str) -> &'static str {
        match key {
            // App
            "app.title" => "OpenTTD Manager Plus",
            "app.version" => "版本",
            "app.license" => "许可证",
            "app.renderer" => "渲染器",

            // Nav
            "nav.home" => "首页",
            "nav.versions" => "版本管理",
            "nav.downloads" => "下载管理",
            "nav.configs" => "配置管理",
            "nav.mods" => "模组管理",
            "nav.settings" => "设置",

            // Home
            "home.welcome" => "欢迎使用 OpenTTD Manager Plus",
            "home.desc" => "管理 OpenTTD 版本、配置、模组，一站式搞定。",
            "home.quick_start" => "快速启动",
            "home.no_versions" => "暂未安装任何版本，请先浏览版本。",
            "home.available" => "可用版本",
            "home.installed" => "已安装",
            "home.active_downloads" => "活跃下载",

            // Versions
            "version.management" => "版本管理",
            "version.fetching" => "正在获取版本列表...",
            "version.none" => "暂无版本。点击刷新获取最新版本。",
            "version.refresh" => "刷新",
            "version.install" => "安装",
            "version.details" => "详情",
            "version.filter_all" => "全部",
            "version.custom_import" => "导入自定义版本",
            "version.import_url" => "从 URL 导入",
            "version.import_file" => "从文件导入",
            "version.import_install" => "从现有安装导入",

            // Downloads
            "download.manager" => "下载管理",
            "download.active" => "活跃下载",
            "download.queued" => "排队中",
            "download.completed" => "已完成",
            "download.failed" => "失败",
            "download.none" => "暂无下载任务。",
            "download.clear" => "清除已完成",

            // Configs
            "config.management" => "配置管理",
            "config.profiles" => "配置方案",
            "config.none" => "暂无配置方案。",
            "config.new_profile" => "新建方案",
            "config.editor" => "openttd.cfg 编辑器",
            "config.editor_hint" => "选择一个方案编辑其配置文件。",
            "config.default" => "默认",
            "config.edit" => "编辑",
            "config.delete" => "删除",

            // Mods
            "mods.management" => "模组管理",
            "mods.search" => "按名称或作者搜索模组...",
            "mods.browser" => "在线模组浏览器",
            "mods.browser_hint" => "从 BaNaNaS 浏览和下载模组。",
            "mods.installed" => "已安装模组",
            "mods.installed_hint" => "暂无已安装模组。",
            "mods.download" => "下载",

            // Settings
            "settings.title" => "设置",
            "settings.general" => "通用",
            "settings.language" => "语言",
            "settings.theme" => "主题",
            "settings.light" => "浅色",
            "settings.dark" => "深色",
            "settings.download" => "下载",
            "settings.mirror" => "镜像源",
            "settings.max_concurrent" => "最大并发下载数",
            "settings.about" => "关于",

            // Common
            "common.loading" => "加载中...",
            "common.error" => "错误",
            "common.success" => "成功",
            "common.cancel" => "取消",
            "common.confirm" => "确认",
            "common.save" => "保存",
            "common.close" => "关闭",

            _ => key,
        }
    }

    fn en(&self, key: &'static str) -> &'static str {
        match key {
            "app.title" => "OpenTTD Manager Plus",
            "app.version" => "Version",
            "app.license" => "License",
            "app.renderer" => "Renderer",

            "nav.home" => "Home",
            "nav.versions" => "Versions",
            "nav.downloads" => "Downloads",
            "nav.configs" => "Configs",
            "nav.mods" => "Mods",
            "nav.settings" => "Settings",

            "home.welcome" => "Welcome to OpenTTD Manager Plus",
            "home.desc" => "Manage OpenTTD versions, configs, and mods — all in one place.",
            "home.quick_start" => "Quick Start",
            "home.no_versions" => "No versions installed. Browse versions first.",
            "home.available" => "Available Versions",
            "home.installed" => "Installed",
            "home.active_downloads" => "Active Downloads",

            "version.management" => "Version Management",
            "version.fetching" => "Fetching versions...",
            "version.none" => "No versions found. Click Refresh.",
            "version.refresh" => "Refresh",
            "version.install" => "Install",
            "version.details" => "Details",
            "version.filter_all" => "All",
            "version.custom_import" => "Import Custom Version",
            "version.import_url" => "Import from URL",
            "version.import_file" => "Import from File",
            "version.import_install" => "Import from Existing Install",

            "download.manager" => "Download Manager",
            "download.active" => "Active",
            "download.queued" => "Queued",
            "download.completed" => "Completed",
            "download.failed" => "Failed",
            "download.none" => "No active downloads.",
            "download.clear" => "Clear Completed",

            "config.management" => "Configuration",
            "config.profiles" => "Configuration Profiles",
            "config.none" => "No configuration profiles.",
            "config.new_profile" => "New Profile",
            "config.editor" => "openttd.cfg Editor",
            "config.editor_hint" => "Select a profile to edit its configuration.",
            "config.default" => "Default",
            "config.edit" => "Edit",
            "config.delete" => "Delete",

            "mods.management" => "Mod Management",
            "mods.search" => "Search mods by name or author...",
            "mods.browser" => "Online Mod Browser",
            "mods.browser_hint" => "Browse and download mods from BaNaNaS.",
            "mods.installed" => "Installed Mods",
            "mods.installed_hint" => "No mods installed.",
            "mods.download" => "Download",

            "settings.title" => "Settings",
            "settings.general" => "General",
            "settings.language" => "Language",
            "settings.theme" => "Theme",
            "settings.light" => "Light",
            "settings.dark" => "Dark",
            "settings.download" => "Download",
            "settings.mirror" => "Mirror Source",
            "settings.max_concurrent" => "Max Concurrent Downloads",
            "settings.about" => "About",

            "common.loading" => "Loading...",
            "common.error" => "Error",
            "common.success" => "Success",
            "common.cancel" => "Cancel",
            "common.confirm" => "Confirm",
            "common.save" => "Save",
            "common.close" => "Close",

            _ => key,
        }
    }
}

/// Quick translation helper using the current locale
pub fn t(key: &'static str) -> &'static str {
    // Static fallback when no context available
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
        "version.management" => "Version Management",
        "version.refresh" => "Refresh",
        "version.install" => "Install",
        "download.manager" => "Download Manager",
        "download.none" => "No active downloads",
        "config.management" => "Configuration",
        "config.none" => "No configuration profiles",
        "config.new_profile" => "New Profile",
        "mods.management" => "Mod Management",
        "mods.search" => "Search mods...",
        "mods.browser_hint" => "Browse and download mods from BaNaNaS.",
        "mods.installed_hint" => "No mods installed.",
        "settings.title" => "Settings",
        "settings.language" => "Language",
        "settings.theme" => "Theme",
        "common.loading" => "Loading...",
        _ => key,
    }
}