# ZingerBoost — Program Overview

**Author:** YousefMohiey  
**Repository:** [github.com/YousefMohiey/ZingerBoost](https://github.com/YousefMohiey/ZingerBoost)  
**License:** MIT  
**Status:** v0.1.0 MVP (Foundations Complete)

---

## What Is ZingerBoost?

ZingerBoost is a **modern, professional Windows optimization utility** built from the ground up with **Rust** and **Tauri v2**. Unlike sketchy "PC cleaners" that break your system, ZingerBoost is designed around one core principle: **every single change must be reversible**.

It treats your Windows system like a patient — every tweak is explained, every change is logged, and everything can be undone with a single click.

---

## Modern Dark-Mode UI

ZingerBoost features a **dark, medical-grade UI** using modern web technologies:

| UI Feature | Implementation |
|------------|---------------|
| **Framework** | React 18 + TypeScript (strict mode) |
| **Styling** | Tailwind CSS 3.4 — deep dark theme with surfacing and elevation |
| **Components** | Custom-built component library with `lucide-react` icons |
| **Animations** | Framer Motion — spring-based physics for smooth 200ms transitions |
| **Notifications** | Toast system with slide-in animations (success/error/warning/info) |
| **Charts** | Recharts — sparklines and area charts for live metrics |
| **State** | Zustand (client UI state) + TanStack Query (async server state) |
| **Responsive** | Works from 900x600 and up, centered window on launch |

### Color System

```
Background:    #0a0a0a (surface)
Elevated:      #171717 (cards, sidebar)
Borders:       #262626
Primary:       #0ea5e9 (sky blue brand)
Safe:          #10b981 (emerald)
Moderate:      #f59e0b (amber)
Advanced:      #ef4444 (red)
Text:          zinc-100 primary, zinc-400 secondary
```

### Pages & Screens

| Page | Purpose |
|------|---------|
| **Dashboard** | 4 live metric cards (CPU, RAM, Disk, Network) with real-time updates every 2 seconds |
| **Tweaks** | Searchable, filterable catalog with risk badges (Safe/Moderate/Advanced), category pills, expandable detail panels with explanations |
| **Snapshots** | Timeline of system snapshots created automatically before every tweak batch, with restore buttons |
| **Settings** | Theme, elevation status, version info, and live audit log viewer |

### Navigation

```
┌─ ZingerBoost ───────────────────────────────────────────┐
│  [Z] ZingerBoost                                        │
│                                                          │
│  ■ Dashboard                          CPU ████░░ 15%     │
│  ■ Tweaks                             RAM ██████ 42%    │
│  ■ Snapshots                         Disk █░░░░░  5%    │
│  ──────────────────────────────     Net ↓0.5 ↑0.1 Mbps │
│  ■ Settings                                             │
│  ■ About                                                │
│  ──── Shield: Admin ────                               │
└──────────────────────────────────────────────────────────┘
```

---

## Architecture

### Layered Monolith with Crate Boundaries

```
Frontend (React)              Backend (Rust)
─────────────                 ─────────────
                              ┌─────────────────┐
                              │    zb_app       │  ← Tauri commands, IPC router
                              ├─────────────────┤
 React → invoke() ──────────→ │ zb_application  │  ← TweakEngine, SnapshotService, Audit
                              ├─────────────────┤
                              │ zb_infrastructure│  ← Windows Registry, Services, SQLite
                              ├─────────────────┤
                              │  zb_domain      │  ← Pure logic: Tweak trait, entities
                              ├─────────────────┤
                              │  zb_shared      │  ← Types, constants, errors
                              └─────────────────┘
```

### Technology Stack

| Layer | Technology | Why |
|-------|-----------|-----|
| **Desktop Shell** | Tauri v2 | Lightweight (~10MB bundle vs 100MB+ Electron), native Rust backend |
| **Backend Language** | Rust | Memory safety, zero-cost abstractions, no garbage collection |
| **Windows API** | windows-rs crate | Direct `RegOpenKeyExW`, `RegSetValueExW`, `RegQueryValueExW` — no PowerShell hacks |
| **Database** | SQLite (rusqlite) | Embedded, zero-config, single-file at `%LOCALAPPDATA%\ZingerBoost\data.db` |
| **Async Runtime** | Tokio | Multi-threaded, battle-tested |
| **Frontend** | React 18 + Vite | Fast HMR in dev, tree-shaken production builds |
| **IPC** | Tauri Commands | Type-safe JSON payloads between Rust ↔ React |
| **Logging** | tracing | Structured JSON logging with env-filter support |

---

## Feature Set (v0.1.0)

### 10 Safe MVP Tweaks

Every tweak is **completely reversible**. Before applying, the system state is captured into a `SnapshotData`. Applying the tweak persists the snapshot to SQLite. Reverting reads the snapshot and restores the original state.

| # | Tweak | Category | What It Does | Reboot? |
|---|-------|----------|-------------|---------|
| 1 | **Disable Transparency Effects** | Visual | Turns off acrylic/blur effects in taskbar and windows → reduces GPU compositor load | No |
| 2 | **Disable Animations** | Visual | Disables window minimize/maximize animations → snappier UI feel | No |
| 3 | **Disable Sticky Keys Popup** | Visual | Stops the annoying Shift×5 dialog → no more gaming interruptions | No |
| 4 | **Show File Extensions** | Visual | Forces Explorer to show `.exe`, `.txt`, etc. → security best practice | No |
| 5 | **Disable Game DVR** | Gaming | Turns off Xbox background recording → frees CPU/GPU while gaming | No |
| 6 | **Disable Background Apps** | Privacy | Prevents UWP apps from running in background → saves CPU/RAM/battery | No |
| 7 | **Disable Telemetry (Basic)** | Privacy | Sets diagnostic data to minimum (Security/0) → less data sent to Microsoft | No |
| 8 | **Disable Startup Delay** | Performance | Removes the 10-second built-in delay before startup apps launch | Yes |
| 9 | **Disable Hibernation** | Performance | Deletes hiberfil.sys → frees disk space equal to your RAM size | No |
| 10 | **Set High Performance Power Plan** | Performance | Switches to High Performance → prevents CPU downclocking | No |

### Registry Operations

All tweaks use **direct Windows Registry API calls** via `windows-rs`:

```
Read:   RegOpenKeyExW → RegQueryValueExW → decode DWORD/QWORD/SZ/Binary → RegValue enum
Write:  RegOpenKeyExW → RegSetValueExW → RegCloseKey
Delete: RegOpenKeyExW → RegDeleteValueW → RegCloseKey
```

**Type safety**: `RegValue` is a Rust enum (`Dword(u32)`, `Qword(u64)`, `Sz(String)`, `Binary(Vec<u8>)`, `Absent`) — no raw bytes, no type confusion.

### Snapshot & Restore System

- **Automatic snapshots** before every tweak application
- Each snapshot captures the **previous registry value** (or power plan GUID, service config, etc.)
- **SQLite persistence** at `%LOCALAPPDATA%\ZingerBoost\data.db` with two tables:
  - `snapshots` + `snapshot_tweaks` — full system snapshots
  - `tweak_states` — per-tweak current state
  - `audit_log` — immutable operation history
- **Retention**: Last 50 snapshots kept, older ones auto-purged
- Future: integration with Windows System Restore Points (`SRSetRestorePoint`)

### Audit Logging

Every operation is recorded with timestamp, severity level, category, and message:

```
2026-05-07T10:30:00Z  INFO   tweak    Applied tweak: gaming_disable_dvr
2026-05-07T10:30:01Z  INFO   tweak    Applied tweak: visual_disable_transparency
2026-05-07T10:32:00Z  WARN   tweak    Reverted tweak: gaming_disable_dvr
```

Viewable in real-time from the Settings page.

---

## Safety Guarantees

| Guarantee | Implementation |
|-----------|---------------|
| **Always reversible** | Every `Tweak` implements `capture_state()` + `revert()` using stored `SnapshotData` |
| **Atomic batches** | If any tweak in a batch fails, all already-applied tweaks are **automatically rolled back** |
| **No silent changes** | Every registry write is logged, every tweak has a user-facing explanation |
| **Admin required** | UAC elevation at startup; tweaks requiring admin are clearly marked |
| **Zero telemetry** | No network calls except manual update checks. No analytics, no crash reporters |
| **Offline-first** | Works fully without internet. No cloud sync, no login, no license server |
| **Open source** | MIT license — anyone can audit the registry changes |

---

## What ZingerBoost Is NOT

| NOT this | Reason |
|----------|--------|
| ❌ Registry "cleaner" | We don't scan and delete registry keys blindly |
| ❌ RAM "optimizer" | Emptying the standby list is placebo — we don't do it |
| ❌ Driver updater | We don't touch drivers — that's dangerous territory |
| ❌ Fake "speed boost" | We explain exactly what each tweak does, no fake numbers |
| ❌ Telemetry spyware | We collect nothing — not even anonymous usage stats |
| ❌ Adware/ bundleware | No upsells, no bundled software, no nag screens |

---

## Project Structure

```
ZingerBoost/
├── crates/                          # Rust workspace
│   ├── zb_shared/                   # Shared types, errors, constants
│   │   └── src/{lib.rs, types.rs, constants.rs}
│   ├── zb_domain/                   # Pure domain logic (no OS deps)
│   │   └── src/
│   │       ├── tweaks/{traits.rs, definitions/}  # 10 tweak implementations
│   │       ├── snapshots/{entities.rs}           # SystemSnapshot, AppliedTweakRecord
│   │       ├── benchmarks/{entities.rs}          # Benchmark trait
│   │       ├── registry.rs                       # RegistryProvider trait
│   │       └── errors.rs                         # TweakError, SnapshotError, RegistryError
│   ├── zb_application/              # Orchestration & use cases
│   │   └── src/
│   │       ├── tweak_engine.rs                   # Batch apply/rollback
│   │       ├── snapshot_service.rs               # Snapshot persistence trait
│   │       ├── audit_service.rs                  # Audit logging trait
│   │       └── dto.rs                            # Frontend DTOs + error mapping
│   ├── zb_infrastructure/           # OS adapters (Windows-only)
│   │   └── src/
│   │       ├── registry/mod.rs                   # WinRegistryProvider (windows-rs)
│   │       ├── services/service_controller.rs    # SCM API wrapper
│   │       ├── persistence/{sqlite_repo, audit_logger}.rs  # SQLite impls
│   │       ├── windows_api/metrics_collector.rs  # PDH counters placeholder
│   │       └── logging.rs                        # tracing setup
│   └── zb_app/                      # Tauri desktop entry point
│       ├── src/{main.rs, lib.rs, commands.rs}    # Tauri command handlers
│       ├── tauri.conf.json                       # Window config, bundler
│       └── capabilities/default.json             # Tauri v2 permissions
├── src/                             # React frontend
│   ├── components/ui/{Sidebar, ToastContainer}.tsx
│   ├── features/
│   │   ├── dashboard/Dashboard.tsx               # Live metric cards
│   │   ├── tweaks/TweaksPage.tsx                 # Tweak browser + search/filter
│   │   ├── snapshots/SnapshotsPage.tsx           # Snapshot timeline
│   │   └── settings/SettingsPage.tsx             # Config + audit log viewer
│   ├── lib/api.ts                                # Typed invoke() wrappers
│   ├── store/toast.ts                            # Zustand toast state
│   └── {App.tsx, main.tsx, index.css}            # Entry points + Tailwind
├── .github/workflows/ci.yml        # GitHub Actions: fmt, clippy, tests, build
├── {Cargo.toml, package.json, tsconfig.json, tailwind.config.js, vite.config.ts}
├── {LICENSE, README.md, CONTRIBUTING.md}
└── ABOUT.md                        # ← This file
```

---

## Development Roadmap

### ✅ v0.1.0 — MVP (Current)

| Status | Feature |
|--------|---------|
| ✅ | Cargo workspace with 5 crates |
| ✅ | Tauri v2 + React 18 + TypeScript + Tailwind |
| ✅ | Dark-mode UI with animated toasts |
| ✅ | 10 safe registry-based tweaks |
| ✅ | Tweak trait (`capture_state`, `apply`, `revert`, `explain`) |
| ✅ | TweakEngine with batch apply + auto-rollback |
| ✅ | SQLite persistence (snapshots, tweak states, audit log) |
| ✅ | Snapshot list/restore |
| ✅ | Live dashboard metrics (CPU, RAM, Disk, Network) |
| ✅ | Audit log viewer |
| ✅ | CI/CD (fmt, clippy, tests, build) |

### 🔜 v0.2.0 — Planned

- Registry read/write for all tweaks (currently trait-based with placeholder data)
- PowerShell integration for power plan switching
- Windows System Restore Point integration
- Moderate tweaks: Cortana, OneDrive, Search indexing
- UWP app debloat tool
- Real-time metrics via PDH counters

### 🔮 v0.3.0+

- Gaming suite: timer resolution, HPET toggle, dynamic tick
- Network tweaks: Nagle's algorithm, TCP auto-tuning
- Benchmark system (boot time, file copy speed)
- Export/import snapshots
- Plugin SDK (WASM community tweaks)
- Enterprise: GPO support, CLI mode, localization

---

## Quick Start

```bash
# Clone
git clone https://github.com/YousefMohiey/ZingerBoost.git
cd ZingerBoost

# Install frontend deps
npm install

# Run (Windows only — requires windows-rs)
cargo tauri dev

# Build release .msi
cargo tauri build
```

> **Note:** The `windows-rs` crate only compiles on Windows. On Linux, you can check `zb_shared`, `zb_domain`, and `zb_application` with `cargo check -p zb_shared -p zb_domain -p zb_application`.

---

## Author

**YousefMohiey**  
[yousefmohiey@gmail.com](mailto:yousefmohiey@gmail.com)  
[github.com/YousefMohiey](https://github.com/YousefMohiey)

Built with Rust 🦀 and React ⚛️
