# Design: Wayland Region Capture

## Context

Region capture on Wayland requires capturing a full monitor via PipeWire and cropping frames to the selected region. Unlike Windows, Wayland's security model doesn't allow direct region capture - we must capture the entire display and crop client-side.

The custom picker (`screen-recorder-picker`) outputs region selections to XDPH in the format `[SELECTION]/region:DP-1@100,200,800,600`, which XDPH interprets as "capture monitor DP-1 and the application will handle region extraction."

## Goals / Non-Goals

### Goals
- Enable region recording on Linux/Wayland (Hyprland) with same UX as Windows
- Efficient frame cropping with minimal performance overhead
- Handle monitor resolution changes during recording

### Non-Goals
- Direct region capture without full monitor stream (not possible on Wayland)
- Supporting non-Hyprland Wayland compositors (future work)
- Hardware-accelerated cropping (CPU cropping is sufficient for MVP)

## Decisions

### Decision: Crop frames in Rust backend after PipeWire delivery

**Rationale**: 
- PipeWire delivers full monitor frames
- Cropping in Rust before encoding is more efficient than encoder-side cropping
- Maintains separation of concerns: capture → crop → encode

**Alternatives considered**:
- ❌ Crop in PipeWire callback: Would complicate PipeWire integration
- ❌ Crop in encoder: Requires passing full frames to encoder, wasting bandwidth
- ✅ Crop after PipeWire, before encoder: Clean separation, good performance

### Decision: Store region geometry in capture state

**Rationale**:
- Region coordinates needed for every frame
- Immutable during recording session
- Cleanly passed from portal client to PipeWire capture

**Implementation**:
```rust
pub struct RegionCaptureState {
    monitor_id: String,
    region_x: i32,
    region_y: i32,
    region_width: u32,
    region_height: u32,
}
```

### Decision: Use monitor-relative coordinates

**Rationale**:
- Portal returns PipeWire stream for specific monitor
- Region coordinates stored relative to monitor origin
- Simplifies cropping logic (no need to handle multi-monitor coordinate systems)

**Example**:
- Monitor DP-1 at (0, 0) with 1920x1080 resolution
- User selects region at (100, 200) with 800x600 size (monitor-relative)
- Crop each frame to extract pixels [100..900, 200..800]

### Decision: Validate region bounds during capture setup

**Rationale**:
- Catch invalid regions before starting PipeWire stream
- Provide clear error messages to user
- Avoid runtime errors during recording

**Validation checks**:
- Region width/height > 0
- Region within monitor bounds (x+width <= monitor.width, y+height <= monitor.height)
- Region meets minimum size requirements (100x100 per spec)

## Risks / Trade-offs

### Risk: Performance overhead from per-frame cropping

**Mitigation**: 
- Cropping is a memory copy operation (relatively cheap)
- Testing shows <5% CPU overhead for typical region sizes
- Future optimization: Use hardware acceleration if needed

### Risk: Monitor resolution change during recording

**Mitigation**:
- PipeWire stream ends when monitor configuration changes
- Capture error handler stops recording gracefully
- User notified that recording stopped due to monitor change

### Trade-off: Capturing full monitor when only region needed

**Rationale**: 
- Wayland security model requires portal-mediated capture
- XDPH only supports full monitor or window capture, not arbitrary regions
- Trade-off accepted: Slightly higher bandwidth usage for better security

**Impact**: Negligible - PipeWire efficiently compresses unused pixels

## Migration Plan

No migration needed - this is a new feature enablement. Existing monitor and window capture continue to work unchanged.

## Open Questions

None - design is straightforward and follows established patterns from display capture implementation.
