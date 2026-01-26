# Change: Update first game screen interactions

## Why
The first screen is playable but needs tighter layout and core UX conveniences (validation, persistence, and better visual feedback) to be useful.

## What Changes
- Right-align the side detail panel within the UI frame.
- Start mistakes at 0 and add a shortcut to validate incorrect entries.
- Persist game state via save/load shortcuts.
- Highlight matching digits when selecting a filled cell.

## Impact
- Affected specs: `specs/play-screen/spec.md` (delta).
- Affected code: `src/input.rs`, `src/state.rs`, `src/ui.rs`.

