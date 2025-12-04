# Change: Refactor Platform Abstraction Architecture

## Why

The codebase is currently Windows-centric with platform-specific code (Windows APIs, `windows-capture` crate) mixed throughout the capture modules. This makes it difficult to:

1. Add support for Linux and macOS platforms
2. Maintain platform-specific code in isolation
3. Share common logic (encoding, state management, UI) across platforms
4. Manage platform-specific dependencies cleanly

A clean separation of platform-dependent and platform-independent code is needed before implementing Linux (Wayland/PipeWire) and macOS (ScreenCaptureKit) capture backends.

## What Changes

- **NEW: Platform abstraction traits** defining the interface for capture operations (window enumeration, monitor enumeration, frame capture)
- **NEW: Platform module organization** with dedicated folders (`capture/windows/`, `capture/linux/`, `capture/macos/`) for platform-specific implementations
- **REFACTORED: Windows capture code** moved from `capture/*.rs` to `capture/windows/` with trait implementations
- **REFACTORED: Cargo.toml** with platform-conditional dependencies using `[target.'cfg(...)'.dependencies]`
- **NEW: Stub implementations** for Linux and macOS that return "not implemented" errors (actual implementations deferred)
- **REFACTORED: Tauri commands** to use the abstraction layer instead of direct Windows API calls
- **REFACTORED: Highlight module** moved to platform-specific location with cross-platform trait

### Out of Scope

- Actual Linux capture implementation (Wayland/PipeWire) - separate change
- Actual macOS capture implementation (ScreenCaptureKit) - separate change
- Audio capture abstraction - separate change
- UI changes - frontend remains unchanged

## Impact

- Affected specs:
  - `window-capture` (implementation becomes platform-dispatched)
  - `display-capture` (implementation becomes platform-dispatched)
  - `region-capture` (implementation becomes platform-dispatched)
  - NEW `platform-abstraction` (cross-platform interfaces)
- Affected code:
  - `src-tauri/src/capture/` - Major restructuring
  - `src-tauri/src/lib.rs` - Updated to use abstraction layer
  - `src-tauri/src/highlight.rs` - Moved to platform module
  - `src-tauri/Cargo.toml` - Platform-conditional dependencies
- No new runtime dependencies (just reorganization)
- Windows functionality unchanged after refactor
