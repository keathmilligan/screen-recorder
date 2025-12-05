# wayland-capture Specification Deltas

## MODIFIED Requirements

### Requirement: Region Capture via PipeWire

The system SHALL capture screen regions by capturing the full monitor via PipeWire and cropping frames to the selected region.

#### Scenario: Capture region from display stream

- **WHEN** recording starts in region mode
- **THEN** a full display stream is obtained via portal for the region's monitor
- **AND** each frame is cropped to the selected region coordinates (monitor-relative)
- **AND** output dimensions match the region size
- **AND** cropped frames are delivered at the configured frame rate

#### Scenario: Region extends beyond display

- **WHEN** the selected region extends beyond display boundaries
- **THEN** the region is clipped to valid boundaries during capture setup
- **AND** a warning is shown if significant clipping occurred
- **AND** recording proceeds with the clipped region

#### Scenario: Region validation before capture

- **WHEN** `start_region_capture()` is called
- **THEN** the backend validates region coordinates against monitor dimensions
- **AND** returns `CaptureError::InvalidRegion` if region is invalid
- **AND** the error message indicates the specific validation failure

#### Scenario: Monitor resolution changes during region recording

- **WHEN** the monitor resolution changes during region recording
- **THEN** the PipeWire stream ends
- **AND** recording stops gracefully
- **AND** partial recording is saved with original region dimensions
- **AND** the user is notified that recording stopped due to monitor change

## ADDED Requirements

### Requirement: Frame Cropping Performance

The system SHALL crop frames efficiently with minimal CPU overhead to avoid impacting recording quality.

#### Scenario: CPU overhead within acceptable limits

- **WHEN** recording a region from a full monitor stream
- **THEN** frame cropping adds no more than 5% CPU overhead compared to full monitor capture
- **AND** frame delivery rate maintains the configured FPS

#### Scenario: Memory-efficient cropping

- **WHEN** cropping frames
- **THEN** only the cropped region is copied to the output frame buffer
- **AND** full monitor frame buffers are released after cropping
- **AND** memory usage scales with region size, not full monitor size
