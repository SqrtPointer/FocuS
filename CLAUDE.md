# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Behavioral guidelines to reduce common LLM coding mistakes. Merge with project-specific instructions as needed.

**Tradeoff:** These guidelines bias toward caution over speed. For trivial tasks, use judgment.

## 1. Think Before Coding

**Don't assume. Don't hide confusion. Surface tradeoffs.**

Before implementing:
- State your assumptions explicitly. If uncertain, ask.
- If multiple interpretations exist, present them - don't pick silently.
- If a simpler approach exists, say so. Push back when warranted.
- If something is unclear, stop. Name what's confusing. Ask.

## 2. Simplicity First

**Minimum code that solves the problem. Nothing speculative.**

- No features beyond what was asked.
- No abstractions for single-use code.
- No "flexibility" or "configurability" that wasn't requested.
- No error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.

Ask yourself: "Would a senior engineer say this is overcomplicated?" If yes, simplify.

## 3. Surgical Changes

**Touch only what you must. Clean up only your own mess.**

When editing existing code:
- Don't "improve" adjacent code, comments, or formatting.
- Don't refactor things that aren't broken.
- Match existing style, even if you'd do it differently.
- If you notice unrelated dead code, mention it - don't delete it.

When your changes create orphans:
- Remove imports/variables/functions that YOUR changes made unused.
- Don't remove pre-existing dead code unless asked.

The test: Every changed line should trace directly to the user's request.

## 4. Goal-Driven Execution

**Define success criteria. Loop until verified.**

Transform tasks into verifiable goals:
- "Add validation" → "Write tests for invalid inputs, then make them pass"
- "Fix the bug" → "Write a test that reproduces it, then make it pass"
- "Refactor X" → "Ensure tests pass before and after"

For multi-step tasks, state a brief plan:
```
1. [Step] → verify: [check]
2. [Step] → verify: [check]
3. [Step] → verify: [check]
```

Strong success criteria let you loop independently. Weak criteria ("make it work") require constant clarification.

---

## Build & Run

```bash
npm install                        # Install frontend dependencies
npm run tauri dev                  # Dev mode with hot-reload
npm run tauri build                # Release build → MSI + NSIS installers
npm run build                      # Frontend-only build (Vite)
```

Full build output: `src-tauri/target/release/bundle/msi/FocuS_0.1.0_x64_en-US.msi`

## Architecture

**Stack**: Tauri v2 (Rust backend, system WebView2 shell) + Svelte 5 (frontend) + Vite (bundler)

**Three Tauri windows** defined in `src-tauri/tauri.conf.json`:
- `search` — centered search bar (680×400, `Alt+Space`)
- `wheel` — radial menu at cursor position (400×400, `Ctrl+Space`)
- `settings` — settings panel (600×500)

All windows use `decorations: false, transparent: true` with CSS `backdrop-filter: blur()` for the Acrylic frosted glass effect.

### Rust Backend (`src-tauri/src/`)

- `lib.rs` — App entry: plugin registration, hotkey setup, Tauri command handler registration
- `hotkey.rs` — Global hotkeys via `tauri-plugin-global-shortcut` (search: Alt+Space, wheel: Ctrl+Space)
- `window.rs` — Window show/hide/position commands (IPC from frontend)
- `acrylic.rs` — CSS-based Acrylic opacity control (native DWM acrylic planned for Phase 2)
- `cursor.rs` — Windows `GetCursorPos` wrapper for wheel positioning
- `config.rs` — JSON config CRUD (`%APPDATA%/FocuS/config.json`), includes search/wheel appearance and hotkey settings
- `tray.rs` — System tray icon + context menu (Settings / Quit)
- `search/engine.rs` — Fuzzy search via `nucleo` crate (fzf algorithm)
- `search/provider.rs` — `SearchItem` and `ItemAction` types
- `scanner/apps.rs` — Windows Start Menu + Registry app scanner, wheel item types, launch command
- `scanner/files.rs` — File indexer placeholder (Phase 3)
- `plugins/runtime.rs` — WASM plugin runtime placeholder (Phase 5)

**Shared state**: `AppState` managed via `tauri::manage()`, holds `Config` and `SearchEngine` wrapped in `Arc<Mutex<>>`.

### Frontend (`src/`)

- `src/App.svelte` — View router (search / wheel / settings)
- `src/components/search/` — Search bar UI (input → nucleo search → results list → launch)
- `src/components/wheel/` — Radial wheel (Canvas 2D, 8 sectors, angle-based hit detection)
- `src/components/settings/` — Appearance (opacity sliders, border color) and wheel layout editor
- `src/lib/stores.ts` — Svelte 5 writable stores (query, results, config, activeSector)
- `src/lib/commands.ts` — Typed wrappers around `invoke()` for all Tauri commands
- `src/lib/wheel.ts` — Wheel geometry constants (`WHEEL_RADIUS=180`, `DEAD_ZONE_RADIUS=40`), sector detection math, Canvas draw helpers

## Key Constraints

- **No Electron** — Tauri uses the OS-native Edge WebView2, no bundled browser
- **Svelte 5 runes** — Use `$props()`, `$state()`, `$effect()`, `$derived()` not legacy `export let` / `$:` / `on:event`
- **Windows only** (`#[cfg(windows)]`) — mouse position, acrylic, and app scanner APIs are Windows-specific
- **Permissions** — Capabilities are in `src-tauri/capabilities/default.json`; each new Tauri command must either be added there or invoked with the proper permission prefix
- **`src-tauri/gen/`** is gitignored — auto-generated by Tauri build

## Tauri v2 Patterns

- `use tauri::Manager;` required for `app.get_webview_window()`, `app.state::<T>()`, `window.eval()`
- Commands registered in `lib.rs` via `tauri::generate_handler![]`, function paths must be to the actual `#[command]` fn location
- `tauri_plugin_shell::ShellExt` — call `.shell()` on `AppHandle`, then `.command(...).output().await`
- Window-level eval: `window.eval(&js_str)` for injecting CSS variables / running JS at runtime
