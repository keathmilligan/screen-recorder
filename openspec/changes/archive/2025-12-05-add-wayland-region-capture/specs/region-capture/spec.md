# region-capture Specification Deltas

## ADDED Requirements

### Requirement: Linux/Wayland Region Capture Support

The system SHALL support region capture on Linux/Wayland (Hyprland) using PipeWire and the custom picker.

#### Scenario: Region capture on Hyprland

- **WHEN** the application runs on Hyprland compositor
- **AND** the user selects a region and starts recording
- **THEN** the portal client requests region capture with monitor ID and geometry
- **AND** the custom picker outputs region format to XDPH: `[SELECTION]/region:<monitor>@<x>,<y>,<w>,<h>`
- **AND** PipeWire delivers full monitor frames
- **AND** frames are cropped to the selected region before encoding
- **AND** the output video dimensions match the selected region size

#### Scenario: Region selection stored in IPC state

- **WHEN** the user confirms a region selection
- **THEN** the main app stores the region in IPC state with source_type "region"
- **AND** the IPC state includes monitor ID and geometry (x, y, width, height)
- **AND** the picker can query this state when XDPH invokes it

#### Scenario: Picker outputs region format

- **WHEN** the picker receives a query with region selection
- **THEN** it outputs `[SELECTION]/region:<monitor_id>@<x>,<y>,<width>,<height>` to stdout
- **AND** XDPH interprets this as a monitor capture request
- **AND** the portal returns a PipeWire stream for the full monitor
