# ZingerBoost v0.2.0 — Session Handoff

**Repo:** https://github.com/YousefMohiey/ZingerBoost
**Version:** 0.2.0 (Flutter + Rust FFI)
**Author:** YousefMohiey
**License:** MIT

## Current State

### Working Backend (Rust)
- 25 tweaks implemented and registered
- Snapshot/restore with retention (last 50)
- Real PDH counter metrics (CPU, Disk) + GlobalMemoryStatusEx (RAM)
- DebloatEngine (PowerShell, takeown, icacls, DISM)
- 34 bloatware targets + 13 protected apps
- Music category added to software catalog
- SQLite persistence at %LOCALAPPDATA%\ZingerBoost\data.db

### Flutter Frontend (structure built, needs `flutter create` on Windows)
- All 6 pages: Dashboard, Tweaks, Snapshots, Debloat, Software, Settings
- All 6 widgets: Sidebar, MetricCard, RiskBadge, TweakCard, SectionHeader, Toast
- Models: Tweak, Metrics, Snapshot, Software, Audit
- Services: RustBridge FFI wrapper
- Theme: Dark/light Material3 + Riverpod provider

### Bridge Crate
- `bridge/Cargo.toml`: cdylib + staticlib
- `bridge/src/lib.rs`: OnceLock<AppState>, init_app() FFI entry with all 25 tweaks
- `bridge/src/api.rs`: 11 FFI functions (JSON-based strings)

## Build Commands

```bash
# Linux: check cross-platform crates
cargo check -p zb_shared -p zb_domain -p zb_application
cargo fmt --all
cargo clippy -p zb_shared -p zb_domain -p zb_application --all-targets -- -D warnings

# Windows: full build
cargo check --workspace

# Flutter (on Windows with Flutter installed)
cd zingerboost_flutter
flutter pub get
flutter build windows
```

## What Was Removed (v0.2.0 cleanup)
- src-tauri/ (old Tauri shell)
- crates/zb_app/ (old Tauri commands)
- src/ (old React frontend)
- package.json, vite.config.ts, tailwind.config.js, etc.
- .obsidian/, ABOUT.md, SESSION.md, V0.2.0PLAN.md
- graphify-out/ cache files

## Next Steps
1. Install Flutter SDK on Windows
2. `cd zingerboost_flutter && flutter create .`
3. Run `flutter_rust_bridge_codegen generate` to connect bridge to Flutter
4. `flutter run -d windows` to test
5. Fix any Flutter compilation errors
6. Test all pages with real Windows data
7. Build release: `flutter build windows`
