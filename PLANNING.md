# ZingerBoost — Master Engineering Plan (v0.4.0)

> **Author:** YousefMohiey  
> **License:** MIT (Open Source)  
> **Repository:** `ZingerBoost`  
> **Target OS:** Windows 10 / Windows 11  
> **Current Stack:** Rust backend + Tauri v2 desktop app (vanilla JS frontend)  

---

## **0. Locked Decisions**

| Parameter | Value |
|-----------|-------|
| **Product Name** | ZingerBoost |
| **Repo** | `ZingerBoost` (GitHub) |
| **Crate Prefix** | `zb_` (e.g., `zb_domain`, `zb_app`) |
| **Elevation** | Request UAC at application startup |
| **Health Score (MVP)** | Deferred; show "X Recommended Actions" instead |
| **Distribution** | Open source, telemetry-free, offline-first |
| **Theme Default** | Dark mode (with Light / System support) |
| **Stack Lock** | **Rust + Tauri v2** — Desktop app with vanilla JS frontend |

---

## **0.1. Current State vs. Target State**

| Component | Current State (v0.0.6) | Target State |
|-----------|------------------------|--------------|
| **Backend** | 5 crates (`zb_shared`, `zb_domain`, `zb_application`, `zb_infrastructure`, `zb_app`) | Keep 5 crates, add `zb_tray/` (background tray) |
| **Tweaks** | **49 defined** + **32 registered** | Register all 49, add more planned |
| **UI** | **Tauri v2** (`zb_app/src/`) + vanilla JS (`index.html`, `app.js`, `style.css`) | Enhance UI with Game Mode, Monitor pages |
| **Frontend** | Vanilla HTML/CSS/JS (no framework, no build step) | Keep vanilla JS, add charts, animations |
| **Debloat** | `DebloatEngine` with 5 methods (PowerShell-heavy) | Refactor to native `windows-rs` COM + DISM |
| **Snapshots** | SQLite-only (`SqliteRepo`) | SQLite + JSON file payload with verification |
| **System Cleaner** | **Exists** (`SystemCleaner` — 9 categories) | Integrated into Tauri UI |
| **Service Manager** | **Exists** (`ServiceController` — 19 services + 10 planned) | Integrated into Tauri UI |
| **Metrics** | **Exists** (`MetricsCollector` — CPU/RAM/Disk via PDH) | Add System Overview (WMI), GPU metrics |
| **Software Catalog** | **Exists** (`zb_shared/src/software.rs` — 30+ apps, 9 categories) | Integrated into Tauri UI |
| **Bloatware Catalog** | **Exists** (`zb_shared/src/software.rs` — 44 apps) | Integrated into Tauri UI |
| **Theme** | Dark purple theme via CSS variables | Add Light/System theme toggle |
| **Tauri Commands** | **30 commands** wired via `invoke_handler` | Add game mode, monitor, preset commands |

---

## **1. System Architecture**

**Current Stack:** Rust backend + Tauri v2 desktop app (vanilla JS frontend).  
**Pattern:** Layered monolith with clear crate boundaries + Tauri command layer.
- `zb_shared`: Types, errors, constants, software catalog, bloatware catalog.
- `zb_domain`: Core traits and entities (`Tweak`, `RegistryProvider`, `Snapshot` entities, `Benchmark` traits). 49 tweak implementations. No `std::fs`, no async, pure logic.
- `zb_application`: Orchestration services (`TweakEngine`, `SnapshotService`, `AuditService`).
- `zb_infrastructure`: OS adapters (`WinRegistryProvider`, `ServiceController`, `DebloatEngine`, `SystemCleaner`, `WingetInstaller`, `MetricsCollector`, SQLite repos).
- `zb_app`: **Tauri v2 desktop app** — `main.rs` entry, `commands.rs` (30 Tauri commands), `state.rs` (AppState), vanilla JS frontend (`index.html`, `app.js`, `style.css`).
- `zb_tray`: **PLANNED** — Background tray app for quick actions and monitoring.
- `server/`: **DELETED** — actix-web server removed during Tauri migration.
- `iced_test/`: **REMOVED** — Old Iced prototype, deleted from repo.

---

## **2. Project Structure (Current)**

```
ZingerBoost/
├── Cargo.toml                       # Workspace root
├── Cargo.lock
├── LICENSE
├── README.md
├── CONTRIBUTING.md
├── assets/banner.svg
├── setup.iss                        # Inno Setup installer script
├── AGENTS.md                        # Agent instructions
├── .cargo/config.toml               # Static CRT linking
│
├── crates/
│   │
│   ├── zb_shared/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── constants.rs
│   │       ├── types.rs             # RegPath, RegValue, RiskLevel, TweakCategory, SystemMetrics, etc.
│   │       └── software.rs          # Software catalog (30+ apps) + bloatware catalog (44 apps) + protected apps
│   │
│   ├── zb_domain/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── errors.rs            # TweakError, SnapshotError, RegistryError, ServiceError, BenchmarkError
│   │       ├── registry.rs          # RegistryProvider trait
│   │       ├── benchmarks/
│   │       │   └── entities.rs
│   │       ├── snapshots/
│   │       │   └── entities.rs      # SystemSnapshot, AppliedTweakRecord
│   │       └── tweaks/
│   │           ├── mod.rs
│   │           ├── traits.rs        # Tweak trait (6 methods)
│   │           └── definitions/     # 49 tweak implementations
│   │               ├── mod.rs
│   │               ├── disable_advertising_id.rs
│   │               ├── disable_aero_shake.rs
│   │               ├── disable_aero_snap.rs
│   │               ├── disable_all_visual_effects.rs
│   │               ├── disable_animations.rs
│   │               ├── disable_background_apps.rs
│   │               ├── disable_combo_animation.rs
│   │               ├── disable_cursor_shadow.rs
│   │               ├── disable_drop_shadows.rs
│   │               ├── disable_explorer_ads.rs
│   │               ├── disable_font_smoothing.rs
│   │               ├── disable_game_dvr.rs
│   │               ├── disable_hibernation.rs
│   │               ├── disable_lock_screen_ads.rs
│   │               ├── disable_meet_now.rs
│   │               ├── disable_menu_delay.rs
│   │               ├── disable_minimax_anim.rs
│   │               ├── disable_peek.rs
│   │               ├── disable_smooth_scroll.rs
│   │               ├── disable_start_suggestions.rs
│   │               ├── disable_startup_delay.rs
│   │               ├── disable_sticky_keys.rs
│   │               ├── disable_taskbar_animations.rs
│   │               ├── disable_taskbar_badges.rs
│   │               ├── disable_telemetry.rs
│   │               ├── disable_thumbnails.rs
│   │               ├── disable_transparency.rs
│   │               ├── set_high_performance.rs
│   │               └── show_file_extensions.rs
│   │               # + 20 more network, gaming, privacy, Windows Update tweaks
│   │
│   ├── zb_application/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── tweak_engine.rs      # Batch apply, auto-rollback, snapshot save, audit log
│   │       ├── snapshot_service.rs  # SnapshotService trait
│   │       ├── audit_service.rs     # AuditService trait
│   │       └── dto.rs
│   │
│   ├── zb_infrastructure/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── logging.rs           # tracing_subscriber init
│   │       ├── registry/
│   │       │   └── mod.rs           # WinRegistryProvider (windows-rs)
│   │       ├── services/
│   │       │   └── mod.rs           # ServiceController (SCM API + sc.exe fallback)
│   │       │   └── service_controller.rs
│   │       ├── persistence/
│   │       │   ├── mod.rs
│   │       │   ├── sqlite_repo.rs   # SqliteRepo (implements SnapshotService)
│   │       │   └── audit_logger.rs  # SqliteAuditLogger (implements AuditService)
│   │       └── windows_api/
│   │           ├── mod.rs
│   │           ├── debloat_engine.rs   # 5-method removal (PowerShell-heavy — see Tech Debt)
│   │           ├── metrics_collector.rs # PDH CPU/RAM/Disk counters
│   │           ├── system_cleaner.rs    # 9-category disk cleaner
│   │           └── winget.rs            # WingetInstaller
│   │
│   └── zb_app/                      # Tauri desktop app (main binary)
│       ├── Cargo.toml
│       ├── build.rs                 # tauri_build::build()
│       ├── tauri.conf.json          # Tauri v2 configuration
│       └── src/
│           ├── main.rs              # Entry point, admin check, Tauri builder
│           ├── commands.rs          # 30 Tauri #[tauri::command] handlers
│           ├── state.rs             # AppState (engine, metrics, cleaner, etc.)
│           ├── index.html           # SPA layout (552 lines)
│           ├── app.js               # State management, DOM, Tauri invoke (948 lines)
│           └── style.css            # Dark purple theme (1328 lines)
│
└── target/
```

**Target Structure (v1.0.0):**
```
ZingerBoost/
├── Cargo.toml
├── crates/
│   ├── zb_shared/          # Unchanged
│   ├── zb_domain/          # Unchanged
│   ├── zb_application/     # Unchanged
│   ├── zb_infrastructure/  # Refactor debloat, add SystemOverviewCollector
│   ├── zb_app/             # Tauri desktop app (current)
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── commands.rs
│   │   │   └── state.rs
│   │   └── src/            # Frontend (vanilla JS)
│   │       ├── index.html
│   │       ├── app.js
│   │       └── style.css
│   └── zb_tray/            # NEW — Background tray app
│       └── src/
│           └── main.rs
│
├── assets/
│   ├── icons/              # SVG icons
│   ├── themes/             # Theme definitions
│   └── presets/            # Preset configurations
│
└── installer/
    └── setup.iss           # Inno Setup installer script
```

---

## **3. Frontend Architecture (Current: Tauri v2 + Vanilla JS)**

### Current (Tauri v2)
- **`zb_app/src/main.rs`** — `#[tokio::main]` async entry, admin elevation check, Tauri builder with 30 `invoke_handler` commands.
- **`zb_app/src/commands.rs`** — 30 `#[tauri::command]` functions that call into `TweakEngine`, `MetricsCollector`, `SystemCleaner`, `DebloatEngine`, `ServiceController`, `SqliteRepo`.
- **`zb_app/src/state.rs`** — `AppState` struct holding `Arc<TweakEngine>`, `Arc<MetricsCollector>`, etc., shared across commands via `tauri::State`.
- **`zb_app/src/index.html`** — SPA layout with sidebar navigation and 10 tabs (Overview, Optimizations, Services, Cleaner, Backups, Debloat, Installer, Game Mode, Monitor, Settings, Audit Log).
- **`zb_app/src/app.js`** — Vanilla JS state management, DOM caching, event delegation, `window.__TAURI__.core.invoke` wrapper, tab switching, metrics polling (5s interval), renderers for all lists.
- **`zb_app/src/style.css`** — Premium dark purple theme with CSS variables, gradients, animations, responsive breakpoints. Inter font from Google Fonts.
- **`tauri.conf.json`** — Tauri v2 config: product name, identifier, CSP, bundle settings, window config (1280x850, min 1100x700).
- **No build step** — HTML/JS/CSS served directly by Tauri's dev server. No `package.json`, no bundler.

### Communication Pattern
```
Frontend (JS)                    Rust Backend
─────────────                    ─────────────
window.__TAURI__.core.invoke     #[tauri::command]
  ("get_tweaks")                 fn get_tweaks(state: State<AppState>)
    ↓                              ↓
  JSON response ←──────────────  TweakEngine::list_tweaks()
                                   ↓
                                 Serialize to JSON via serde
```

### Target Enhancements
- Add Game Mode page with process detection and per-game profiles.
- Add Monitor page with real-time GPU, network, and temperature metrics.
- Add Preset System (Safe/Balanced/Aggressive) with one-click apply.
- Add circular progress indicators and sparkline charts for metrics.
- Add Light/System theme toggle (currently dark-only).
- Add backup export/import functionality.

---

## **4. Backend Architecture**

- **Crate Boundaries:** Domain has zero dependencies on infrastructure or Tauri. Application depends only on Domain. Infrastructure implements domain traits.
- **Construction:** `zb_app/src/main.rs` constructs `Arc<dyn RegistryProvider>` etc. and injects into application services, then wraps in `AppState` for Tauri.
- **Commands:** `#[tauri::command]` functions in `commands.rs` receive `State<AppState>`, call application services, return `Result<serde_json::Value, String>`.
- **Error Handling:** `thiserror` in domain (`TweakError`, `RegistryError`). `anyhow` at app boundaries. Commands convert errors to user-friendly strings.

---

## **5. Module Separation**

| Crate | Responsibility |
|-------|---------------|
| `zb_shared` | Common types, constants, software catalog, bloatware catalog |
| `zb_domain` | `Tweak` trait, `Snapshot` entities, `RegistryValue` enum, 49 tweak implementations, pure logic |
| `zb_application` | `TweakEngine`, `SnapshotService`, `AuditService`, DTOs |
| `zb_infrastructure` | `WinRegistryProvider`, `ServiceController`, `DebloatEngine`, `SystemCleaner`, `WingetInstaller`, `MetricsCollector`, SQLite repos |
| `zb_app` (CURRENT) | Tauri v2 desktop app — `main.rs` entry, `commands.rs` (30 commands), `state.rs`, vanilla JS frontend |
| `zb_tray` (PLANNED) | Background tray app for quick actions and hardware monitoring |
| `server` (DELETED) | actix-web HTTP server — removed during Tauri migration |

---

## **6. Tweak Engine Design**

**Current implementation (`zb_application/src/tweak_engine.rs`):**
- Holds `Vec<Arc<dyn Tweak>>`.
- `apply_single(id)` — captures state, applies, saves snapshot, logs audit.
- `apply_batch(ids)` — sequential, creates one `SystemSnapshot`, auto-rolls back on failure.
- `revert(id)` — reads last snapshot from `SnapshotService`, calls `tweak.revert()`.

**Issues:**
- Snapshots are SQLite-only (JSON blob in `snapshot_tweaks` table). No file-based payload verification.
- No `Windows Restore Point` integration yet.
- `apply_single` creates TWO snapshots (one via `save_applied`, one via `save_snapshot`) — redundant.

**Target:**
- Keep current engine logic.
- Add file-based JSON payload storage in `%LOCALAPPDATA%\ZingerBoost\snapshots\{id}.json`.
- Verify file exists and `len() > 0` before applying tweaks.
- Add `SRSetRestorePoint` FFI call before batch ops.

---

## **7. Restore System Design**

- **Current:** SQLite `snapshot_tweaks` table stores JSON `SnapshotData`.
- **Target:** File-based JSON payload + SQLite metadata. Auto-purge after 50 snapshots.
- **Restoration:** Select snapshot → iterate records → call `tweak.revert()`. Continue on individual revert failures.

---

## **8. Snapshot/Backup System (NEEDS REFACTORING)**

**Current:**
- SQLite `snapshots`, `snapshot_tweaks`, `tweak_states` tables.
- `SqliteRepo` implements `SnapshotService`.
- `SystemSnapshot` stored as rows in SQLite.
- Retention: last 50 snapshots (auto-purge in `save_snapshot`).

**Missing:**
- File-based JSON payload (`%LOCALAPPDATA%\ZingerBoost\snapshots\{id}.json`).
- File verification (`exists && size > 0`).
- Export/Import functionality.

**Refactor plan:**
1. Add `SnapshotFileStore` in `zb_infrastructure/src/persistence/`.
2. `save_snapshot` writes JSON to disk THEN inserts into SQLite.
3. Verify file before returning success.
4. `restore_snapshot` reads JSON from disk (fallback to SQLite blob).

---

## **9. Registry Management Strategy**

**Current:** `WinRegistryProvider` in `zb_infrastructure/src/registry/mod.rs`.
- Uses `windows-rs` (`RegOpenKeyExW`, `RegQueryValueExW`, `RegSetValueExW`, `RegDeleteValueW`).
- Supports `Dword`, `Qword`, `Sz`, `ExpandSz`, `Binary`, `Absent`.
- Properly opens keys with `KEY_READ` / `KEY_ALL_ACCESS`.
- Properly closes keys with `RegCloseKey`.

**Status:** ✅ Fully implemented and working.

---

## **10. Windows Services Management**

**Current:** `ServiceController` in `zb_infrastructure/src/services/service_controller.rs`.
- `query_services()` — queries 19 safe-to-disable services via SCM API (`OpenSCManagerW`, `OpenServiceW`, `QueryServiceStatus`).
- `get_start_type_sc()` — falls back to `sc.exe qc` for start type.
- `stop_service()` — uses `sc.exe stop`.
- `set_startup_type()` — uses `sc.exe config`.
- `disable_service()` — combines stop + disable.

**Safe to Disable list (19 services + 10 planned = 29 total):**

**Already Implemented (19):**
`DiagTrack`, `dmwappushservice`, `SysMain`, `WSearch`, `Fax`, `XboxNetApiSvc`, `XblAuthManager`, `XblGameSave`, `XboxGipSvc`, `MapsBroker`, `lfsvc`, `wcncsvc`, `WMPNetworkSvc`, `RemoteRegistry`, `SharedAccess`, `WerSvc`, `WpnService`, `PcaSvc`, `FontCache`.

**Planned Additions (10):**
| Service | Display Name | Why Safe |
|---------|-------------|----------|
| `CDPSvc` | Connected Devices Platform | Syncs phones/contacts — rarely used on desktops. |
| `TabletInputService` | Tablet Input Service | Only needed for touchscreens; safe on non-touch PCs. |
| `WbioSrvc` | Windows Biometric Service | Only needed for fingerprint/face readers; safe without biometrics. |
| `BcastDVRUserService_*` | GameDVR and Broadcast | DVR recording service; safe if Game DVR is already disabled. |
| `OneSyncSvc_*` | OneSync (Contacts/Calendar sync) | Syncs Mail/Calendar data; safe if not using UWP Mail/Calendar. |
| `UnistoreSvc_*` | Unified Store (User Data) | Supports Mail/Calendar/People; safe if not using those apps. |
| `UserDataSvc_*` | User Data Access | Provides user data to UWP apps; safe if debloated. |
| `PimIndexMaintenanceSvc_*` | Contact/Calendar indexing | Indexes PIM data; safe if Mail/Calendar removed. |
| `WpnUserService_*` | Windows Push Notifications (per-user) | Per-user push notifications; safe if notifications not needed. |
| `WalletService` | Wallet Service | Manages digital payments/cards; rarely used on desktop. |

**Status:** 19 implemented. 10 planned. Needs Iced UI integration.

---

## **11. PowerShell Integration Strategy (CURRENT — NEEDS REFACTORING)**

**Current usage (TOO MUCH — violates plan rules):**
- `DebloatEngine::try_powershell_remove()` — `Get-AppxPackage | Remove-AppxPackage`.
- `DebloatEngine::remove_windows_ads()` — massive PowerShell script setting registry keys.
- `DebloatEngine::remove_widgets()` — PowerShell `Get-AppxPackage` + registry.
- `WingetInstaller::remove_appx()` — PowerShell `Remove-AppxPackage`.
- `WingetInstaller::remove_provisioned_appx()` — PowerShell `Remove-AppxProvisionedPackage`.
- `ServiceController::get_start_type_sc()` — uses `sc.exe` (acceptable).
- `ServiceController::stop_service()` — uses `sc.exe` (acceptable).
- `SystemCleaner::clean_recycle_bin()` — uses PowerShell `Clear-RecycleBin`.

**Refactor plan:**
1. **Debloat:** Replace PowerShell with `windows-rs` COM `PackageManager` for user apps + `dism.exe` for provisioned packages.
2. **Ads removal:** Convert to native `WinRegistryProvider` writes (already have the registry keys in plan Section 25-B).
3. **Widgets removal:** Convert to native registry + `PackageManager`.
4. **Cleaner:** Replace `Clear-RecycleBin` PowerShell with `SHEmptyRecycleBinW` Windows API.
5. **Winget:** Keep `winget.exe` calls for software installation (this is the only acceptable external process usage).

---

## **12. Windows API Integration (windows-rs)**

**Current features enabled:**
- `Win32_Foundation`, `Win32_System_Registry`, `Win32_System_Services`, `Win32_System_SystemInformation`, `Win32_System_Threading`, `Win32_System_Power`, `Win32_System_Performance`, `Win32_NetworkManagement_IpHelper`, `Win32_Security`, `Win32_Storage_FileSystem`, `Win32_UI_WindowsAndMessaging`.

**Missing for Iced migration:**
- `windows::Services::Deployment::PackageManager` for COM debloat.
- `SRSetRestorePoint` for system restore points.
- `SHEmptyRecycleBinW` for cleaner.
- WMI (`WbemScripting.SWbemLocator` or `windows::Win32::System::Wmi`) for system overview.

---

## **13–22. Security, Admin, Logging, Error Handling, Async, Metrics, Benchmarks, Plugin, Config, Settings**

These sections remain as defined in the previous plan. See Sections 13–22 of `ZingerBoost_Master_Plan.md` for full details.

**Additions:**
- **System Cleaner** (NEW SECTION — see Section 22-A below).
- **Service Manager UI** must be added to pages list.

---

## **22-A. System Cleaner (EXISTS — NEEDS INTEGRATION)**

**Current implementation:** `SystemCleaner` in `zb_infrastructure/src/windows_api/system_cleaner.rs`.

**Categories (9):**
| ID | Name | Risk |
|----|------|------|
| `recycle_bin` | Recycle Bin | safe |
| `temp_files` | Temporary Files | safe |
| `browser_cache` | Browser Cache (Chrome, Edge, Firefox, Brave) | safe |
| `windows_temp` | Windows Temp | safe |
| `windows_logs` | Windows Logs | moderate |
| `windows_update` | Windows Update Cache | moderate |
| `prefetch` | Prefetch Data | moderate |
| `dns_cache` | DNS Cache | safe |
| `thumbnails` | Thumbnail Cache | safe |

**Issues:**
- `clean_recycle_bin` uses PowerShell `Clear-RecycleBin`. Replace with `SHEmptyRecycleBinW`.
- `scan_browser_cache` hardcodes browser paths. Should detect profiles dynamically.
- `remove_dir_contents` silently ignores errors — should report partial failures.

**Iced UI:** Add "System Cleaner" page with scan → results → clean flow.

---

## **23. Safe Tweak Categorization**

| Category | Tweaks Included (Count) |
|----------|------------------------|
| **Visual** | Disable transparency, disable animations, disable all visual effects, disable thumbnails, disable drop shadows, disable font smoothing, disable cursor shadow, disable taskbar animations, disable taskbar badges, disable combo animation, disable smooth scroll, disable menu delay, disable minimize/maximize anim, disable peek, show file extensions (15) |
| **Privacy** | Disable telemetry, disable background apps, disable advertising ID, disable explorer ads, disable lock screen ads, disable start suggestions, disable meet now, **disable Cortana (registry)**, **disable location services**, **disable activity history**, **disable tailored experiences**, **disable feedback frequency** (12) |
| **Performance** | Disable hibernation, set high performance power plan, disable startup delay (3) |
| **Gaming** | Disable Game DVR, disable Aero Shake, disable Aero Snap, **enable HW GPU scheduling**, **disable fullscreen optimizations**, **disable memory compression** (6) |
| **Debloat** | Remove UWP apps (handled by DebloatEngine, not tweak trait) |
| **Network** | **Disable Nagle's Algorithm**, **disable network throttling index**, **set TCP auto-tuning normal**, **disable Wi-Fi Sense** (4) |
| **Windows Update** | **Disable automatic driver updates**, **disable WU automatic reboot**, **disable delivery optimization** (3) |
| **Startup** | Disable sticky keys popup (1) |

**Total: 44 tweaks** (29 implemented + 15 planned).

---

## **24. Risk-Level System**

Same as previous plan. See Section 24.

---

## **25. Existing Tweaks (29 Implemented)**

All tweaks are in `crates/zb_domain/src/tweaks/definitions/` and registered in `server/src/app.rs`.

| # | File | ID | Name | Category | Risk |
|---|------|-----|------|----------|------|
| 1 | `disable_transparency.rs` | `visual_disable_transparency` | Disable Transparency | Visual | Safe |
| 2 | `disable_animations.rs` | `visual_disable_animations` | Disable Animations | Visual | Safe |
| 3 | `disable_all_visual_effects.rs` | `visual_disable_all_effects` | Disable All Visual Effects | Visual | Safe |
| 4 | `disable_thumbnails.rs` | `visual_disable_thumbnails` | Disable Thumbnails | Visual | Safe |
| 5 | `disable_drop_shadows.rs` | `visual_disable_drop_shadows` | Disable Drop Shadows | Visual | Safe |
| 6 | `disable_font_smoothing.rs` | `visual_disable_font_smoothing` | Disable Font Smoothing | Visual | Safe |
| 7 | `disable_cursor_shadow.rs` | `visual_disable_cursor_shadow` | Disable Cursor Shadow | Visual | Safe |
| 8 | `disable_taskbar_animations.rs` | `visual_disable_taskbar_anim` | Disable Taskbar Animations | Visual | Safe |
| 9 | `disable_taskbar_badges.rs` | `visual_disable_taskbar_badges` | Disable Taskbar Badges | Visual | Safe |
| 10 | `disable_combo_animation.rs` | `visual_disable_combo_anim` | Disable Combo Animation | Visual | Safe |
| 11 | `disable_smooth_scroll.rs` | `visual_disable_smooth_scroll` | Disable Smooth Scroll | Visual | Safe |
| 12 | `disable_menu_delay.rs` | `visual_disable_menu_delay` | Disable Menu Delay | Visual | Safe |
| 13 | `disable_minimax_anim.rs` | `visual_disable_minimax_anim` | Disable Min/Max Animation | Visual | Safe |
| 14 | `disable_peek.rs` | `visual_disable_peek` | Disable Peek | Visual | Safe |
| 15 | `show_file_extensions.rs` | `visual_show_extensions` | Show File Extensions | Visual | Safe |
| 16 | `disable_telemetry.rs` | `privacy_disable_telemetry` | Disable Telemetry | Privacy | Safe |
| 17 | `disable_background_apps.rs` | `privacy_disable_background_apps` | Disable Background Apps | Privacy | Safe |
| 18 | `disable_advertising_id.rs` | `privacy_disable_ad_id` | Disable Advertising ID | Privacy | Safe |
| 19 | `disable_explorer_ads.rs` | `privacy_disable_explorer_ads` | Disable Explorer Ads | Privacy | Safe |
| 20 | `disable_lock_screen_ads.rs` | `privacy_disable_lock_ads` | Disable Lock Screen Ads | Privacy | Safe |
| 21 | `disable_start_suggestions.rs` | `privacy_disable_start_suggestions` | Disable Start Suggestions | Privacy | Safe |
| 22 | `disable_meet_now.rs` | `privacy_disable_meet_now` | Disable Meet Now | Privacy | Safe |
| 23 | `disable_hibernation.rs` | `perf_disable_hibernation` | Disable Hibernation | Performance | Safe |
| 24 | `set_high_performance.rs` | `perf_high_performance` | Set High Performance | Performance | Safe |
| 25 | `disable_startup_delay.rs` | `startup_disable_delay` | Disable Startup Delay | Startup | Safe |
| 26 | `disable_game_dvr.rs` | `gaming_disable_dvr` | Disable Game DVR | Gaming | Safe |
| 27 | `disable_aero_shake.rs` | `visual_disable_aero_shake` | Disable Aero Shake | Visual | Safe |
| 28 | `disable_aero_snap.rs` | `visual_disable_aero_snap` | Disable Aero Snap | Visual | Safe |
| 29 | `disable_sticky_keys.rs` | `access_disable_sticky_keys` | Disable Sticky Keys Popup | Startup | Safe |

**Planned Tweaks (15):**

| # | File | ID | Name | Category | Risk |
|---|------|-----|------|----------|------|
| 30 | `disable_nagles_algorithm.rs` | `network_disable_nagle` | Disable Nagle's Algorithm | Network | Moderate |
| 31 | `disable_network_throttling.rs` | `network_disable_throttling` | Disable Network Throttling Index | Network | Moderate |
| 32 | `set_tcp_autotuning_normal.rs` | `network_tcp_autotuning` | Set TCP Auto-Tuning Normal | Network | Safe |
| 33 | `disable_wifi_sense.rs` | `network_disable_wifi_sense` | Disable Wi-Fi Sense | Network | Safe |
| 34 | `enable_hw_gpu_scheduling.rs` | `gaming_hw_gpu_scheduling` | Enable Hardware GPU Scheduling | Gaming | Safe |
| 35 | `disable_fullscreen_optimizations.rs` | `gaming_disable_fs_optimizations` | Disable Fullscreen Optimizations | Gaming | Safe |
| 36 | `disable_memory_compression.rs` | `perf_disable_mem_compression` | Disable Memory Compression | Performance | Moderate |
| 37 | `disable_auto_driver_updates.rs` | `wu_disable_auto_drivers` | Disable Automatic Driver Updates | WindowsUpdate | Moderate |
| 38 | `disable_wu_auto_reboot.rs` | `wu_disable_auto_reboot` | Disable WU Automatic Reboot | WindowsUpdate | Safe |
| 39 | `disable_delivery_optimization.rs` | `wu_disable_delivery_opt` | Disable Delivery Optimization | WindowsUpdate | Safe |
| 40 | `disable_cortana_registry.rs` | `privacy_disable_cortana_reg` | Disable Cortana (Registry) | Privacy | Safe |
| 41 | `disable_location_services.rs` | `privacy_disable_location` | Disable Location Services | Privacy | Safe |
| 42 | `disable_activity_history.rs` | `privacy_disable_activity_hist` | Disable Activity History | Privacy | Safe |
| 43 | `disable_tailored_experiences.rs` | `privacy_disable_tailored` | Disable Tailored Experiences | Privacy | Safe |
| 44 | `disable_feedback_frequency.rs` | `privacy_disable_feedback` | Disable Feedback Frequency | Privacy | Safe |

---

## **25-A. Disable All Animations Tweak (FIXED & ENHANCED)**

**Current implementation (`disable_animations.rs`):**
- Only modifies `UserPreferencesMask` bit 1.
- Does NOT modify `VisualFXSetting`, `TaskbarAnimations`, `DragFullWindows`, etc.

**Fix needed:** Expand to match the comprehensive list in the previous plan (Section 25-A):
1. `WindowArrangementActive` = `0`
2. `DragFullWindows` = `0`
3. `UserPreferencesMask` = `90 12 03 80 10 00 00 00`
4. `VisualFXSetting` = `2`
5. `TaskbarAnimations` = `0`
6. `ListviewShadow` = `0`
7. `ListviewAlphaSelect` = `0`

---

## **25-B. Disable Windows Ads & Widgets Tweak (PARTIALLY IMPLEMENTED)**

**Current:** `disable_explorer_ads.rs`, `disable_lock_screen_ads.rs`, `disable_start_suggestions.rs`, `disable_advertising_id.rs`, and `DebloatEngine::remove_windows_ads()` (PowerShell).

**Issue:** The registry tweaks are scattered across 4 separate tweak files. The comprehensive 23-key list from the previous plan (Section 25-B) is partially covered but not complete.

**Fix:** Create a single comprehensive `disable_windows_ads.rs` tweak that sets ALL 23 registry keys via `WinRegistryProvider`.

---

## **26. Dangerous Tweaks to AVOID**

Same as previous plan.

---

## **27. Network Optimization Tweaks**

**Safe:**
- **Disable Wi-Fi Sense** (`HKLM\SOFTWARE\Microsoft\WcmSvc\wifinetworkmanager\config\AutoConnectAllowedOEM` = `0`) — stops Windows from auto-connecting to suggested open hotspots.

**Moderate:**
- **Disable Nagle's Algorithm** (`TcpNoDelay` = `1` per interface via `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces\{GUID}\TcpNoDelay`) — reduces latency in online games and real-time apps by sending packets immediately instead of buffering.
- **Disable Network Throttling Index** (`HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\NetworkThrottlingIndex` = `0xffffffff`) — stops Windows from throttling network traffic when multimedia playback is detected.
- **Set TCP Auto-Tuning to Normal** (`netsh interface tcp set global autotuninglevel=normal`) — prevents aggressive TCP window scaling that can cause instability on some networks.

---

## **28. Gaming Optimization Tweaks**

**MVP Safe:**
- Disable Game DVR (`AllowGameDVR = 0`, `GameDVR_Enabled = 0`) — already implemented.
- Disable Xbox Game Bar (Appx + registry) — handled by DebloatEngine.
- **Enable Hardware-Accelerated GPU Scheduling** (`HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers\Scheduler\HwSchMode` = `2`) — Win10 2004+/Win11 feature that reduces input latency by offloading GPU scheduling to hardware.
- Set High Performance power plan — already implemented.

**Post-MVP Safe:**
- **Disable Fullscreen Optimizations** (`HKCU\System\GameConfigStore\GameDVR_FSEBehaviorMode` = `2`) — forces true exclusive fullscreen in legacy games, reducing latency.
- **Disable Memory Compression** (`HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\DisableCompression` = `1`) — stops Windows from compressing inactive memory pages. May reduce stutter in RAM-heavy games at the cost of higher swap usage.

**Advanced (v1.1):**
- Timer resolution adjustments (`NtSetTimerResolution`)
- Disable HPET via `bcdedit`
- CPU priority separation tweaks
- Disable dynamic tick (`bcdedit /set disabledynamictick yes`)

---

## **29. Privacy Optimization Tweaks**

**Implemented:**
- Disable Telemetry (`AllowTelemetry = 0`)
- Disable Diagnostic Tracking Service (`DiagTrack`)
- Disable push-to-install service (`dmwappushservice`)
- Disable Advertising ID
- Disable Background Apps
- Disable Explorer Ads, Lock Screen Ads, Start Suggestions
- Disable Meet Now

**Planned:**
- **Disable Cortana (Registry)** (`HKLM\SOFTWARE\Policies\Microsoft\Windows\Windows Search\AllowCortana` = `0`, `HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Search\BingSearchEnabled` = `0`, `HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Search\CortanaConsent` = `0`) — deep disable beyond just removing the AppX package.
- **Disable Location Services** (`HKLM\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors\DisableLocation` = `1`) — prevents apps and Windows from accessing GPS/location data.
- **Disable Activity History / Timeline** (`HKLM\SOFTWARE\Policies\Microsoft\Windows\System\PublishUserActivities` = `0`, `HKLM\SOFTWARE\Policies\Microsoft\Windows\System\EnableActivityFeed` = `0`) — stops Windows from recording and syncing app usage history across devices.
- **Disable Tailored Experiences** (`HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Privacy\TailoredExperiencesWithDiagnosticDataEnabled` = `0`) — stops Microsoft from using diagnostic data to deliver personalized tips, ads, and recommendations.
- **Disable Feedback Frequency** (`HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Feedback\FrequencyPeriod` = `0`, `HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Feedback\FrequencyPeriodLastTime` = `0`) — stops Windows from prompting for feedback and sending "occasional" diagnostic data to Microsoft.

---

## **30. Debloat System (DEDICATED — NEEDS MAJOR REFACTOR)**

**Current:** `DebloatEngine` in `zb_infrastructure/src/windows_api/debloat_engine.rs`.

**Current methods:**
1. `try_winget_uninstall()` — `winget uninstall`.
2. `try_powershell_remove()` — `Get-AppxPackage | Remove-AppxPackage` ❌.
3. `try_dism_remove()` — `dism /Online /Remove-ProvisionedAppxPackage` ✅.
4. `try_registry_remove()` — PowerShell registry delete ❌.
5. `try_filesystem_remove()` — `takeown` + `icacls` + `rmdir` ✅ (brute force).

**Also exists:**
- `remove_windows_ads()` — PowerShell script ❌.
- `remove_widgets()` — PowerShell script ❌.

**Refactor plan:**
1. Replace Method 2 (PowerShell AppX) with `windows-rs` COM `PackageManager::RemovePackageAsync`.
2. Replace Method 4 (registry delete) with native `WinRegistryProvider::delete`.
3. Replace `remove_windows_ads()` with native registry writes.
4. Replace `remove_widgets()` with `PackageManager` + native registry.
5. Keep Method 1 (winget) and Method 3 (DISM) and Method 5 (filesystem).

---

## **31. Restore Removed Apps System**

Not yet implemented. Store removed package names in SQLite. Offer reinstall via Microsoft Store URI.

---

## **32. App Installation Manager**

**Current:** `WingetInstaller` in `zb_infrastructure/src/windows_api/winget.rs`.
- `is_available()` — checks `winget --version`.
- `install(package_id)` — `winget install --id <id> --silent`.
- `remove_appx()` — PowerShell ❌.
- `remove_provisioned_appx()` — PowerShell ❌.

**Software catalog:** Already fully implemented in `zb_shared/src/software.rs` with 30+ apps across 9 categories.

**Iced UI:** Needs a dedicated "Install Apps" page with category tabs.

---

## **33–35. Update, Installer, Auto-Update**

Not yet implemented. Keep previous plan sections.

---

## **36. Database/Storage Approach**

**Current:** SQLite 3 (`data.db` in `%LOCALAPPDATA%\ZingerBoost`).
- Tables: `snapshots`, `snapshot_tweaks`, `tweak_states`, `audit_log`.
- WAL mode enabled. Foreign keys enabled.
- Migrations via `rusqlite_migration`.

**Missing:** `benchmark_runs`, `settings`, `removed_apps` tables.

---

## **37–39. Caching, Telemetry, Offline**

Same as previous plan.

---

## **40. Recommended Pages/Screens (Tauri Target)**

1. **Overview** — Live metrics, system overview, quick actions, recommended actions, preset selector.
2. **Optimizations** — Filterable tweak catalog with risk badges, apply/revert, category tabs.
3. **Tweak Detail** — Explanation, technical details, affected registry keys.
4. **Debloat** — Bloatware removal with checkboxes, scan + remove flow.
5. **Installer** — Software installer with categories, winget-based install.
6. **Cleaner** — Scan and clean 9 categories, size estimates, progress tracking.
7. **Services** — List, stop, and disable Windows services, status indicators.
8. **Backups** — Timeline, restore wizard, export/import JSON.
9. **Game Mode** — Per-game profiles, auto-apply on process detection, temporary tweaks.
10. **Monitor** — Real-time hardware monitoring (CPU, RAM, GPU, network, disk).
11. **Settings** — General, appearance (Dark/Light/System), safety, backups, about.
12. **Audit Log** — Live audit stream, filter by level/category, clear log.

---

## **41. Dashboard Design Ideas**

**Current:** `MetricsCollector` returns CPU %, RAM %, Disk %, Network up/down. Polled every 5s via `setInterval` in `app.js`.

**Missing:**
- System Overview (CPU name, cores, GPU, motherboard, OS build, uptime, disk free space).
- Sparkline history for metrics (1-hour trend).
- Circular progress indicators for CPU, RAM, Disk.
- System Health Score (0-100 based on optimizations applied).
- Quick Actions panel (Clean Temp, Boost Now, Scan Issues).
- Auto-refresh via Tauri event emission instead of polling.

**Fix:** Add `SystemOverviewCollector` in `zb_infrastructure/src/windows_api/` using WMI. Replace polling with Tauri `app.emit()` for real-time updates.

---

## **42. Sidebar/Navigation Structure (Current)**

```
┌─ ZingerBoost ──────────────┐
│  [Icon] Overview           │
│  [Icon] Optimizations      │
│  [Icon] Services           │
│  [Icon] Cleaner            │
│  [Icon] Backups            │
│  [Icon] Debloat            │
│  [Icon] Installer          │
│  [Icon] Game Mode          │  ← NEW
│  [Icon] Monitor            │  ← NEW
│  ───────────────────────── │
│  [Icon] Settings           │
│  [Icon] Audit Log          │
└────────────────────────────┘
```

---

## **43. Color System & Design Language**

**Current:** Dark purple theme via CSS custom properties in `style.css`.

```css
:root {
  --bg-primary: #0F172A;      /* slate-950 */
  --bg-card: #1E293B;         /* slate-900 */
  --primary: #3B82F6;         /* blue-500 */
  --secondary: #10B981;       /* emerald-500 */
  --accent: #8B5CF6;          /* purple-500 */
  --text-primary: #F8FAFC;    /* slate-50 */
  --text-muted: #94A3B8;      /* slate-400 */
  --warning: #F59E0B;         /* amber-500 */
  --danger: #EF4444;          /* red-500 */
}
```

**Target:** Add Light and System theme variants. Theme toggle in Settings page. CSS variables switch via `data-theme` attribute on `<html>`.

---

## **44–46. Animations, Charts, Widgets**

Same as previous Iced plan. Use `iced_anim` + custom `Canvas` for sparklines.

---

## **47. Development Roadmap (Updated)**

| Week | Focus |
|------|-------|
| **1** | Refactor `DebloatEngine` to native `windows-rs` COM + DISM. Add `SystemOverviewCollector` (WMI). Fix `DisableAnimationsTweak`. |
| **2** | Add file-based snapshot verification. Add `settings` table. Add `removed_apps` table. Add System Restore Point FFI. |
| **3** | UI: Overview page with circular progress, sparkline charts, system health score, quick actions. |
| **4** | UI: Optimizations page with category tabs, risk badges, apply/revert. Debloat page with checkboxes. Cleaner page. |
| **5** | UI: Installer page, Services page, Backups page with export/import. Settings page with theme toggle. |
| **6** | UI: Game Mode page with per-game profiles. Monitor page with GPU/network metrics. Audit Log page. |
| **7** | Preset System (Safe/Balanced/Aggressive). Background tray app (`zb_tray`). Polish, animations, responsive design. |
| **8** | Inno Setup installer (`setup.iss`). Documentation. Testing. Release v1.0.0. |

---

## **48. MVP Roadmap (v1.0.0 — Tauri Release)**

**Must-have for v1.0.0:**
- [ ] Refactor `DebloatEngine` — remove PowerShell, use native COM + DISM
- [ ] Fix `DisableAnimationsTweak` — comprehensive animation removal
- [ ] Add `SystemOverviewCollector` (WMI)
- [ ] Add file-based snapshot verification
- [ ] Overview page with live metrics + system overview + circular progress
- [ ] Optimizations page with apply/revert (**49 tweaks**: register all)
- [ ] Debloat page with checkboxes
- [ ] Cleaner page with 9 categories
- [ ] Services page with status indicators
- [ ] Installer page with software catalog
- [ ] Backups page with export/import
- [ ] Game Mode page
- [ ] Monitor page with GPU/network metrics
- [ ] Settings page with Dark/Light/System theme toggle
- [ ] Preset System (Safe/Balanced/Aggressive)
- [ ] Inno Setup installer

---

## **49. Scaling Roadmap**

**v0.0.6:** Tauri migration complete. 30 commands, vanilla JS frontend.
**v1.0.0:** Full UI redesign. 49 tweaks, Game Mode, Monitor, Presets, theme system.
**v1.1.0:** Background tray app (`zb_tray`), Windows Restore Point integration, benchmark system.
**v1.2.0:** OneDrive removal, Search indexing tweak, advanced debloat (system apps).
**v1.3.0:** Timer resolution, HPET, CPU priority, dynamic tick (advanced gaming).
**v2.0.0:** Plugin SDK (WASM), community tweak repository, enterprise features.

---

## **50. Testing Strategy**

Same as previous plan.

---

## **51. CI/CD Suggestions**

**Current:** GitHub Actions with `windows-latest` runner for `cargo check --workspace`.

**Update needed:**
- `ci.yml` must build `zb_app` (Tauri) on Windows with `cargo tauri build`.
- Add Tauri CLI installation step.
- Add Inno Setup for `setup.iss` packaging.
- Frontend has no build step (vanilla JS), so no `npm install` needed.

---

## **52. Release Strategy**

Same as previous plan.

---

## **53. Example Rust Modules**

Same as previous plan. Add `SystemOverviewCollector` example.

---

## **54. Example Tauri Application Entry**

```rust
// zb_app/src/main.rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check admin elevation
    if !is_admin() {
        relaunch_as_admin();
        return Ok(());
    }

    // Initialize logging
    init_logging();

    // Initialize SQLite database
    let repo = SqliteRepo::new(&data_dir())?;

    // Build TweakEngine with all tweaks
    let provider = Arc::new(WinRegistryProvider::new());
    let engine = Arc::new(TweakEngine::new(make_all_tweaks(provider.clone()), Arc::new(repo.clone())));

    // Build AppState
    let state = AppState {
        engine,
        metrics: Arc::new(MetricsCollector::new()),
        cleaner: Arc::new(SystemCleaner::new()),
        // ...
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_tweaks,
            apply_tweak,
            revert_tweak,
            get_metrics,
            get_services,
            // ... 30 commands total
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application")
}
```

**Tauri Command Pattern:**
```rust
#[tauri::command]
async fn get_tweaks(state: tauri::State<'_, AppState>) -> Result<serde_json::Value, String> {
    let tweaks = state.engine.list_tweaks();
    serde_json::to_value(&tweaks).map_err(|e| e.to_string())
}
```

**Frontend Invoke Pattern:**
```javascript
const tweaks = await window.__TAURI__.core.invoke('get_tweaks');
```

---

## **55. Existing Code Patterns**

### Tweak struct pattern:
```rust
pub struct DisableGameDvrTweak {
    pub provider: Option<Arc<dyn RegistryProvider>>,
}
impl DisableGameDvrTweak {
    pub fn new() -> Self { Self { provider: None } }
    pub fn with_provider(provider: Arc<dyn RegistryProvider>) -> Self { ... }
}
```

### Registry read/write pattern:
```rust
let path = RegPath::hkcu(r"System\GameConfigStore");
provider.read(&path, "GameDVR_Enabled").await?;
provider.write(&path, "GameDVR_Enabled", &RegValue::Dword(0)).await?;
```

---

## **56. Tech Debt & Known Issues**

| # | Issue | Severity | Fix |
|---|-------|----------|-----|
| 1 | `DebloatEngine` relies heavily on PowerShell | **CRITICAL** | Refactor to `windows-rs` COM `PackageManager` + native registry |
| 2 | `DisableAnimationsTweak` only clears one bit | **HIGH** | Expand to full animation removal (Section 25-A) |
| 3 | `SystemCleaner::clean_recycle_bin` uses PowerShell | **MEDIUM** | Replace with `SHEmptyRecycleBinW` |
| 4 | `WingetInstaller::remove_appx` uses PowerShell | **MEDIUM** | Move removal logic to `DebloatEngine` |
| 5 | Snapshots are SQLite-only, no file verification | **MEDIUM** | Add `SnapshotFileStore` + verification |
| 6 | 49 tweaks defined but only 32 registered | **MEDIUM** | Register remaining 17 tweaks in `make_all_tweaks()` |
| 7 | `SystemCleaner` silently ignores all errors | **MEDIUM** | Return `Result` with partial failure reporting |
| 8 | `MetricsCollector::current()` blocks for 100ms (sleep) | **MEDIUM** | Move PDH sleep to background task |
| 9 | `iced_test/` crate is orphaned dead code | **FIXED** | Removed from repo |
| 10 | `AGENTS.md` references Iced but project uses Tauri | **FIXED** | `AGENTS.md` correctly describes Tauri v2 stack |
| 11 | Frontend uses polling (5s `setInterval`) for metrics | **LOW** | Replace with Tauri `app.emit()` for push-based updates |
| 12 | No Light/System theme (dark-only CSS) | **LOW** | Add theme variants with CSS variables |

---

## **57. Recommended Rust Crates (Current + Target)**

| Purpose | Current Crate | Target Crate |
|---------|--------------|--------------|
| Desktop Framework | `tauri` v2 | `tauri` v2 |
| Async | `tokio` | `tokio` |
| Windows API | `windows` | `windows` |
| DB | `rusqlite` + `rusqlite_migration` | `rusqlite` + `rusqlite_migration` |
| Serialization | `serde`, `serde_json` | `serde`, `serde_json` |
| Errors | `thiserror`, `anyhow` | `thiserror`, `anyhow` |
| Logging | `tracing`, `tracing-subscriber` | `tracing`, `tracing-subscriber` |
| Testing | `mockall` | `mockall` |
| Time | `chrono` | `chrono` |
| UUID | `uuid` | `uuid` |
| HTTP | `reqwest` (update checks only) | `reqwest` |
| Regex | `regex` | `regex` |
| System Tray | — | `tauri-plugin-single-instance` + tray API |
| File Dialogs | — | `rfd` (via Tauri dialog API) |

---

## **58. Strict AI Agent Rules**

**These rules apply to ANY AI agent working on this codebase:**

1. **STACK LOCK:** The tech stack is **Rust + Tauri v2** with vanilla JS frontend. You are forbidden from changing it.
2. **NO SUBSTITUTIONS:** Do NOT switch to Iced, React, TypeScript, Flutter, HTML-only, Raui, Electron, .NET MAUI, Qt, GTK, egui, or any other framework.
3. **UI IN VANILLA JS ONLY:** All UI code MUST be in `.html`, `.js`, `.css` files. No React, Vue, Svelte, Angular, or any frontend framework.
4. **PRESERVE ARCHITECTURE:** Do not rename or restructure the `zb_*` crates. New features must fit inside the existing boundaries.
5. **DEBLOAT IS NATIVE:** The debloat engine MUST use `windows-rs` COM APIs (`PackageManager`) or `dism.exe`. NEVER use PowerShell `Remove-AppxPackage` pipelines.
6. **SNAPSHOTS ARE MANDATORY:** Every batch tweak apply and every debloat operation MUST create a verified snapshot BEFORE modifying the system. If snapshot fails, abort.
7. **DASHBOARD IS LIVE:** The System Overview on the Dashboard MUST query live WMI / Performance Counter data from the Rust backend and refresh via Tauri events or polling. Static placeholder text is a bug.
8. **THEME SUPPORT:** The app MUST support Dark, Light, and System themes via CSS custom properties.
9. **MINIMAL CHANGES:** Only modify what is explicitly requested. Do not refactor unrelated code.
10. **ASK BEFORE CHANGING:** If a request conflicts with these rules or the existing architecture, STOP and ask the user. Do not "fix" it by switching technologies.
11. **NO COMMITS:** Never run `git commit`, `git push`, or any git mutations unless explicitly told to.
12. **RESPECT EXISTING CODE:** The project already has 49 tweaks (32 registered), a software catalog, a bloatware catalog, a service controller, a metrics collector, a system cleaner, and 30 Tauri commands. Do not rewrite them unless asked. Build ON TOP of them.

---

## **Final Summary**

**ZingerBoost v0.0.6** has a **solid Rust backend + Tauri v2 desktop app** with:
- 49 tweaks defined (32 registered), all reversible.
- Full registry provider (`windows-rs`).
- Tweak engine with batch apply and auto-rollback.
- SQLite persistence with migrations.
- Metrics collector (CPU/RAM/Disk via PDH).
- Service controller (19 safe-to-disable services).
- System cleaner (9 categories).
- Software catalog (30+ apps, 9 categories).
- Bloatware catalog (44 apps).
- Debloat engine (5 methods, but PowerShell-heavy — needs refactor).
- Tauri v2 desktop app with vanilla JS frontend (30 commands).
- Dark purple theme via CSS variables.

**What needs to be built:**
- Refactor debloat to native Windows APIs (remove PowerShell).
- Fix animation tweak comprehensiveness.
- Add system overview (WMI) with GPU metrics.
- Add file-based snapshot verification.
- Add Game Mode, Monitor, Preset System pages.
- Add Light/System theme toggle.
- Add backup export/import.
- Build background tray app (`zb_tray`).
- Package with Inno Setup installer.

**Author:** YousefMohiey

============================================================
