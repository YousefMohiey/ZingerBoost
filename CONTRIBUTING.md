# 🤝 Contributing to ZingerBoost

Thank you for your interest in contributing! **ZingerBoost** is an open-source project focused on safe, transparent, and reversible Windows optimization. Whether you're fixing a bug, adding a new tweak, or improving documentation — we appreciate your help.

---

## 🚀 Quick Start

1. **Fork** the repository
2. **Clone** your fork: `git clone https://github.com/YOUR_USERNAME/ZingerBoost.git`
3. **Create a branch:** `git checkout -b feature/amazing-feature`
4. **Make your changes** and test them
5. **Commit:** `git commit -m 'feat: add amazing feature'`
6. **Push:** `git push origin feature/amazing-feature`
7. **Open a Pull Request**

---

## 🛡️ Safety First

ZingerBoost modifies Windows systems. **Safety is our highest priority.**

- ✅ **Never** implement tweaks that cannot be fully reverted
- ✅ **Always** validate registry paths against allow-lists
- ✅ **Test** every tweak on a fresh Windows VM before submitting
- ✅ **Document** exactly what each tweak changes and why
- ✅ **Protect** system-critical apps (Store, Calculator, Terminal, etc.)

---

## 📝 Code Style

### Rust

```bash
# Formatting
cargo fmt --all

# Linting (must pass with zero warnings)
cargo clippy -p zb_shared -p zb_domain -p zb_application --all-targets -- -D warnings
```

### Dart / Flutter

```bash
cd zingerboost_flutter
flutter analyze
flutter test
```

### Commit Messages

We follow **Conventional Commits**:

| Type | Description |
|------|-------------|
| `feat:` | New feature |
| `fix:` | Bug fix |
| `docs:` | Documentation only |
| `style:` | Formatting, missing semicolons, etc. |
| `refactor:` | Code change that neither fixes a bug nor adds a feature |
| `test:` | Adding or correcting tests |
| `chore:` | Changes to build process or auxiliary tools |

Examples:
- `feat: add gaming performance tweak`
- `fix: resolve snapshot restore edge case`
- `docs: update software catalog with Brave Browser`

---

## 🔧 Adding a New Tweak

1. **Create** a new file in `crates/zb_domain/src/tweaks/definitions/`
2. **Implement** the `Tweak` trait (6 methods):
   - `name()` — human-readable name
   - `category()` — Visual, Privacy, Performance, or Gaming
   - `risk_level()` — Safe, Caution, or Advanced
   - `capture_state()` — save current state for revert
   - `apply()` — apply the tweak
   - `revert()` — restore captured state
3. **Add** `pub fn new() -> Self` and `pub fn with_provider(provider: Arc<dyn RegistryProvider>) -> Self`
4. **Register** the tweak in `definitions/mod.rs`
5. **Register** the tweak in `bridge/src/lib.rs` tweaks vector
6. **Add tests** in `zb_domain/tests/`
7. **Update** README.md if the tweak is user-facing

> See existing tweaks in `crates/zb_domain/src/tweaks/definitions/` for examples.

---

## 🧪 Testing

### Linux (Partial Check)

```bash
cargo check -p zb_shared -p zb_domain -p zb_application
cargo fmt --all
cargo clippy -p zb_shared -p zb_domain -p zb_application --all-targets -- -D warnings
```

### Windows (Full Check)

```bash
cargo check --workspace
cargo test --workspace
```

> Note: `zb_infrastructure` and `bridge` depend on `windows-rs` and only compile on Windows.

---

## 🏗️ Project Structure

```
ZingerBoost/
├── bridge/                  # Rust FFI bridge (cdylib)
├── crates/
│   ├── zb_shared/           # Types, constants, software catalog
│   ├── zb_domain/           # Tweak trait + implementations
│   ├── zb_application/      # TweakEngine, SnapshotService
│   └── zb_infrastructure/   # WinRegistry, SQLite, Winget, PDH
└── zingerboost_flutter/     # Flutter desktop app
```

---

## 💬 Questions?

- Open an [**Issue**](https://github.com/YousefMohiey/ZingerBoost/issues) for bugs or feature requests
- Open a [**Discussion**](https://github.com/YousefMohiey/ZingerBoost/discussions) for general questions

---

## 📜 Code of Conduct

Be respectful, constructive, and helpful. This project is built for the community, by the community. Harassment or toxic behavior will not be tolerated.

---

<div align="center">

**Thank you for making ZingerBoost better!** ⚡

</div>
