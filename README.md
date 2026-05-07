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
| Styling | Tailwind CSS |
| State | Zustand (client) + TanStack Query (server) |
| Charts | Recharts |
| Animation | Framer Motion |
| Async | Tokio |
| Windows API | windows-rs |
| Database | SQLite (rusqlite) |

## MVP Features (v0.1.0)

- [x] Workspace + build pipeline
- [x] Registry read/write via windows-rs
- [x] Tweak trait + 10 safe implementations
- [x] Snapshot create/restore with SQLite persistence
- [x] Dashboard with live CPU/RAM/Disk/Network metrics
- [x] Tweaks browser with search, filters, risk badges, and explanations
- [x] Dark mode UI with toast notifications
- [x] Audit logging with SQLite backend
- [x] CI/CD with GitHub Actions

## 10 Safe MVP Tweaks

1. **Disable Transparency Effects** — Reduce GPU compositor load
2. **Disable Animations** — Snappier window minimize/maximize
3. **Disable Startup Delay** — Remove 10-second startup app delay
4. **Disable Game DVR** — Free up CPU/GPU while gaming
5. **Disable Hibernation** — Free disk space equal to RAM size
6. **Show File Extensions** — Security best practice in Explorer
7. **Disable Sticky Keys Popup** — No more Shift×5 interruptions
8. **Set High Performance Power Plan** — Maximum CPU responsiveness
9. **Disable Background Apps** — Stop UWP apps from running in background
10. **Disable Telemetry (Basic)** — Minimum diagnostic data collection

## Architecture

```
ZingerBoost/
├── crates/
│   ├── zb_shared/        # Common types, errors, constants
│   ├── zb_domain/        # Core traits (Tweak, RegistryProvider), entities, 10 tweak impls
│   ├── zb_application/   # TweakEngine, SnapshotService, AuditService, DTOs
│   ├── zb_infrastructure/# WinRegistryProvider (windows-rs), SQLite repos, metrics
│   └── zb_app/           # Tauri command router and entry point
└── src/                  # React frontend
    ├── components/ui/    # Sidebar, ToastContainer
    ├── features/         # Dashboard, Tweaks, Snapshots, Settings
    ├── lib/api.ts        # Typed Tauri invoke wrappers
    └── store/            # Zustand toast store
```

## Risk Levels

| Level | Color | Behavior |
|-------|-------|----------|
| Safe | Emerald | Toggle immediately, no confirmation |
| Moderate | Amber | Confirmation dialog, may need reboot |
| Advanced | Red | Hidden in Expert Mode, detailed warning |

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

# Run in development mode (Windows only)
cargo tauri dev
```

### Building

```bash
# Release build (Windows only)
cargo tauri build
```

> **Note:** The `windows-rs` crate only compiles on Windows. Linux/macOS can compile `zb_shared`, `zb_domain`, and `zb_application` for code review, but the full app requires a Windows host.

## Testing

```bash
# Run unit tests for platform-agnostic crates
cargo test -p zb_shared -p zb_domain -p zb_application
```

## Screenshots

*Coming soon*

## Author

**YousefMohiey** — [yousefmohiey@gmail.com](mailto:yousefmohiey@gmail.com)

## License

MIT © 2026 YousefMohiey
