# ui-theme Specification

## Purpose
TBD - created by archiving change update-ui-theme. Update Purpose after archive.
## Requirements
### Requirement: Dark Cool Gray Theme

The application SHALL display a dark cool gray gradient theme as the default visual appearance.

#### Scenario: Dark gradient background applied

- **WHEN** the application window is displayed
- **THEN** the background SHALL show a cool gray gradient (dark gray tones)
- **AND** all text colors SHALL provide sufficient contrast for readability

#### Scenario: Component colors match theme

- **WHEN** the application window is displayed
- **THEN** buttons, lists, and input areas SHALL use complementary dark gray tones
- **AND** accent colors SHALL be visible against the dark background

### Requirement: Tab-Style Mode Selection

The capture mode selection interface SHALL use a left-aligned tab-style layout.

#### Scenario: Mode tabs displayed on left

- **WHEN** the mode selection area is rendered
- **THEN** the Window, Region, and Display tabs SHALL be aligned to the left
- **AND** the tabs SHALL be displayed in a horizontal row

#### Scenario: Active tab indication

- **WHEN** a capture mode tab is selected
- **THEN** the active tab SHALL be visually distinguished (e.g., underline, background change)
- **AND** inactive tabs SHALL appear visually muted

#### Scenario: Tab interaction

- **WHEN** a user clicks an inactive tab
- **THEN** that tab SHALL become the active tab
- **AND** the corresponding capture mode section SHALL be displayed

### Requirement: Fixed Window Size

The application window SHALL have a fixed size that contains all UI elements without overflow.

#### Scenario: Window dimensions prevent overflow

- **WHEN** the application window is displayed
- **THEN** all UI elements (tabs, lists, buttons, status) SHALL be fully visible
- **AND** no content SHALL overflow or be clipped

#### Scenario: Window resize disabled

- **WHEN** the user attempts to resize the application window
- **THEN** the window size SHALL remain fixed

