# ZingerBoost — Strict AI Agent Rules

> **File:** `ZingerBoost_STRICT_RULES.md`  
> **Purpose:** Hard constraints for any AI (or human) working on the ZingerBoost codebase.  
> **Rule #0:** If you are unsure, STOP and ask the user. Do not guess.

---

## 1. Technology Stack Lock

| Layer | Current | Target | Forbidden Alternatives |
|-------|---------|--------|------------------------|
| **Language** | Rust | Rust | C#, C++, Python, JavaScript, TypeScript, Go, Java, Dart |
| **Backend Crates** | `zb_shared`, `zb_domain`, `zb_application`, `zb_infrastructure` | Same | Do not rename or delete these crates |
| **Current UI** | `actix-web` server (`server/` crate) | — | Do not add React/Vue/Flutter to `server/` |
| **Target UI** | — | Iced (Rust) | Tauri, React, Vue, Svelte, Angular, Solid, Flutter, Electron, .NET MAUI, Qt, GTK, egui, Slint |
| **Build Tool** | Cargo | Cargo | Webpack, Vite, npm, yarn, pnpm |
| **Async Runtime** | Tokio | Tokio | async-std (unless absolutely required) |
| **Renderer** | — | Iced built-in (wgpu / tiny-skia) | Custom OpenGL, DirectX, WebGL |

**Consequence of violation:** Any code written in a forbidden stack will be rejected and must be rewritten.

---

## 2. Absolute Prohibitions

1. **NO STACK SUBSTITUTION.** Do not switch the project to Tauri, React, TypeScript, Flutter, HTML-only, Raui, Electron, .NET MAUI, Qt, GTK, egui, or any other framework because of a "bug" or "limitation." The correct fix is to solve the problem within the locked stack.
2. **NO WEB TECHNOLOGIES in new UI.** No HTML, CSS, JavaScript, TypeScript, JSX, TSX, Tailwind, or npm packages in `zb_app/`.
3. **NO POWERSHELL DEBLOATING.** The debloat engine must use native `windows-rs` COM APIs (`PackageManager`) or call `dism.exe` directly. Do NOT write PowerShell scripts that pipe `Get-AppxPackage | Remove-AppxPackage`.
4. **NO REWRITING EXISTING CRATES.** `zb_shared`, `zb_domain`, `zb_application`, `zb_infrastructure` are already built and working. Do NOT rewrite them. Build ON TOP of them.
5. **NO ARCHITECTURE REWRITES.** Do not rename crates, move the `server/` or `zb_app/` folders, or change the Iced architecture (State/Message/Update/View) without explicit user approval.

---

## 3. Code Quality & Safety Rules

1. **Snapshots are Mandatory.** Before ANY system modification (tweak apply, debloat remove), a snapshot MUST be created. If snapshot creation fails, the operation aborts.
2. **Dashboard is Live.** The System Overview on the Dashboard MUST query live WMI / Performance Counter data from the Rust backend and refresh via Iced `subscription`. Static placeholder text is considered a broken feature.
3. **Theme Support.** The app MUST support Dark, Light, and System themes via Iced custom `Palette` and `Theme`.
4. **Registry Safety.** All registry writes must be reversible. `capture_state` must read the current value before overwriting. Affected keys must be listed in tweak metadata.
5. **Least Privilege.** Even when running as admin, only enable required privileges temporarily.

---

## 4. File & Naming Conventions

- **Rust crates:** `zb_` prefix (e.g., `zb_domain`).
- **Rust files:** `snake_case.rs`.
- **UI modules:** `snake_case.rs` inside `zb_app/src/view/`.
- **Custom widgets:** `snake_case.rs` inside `zb_app/src/widgets/`.
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
