# ZingerBoost — AI Handoff (2026-05-17)

> **Commit:** `94186e5` | **Branch:** `main` | **Repo:** github.com/YousefMohiey/ZingerBoost

## What This Is

A Windows system optimization desktop app. Think "Windows debloater + tweaker + cleaner" in a single GUI.
Built in **Rust + Iced 0.13** (pure Rust, no web tech). Replaces the old actix-web server with a native desktop app.

## Tech Stack (LOCKED — NEVER CHANGE)

| Layer | Tech |
|-------|------|
| Language | Rust (edition 2021) |
| GUI | Iced 0.13 (Elm architecture) |
| Async | Tokio |
| Windows APIs | windows-rs 0.58 |
| Database | SQLite via rusqlite (bundled) |
| Build | Cargo workspace |

**Forbidden:** Tauri, React, Flutter, Electron, TypeScript, HTML, CSS, JavaScript, Python, C#, Qt, GTK, egui, Slint.

## Architecture (5 Crates)

| Crate | Purpose |
|-------|---------|
| `zb_shared` | Types, constants, software catalog (30+ apps), bloatware catalog (34 apps) |
| `zb_domain` | `Tweak` trait + 29 implementations, `RegistryProvider` trait, snapshot entities |
| `zb_application` | `TweakEngine`, `SnapshotService`, `AuditService` |
| `zb_infrastructure` | `WinRegistryProvider`, `ServiceController`, `DebloatEngine`, `SystemCleaner`, `WingetInstaller`, `MetricsCollector`, SQLite persistence |
| `zb_app` | Iced desktop app (State/Message/Update/View) |

## What Was Fixed (This Session)

- Compile error: missing `use tracing` in `service_controller.rs`
- Registry handle leak in `WinRegistryProvider::read()`
- `RegOpenKeyExW` errors all mapped to `AccessDenied` — now properly categorized
- GUI now wires through `TweakEngine` with SQLite persistence — applying tweaks creates snapshots + audit logs
- `restore_snapshot()` previously returned Ok for non-existent IDs
- Added `RegValue::MultiSz` variant for `REG_MULTI_SZ` support
- Wrong winget_ids: Paint3D (`MSPaint` → `MSPaint3D`), Ads & Widgets (was `Cortana`)
- All hardcoded `C:\` paths replaced with `SystemDrive`/`windir` env vars
- `hiberfil.sys` path now uses SystemDrive
- `std::thread::sleep` in async fn → `tokio::task::spawn_blocking`
- PDH handle leaks: `PdhCloseQuery` + `PdhRemoveCounter` now called
- `apply_batch` now calls `save_applied` for each tweak (individual revert after batch works)
- All `std::process::Command` calls use `CREATE_NO_WINDOW (0x08000000)` — no console pop-ups
- Status display fixed: notification bar at top with dismiss (X) button instead of replacing page
- Recycle Bin cleaner: removed broken `cmd /c rd` approach, only uses `powershell Clear-RecycleBin`
- Clean operations wrapped in `spawn_blocking` to avoid UI freezes
- Static CRT linking via `.cargo/config.toml` (no VC++ redistributable needed)
- Added 15 integration tests (`crates/zb_app/tests/integration_test.rs`), all pass
- UTF-16 buffer decoding fixed (`chunks_exact` → `chunks(2)`)
- `restore_snapshot` now properly checks snapshot existence in `snapshots` table first

## Current State

```
cargo check --workspace      → PASSES (zero warnings)
cargo fmt --all -- --check   → PASSES
cargo clippy -p zb_shared -p zb_domain -p zb_application --all-targets -- -D warnings → PASSES
cargo build --release -p zb_app → PASSES
cargo test -p zb_app          → 15/15 PASS
```

Binary: `target\release\zb_app.exe` (~15.7 MB, statically linked CRT)

## How to Build

```powershell
cd ZingerBoost
cargo build --release -p zb_app
# Output: target\release\zb_app.exe
```

## How to Test

```powershell
cargo test -p zb_app
cargo fmt --all -- --check
cargo clippy -p zb_shared -p zb_domain -p zb_application --all-targets -- -D warnings
```

## What Still Needs Work (Known Issues)

### Critical
- [ ] `audit_logger.rs`: `new()`/`new_in_memory()` never run migrations — audit_log table may not exist
- [ ] `audit_logger.rs`: `log()` silently discards INSERT errors via `let _ =`
- [ ] `audit_logger.rs`: `get_recent()` silently discards `.prepare()` and `.query_map()` errors

### High
- [ ] PowerShell code injection in `debloat_engine.rs` — names interpolated without escaping `$`, `(`, `)`, etc.
- [ ] `remove_windows_ads()` PowerShell doesn't create parent registry keys first (silently fails on fresh install)
- [ ] Blocking `rusqlite` calls in async context — should use `spawn_blocking`
- [ ] `get_recent()` corrupts unparseable timestamps to epoch zero

### Medium
- [ ] N+1 query in `list_snapshots()` — could use single JOIN query
- [ ] `tweak_states.last_snapshot_id` column never populated
- [ ] `items_removed` always 0 in `CleanResult`
- [ ] `bytes_freed` reports before-size, not actual delta
- [ ] `scan_thumbnail_cache` scans entire Explorer appdata, not just thumbnails
- [ ] `remove_dir_contents` silently ignores all per-file errors
- [ ] No `#[cfg(target_os = "windows")]` guards on Windows-only crates
- [ ] `LOGS_DIR` constant defined but no file-based logging configured
- [ ] `SoftwarePackage` struct missing `version` field
- [ ] `AuditEntry` missing `id` field (DB has it)

### Low
- [ ] `get_protected_apps()` returns `&str` not `SoftwarePackage` (inconsistent)
- [ ] `Id` type alias is `String` but `SystemSnapshot.id` uses `Uuid` directly
- [ ] `AUDIT_LOG_RETENTION_DAYS` is signed `i64` (should be unsigned)
- [ ] `bloat_candycrush` winget_id has unusual casing
- [ ] Unused deps in `zb_shared/Cargo.toml` (`thiserror`, `uuid`)

## Absolute Rules for AI Agents

1. **NEVER change the tech stack.** Rust + Iced only. No React, Tauri, Flutter, Electron, etc.
2. **All UI code MUST be Rust (.rs) using Iced widgets.**
3. **Do NOT rewrite existing crates.** Build ON TOP of them.
4. **Debloat engine MUST use native Windows APIs** — no PowerShell for app removal.
5. **Snapshots are MANDATORY before any system modification.**
6. **Dashboard System Overview MUST use live data** from WMI/Performance Counters.
7. **Theme Support:** Dark, Light, System via Iced custom `Palette`/`Theme`.
8. **Only make MINIMAL changes.** Do not refactor unrelated code.
9. **NEVER run `git commit`/`git push`** unless explicitly told.
10. **If a request conflicts with these rules, STOP and ask.**

## Emergency Stop Phrases

> **"STACK LOCK: We are using Rust + Iced. Do not change the stack."**
> **"EXISTING CODE: crates are already built. Do not rewrite them."**
> **"MINIMAL CHANGE: Only edit the files I asked about."**

## Iced 0.13 Gotchas

1. ALL views must be inline in `view(&self) -> Element<Message>` — standalone functions cause E0106
2. No explicit lifetimes on `Element` — use `Element<Message>` not `Element<'static, Message>`
3. `Border::default()` not `Border::rounded()` — API changed
4. Button `on_press` returns `Element<Message>` via `.into()`
5. `Task::perform` for async operations, `Subscription` for periodic timers
6. `Message` must derive `Debug + Clone`

## Adding a New Tweak

1. Create file in `crates/zb_domain/src/tweaks/definitions/`
2. Implement `Tweak` trait (6 methods: `metadata`, `is_applied`, `capture_state`, `apply`, `revert`, `explain`)
3. Add `pub fn new()` and `pub fn with_provider(Arc<RegistryProvider>)`
4. Register in `definitions/mod.rs`
5. Register in `zb_app/src/lib.rs` `make_all_tweaks()` function
