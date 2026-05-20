# ZingerBoost — Agent Instructions

## Tech Stack (LOCKED — NEVER CHANGE)

| Layer | Technology |
|-------|-------------|
| Language | Rust (edition 2021) |
| GUI | **Tauri v2** + Vanilla JS |
| Backend crates | `zb_shared`, `zb_domain`, `zb_application`, `zb_infrastructure` |
| App crate | `zb_app` (Tauri desktop binary) |
| Async | Tokio |
| Windows API | `windows-rs` (latest) |
| Database | SQLite via `rusqlite` (bundled) |
| Build | Cargo workspace |

## Quick Check

```bash
cargo check --release -p zb_app
cargo fmt --all
```

## Architecture

```
crates/
  zb_shared/       Types, constants, software catalog (30+ apps), bloatware catalog (44 apps)
  zb_domain/       Tweak trait + 44 implementations, RegistryProvider trait, snapshot entities
  zb_application/  TweakEngine, SnapshotService, AuditService
  zb_infrastructure/  WinRegistryProvider, ServiceController, DebloatEngine, SystemCleaner, WingetInstaller, MetricsCollector
  zb_app/          Tauri desktop app (HTML/CSS/JS frontend)
```

## Frontend Files

Located in `crates/zb_app/src/`:
- `index.html` — Entry point
- `app.js` — Vanilla JS logic
- `style.css` — Styling
- `commands.rs` — Tauri IPC commands
- `state.rs` — State management
- `tauri.conf.json` — Tauri configuration

## Commands

```bash
# Build
cargo build --release -p zb_app

# Run in dev mode
cargo tauri dev -p zb_app

# Check specific crate
cargo check -p zb_infrastructure
```

## Build Requirements

- Rust (latest stable)
- Visual Studio Build Tools with C++ workload
- WebView2 Runtime (bundled with Windows 10/11)