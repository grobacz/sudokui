## MODIFIED Requirements
### Requirement: Play Screen Layout
The system SHALL render the play screen layout to match the simplified structure defined in `ui-mock.md` (header, 9x9 grid with labels and 3x3 separators, side panel, and footer shortcuts).

#### Scenario: Layout uses minimal space
- **WHEN** the play screen is rendered
- **THEN** the UI uses only as much terminal space as necessary
- **AND** the side panel is aligned to the right edge of the UI frame

### Requirement: Mistake Counting and Validation
The system SHALL start the mistakes counter at `0/3` and provide a shortcut to validate incorrect filled cells.

#### Scenario: Validate incorrect entries
- **GIVEN** the user has entered one or more values
- **WHEN** the user triggers the validate shortcut
- **THEN** any incorrect filled cells are highlighted as wrong
- **AND** the mistake counter increments for newly-detected wrong cells

### Requirement: Save and Load
The system SHALL allow saving and loading the current game state via shortcuts.

#### Scenario: Save and load
- **WHEN** the user triggers save
- **THEN** the current play state is persisted
- **AND WHEN** the user triggers load
- **THEN** the play state is restored

### Requirement: Highlight Matching Digits
The system SHALL highlight all visible cells matching the selected cell's digit when the selection is on a filled cell.

#### Scenario: Highlight matching digits
- **WHEN** the selected cell contains a digit
- **THEN** all other visible cells containing that same digit are highlighted

