# ZingerBoost — Agent Instructions (v0.4.0)

> **Last Updated:** 2026-05-10  
> **Target UI:** Iced (Rust) — `server/` (actix-web) is transitional and will be deleted.

---

## Quick Check (Linux)

```bash
cargo check -p zb_shared -p zb_domain -p zb_application
cargo fmt --all
cargo clippy -p zb_shared -p zb_domain -p zb_application --all-targets -- -D warnings
```

## Windows-Only Crates

`zb_infrastructure` and `server` depend on `windows-rs` — only compile on Windows. CI runs `cargo check --workspace` on `windows-latest`.

## Architecture (v0.4.0 — Rust Backend + actix-web server → Iced)

```
crates/
  zb_shared/                 # Types, constants, software catalog, bloatware catalog
    src/software.rs          # 30+ apps, 9 categories + 34 bloatware apps + protected list
    src/types.rs             # RegPath, RegValue, RiskLevel, TweakCategory, SystemMetrics, etc.
    src/constants.rs
  zb_domain/                 # Tweak trait + 29 implementations + RegistryProvider trait
    src/tweaks/traits.rs     # Tweak trait (6 methods)
    src/tweaks/definitions/  # 29 tweak .rs files
    src/registry.rs          # RegistryProvider trait
    src/snapshots/           # SystemSnapshot, AppliedTweakRecord
    src/benchmarks/          # Benchmark traits
    src/errors.rs            # TweakError, SnapshotError, RegistryError, ServiceError, BenchmarkError
  zb_application/            # TweakEngine, SnapshotService, AuditService
    src/tweak_engine.rs      # Batch apply, auto-rollback, snapshot save, audit log
    src/snapshot_service.rs  # SnapshotService trait
    src/audit_service.rs     # AuditService trait
    src/dto.rs
  zb_infrastructure/         # WinRegistryProvider, SQLite, Winget, PDH, DebloatEngine, SystemCleaner
    src/registry/mod.rs      # WinRegistryProvider (windows-rs)
    src/services/            # ServiceController (SCM API + sc.exe)
    src/persistence/         # SqliteRepo, SqliteAuditLogger, init_database
    src/windows_api/
      debloat_engine.rs      # 5-method removal (PowerShell-heavy — needs refactor)
      metrics_collector.rs   # PDH CPU/RAM/Disk counters
      system_cleaner.rs      # 9-category disk cleaner
      winget.rs              # WingetInstaller
    src/logging.rs           # tracing_subscriber init

server/                      # CURRENT UI — actix-web HTTP server (WILL BE DELETED)
  src/lib.rs                 # HttpServer on 127.0.0.1:19999
  src/app.rs                 # AppState (engine, metrics, winget, cleaner, services)
  src/api.rs                 # REST endpoints (BUG: uses block_on inside async)

zb_app/                      # TARGET UI — Iced desktop app (DOES NOT EXIST YET)
```

## Critical Gotchas

### 1. `#![allow(clippy::new_without_default)]` in `zb_domain/src/lib.rs`
Every tweak struct has `pub fn new()` without `Default`.

### 2. Migrations MUST be `fn migrations()`, not `const`
```rust
fn migrations() -> Migrations<'static> { Migrations::new(vec![...]) }
// NOT: const MIGRATIONS: Migrations<'static> = ...
```
Rust 2024 forbids `Migrations::new()` in const.

### 3. `REG_SAM_FLAGS`, NOT bare `u32`
```rust
fn open_key(&self, path: &RegPath, access: REG_SAM_FLAGS) -> Result<HKEY, ...>
```

### 4. `init_database()` returns `anyhow::Error`
Because `rusqlite_migration::Error` has no `From` into `rusqlite::Error`.

### 5. Shared SQLite connection
Both `SqliteRepo` and `SqliteAuditLogger` share one `Arc<Mutex<Connection>>` via `from_connection()`.

### 6. Tweak structs: no `#[derive(Debug)]` if containing `Arc<dyn RegistryProvider>`
Trait objects don't implement Debug.

### 7. RegistryProvider trait lives in `zb_domain`
Not `zb_infrastructure` — avoids circular dependency.

### 8. `server/src/api.rs` has a CRITICAL BUG
Uses `tokio::runtime::Handle::current().block_on()` inside async handlers. This blocks the async runtime. Fix: remove `block_on` and `.await` directly.

### 9. `server/` will be deleted after Iced migration
Do not invest in improving `server/` unless explicitly asked. Focus on `zb_app/` (Iced).

### 10. `zb_app` crate does NOT exist yet
It must be created as a new workspace member.

### 11. PowerShell is FORBIDDEN for debloat
`DebloatEngine` currently uses PowerShell heavily. This is tech debt. Refactor to `windows-rs` COM `PackageManager` + native registry.

## Adding a New Tweak

1. Create file in `crates/zb_domain/src/tweaks/definitions/`
2. Implement `Tweak` trait (6 methods)
3. Add `pub fn new() -> Self` + `pub fn with_provider(provider: Arc<RegistryProvider>) -> Self`
4. Register in `definitions/mod.rs`
5. Register in `server/src/app.rs` tweaks vec (temporary — will move to `zb_app`)

## CI

| Job | Runner | Command |
|-----|--------|---------|
| `check` | ubuntu | `cargo fmt --all -- --check` + `cargo clippy -p zb_shared -p zb_domain -p zb_application --all-targets -- -D warnings` |
| `check-all` | windows | `cargo check --workspace` |

## Release

Trigger: push tag `v*` or `workflow_dispatch`.  
Current: builds `cargo build --release -p zingerboost` (server binary).  
Target: builds `cargo build --release -p zb_app` (Iced desktop app) + `cargo-wix` MSI.

## Strict Rules

1. **STACK LOCK:** Rust + Iced. No Flutter, React, Tauri, Electron, etc.
2. **NO REWRITES:** `zb_shared`, `zb_domain`, `zb_application`, `zb_infrastructure` are done. Build ON TOP.
3. **NO POWERSHELL DEBLOAT:** Use `windows-rs` COM + DISM.
4. **MINIMAL CHANGES:** Only edit what is asked.
5. **NO GIT MUTATIONS** unless explicitly told.
