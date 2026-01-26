# Change: Add session persistence + new game shortcut

## Why
It should be possible to close SUDOKUI and continue later without manually saving/loading. Users also need a fast way to start over.

## What Changes
- Automatically load the last session (board + timer + UI zoom) on startup when available.
- Automatically save the current session on quit so progress persists between runs.
- Add a **New Game** shortcut (`Ctrl+n`) that resets the board and timer.
- Keep manual Save/Load shortcuts, but align them with the same on-disk session file.

## Defaults
- Default session file: `${XDG_STATE_HOME:-$HOME/.local/state}/sudokui/session.json` (best-effort; fall back to `./sudokui-save.json` if needed).

## Impact
- Affected specs: `specs/play-screen/spec.md` (delta).
- Affected code: `src/app.rs`, `src/input.rs`, `src/state.rs`, `src/ui.rs`.
