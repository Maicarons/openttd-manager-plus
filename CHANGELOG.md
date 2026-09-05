# Changelog

All notable changes to OpenTTD Manager Plus will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-09-06

### Phase 4: Mobile App & Security (2026-09-06)

#### Added
- Mobile app with bottom navigation (Home, Versions, Downloads, Settings)
- APK version list with download/install buttons for Android
- MobileState reactive state management with Dioxus Signals
- Performance optimization: CacheWarmer, LazyResource, ParallelInit
- Security hardening: PathSanitizer, UrlValidator, FileValidator
- 8 security unit tests
- CI/CD pipeline: GitHub Actions (ci.yml, build.yml, release.yml)
- Release build scripts (scripts/release.sh, release.bat)
- Full Dioxus.toml mobile configuration with Android/iOS permissions

### Phase 3: Enhanced Features (2026-09-06)

#### Added
- Custom version import (URL, local file, existing install)
- Save/archive management (scan, import, export, backup, restore, search)
- 32 save management tests
- 16 custom import tests
- I18nManager with reactive locale switching (Chinese/English, 60+ keys)
- ThemeManager with reactive light/dark switching
- Settings page with language and theme toggles

### Phase 2: Core Features (2026-09-06)

#### Added
- CMClient version source (GitHub Releases API)
- BaNaNaS mod metadata source (NewGRF, AI, GameScript, Music)
- DownloadQueue with priority, retry, concurrency control
- Desktop UI connected to core libraries (AppState signals)
- Version filter tabs (All, Official, JGRPP, CMClient)
- Download progress bars and queue statistics UI
- Configuration profiles list UI
- Mod browser with type tabs

### Phase 1: MVP Foundation (2026-09-06)

#### Added
- Cargo workspace with 6 members
- Core version models (VersionInfo, VersionSource, VersionType)
- Official OpenTTD version fetcher
- JGRPP version fetcher
- Manifest cache with configurable TTL
- Download engine with streaming, progress, retry
- Mirror selector with latency testing (direct, ghproxy, ghfast)
- SHA-256 and ZIP integrity verification
- ArgsBuilder for OpenTTD command-line arguments
- GameRunner with platform-specific process management
- InstanceManager with CRUD and JSON persistence
- ProfileManager with Isolated/Shared/Hybrid config modes
- ConfigFile parser for openttd.cfg
- Dioxus Native desktop skeleton with 6 pages
- 154 unit tests
- VitePress documentation with 15+ documents
- AGPL-3.0 license

### Technical Details

- **Framework**: Dioxus 0.7.10 with Blitz/WGPU native renderer (no WebView)
- **Language**: Rust 1.80+
- **Platforms**: Windows, Linux, macOS, Android, iOS
- **Tests**: 223 unit tests across all core modules
- **License**: AGPL-3.0