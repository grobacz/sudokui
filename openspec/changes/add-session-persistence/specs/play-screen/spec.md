## ADDED Requirements

### Requirement: Session auto-resume
The system SHALL attempt to load the most recent saved session on startup.

#### Scenario: Saved session exists
- **WHEN** the user starts SUDOKUI and a valid session file exists
- **THEN** the board, timer, and UI zoom level are restored

#### Scenario: No saved session exists
- **WHEN** the user starts SUDOKUI and no session file exists
- **THEN** a new game starts with default settings

### Requirement: New game shortcut
The system SHALL provide a keyboard shortcut to start a new game, resetting the board and timer.

#### Scenario: Start a new game
- **WHEN** the user triggers the New Game shortcut (`Ctrl+n`)
- **THEN** the board is reset to a new puzzle
- **AND** the timer resets to 00:00:00

## MODIFIED Requirements

### Requirement: Save/Load defaults
The system SHALL use a stable default save location suitable for persistence between runs (not the current working directory).

#### Scenario: Manual save/load uses the same session file
- **WHEN** the user saves and later loads using the manual shortcuts
- **THEN** the session restores from the same on-disk session file used for auto-resume
