# Contributing to ZingerBoost

Thank you for your interest in contributing! ZingerBoost is an open-source project focused on safe, transparent Windows optimization.

## How to Contribute

1. **Fork** the repository
2. **Create a branch** (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open a Pull Request**

## Development Guidelines

### Safety First

- **Never** implement tweaks that cannot be fully reverted
- **Always** validate registry paths against allow-lists
- **Test** every tweak on a fresh Windows VM before submitting
- **Document** exactly what each tweak changes and why

### Code Style

- Rust: `cargo fmt` and `cargo clippy` must pass
- TypeScript: strict mode enabled
- Commit messages: follow conventional commits

### Adding a New Tweak

1. Implement the `Tweak` trait in `crates/zb_domain/src/tweaks/definitions/`
2. Add metadata (name, category, risk level, explanation)
3. Implement `capture_state`, `apply`, and `revert`
4. Add tests in `zb_domain/tests/`
5. Register in `zb_app/src/commands.rs`

## Questions?

Open an issue or discussion on GitHub.

## Code of Conduct

Be respectful, constructive, and helpful. This project is for the community.
