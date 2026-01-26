## ADDED Requirements
### Requirement: Play Screen Layout
The system SHALL render the play screen layout to match the simplified structure defined in `ui-mock.md` (header, 9x9 grid with labels and 3x3 separators, side panel, and footer shortcuts).

#### Scenario: Initial play screen render
- **WHEN** the game enters play mode
- **THEN** the header shows title, difficulty, timer, and mistakes
- **AND** the grid is rendered with row/column labels and 3x3 block separators
- **AND** the grid uses square-like cells (single-character value slots)
- **AND** the side panel shows selected cell, value, candidates, mode, and actions
- **AND** the footer lists the primary navigation shortcuts

### Requirement: Cell Selection Navigation
The system SHALL move the selected cell using arrow keys or HJKL and reflect the selection in the grid and status panel.

#### Scenario: Move selection within grid
- **WHEN** the user presses an arrow key or H/J/K/L
- **THEN** the selected cell moves one step in that direction
- **AND** the status panel updates the selected row and column

#### Scenario: Prevent selection from leaving grid
- **WHEN** the selected cell is on the grid edge and the user presses a key that would move outside
- **THEN** the selection remains on the edge cell

### Requirement: Digit Entry and Notes Mode
The system SHALL accept digits 1-9 to enter values and support notes mode toggled by `n`.

#### Scenario: Enter value in normal mode
- **WHEN** notes mode is off and the user presses a digit key
- **THEN** the selected cell value is set to that digit
- **AND** the status panel displays the value

#### Scenario: Toggle candidate in notes mode
- **WHEN** notes mode is on and the user presses a digit key
- **THEN** the digit is added or removed from the cell candidates
- **AND** the status panel displays the candidate list

### Requirement: Given Visibility and Help
The system SHALL provide shortcut toggles for given visibility (`g`) and a help view (`?`).

#### Scenario: Toggle given visibility
- **WHEN** the user presses `g`
- **THEN** given cells toggle between visible and hidden

#### Scenario: Show help view
- **WHEN** the user presses `?`
- **THEN** a help view listing shortcuts is displayed

### Requirement: Action Shortcuts
The system SHALL map action shortcuts for undo (`u`), redo (`r`), hint (`h`), clear (`c`), save (`s`), and quit (`q`).

#### Scenario: Trigger action handlers
- **WHEN** the user presses an action shortcut key
- **THEN** the corresponding action handler is invoked

### Requirement: Shortcut Navigation Tests
The system SHALL include automated tests that verify navigation bounds and shortcut mappings.

#### Scenario: Verify input mappings
- **WHEN** the shortcut mapping tests run
- **THEN** arrow/HJKL movement and action shortcuts are validated against expected commands
