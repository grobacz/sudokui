## ADDED Requirements
### Requirement: UI Zoom Shortcuts
The system SHALL provide shortcuts to zoom the play UI in and out.

#### Scenario: Zoom in/out
- **WHEN** the user presses `+`
- **THEN** the play UI zoom level increases (up to a maximum)
- **AND WHEN** the user presses `-`
- **THEN** the play UI zoom level decreases (down to a minimum)

### Requirement: Help Mentions Zoom
The system SHALL display the zoom shortcuts in the help view.

#### Scenario: Help lists zoom shortcuts
- **WHEN** the user opens the help view
- **THEN** the help text includes `+` / `-` zoom shortcuts

