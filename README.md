# ZingerBoost

> **Safe, Reversible, Transparent Windows Optimization**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**ZingerBoost** is a modern, open-source Windows optimization utility built with **Rust** and **Tauri**. It focuses on **safety first**: every tweak is reversible, every change is logged, and nothing is hidden.

## Philosophy

- **Stability over raw performance gains**
- **Every tweak must be reversible**
- **Never break Windows functionality**
- **Clear explanations for all tweaks**
- **Professional utility, not fake optimization software**
- **Zero telemetry, offline-first**

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Backend | Rust (Tauri v2) |
| Frontend | React 18 + TypeScript + Vite |
| Styling | Tailwind CSS + shadcn/ui |
| State | Zustand (client) + TanStack Query (server) |
| Charts | Recharts |
| Async | Tokio |
| Windows API | windows-rs |
| Database | SQLite (rusqlite) |

## MVP Features (v0.1.0)

- [x] Workspace + build pipeline
- [x] Registry read/write via windows-rs
- [x] Tweak trait + safe implementations
- [x] Snapshot create/restore
- [x] Dashboard with live CPU/RAM
- [x] Tweaks browser with apply/revert
- [x] Dark mode UI
- [x] Audit logging
- [ ] MSI installer

## Safe Tweaks (MVP)

1. Disable Transparency Effects
2. Disable Animations
3. Disable Startup Delay
4. Disable Game DVR
5. Disable Hibernation
6. Show File Extensions
7. Disable Sticky Keys Popup
8. Set High Performance Power Plan
9. Disable Background Apps
10. Disable Telemetry (Basic)

## Development

### Prerequisites

- [Rust](https://rustup.rs/)
- [Node.js](https://nodejs.org/) (v20+)
- Windows 10/11 (for full feature support)

### Setup

```bash
git clone https://github.com/YousefMohiey/ZingerBoost.git
cd ZingerBoost

# Install frontend dependencies
npm install

# Run in development mode
cargo tauri dev
```

### Building

```bash
# Release build
cargo tauri build
```

## Architecture

```
ZingerBoost/
├── crates/
│   ├── zb_shared/        # Common types, errors, constants
│   ├── zb_domain/        # Core traits and entities (pure logic)
│   ├── zb_application/   # Orchestration services
│   ├── zb_infrastructure/# OS adapters (registry, services, SQLite)
│   └── zb_app/           # Tauri command router and entry point
└── src/                  # React frontend
```

## Risk Levels

| Level | Color | Behavior |
|-------|-------|----------|
| Safe | Emerald | Toggle immediately, no confirmation |
| Moderate | Amber | Confirmation dialog, may need reboot |
| Advanced | Red | Hidden in Expert Mode, detailed warning |

## Author

**YousefMohiey** — [yousefmohiey@gmail.com](mailto:yousefmohiey@gmail.com)

## License

MIT © 2026 YousefMohiey
