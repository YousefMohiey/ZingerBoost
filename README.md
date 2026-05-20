# ZingerBoost v0.0.6

### Safe, Reversible Windows Optimization

Built with **Rust + Tauri v2**. 44 tweaks, 19 services, 9 cleaner categories, 44 debloat targets, 30+ software installer — all calling real Windows APIs.

## Download

Download the latest release from: https://github.com/YousefMohiey/ZingerBoost/releases

## Build

```powershell
git clone https://github.com/YousefMohiey/ZingerBoost.git
cd ZingerBoost
cargo build --release -p zb_app
```

The executable will be at: `target/release/zb_app.exe`

## Features

- **Tweaks** — 44 safe, reversible system optimizations
- **Services** — Manage 19 Windows services
- **Cleaner** — 9 categories of system cleanup
- **Debloat** — Remove 44 bloatware packages
- **Software** — Install 30+ popular applications via Winget

## Tech Stack

| Layer | Tech |
|-------|------|
| GUI | Tauri v2 + Vanilla JS |
| Backend | Rust workspace (5 crates) |
| Windows API | windows-rs 0.58+ |
| Database | SQLite (rusqlite) |

## License

MIT — Open source, telemetry-free, offline-first

## Author

YousefMohiey