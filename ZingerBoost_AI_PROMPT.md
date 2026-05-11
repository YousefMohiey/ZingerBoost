# ZingerBoost — AI Prompt Template

> **File:** `ZingerBoost_AI_PROMPT.md`  
> **Purpose:** Copy-paste this prompt at the start of every new AI chat session to prevent stack drift and enforce rules.

---

## INSTRUCTIONS FOR THE USER

1. **Copy the block below** (between the `---` lines).
2. **Paste it into the FIRST message** of any new chat with an AI assistant.
3. **Fill in the `[CURRENT TASK]` and `[ACCEPTANCE CRITERIA]` sections** before sending.
4. **Do NOT skip this prompt.** If the AI tries to change the stack, paste the relevant rule again.

---

## PASTE THIS INTO THE CHAT

```text
# STRICT CONTEXT — ZingerBoost

I am working on **ZingerBoost**, a Windows system optimization desktop application.

## PROJECT STATE
- **Backend:** 4 Rust crates (`zb_shared`, `zb_domain`, `zb_application`, `zb_infrastructure`) — ALREADY BUILT AND WORKING.
- **Tweaks:** 29 tweaks already implemented in `zb_domain/src/tweaks/definitions/`.
- **Current UI:** `actix-web` HTTP server in `server/` crate (transitional, will be deleted).
- **Target UI:** Iced desktop app (`zb_app/` crate — does not exist yet).
- **Software catalog:** Already exists in `zb_shared/src/software.rs` (30+ apps).
- **Bloatware catalog:** Already exists in `zb_shared/src/software.rs` (34 apps).
- **System Cleaner:** Already exists in `zb_infrastructure/src/windows_api/system_cleaner.rs`.
- **Service Manager:** Already exists in `zb_infrastructure/src/services/service_controller.rs`.
- **Metrics:** Already exists in `zb_infrastructure/src/windows_api/metrics_collector.rs`.

## LOCKED TECH STACK (NEVER CHANGE)
- **Language:** Rust
- **Target GUI Framework:** Iced (pure Rust, Elm architecture)
- **Current UI (transitional):** actix-web — do NOT add React/Flutter to it
- **Async Runtime:** Tokio
- **Build Tool:** Cargo
- **OS Integration:** windows-rs (WMI, Registry, Services, PackageManager, DISM)
- **Database:** SQLite (rusqlite)
- **No web technologies in new UI.** No HTML, CSS, JavaScript, TypeScript, React, Vue, Tailwind, Tauri, Electron, Flutter, or Qt.

## ABSOLUTE RULES
1. **NEVER change the tech stack.** Do NOT switch to React, Tauri, TypeScript, Flutter, HTML-only, egui, Electron, .NET MAUI, Vue, Svelte, or anything else. If you hit a problem, solve it WITHIN Rust and Iced.
2. **All new UI code MUST be Rust (.rs) using Iced widgets.** No `.tsx`, `.html`, `.vue`, or `.svelte`.
3. **Backend MUST be Rust.** No C#, Python, Node.js, or Go backends.
4. **Do NOT rewrite existing crates.** `zb_shared`, `zb_domain`, `zb_application`, `zb_infrastructure` are already built. Build ON TOP of them.
5. **Preserve the crate structure:** `zb_shared`, `zb_domain`, `zb_application`, `zb_infrastructure`, `zb_app`.
6. **Debloat engine MUST use native Windows APIs (`windows-rs` COM / `dism.exe`).** NEVER use PowerShell scripts for removing apps.
7. **Snapshots are MANDATORY.** Before ANY system modification, create and verify a snapshot. If it fails, abort the operation.
8. **Dashboard System Overview MUST be live data** from the Rust backend (WMI / Performance Counters), refreshed via Iced `subscription`. NO static placeholder text.
9. **Theme Support:** The app MUST support Dark, Light, and System themes via Iced custom `Palette` and `Theme`.
10. **Only make MINIMAL changes.** Do not refactor unrelated code.
11. **Do NOT run `git commit`, `git push`, or any git mutations** unless I explicitly tell you to.
12. **If a request conflicts with these rules, STOP and ask me.** Do not "fix" it by switching technologies.

## PROJECT DOCUMENTS
- Master Plan: /home/verhafter/Documents/ZingerBoost_Master_Plan.md
- Strict Rules: /home/verhafter/Documents/ZingerBoost_STRICT_RULES.md
- Project root: /home/verhafter/ZingerBoost

## CURRENT TASK
[REPLACE THIS WITH WHAT YOU WANT DONE]

## ACCEPTANCE CRITERIA
[REPLACE THIS WITH HOW YOU WILL VERIFY IT]
```

---

## EXAMPLE USAGE

### Example 1: Fix a specific tweak
```text
## CURRENT TASK
Fix the "Disable Game DVR" tweak. It is not actually disabling Game DVR on Windows 11 23H2. Update the registry keys in `crates/zb_domain/src/tweaks/definitions/disable_game_dvr.rs` to include both HKCU and HKLM paths, and ensure `capture_state` reads the existing values correctly.

## ACCEPTANCE CRITERIA
- Game DVR toggle shows "Applied" after clicking.
- Registry keys `GameDVR_Enabled`, `AllowGameDVR`, and `AppCaptureEnabled` are set to `0`.
- `capture_state` stores the previous values so revert works.
```

### Example 2: Add a new UI page in Iced
```text
## CURRENT TASK
Add a "Music" category tab to the Install Apps page in `crates/zb_app/src/view/install_apps.rs`. It should contain cards for Spotify and Anghami using the existing data from `zb_shared::software::get_software_catalog()`.

## ACCEPTANCE CRITERIA
- Music tab is visible in the Install Apps page.
- Spotify and Anghami cards render with correct names and install buttons.
- Dark and Light modes both look correct.
```

### Example 3: Refactor debloat engine
```text
## CURRENT TASK
Refactor `crates/zb_infrastructure/src/windows_api/debloat_engine.rs` to remove PowerShell usage. Replace `try_powershell_remove()` with native `windows-rs` COM `PackageManager::RemovePackageAsync`. Replace `remove_windows_ads()` with native `WinRegistryProvider` writes.

## ACCEPTANCE CRITERIA
- Debloat engine no longer calls `powershell.exe` for app removal.
- App removal still works for UWP packages.
- Ads removal still sets all required registry keys.
```

---

## EMERGENCY STOP PHRASES

If the AI starts drifting (e.g., suggesting React, generating HTML files, rewriting the backend in Python), paste one of these:

> **"STACK LOCK: We are using Rust + Iced. Do not change the stack."**

> **"RULE 1 VIOLATION: You are trying to switch technologies. Stop. Solve the problem within Rust and Iced."**

> **"MINIMAL CHANGE: Only edit the files I asked about. Do not rename crates or move folders."**

> **"EXISTING CODE: zb_shared, zb_domain, zb_application, zb_infrastructure are already built. Do not rewrite them."**

---

**Last Updated:** 2026-05-10  
**Author:** YousefMohiey
