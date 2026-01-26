# Capability: Play Screen

## Requirements

### Requirement: Play screen layout
The system SHALL render a play screen containing:
- A header line with difficulty, timer, and mistakes.
- A 9x9 Sudoku grid with a visible selection cursor.
- A side panel showing selection, mode, and actions.
- A footer line with key shortcut hints.

#### Scenario: First render
- **WHEN** the user starts the application
- **THEN** the play screen renders without crashing

### Requirement: Keyboard controls
The system SHALL support keyboard controls for navigation and game actions.

#### Scenario: Navigation and input
- **WHEN** the user presses arrow keys or `H/J/K/L`
- **THEN** the selection moves within the 9x9 bounds
- **WHEN** the user presses digits `1`–`9`
- **THEN** the selected cell is updated according to the current input mode

### Requirement: Mode and visibility toggles
The system SHALL support toggles for notes mode, givens visibility, and help.

#### Scenario: Toggle mode and help
- **WHEN** the user presses `n`
- **THEN** input mode toggles between normal and notes
- **WHEN** the user presses `g`
- **THEN** givens visibility toggles
- **WHEN** the user presses `?`
- **THEN** the help popup toggles

### Requirement: Validation and zoom
The system SHALL provide mistake validation and UI zoom controls.

#### Scenario: Validate and zoom
- **WHEN** the user presses `v`
- **THEN** mistakes are validated and counted
- **WHEN** the user presses `+` or `-`
- **THEN** the UI zoom level changes within defined bounds

### Requirement: Manual save/load
The system SHALL allow manual saving and loading of the current session state.

#### Scenario: Save and load in the current directory
- **WHEN** the user presses `s`
- **THEN** a JSON save file named `sudokui-save.json` is written to the current working directory
- **WHEN** the user presses `o`
- **THEN** the game state is restored from `sudokui-save.json` if it exists and is valid

### Requirement: Quit
The system SHALL allow quitting the application.

#### Scenario: Quit
- **WHEN** the user presses `q`
- **THEN** the application exits cleanly and restores the terminal state

