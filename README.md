<p align="center">
  <img src="https://img.shields.io/badge/ZingerBoost-v0.2.0-0ea5e9?style=for-the-badge" alt="Version">
  <img src="https://img.shields.io/badge/Rust-1.95%2B-dea584?style=for-the-badge&logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/Flutter-3.x-02569B?style=for-the-badge&logo=flutter" alt="Flutter">
  <img src="https://img.shields.io/badge/Windows-10%2B-0078d6?style=for-the-badge&logo=windows" alt="Windows">
</p>

<p align="center">
  <img src="https://img.shields.io/badge/license-MIT-success?style=flat-square" alt="License">
  <img src="https://img.shields.io/badge/open--source-trust-green?style=flat-square" alt="Open Source">
  <img src="https://img.shields.io/badge/telemetry-none-red?style=flat-square" alt="No Telemetry">
  <img src="https://img.shields.io/badge/offline--first-yes-blue?style=flat-square" alt="Offline First">
</p>

---

# ZingerBoost

### Safe, Reversible, Transparent — Windows Optimization Done Right

**ZingerBoost** is a professional Windows optimization utility built with **Rust** and **Flutter**. 25 safe, reversible tweaks, a 34-app bloatware removal engine, and a 30+ app software installer — all in a beautiful Material 3 dark UI.

> **Every tweak is reversible. Every change is logged. Nothing is hidden.**

---

## Features

### 25 Tweaks

| Category | Count | Examples |
|----------|-------|----------|
| Visual | 14 | Transparency, Animations, Menu Delay, Cursor Shadow, Aero Shake, Smooth Scroll, Taskbar Badges |
| Privacy | 7 | Telemetry, Background Apps, Lock Screen Ads, Start Ads, Advertising ID |
| Performance | 3 | Startup Delay, Hibernation, High Performance Power Plan |
| Gaming | 1 | Game DVR |

### Debloat Engine (34 targets)
Removes pre-installed Windows bloatware via PowerShell + takeown + icacls + DISM. Keeps Notepad, Calculator, Store, Photos, Camera, Terminal, Snipping Tool, and system runtimes.

### Software Installer (30+ apps)
Install apps via Winget across 9 categories: Browsers, Media Players, Music, Gaming, Utilities, Drivers, Communication, Development, Cloud Storage.

### Safety
- Auto-snapshots before every tweak, one-click rollback
- 50-snapshot retention with SQLite persistence
- Full audit log of every operation
- Zero telemetry, fully offline

---

## Quick Start

```bash
git clone https://github.com/YousefMohiey/ZingerBoost.git
cd ZingerBoost/zingerboost_flutter

# Install Flutter deps
flutter pub get

# Build and run on Windows
flutter build windows
```

---

## Architecture

```
ZingerBoost/
├── bridge/                  # Rust FFI bridge (cdylib)
│   └── src/
│       ├── lib.rs           # AppState + init_app() FFI entry
│       └── api.rs           # 11 FFI functions (JSON-based)
├── crates/                  # Rust workspace
│   ├── zb_shared/           # Types, constants, software catalog
│   ├── zb_domain/           # Tweak trait + 25 implementations
│   ├── zb_application/      # TweakEngine, SnapshotService, AuditService
│   └── zb_infrastructure/   # WinRegistryProvider, SQLite, Winget, PDH, Debloat
└── zingerboost_flutter/     # Flutter desktop app
    └── lib/
        ├── models/          # Tweak, Metrics, Snapshot, Software, Audit
        ├── services/        # RustBridge FFI wrapper
        ├── providers/       # Riverpod state
        ├── widgets/         # Sidebar, MetricCard, RiskBadge, TweakCard, Toast
        ├── pages/           # Dashboard, Tweaks, Snapshots, Debloat, Software, Settings
        └── theme/           # Dark/light ThemeData + theme provider
```

---

## Development

```bash
# Check Rust cross-platform crates (Linux/macOS)
cargo check -p zb_shared -p zb_domain -p zb_application
cargo fmt --all
cargo clippy -p zb_shared -p zb_domain -p zb_application --all-targets -- -D warnings

# On Windows: build all
cargo check --workspace
```

> The full workspace only compiles on Windows (windows-rs dependency).

---

## Author

**YousefMohiey** — [yousefmohiey@gmail.com](mailto:yousefmohiey@gmail.com) · [github.com/YousefMohiey](https://github.com/YousefMohiey)

## License

MIT © 2026 YousefMohiey
