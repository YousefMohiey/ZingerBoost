# ZingerBoost — Agent Instructions (v0.0.5)

## Tech Stack (LOCKED — NEVER CHANGE)

| Layer | Technology |
|-------|-----------|
| Language | Rust (edition 2021) |
| GUI | **Iced 0.13** (pure Rust, Elm architecture) |
| Backend crates | `zb_shared`, `zb_domain`, `zb_application`, `zb_infrastructure` |
| App crate | `zb_app` (Iced desktop binary) |
| Async | Tokio |
| Windows API | `windows-rs` 0.58 (registry, services, PDH, restore points) |
| Database | SQLite via `rusqlite` (bundled) |
| Build | Cargo workspace |

## Quick Check (Linux)

```bash
cargo check -p zb_shared -p zb_domain -p zb_application
cargo fmt --all
cargo clippy -p zb_shared -p zb_domain -p zb_application --all-targets -- -D warnings
```

## Architecture

```
crates/
  zb_shared/       Types, constants, software catalog (30+ apps), bloatware catalog (34 apps)
  zb_domain/        Tweak trait + 29 implementations, RegistryProvider trait, snapshot entities
  zb_application/   TweakEngine, SnapshotService, AuditService
  zb_infrastructure/ WinRegistryProvider, ServiceController, DebloatEngine, SystemCleaner,
                     WingetInstaller, MetricsCollector, SQLite persistence
  zb_app/           Iced desktop app (State/Message/Update/View)
    src/main.rs     Entry with #![windows_subsystem = "windows"]
    src/lib.rs      App struct, Message enum, update(), view() with inline views
```

## Critical Gotchas for Iced 0.13

1. **ALL views must be inline in `view(&self) -> Element<Message>`** — standalone functions returning `Element<Message>` cause E0106 lifetime errors.
2. **No explicit lifetimes on Element** — `Element<Message>` works inside methods. `Element<'static, Message>` breaks view signature.
3. **`Border::default()` not `Border::rounded()`** — API changed, use struct literal.
4. **`Length::Fixed(180.0)` — must be f32/f64** — not integer.
5. **`async move` for `Task::perform` closures** — borrowed values need `move` keyword.
6. **`format!("{0}", var)` not `format!("{var}")`** — inline format not supported on older Rust.
7. **`Message` must derive `Debug + Clone`** — required by Iced.
8. **Button `on_press` returns `Element<Message>` via `.into()`** — closures on buttons return `Element`.

## Adding a Tweak

1. Create file in `crates/zb_domain/src/tweaks/definitions/`
2. Implement `Tweak` trait (6 methods: `metadata`, `is_applied`, `capture_state`, `apply`, `revert`, `explain`)
3. Add `pub fn new()` and `pub fn with_provider(Arc<RegistryProvider>)`
4. Register in `definitions/mod.rs`
5. Register in `zb_app/src/lib.rs` `make_all_tweaks()` function

## CI

| Job | Runner | Command |
|-----|--------|---------|
| `check` | ubuntu | `cargo fmt --all -- --check` + `cargo clippy -p zb_shared -p zb_domain -p zb_application --all-targets -- -D warnings` |
| `check-all` | windows | `cargo check --workspace` |

## Release

```bash
git tag v0.0.6 && git push origin v0.0.6
```
Workflow builds `cargo build --release -p zb_app` and uploads `zingerboost.exe`.
