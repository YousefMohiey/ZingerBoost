<p align="center">
  <img src="https://img.shields.io/badge/ZingerBoost-v0.2.0-0ea5e9?style=for-the-badge" alt="Version">
  <img src="https://img.shields.io/badge/Rust-1.95%2B-dea584?style=for-the-badge&logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/Flutter-3.x-02569B?style=for-the-badge&logo=flutter" alt="Flutter">
  <img src="https://img.shields.io/badge/Windows-10%2B-0078d6?style=for-the-badge&logo=windows" alt="Windows">
</p>

<p align="center">
  <img src="https://img.shields.io/github/downloads/YousefMohiey/ZingerBoost/total?style=flat-square&color=0ea5e9" alt="Downloads">
  <img src="https://img.shields.io/github/v/release/YousefMohiey/ZingerBoost?style=flat-square&color=0ea5e9" alt="Release">
  <img src="https://img.shields.io/badge/license-MIT-success?style=flat-square" alt="MIT">
  <img src="https://img.shields.io/badge/telemetry-none-red?style=flat-square" alt="No Telemetry">
</p>

# ZingerBoost

### Safe, Reversible Windows Optimization

A professional Windows tuning utility. 25 tweaks, 19 service controls, 9 disk cleaners, 34 bloatware targets — all reversible, all logged, zero telemetry.

---

## Features

| Category | Count | Details |
|----------|-------|---------|
| **Registry Tweaks** | 25 | Visual, Privacy, Performance, Gaming |
| **Service Manager** | 19 | Stop/disable resource-heavy Windows services |
| **System Cleaner** | 9 | Cache, temp files, logs, browser data |
| **Debloat Engine** | 34 | Remove pre-installed Windows bloatware |
| **Software Installer** | 30+ | Install apps via Winget across 9 categories |

### Safety Guarantees

- **Every tweak is reversible** — state captured before changes
- **50-snapshot retention** — restore to any previous state
- **Full audit log** — every operation recorded in SQLite
- **Zero telemetry** — no network calls, fully offline
- **Open source** — MIT licensed, anyone can audit

---

## Quick Start

Download the latest release from [Releases](https://github.com/YousefMohiey/ZingerBoost/releases), extract all files, and run `zingerboost_flutter.exe`.

```powershell
# Or build from source:
git clone https://github.com/YousefMohiey/ZingerBoost.git
cd ZingerBoost

# Build the Rust backend
cargo build --release -p zingerboost_bridge

# Build the Flutter frontend (requires Flutter SDK)
cd zingerboost_flutter
flutter create --platforms windows .
flutter pub get
flutter build windows
```

---

## Architecture

```
ZingerBoost/
├── bridge/                  # Rust FFI (cdylib)
├── crates/
│   ├── zb_shared/           # Types, constants, software catalog
│   ├── zb_domain/           # Tweak trait + 25 implementations
│   ├── zb_application/      # TweakEngine, SnapshotService
│   └── zb_infrastructure/   # WinRegistry, SQLite, Winget, PDH
└── zingerboost_flutter/     # Flutter desktop app
    └── lib/
        ├── models/          # Data classes
        ├── services/        # Rust bridge
        ├── pages/           # 7 pages
        ├── widgets/         # Reusable components
        └── theme/           # Dark/light theme
```

---

## Author

**YousefMohiey** — [yousefmohiey@gmail.com](mailto:yousefmohiey@gmail.com) · [github.com/YousefMohiey](https://github.com/YousefMohiey)

MIT License
