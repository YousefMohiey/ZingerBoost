# ZingerBoost — Agent Instructions (v0.2.0 Flutter)

## Quick Check (Linux)

```bash
cargo check -p zb_shared -p zb_domain -p zb_application
cargo fmt --all
cargo clippy -p zb_shared -p zb_domain -p zb_application --all-targets -- -D warnings
```

## Windows-Only Crates

`zb_infrastructure`, `bridge` depend on `windows-rs` — only compile on Windows. CI runs `cargo check --workspace` on `windows-latest`.

## Architecture (v0.2.0 — Flutter + Rust FFI)

```
bridge/                      # Rust FFI bridge (cdylib + staticlib)
  Cargo.toml                 # Depends on all 4 core crates
  src/lib.rs                 # AppState + OnceLock<AppState> + init_app() FFI
  src/api.rs                 # 11 FFI functions returning JSON Strings
crates/
  zb_shared/                 # Types, constants, software catalog
  zb_domain/                 # Tweak trait + 25 implementations + RegistryProvider trait
  zb_application/            # TweakEngine, SnapshotService, AuditService
  zb_infrastructure/         # WinRegistryProvider, SQLite, Winget, PDH, DebloatEngine
zingerboost_flutter/         # Flutter desktop app
  lib/
    models/                  # TweakMetadata, SystemMetrics, SystemSnapshot, etc.
    services/rust_bridge.dart # FFI wrapper (flutter_rust_bridge-ready)
    pages/                   # 6 pages: dashboard, tweaks, snapshots, debloat, software, settings
    widgets/                 # app_sidebar, metric_card, risk_badge, tweak_card, toast_overlay
    theme/                   # Dark/light ThemeData + Riverpod theme provider
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

### 5. `package-lock.json` is gitignored
Platform-specific. CI uses `npm install` not `npm ci`.

### 6. PostCSS/Tailwind configs must be `.cjs`
`package.json` has `"type": "module"`. CommonJS configs need `.cjs` extension.

### 7. Shared SQLite connection
Both `SqliteRepo` and `SqliteAuditLogger` share one `Arc<Mutex<Connection>>` via `from_connection()`.

### 8. Tweak structs: no `#[derive(Debug)]` if containing `Arc<dyn RegistryProvider>`
Trait objects don't implement Debug.

### 9. RegistryProvider trait lives in `zb_domain`
Not `zb_infrastructure` — avoids circular dependency.

### 10. Bridge crate uses `OnceLock<AppState>` for global state
`init_app()` must be called before any API function. All API functions access `APP.get().unwrap()`.

## Adding a New Tweak

1. Create file in `crates/zb_domain/src/tweaks/definitions/`
2. Implement `Tweak` trait (6 methods)
3. Add `pub fn new() -> Self` + `pub fn with_provider(provider: Arc<RegistryProvider>) -> Self`
4. Register in `definitions/mod.rs`
5. Register in `bridge/src/lib.rs` tweaks vec

## CI

| Job | Runner | Command |
|-----|--------|---------|
| `check` | ubuntu | `cargo fmt --all -- --check` + `cargo clippy -p zb_shared -p zb_domain -p zb_application --all-targets -- -D warnings` |
| `check-all` | windows | `cargo check --workspace` |

## Release

Trigger: push tag `v*` or `workflow_dispatch`. Builds via `flutter build windows` + `cargo build --release` for the `.dll`.
