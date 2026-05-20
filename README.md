# ZingerBoost

<div align="center">

[![Release](https://img.shields.io/github/v/release/YousefMohiey/ZingerBoost?include_prereleases&style=flat)](https://github.com/YousefMohiey/ZingerBoost/releases/latest)
[![License](https://img.shields.io/github/license/YousefMohiey/ZingerBoost?style=flat)](LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/YousefMohiey/ZingerBoost/ci.yml?branch=main)](https://github.com/YousefMohiey/ZingerBoost/actions)
[![Platform](https://img.shields.io/badge/platform-Windows%2010%2F11-blue?style=flat)](https://www.microsoft.com/windows/)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?style=flat)](https://www.rust-lang.org/)

**Safe, Reversible Windows Optimization**

</div>

---

## About

ZingerBoost is a powerful yet safe Windows optimization tool that helps you:
- ⚡ Apply **49 system tweaks** to improve performance
- 🧹 Clean **9 categories** of junk files
- 🔧 Manage **19 Windows services**
- 📦 Remove **44 bloatware** packages
- 📥 Install **30+ popular applications** via Winget

All changes are **reversible** - create snapshots before any modification and restore if needed.

## Download

**[Download Latest Release](https://github.com/YousefMohiey/ZingerBoost/releases/latest)**

Choose between:
- `ZingerBoost_0.0.6_x64-setup.exe` - NSIS Installer (recommended)
- `ZingerBoost_0.0.6_x64.msi` - MSI Installer
- `ZingerBoost.exe` - Portable version

## Features

### System Tweaks
- Visual performance optimizations
- Privacy & telemetry controls  
- Network & startup tuning
- Gaming optimizations (Game Mode, GPU scheduling)

### System Cleaner
- Temp files cleanup
- Browser cache cleaning
- Windows update cleanup
- Recycle bin & logs

### Service Manager
- Enable/Disable Windows services
- View service status
- Risk indicators for each service

### Software Center
- Install popular apps via Winget
- Remove bloatware packages
- Software catalog with 30+ apps

### System Metrics
- Real-time CPU, RAM, Disk usage
- Network activity monitoring
- Live dashboard updates

## Requirements

- Windows 10 or Windows 11
- Administrator privileges (required for system changes)
- WebView2 Runtime (included in installer)

## Build from Source

```powershell
# Clone repository
git clone https://github.com/YousefMohiey/ZingerBoost.git
cd ZingerBoost

# Build
cargo build --release -p zb_app

# Run
./target/release/zb_app.exe
```

## Tech Stack

| Component | Technology |
|-----------|------------|
| Language | Rust |
| GUI | Tauri v2 + Vanilla JS |
| Windows API | windows-rs |
| Database | SQLite |
| Build | Cargo |

## Project Structure

```
ZingerBoost/
├── crates/
│   ├── zb_shared/       # Types, constants, software catalog
│   ├── zb_domain/      # Tweak trait & 49 implementations
│   ├── zb_application/ # TweakEngine, SnapshotService, AuditService
│   ├── zb_infrastructure/ # Registry, Services, Cleaner, Metrics
│   └── zb_app/         # Tauri desktop application
├── README.md
├── LICENSE
└── CONTRIBUTING.md
```

## Safety

- ✅ All tweaks are reversible
- ✅ Snapshots created before every change
- ✅ No telemetry or data collection
- ✅ Open source - inspect the code yourself
- ✅ Works offline - no internet required

## License

MIT License - See [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

<div align="center">

**Made with Rust 🦀 for Windows**

</div>