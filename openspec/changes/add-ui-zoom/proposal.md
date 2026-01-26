# Change: Add UI zoom in/out

## Why
The TUI size needs to be adjustable for different terminal sizes and user preferences.

## What Changes
- Add `+` / `-` shortcuts to zoom the play UI in/out.
- Render the grid at different zoom levels.
- Document the shortcuts in the help popup.

## Impact
- Affected specs: `specs/play-screen/spec.md` (delta).
- Affected code: `src/input.rs`, `src/state.rs`, `src/ui.rs`.

