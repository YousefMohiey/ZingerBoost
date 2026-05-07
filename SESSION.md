# ZingerBoost — Session Handoff

> **Date:** May 7, 2026  
> **Author:** YousefMohiey  
> **Repo:** https://github.com/YousefMohiey/ZingerBoost  
> **Release:** https://github.com/YousefMohiey/ZingerBoost/releases/tag/v0.1.0

---

## Project Overview

ZingerBoost is a **Windows optimization utility** built with **Rust + Tauri v2 + React + TypeScript**. It provides safe, reversible tweaks, a software installer via Winget, and bloatware removal — all with a modern dark-mode UI.

### Key Principles
- Every tweak is **reversible** (snapshot before every change)
- **Zero telemetry**, offline-first
- **Open source** (MIT)
- Professional, not fake "PC cleaner"

---

## Current State (v0.1.0)

### What's Working

| Feature | Status |
|---------|--------|
| Dark-mode UI (Dashboard/Tweaks/Snapshots/Software/Settings) | ✅ |
| 10 safe registry tweaks with snapshot/restore | ✅ |
| Software installer (30+ apps via Winget, 8 categories) | ✅ |
| Debloat (21 bloatware apps — keeps Notepad, Calculator, Store, Photos) | ✅ |
| SQLite persistence at `%LOCALAPPDATA%\ZingerBoost\data.db` | ✅ |
| Toast notifications, search, filters, animations | ✅ |
| Audit logging | ✅ |
| CI: fmt, clippy, test, check-all (Windows) | ✅ |
| Release: .msi + .exe via GitHub Actions | ✅ |

### 10 Tweaks
| ID | Name | Category |
|----|------|----------|
| `visual_disable_transparency` | Disable Transparency Effects | Visual |
| `visual_disable_animations` | Disable Animations | Visual |
| `visual_disable_sticky_keys` | Disable Sticky Keys Popup | Visual |
| `visual_show_extensions` | Show File Extensions | Visual |
| `gaming_disable_dvr` | Disable Game DVR | Gaming |
| `privacy_disable_background_apps` | Disable Background Apps | Privacy |
| `privacy_disable_telemetry` | Disable Telemetry (Basic) | Privacy |
| `performance_disable_startup_delay` | Disable Startup Delay | Performance |
| `performance_disable_hibernation` | Disable Hibernation | Performance |
| `performance_high_power` | High Performance Power Plan | Performance |

---

## Project Structure

```
ZingerBoost/
├── src-tauri/                    # Tauri v2 binary entry point (STANDARD LAYOUT)
│   ├── Cargo.toml                # zingerboost binary — depends on zb_app
│   ├── build.rs                  # tauri_build::build()
│   ├── tauri.conf.json           # Window config, bundler, capabilities
│   ├── capabilities/default.json # Tauri permissions
│   ├── icons/                    # Placeholder icons (32x32, 128x128, ico)
│   └── src/main.rs               # Wires up AppState, registry provider, 10 tweaks, engine
│
├── crates/                       # Rust workspace crates
│   ├── zb_shared/                # Common types, errors, software catalog
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── types.rs          # RegPath, RegValue, TweakMetadata, SnapshotData, etc.
│   │       ├── constants.rs      # APP_NAME, DATA_DIR, risk levels, categories
│   │       └── software.rs       # 30+ app catalog + 21 bloatware + protected apps
│   │
│   ├── zb_domain/                # Core traits + entities (pure logic, no OS deps)
│   │   └── src/
│   │       ├── lib.rs            # #![allow(clippy::new_without_default)]
│   │       ├── tweaks/traits.rs  # Tweak trait (is_applied, capture_state, apply, revert, explain)
│   │       ├── tweaks/definitions/  # 10 tweak implementations
│   │       ├── snapshots/entities.rs # SystemSnapshot, AppliedTweakRecord
│   │       ├── registry.rs       # RegistryProvider trait
│   │       ├── benchmarks/       # Benchmark trait (not yet used)
│   │       └── errors.rs         # TweakError, SnapshotError, RegistryError, etc.
│   │
│   ├── zb_application/           # Orchestration services
│   │   └── src/
│   │       ├── tweak_engine.rs   # TweakEngine: apply_single, apply_batch (auto-rollback), revert
│   │       ├── snapshot_service.rs # SnapshotService trait
│   │       ├── audit_service.rs  # AuditService trait
│   │       └── dto.rs            # Frontend DTOs + error mapping helpers
│   │
│   ├── zb_infrastructure/        # OS adapters (Windows-only)
│   │   └── src/
│   │       ├── registry/mod.rs   # WinRegistryProvider (RegOpenKeyExW, RegSetValueExW, etc.)
│   │       ├── persistence/sqlite_repo.rs # init_database(), SqliteRepo (trait impl)
│   │       ├── persistence/audit_logger.rs # SqliteAuditLogger (trait impl)
│   │       ├── services/service_controller.rs # Windows SCM placeholder
│   │       ├── windows_api/metrics_collector.rs # Live metrics placeholder
│   │       ├── windows_api/winget.rs # Winget install + PowerShell AppX removal
│   │       └── logging.rs        # tracing-subscriber init
│   │
│   └── zb_app/                   # Library — AppState struct + Tauri command handlers
│       └── src/
│           ├── lib.rs            # AppState { engine, metrics_collector, winget }
│           └── commands.rs       # 12 Tauri commands (list_tweaks, apply_tweak, etc.)
│
├── src/                          # React frontend
│   ├── App.tsx                   # Routes: / /tweaks /snapshots /software /settings
│   ├── main.tsx                  # Entry: QueryClientProvider + BrowserRouter
│   ├── index.css                 # Tailwind base
│   ├── lib/api.ts                # Typed invoke() wrappers
│   ├── store/toast.ts            # Zustand toast store
│   ├── components/ui/
│   │   ├── Sidebar.tsx           # Collapsible nav (Dashboard/Tweaks/Snapshots/Software/Settings)
│   │   └── ToastContainer.tsx    # Animated toast notifications
│   └── features/
│       ├── dashboard/Dashboard.tsx    # Live CPU/RAM/Disk/Network cards
│       ├── tweaks/TweaksPage.tsx      # Tweak browser: search, filter, apply/revert
│       ├── snapshots/SnapshotsPage.tsx # Snapshot timeline from backend
│       ├── software/SoftwarePage.tsx   # Install tab + Debloat tab
│       └── settings/SettingsPage.tsx   # Config + audit log viewer
│
├── public/vite.svg               # Favicon
├── .github/workflows/
│   ├── ci.yml                    # fmt, clippy (ubuntu), test, check-all (windows)
│   └── release.yml               # Triggered on tag v*: builds .msi + .exe, uploads to release
│
├── Cargo.toml                    # Workspace root (6 members + src-tauri)
├── package.json                  # Frontend deps (React, Tailwind, Framer Motion, etc.)
├── tsconfig.json                 # Strict TypeScript config
├── vite.config.ts                # Vite bundler config
├── tailwind.config.cjs           # Tailwind theme (brand colors, risk colors, dark mode)
├── postcss.config.cjs            # PostCSS config
├── .gitignore                    # node_modules, target, dist, package-lock.json
├── README.md                     # Professional README with badges and features
├── ABOUT.md                      # Detailed program documentation
├── CONTRIBUTING.md               # Contributing guidelines
├── LICENSE                       # MIT
└── SESSION.md                    # ← This file
```

---

## Build & Run

### On Windows VM
```powershell
git clone https://github.com/YousefMohiey/ZingerBoost.git
cd ZingerBoost
npm install
cargo tauri dev      # Development with hot reload
cargo tauri build    # Production .msi + .exe
```

### On Linux (cross-platform crates only)
```bash
cargo check -p zb_shared -p zb_domain -p zb_application
cargo clippy -p zb_shared -p zb_domain -p zb_application --all-targets -- -D warnings
cargo test -p zb_shared -p zb_domain -p zb_application
cargo fmt --all
```

> **Note:** `zb_infrastructure` and `src-tauri` depend on `windows-rs` which only compiles on Windows.

---

## GitHub CI Flow

| Job | Runner | What it does | Time |
|-----|--------|-------------|------|
| `check` | ubuntu | `cargo fmt --check` + `cargo clippy` (cross-platform crates) | ~30s |
| `test` | windows | `cargo test` (cross-platform crates) | ~2min |
| `check-all` | windows | `cargo check --workspace` (catches all compile errors) | ~3min |

**Release workflow** (`release.yml`):
- Trigger: push tag `v*` OR manual `workflow_dispatch`
- `npm install` → `cargo install tauri-cli --locked` → `cargo tauri build`
- Uploads `.msi` + `.exe` to GitHub Release

### Quick Build Test (no push needed)
Go to: https://github.com/YousefMohiey/ZingerBoost/actions/workflows/release.yml → **Run workflow** → triggers build without pushing

---

## Key Architectural Decisions

1. **RegistryProvider trait in zb_domain** — Avoids circular dependency. zb_infrastructure implements it via windows-rs.
2. **`fn migrations()` instead of `const MIGRATIONS`** — Rust 2024 edition doesn't allow `Migrations::new()` in const context.
3. **`anyhow::Error` for `init_database()`** — Avoids `From<rusqlite_migration::Error>` mismatch.
4. **`REG_SAM_FLAGS` not bare `u32`** — Required by windows-rs 0.58 for `RegOpenKeyExW`.
5. **`#![allow(clippy::new_without_default)]`** in zb_domain — Every tweak has `new()` without `Default`.
6. **Shared `Arc<Mutex<Connection>>`** — Snapshot repo and audit logger share one SQLite connection.
7. **`package-lock.json` in .gitignore** — Generated platform-specific, causes rollup native module issues in CI.
8. **`.cjs` extension for PostCSS/Tailwind configs** — package.json has `"type": "module"`, so CommonJS files need `.cjs`.

---

## Known Issues & Future Work

### Known Limitations
- Registry tweaks work correctly but haven't been tested on all Windows builds
- Metrics collector returns placeholder data (not real PDH counters yet)
- Hibernation and Power Plan tweaks use placeholder logic (need powercfg integration)
- Icons are simple colored placeholders (replace with proper design)
- No error state shown in UI when Winget is not available

### Next Steps for v0.2.0
- [ ] Real PDH counter integration for live CPU/RAM/Disk metrics
- [ ] powercfg integration for Power Plan and Hibernation tweaks
- [ ] UWP app removal via `Get-AppxPackage | Remove-AppxPackage`
- [ ] Windows System Restore Point integration (`SRSetRestorePoint`)
- [ ] Moderate tweaks: Cortana, OneDrive, Search indexing
- [ ] Better error handling when winget is missing
- [ ] Proper app icons (replace placeholders)

### Remaining Tweaks to Implement
- Disable Cortana (Moderate)
- Disable OneDrive integration (Moderate)
- Disable Windows Search indexing (Moderate)
- Disable fullscreen optimizations (Safe)
- Disable location services (Safe)
- Network tweaks: Nagle's algorithm, TCP auto-tuning (Moderate)

---

## Common Fixes Applied During Development

| Problem | Fix |
|---------|-----|
| `cargo fmt --check` failing | Run `cargo fmt --all` before committing |
| Clippy `new_without_default` | Added `#![allow(clippy::new_without_default)]` to zb_domain lib.rs |
| Clippy `empty_line_after_doc_comments` | Changed `///` to `//!` for module docs |
| `npm ci` rollup native module error | Removed `package-lock.json` from git, use `npm install` in CI |
| PostCSS/Tailwind CommonJS in ESM project | Renamed to `.cjs` extension |
| `const MIGRATIONS` compile error (Rust 2024) | Changed to `fn migrations()` returning `Migrations` |
| `REG_SAM_FLAGS` type mismatch | Removed `.0` from `KEY_READ.0` / `KEY_ALL_ACCESS.0` |
| Orphan rule for `From` impls | Replaced with explicit `app_error_from_*` helper functions |
| `SystemSnapshot` not exported | Added `pub use entities::*;` to snapshots/mod.rs |
| Double `Arc<Arc<MetricsCollector>>` | Removed outer `Arc::new()` in main.rs |
| ToastContainer import path wrong | Fixed `../store/toast` → `../../store/toast` |
| Missing `new()` on hibernation/power tweaks | Added constructors |
| Missing `package-lock.json` for npm ci | Added to .gitignore, removed from git |
| `icons/icon.ico` not found | Created minimal placeholder PNGs and ICO |
| MSI glob path wrong in release workflow | Added both `target/` and `src-tauri/target/` paths |

---

## Commands Reference

```bash
# Local development on Linux
cargo check -p zb_shared -p zb_domain -p zb_application
cargo fmt --all
cargo clippy -p zb_shared -p zb_domain -p zb_application --all-targets -- -D warnings
npx tsc --noEmit

# Release (on Windows or via GitHub Actions)
git tag v0.2.0
git push origin v0.2.0
# Or trigger manually: Actions → Release → Run workflow

# Git remote (PAT-based)
git remote set-url origin https://TOKEN@github.com/YousefMohiey/ZingerBoost.git
git push origin main
git remote set-url origin https://github.com/YousefMohiey/ZingerBoost.git  # cleanup
```

---

## TODO for Next Session

1. Test on Windows VM — verify all tweaks apply correctly
2. Add proper icons
3. Implement powercfg integration (SetHighPerformance, DisableHibernation)
4. Add PDH metrics collector (real CPU/RAM/Disk data)
5. Create proper error states in UI (winget not found, admin not elevated)
6. Add remaining tweaks for v0.2.0
7. Consider adding automated tests for tweak logic
