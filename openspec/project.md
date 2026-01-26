# Project Context

## Purpose
SUDOKUI is a terminal-based Sudoku game focused on a lightweight TUI experience.

## Tech Stack
- Rust (edition 2021)
- ratatui (TUI rendering)
- crossterm (terminal backend)
- serde + serde_json (serialization)

## Project Conventions

### Code Style
- Follow standard Rust formatting via `cargo fmt`.
- Prefer clear, descriptive names over abbreviations.
- Keep modules small and focused on a single responsibility.

### Architecture Patterns
- Single binary entry at `src/main.rs`.
- TUI layout and rendering managed with ratatui + crossterm.

### Testing Strategy
- Use `cargo test` for unit and integration tests as features grow.
- Run `cargo check` or `cargo clippy` after code changes.
- Run `cargo test` after completing a feature.

### Git Workflow
- Prefer small, focused commits with descriptive messages.
- Use feature branches for non-trivial changes.

## Domain Context
- Sudoku rules: fill a 9x9 grid so each row, column, and 3x3 box contains digits 1–9 exactly once.
- TUI-first interaction: keyboard-driven navigation and inputs.

## Important Constraints
- Terminal-only UI (no GUI).
- Rust 2021 edition.
- Quality gates: `cargo check` or `cargo clippy` after code changes; `cargo test` after completing a feature.

## External Dependencies
- ratatui
- crossterm
- serde
- serde_json
