# ZingerBoost — Strict AI Agent Rules v1.0.0

> **File:** `ZingerBoost_STRICT_RULES.md`  
> **Purpose:** Hard constraints for any AI (or human) working on the ZingerBoost codebase.  
> **Rule #0:** If you are unsure, STOP and ask the user. Do not guess.  
> **Version:** 1.0.0 (Major Redesign)

---

## 1. Technology Stack Lock

| Layer | Current | Target | Forbidden Alternatives |
|-------|---------|--------|------------------------|
| **Language** | Rust | Rust | C#, C++, Python, JavaScript, TypeScript, Go, Java, Dart |
| **Backend Crates** | `zb_shared`, `zb_domain`, `zb_application`, `zb_infrastructure`, `zb_app`, `zb_tray` | Same | Do not rename or delete these crates |
| **UI Framework** | Tauri v2 + Vanilla JS | Tauri v2 + Vanilla JS | Iced, React, Vue, Svelte, Angular, Solid, Flutter, Electron, .NET MAUI, Qt, GTK, egui, Slint |
| **Frontend** | Vanilla HTML/CSS/JS (no framework, no build step) | Same | No React, Vue, Svelte, Angular, TypeScript, npm, bundlers |
| **Build Tool** | Cargo | Cargo | Webpack, Vite, npm, yarn, pnpm |
| **Async Runtime** | Tokio | Tokio | async-std (unless absolutely required) |
| **Renderer** | Tauri WebView2 (Edge Chromium) | Same | Custom OpenGL, DirectX, WebGL |
| **Windows API** | `windows-rs` 0.58 | Same | winapi, windows-targets (old) |

**Consequence of violation:** Any code written in a forbidden stack will be rejected and must be rewritten.

---

## 1.1 Allowed Exceptions

| Use Case | Allowed Tool | Reason |
|----------|--------------|--------|
| **Software Installation** | `winget.exe` process | No Rust API for package managers |
| **DISM Operations** | `dism.exe` process | No Rust API for deployment tools |
| **PowerShell (Emergency)** | `powershell.exe` | ONLY if no windows-rs API exists (must be documented) |
| **WMI Queries** | `windows-rs` COM or WMI | Prefer native COM, fallback to WMI |

---

## 2. Absolute Prohibitions

1. **NO STACK SUBSTITUTION.** Do not switch the project to Iced, React, TypeScript, Flutter, HTML-only, Raui, Electron, .NET MAUI, Qt, GTK, egui, or any other framework because of a "bug" or "limitation." The correct fix is to solve the problem within the locked stack.
2. **NO FRONTEND FRAMEWORKS.** No React, Vue, Svelte, Angular, TypeScript, JSX, TSX, Tailwind, npm packages, or bundlers in `zb_app/` or `zb_tray/`. Frontend is vanilla HTML/CSS/JS only.
3. **NO POWERSHELL DEBLOATING.** The debloat engine must use native `windows-rs` COM APIs (`PackageManager`) or call `dism.exe` directly. Do NOT write PowerShell scripts that pipe `Get-AppxPackage | Remove-AppxPackage`.
4. **NO REWRITING EXISTING CRATES.** `zb_shared`, `zb_domain`, `zb_application`, `zb_infrastructure` are already built and working. Do NOT rewrite them. Build ON TOP of them.
5. **NO ARCHITECTURE REWRITES.** Do not rename crates, move folders, or change the Tauri command architecture (`#[tauri::command]` + `AppState`) without explicit user approval.
6. **NO EMOJIS IN UI.** Use SVG icons (Lucide-style) for all UI elements. Emojis are unprofessional and don't scale.
7. **NO HARDCODED COLORS.** Use CSS variables from `style.css` (`--primary`, `--secondary`, `--accent`, etc.).
8. **NO MAGIC NUMBERS.** Define constants for sizes, spacing, durations in JS (e.g., `const CARD_PADDING = 16`, `const METRICS_INTERVAL_MS = 5000`).

---

## 2.1 UI/UX Rules (v1.0.0)

1. **Circular Progress for Metrics** - All percentage metrics (CPU, RAM, Disk) use CSS `conic-gradient` indicators
2. **Sparkline History** - Show 1-hour trend for hardware metrics using SVG `<path>`
3. **Risk Badges** - Color-coded (Green/Yellow/Red) for all tweaks and operations
4. **Loading States** - Disable buttons during async operations, show CSS spinner
5. **Error Feedback** - Clear error messages near the problem, not just status bar
6. **Hover Feedback** - All interactive elements change color/opacity on hover (CSS `:hover`)
7. **Focus States** - Visible focus rings for keyboard navigation (CSS `:focus-visible`)
8. **Consistent Spacing** - Use 4px grid system (4, 8, 12, 16, 24, 32px)
9. **No Layout Shift** - Reserve space for async content (min-height, aspect-ratio)
10. **Touch Targets** - Minimum 44x44px for all clickable elements
11. **Tauri Invoke Pattern** - Use `window.__TAURI__.core.invoke()` for all backend calls
12. **Event-Driven Updates** - Prefer Tauri `app.emit()` over `setInterval` polling where possible

---

## 3. Code Quality & Safety Rules

1. **Snapshots are Mandatory.** Before ANY system modification (tweak apply, debloat remove), a snapshot MUST be created. If snapshot creation fails, the operation aborts.
2. **Dashboard is Live.** The System Overview on the Dashboard MUST query live WMI / Performance Counter data from the Rust backend and refresh via Tauri events or polling. Static placeholder text is considered a broken feature.
3. **Theme Support.** The app MUST support Dark, Light, and System themes via CSS custom properties (`data-theme` attribute on `<html>`).
4. **Registry Safety.** All registry writes must be reversible. `capture_state` must read the current value before overwriting. Affected keys must be listed in tweak metadata.
5. **Least Privilege.** Even when running as admin, only enable required privileges temporarily.
6. **Tauri Command Safety.** All `#[tauri::command]` functions must handle errors gracefully and return user-friendly error strings, not Rust error types.

---

## 4. File & Naming Conventions

- **Rust crates:** `zb_` prefix (e.g., `zb_domain`).
- **Rust files:** `snake_case.rs`.
- **Tauri commands:** `snake_case` function names in `commands.rs`.
- **Frontend files:** `index.html`, `app.js`, `style.css` in `zb_app/src/`.
- **CSS classes:** BEM-style or kebab-case (e.g., `.metric-card`, `.sidebar__item`).
- **JS constants:** `SCREAMING_SNAKE_CASE` (e.g., `METRICS_INTERVAL_MS`).
- **Constants / Enums:** `SCREAMING_SNAKE_CASE` (Rust).

---

## 5. Git Rules

- **NEVER commit or push.** Do not run `git commit`, `git push`, `git rebase`, `git reset`, or any mutation unless the user explicitly types the command themselves or asks you to do it.
- Do not amend commits.
- Do not create branches unless asked.

---

## 6. Decision Making Protocol

| Situation | Action |
|-----------|--------|
| Requirement is clear | Implement within the locked stack. |
| Requirement conflicts with stack | Explain the conflict and ask user for direction. Do NOT switch stacks. |
| Requirement is ambiguous | Ask clarifying questions before coding. |
| Bug exists in existing code | Fix the bug within the existing architecture. Do not rewrite the module in a different language. |
| You want to "improve" something not asked | Do not. Only make minimal changes to achieve the user's goal. |
| Existing code already does what you need | Use it. Do not rewrite it. |

---

## 7. Enforcement

Paste these rules into every new chat session. The user has the right to reject any output that violates them.

**Last Updated:** 2026-05-10  
**Author:** YousefMohiey
