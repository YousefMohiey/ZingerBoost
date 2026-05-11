# ZingerBoost v0.0.5

### Safe, Reversible Windows Optimization — Pure Rust

Built with **Rust + Iced**. 29 tweaks, 19 services, 9 cleaner categories, 34 debloat targets, 30+ software installer — all calling real Windows APIs.

## Download

```powershell
# Download from releases
# Run zingerboost.exe
# Requires VC++ Redistributable (one-time):
# https://aka.ms/vs/17/release/vc_redist.x64.exe
```

## Build

```powershell
git clone https://github.com/YousefMohiey/ZingerBoost.git
cd ZingerBoost
cargo build --release -p zb_app
```

## Tech Stack

| Layer | Tech |
|-------|------|
| GUI | Iced 0.13 (pure Rust) |
| Backend | Rust workspace (5 crates) |
| Windows API | windows-rs 0.58 |
| Database | SQLite (rusqlite) |

## Author

YousefMohiey — MIT License
