# Master Plan: Add First Game Screen

## Objective
Deliver the first playable TUI screen per `ui-mock.md`, including input handling, state-backed panels, and tests for navigation/shortcuts.

## Architecture Overview
- Entry: `src/main.rs` runs the TUI event/render loop.
- UI: ratatui layout and rendering for header, grid, side panel, footer.
- Input: centralized shortcut mapping -> domain actions.
- State: play-screen state for grid, selection, mode, timer, mistakes, hints.
- Tests: unit tests for input mapping and navigation bounds.

## Components (Dependency Order)
1. **Play State**
   - Define state structs and pure helpers for selection/mode/entries.
2. **Input Mapping**
   - Map keys (arrows/HJKL, digits, shortcuts) to actions using pure functions.
3. **Render Layout**
   - ratatui layout and widgets aligned to `ui-mock.md`.
4. **Main Loop Integration**
   - Wire state + input + render into `main.rs` event loop.
5. **Tests**
   - Unit tests for shortcut mapping and navigation bounds.

## Validation
- After code changes: `cargo check` or `cargo clippy`.
- After feature complete: `cargo test`.

## Open Questions
- Help shortcut (`?`) output location (modal vs footer hint).
- Save action behavior (no-op vs minimal format).
