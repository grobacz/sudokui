# Change: Add first game screen UI

## Why
The project needs a playable first screen that matches the agreed UI mock and establishes the keyboard-driven interaction model.

## What Changes
- Render the simplified play screen layout from `ui-mock.md`.
- Implement keyboard navigation and shortcut handling for movement, input modes, and actions.
- Populate status/action panels from live game state.
- Add automated tests for shortcut mapping and navigation behavior.

## Current State
- `src/main.rs` prints a placeholder message only.
- No TUI rendering or input handling exists yet.
- `ui-mock.md` defines the intended layout and shortcuts.

## Feature Coverage
**Implemented**
- Application entrypoint (prints "Hello, world!").

**Still Needed**
- TUI layout (header, grid, side panel, footer) aligned to `ui-mock.md`.
- Keyboard navigation (arrows/HJKL) and digit entry.
 - Notes mode, toggle givens, and help shortcut handling.
- Action shortcuts (undo/redo/hint/clear/save/quit).
- Game state model for selection, mode, timer, and mistakes.
- Automated tests covering navigation and shortcut behavior.
 

## Impact
- Affected specs: `specs/play-screen/spec.md` (new).
- Affected code: `src/main.rs` plus new UI/input/state modules.
