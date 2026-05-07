# ZingerBoost — Agent Instructions

## Quick Check (Linux — run before every push)

```bash
cargo check -p zb_shared -p zb_domain -p zb_application
cargo fmt --all
cargo clippy -p zb_shared -p zb_domain -p zb_application --all-targets -- -D warnings
npx tsc --noEmit
```

## Windows-Only Crates

`zb_infrastructure` and `src-tauri` depend on `windows-rs` and only compile on **Windows**. On Linux, only check the cross-platform crates above. CI runs `cargo check --workspace` on `windows-latest` to catch full build errors.

## Architecture: Tauri v2 Standard Layout

```
src-tauri/          ← Tauri binary + config (cargo tauri dev works from root)
  tauri.conf.json   → frontendDist: "../dist", devUrl: "http://localhost:1420"
crates/zb_app/      ← Library only: AppState + command handlers (NO binary)
crates/zb_domain/   ← Tweak trait, entities, RegistryProvider trait
crates/zb_infrastructure/ ← WinRegistryProvider, SQLite, Winget, Metrics
```

`zb_app` is a **library** — its old `main.rs` and `build.rs` were deleted. The binary lives in `src-tauri/`.

## Dependency Rules

- `RegistryProvider` trait lives in **`zb_domain`**, not `zb_infrastructure` (avoids circular dep)
- `zb_infrastructure` depends on ALL: `zb_shared`, `zb_domain`, `zb_application`
- `src-tauri/Cargo.toml` depends on `zb_app`, `zb_domain`, `zb_application`, `zb_infrastructure`

## Critical Gotchas

### 1. `#![allow(clippy::new_without_default)]` in `zb_domain/src/lib.rs`
Do NOT remove this. Every tweak struct has `pub fn new()` without `Default`. Clippy fails otherwise.

### 2. Migrations MUST be a `fn`, not `const`
```
// WRONG: const MIGRATIONS: Migrations<'static> = Migrations::new(vec![...]);
// RIGHT: fn migrations() -> Migrations<'static> { Migrations::new(vec![...]) }
```
Rust 2024 edition forbids `Migrations::new()` in const context. Use `concat!()` for multi-line SQL strings inside the function.

### 3. `REG_SAM_FLAGS`, NOT bare `u32` for registry access
```rust
// WRONG: self.open_key(path, KEY_READ.0)
// RIGHT: self.open_key(path, KEY_READ)
fn open_key(&self, path: &RegPath, access: REG_SAM_FLAGS) -> Result<HKEY, ...>
```

### 4. `init_database()` returns `anyhow::Error`, not `rusqlite::Error`
Because `rusqlite_migration::Error` has no `From` impl into `rusqlite::Error`.

### 5. `package-lock.json` is gitignored
Platform-specific (Linux-generated lock misses `@rollup/rollup-win32-x64-msvc`). CI uses `npm install` (not `npm ci`). If the lock file appears in git, remove it and add to `.gitignore`.

### 6. PostCSS and Tailwind configs are `.cjs`
`package.json` has `"type": "module"`. CommonJS configs MUST be `postcss.config.cjs` and `tailwind.config.cjs`.

### 7. SnapshotService + AuditService share ONE SQLite connection
`init_database()` returns `Arc<Mutex<Connection>>`. Both `SqliteRepo::from_connection()` and `SqliteAuditLogger::from_connection()` take clones of the same Arc.

### 8. Tweaks: always add `pub fn new()` → `Self`
Every tweak struct needs `new()` and optionally `with_provider(Arc<dyn RegistryProvider>)`. No `#[derive(Debug)]` on tweaks containing `Arc<dyn RegistryProvider>` (trait objects don't implement Debug).

### 9. Snapshot types MUST be re-exported
`crates/zb_domain/src/snapshots/mod.rs` must have `pub use entities::{SystemSnapshot, AppliedTweakRecord};`. Other crates import from `zb_domain::snapshots::SystemSnapshot`.

### 10. UI import paths
Components in `src/components/ui/` import store as `../../store/toast` (2 levels up to `src/`).

## Adding a New Tweak

1. Create file in `crates/zb_domain/src/tweaks/definitions/`
2. Implement `Tweak` trait (6 methods: `metadata`, `is_applied`, `capture_state`, `apply`, `revert`, `explain`)
3. Add `pub fn new()` → `Self` and `Default` impl (or rely on crate-level `#[allow]`)
4. If using registry: add `provider: Option<Arc<RegistryProvider>>` field + `with_provider()` builder
5. Add to `definitions/mod.rs`: `pub mod` + `pub use`
6. Register in `src-tauri/src/main.rs`: add to `tweaks` vec

## CI

| Job | Runner | Command |
|-----|--------|---------|
| `check` | ubuntu | `cargo fmt --all -- --check` + `cargo clippy -p zb_shared -p zb_domain -p zb_application --all-targets -- -D warnings` |
| `test` | windows | `cargo test -p zb_shared -p zb_domain -p zb_application` |
| `check-all` | windows | `cargo check --workspace` (fast compile check, ~3min) |

## Release

Trigger: push tag `v*` or manual `workflow_dispatch` from Actions tab.
```bash
git tag v0.2.0
git push origin v0.2.0
```

Release workflow builds `.msi` + `.exe` on `windows-latest` and attaches them to the GitHub Release. MSI output paths checked: `target/release/bundle/msi/*.msi` and `src-tauri/target/release/bundle/msi/*.msi`.

## More Context

See `SESSION.md` for full project history, architecture decisions, and known issues.
