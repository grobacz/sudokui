## Context
The first playable screen should match the simplified terminal UI defined in `ui-mock.md`, centered on a 9x9 grid with a status/action side panel and a keyboard-only workflow.

## Goals / Non-Goals
**Goals**
- Render the full layout (header, grid, side panel, footer) in the terminal.
- Support keyboard navigation and shortcuts shown in the mock.
- Keep the status/action panels in sync with selection and mode.

**Non-Goals**
- Full puzzle generation or solving logic.
- Persistence or save file format design.
- Advanced animations or theming beyond the layout.

## Decisions
- Introduce a dedicated play screen module with a state struct that tracks grid data, selection, mode, timer, and mistakes.
- Centralize keyboard shortcut mapping in a single input handler for easier testing.
- Use a ratatui layout split for header, grid+side panel, and footer.
- Start with a static puzzle definition until generation is specified.

## Risks / Trade-offs
- Some action handlers may be stubs until game logic exists.
- Timer accuracy depends on the render loop cadence.

## Migration Plan
1. Replace the placeholder `main` output with a TUI loop that renders the play screen.
2. Add new modules for state, input handling, and rendering.

## Open Questions
- Should the help shortcut (`?`) open a modal or update a footer hint line?
- Should the save action be a no-op placeholder or require a minimal format now?
