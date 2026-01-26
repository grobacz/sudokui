## 1. Implementation
- [x] 1.1 Define a stable default save path (XDG state dir on Unix; sane fallback).
- [x] 1.2 Auto-load state at startup; fall back to a new puzzle when missing/invalid.
- [x] 1.3 Auto-save state on quit (best-effort; do not block exit on failure).
- [x] 1.4 Add New Game command + shortcut and reset timer/board.
- [x] 1.5 Update help footer/popup to include New Game and persistence behavior.
- [x] 1.6 Add/adjust tests for key mapping and save/load round-trip with zoom + timer.

## 2. Validation
- [x] 2.1 Run `cargo fmt`.
- [x] 2.2 Run `cargo check`.
- [x] 2.3 Run `cargo test`.
