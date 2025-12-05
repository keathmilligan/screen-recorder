# Change: Add Wayland Region Capture

## Why

The region selection and recording feature is fully implemented for Windows but not yet enabled for Linux/Wayland (Hyprland). The infrastructure is already in place:
- Region selection overlay UI works across platforms
- Custom picker (`screen-recorder-picker`) already supports region output format
- IPC protocol includes region geometry
- Portal client has region capture method ready

Only the Linux capture backend's `start_region_capture()` method needs implementation to complete the feature.

## What Changes

- **Implement `start_region_capture()` in Linux backend** (`src-tauri/src/capture/linux/mod.rs`):
  - Call portal client's `request_region_capture()` with monitor ID and region geometry
  - Receive PipeWire stream for the full monitor
  - Crop incoming frames to the selected region coordinates
  - Return frame receiver and stop handle as expected by the trait

- Region capture on Wayland/Hyprland works by:
  1. User selects region in overlay (already works)
  2. Main app stores region selection in IPC state
  3. Portal client requests monitor capture with region metadata
  4. Custom picker returns region format to XDPH: `[SELECTION]/region:DP-1@x,y,w,h`
  5. PipeWire stream captures full monitor
  6. Rust backend crops each frame to region dimensions before encoding

- **Update specs**:
  - `wayland-capture` - Document region capture implementation
  - `region-capture` - Note Linux/Wayland platform support

## Impact

- Affected specs:
  - `wayland-capture` (add region capture implementation details)
  - `region-capture` (mark as supported on Linux/Wayland)

- Affected code:
  - `src-tauri/src/capture/linux/mod.rs` - Implement `start_region_capture()`
  - `src-tauri/src/capture/linux/pipewire_capture.rs` - May need frame cropping logic

- Dependencies: No new dependencies required (PipeWire and portal client already integrated)

- User-visible changes:
  - Region recording becomes available on Linux/Wayland (Hyprland)
  - Feature parity with Windows for region capture
