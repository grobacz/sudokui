# IMPORTANT
Keep communication related to this project in english

<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

# Project Stack (Keep Updated)

- Language: Rust (edition 2021)
- TUI: ratatui + crossterm
- Serialization: serde + serde_json
- Static analysis: `cargo clippy`
- Syntax check: `cargo check`

# Documentation Prompts

- When the stack changes, update this file and any related docs.
- Document all development commands here whenever they change.

# Development Commands

- Build: `cargo build`
- Run: `cargo run`
- Check (syntax/type): `cargo check`
- Lint (static analysis): `cargo clippy`
- Format: `cargo fmt`
- Test: `cargo test`

# Quality Gates

- After any code file change, run `cargo check` or `cargo clippy`.
- After completing a feature, run `cargo test` as the final step.
