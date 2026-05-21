# ZingerBoost - Handoff

## State of the Project
ZingerBoost has been migrated to **Tauri v2** with a vanilla JS frontend. The backend is written in **Rust** using `windows-rs` for native API integration, maintaining a 5-crate workspace. The project aims to provide safe, reversible Windows optimizations, system cleaning, service management, and software installation.

### Current Version: `v0.0.6`
- **Release Status**: Live on GitHub (`v0.0.6`).
- **Binaries**: Proper NSIS and MSI installers are configured and attached to the release.
- **Data Persistence**: Uses `rusqlite` for database management.

## Key Accomplishments in the Last Session
1. **Uninstaller Reliability**: 
   - A major issue where the app couldn't uninstall itself while running has been resolved.
   - The "Uninstall" button in the app now dynamically generates a background `.cmd` script in `%TEMP%` that completely detaches from the app process.
   - The script waits for ZingerBoost to gracefully close, completely wipes the `%LOCALAPPDATA%\ZingerBoost` and `%APPDATA%\ZingerBoost` directories (ensuring databases and temp files are gone), runs the NSIS uninstaller (`Uninstall ZingerBoost.exe`) silently (`/S`), and finally deletes itself.
   - Zero traces remain.
   
2. **Network Metrics**:
   - The network monitoring fallback was completely rewritten.
   - Previously relied on `netstat` and `PDH` (Performance Data Helpers), which broke down on VMs and PCs with non-English Windows layouts.
   - Replaced with the universal `GetIfTable2` Windows API (via `windows-rs` `IpHelper`), accurately gathering interface traffic entirely ignoring virtualization layers and localizations.

3. **Repository Clean-Up**:
   - Outdated `BUILD.md` and `HANDOFF.md` files were deleted.
   - `README.md` was significantly updated to look professional with badges, features, and installer instructions.

## Workspace Architecture
- `zb_shared`: Common types, constants, and the software catalog.
- `zb_domain`: The `Tweak` trait and all tweak implementations (32 currently active in production).
- `zb_infrastructure`: Windows APIs (Registry, Services, Cleaner, Metrics), and SQLite persistence.
- `zb_application`: The business logic (`TweakEngine`, `SnapshotService`, `AuditService`).
- `zb_app`: The Tauri v2 application, serving the frontend, routing commands, and bundling installers.

## Important Notes for the Next Developer
- **GUI Limitations**: The frontend is vanilla HTML/CSS/JS. Ensure consistency when adding new components. No frameworks (React/Vue/etc.) are used.
- **Tauri Configuration**: Do not add back empty or null fields to `tauri.conf.json`'s `nsis` bundle options as it causes schema validation failures.
- **Metrics Module**: `metrics_collector.rs` manages real-time telemetry. Modify it cautiously, as memory and network counters rely on precise Windows API un-safe abstractions.
- **Safety**: ZingerBoost's promise is safety. Ensure all tweaks continue to have a `.revert()` function, properly implemented.

Enjoy working on ZingerBoost! 🚀
