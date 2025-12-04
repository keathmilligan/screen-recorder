# Tasks: Add Wayland/Linux Support (Hyprland)

## Phase 1: Picker Service + IPC + Display Capture

### 1.1 Platform Abstraction Layer (COMPLETED)
- [x] 1.1.1 Create `CaptureBackend` trait in `src-tauri/src/capture/mod.rs`
- [x] 1.1.2 Move Windows code to `src-tauri/src/capture/windows/` module
- [x] 1.1.3 Implement `WindowsBackend` wrapping existing code
- [x] 1.1.4 Add conditional compilation gates
- [x] 1.1.5 Update `Cargo.toml` with platform-specific dependencies
- [x] 1.1.6 Create Linux stub backend
- [x] 1.1.7 Create macOS stub backend

### 1.2 Picker Service Binary
- [x] 1.2.1 Create `src-picker/` crate with Cargo.toml
- [x] 1.2.2 Implement D-Bus service registration (`org.freedesktop.impl.portal.desktop.screenrecorder`)
- [x] 1.2.3 Implement `org.freedesktop.impl.portal.ScreenCast` interface
  - [x] Properties: AvailableSourceTypes, AvailableCursorModes, version
  - [x] CreateSession method
  - [x] SelectSources method  
  - [x] Start method (queries main app via IPC)
- [x] 1.2.4 Implement IPC client (Unix socket connection to main app)
- [x] 1.2.5 Handle "main app not running" case (deny request)
- [x] 1.2.6 Add logging for debugging

### 1.3 IPC Protocol
- [x] 1.3.1 Define IPC message types (JSON-based)
  - [x] QuerySelection request
  - [x] Selection response (source_type, source_id, geometry)
  - [x] NoSelection response
- [x] 1.3.2 Implement IPC server in main app (`src-tauri/src/capture/linux/ipc_server.rs`)
- [x] 1.3.3 Implement IPC client in picker (`src-picker/src/ipc_client.rs`)
- [x] 1.3.4 Handle connection/reconnection logic
- [x] 1.3.5 Socket path: `$XDG_RUNTIME_DIR/screen-recorder/picker.sock`

### 1.4 Hyprland Monitor Enumeration
- [x] 1.4.1 Create Hyprland IPC in `src-tauri/src/capture/linux/mod.rs` (using hyprland crate)
- [x] 1.4.2 Implement Hyprland IPC socket connection (via hyprland crate)
- [x] 1.4.3 Implement `list_monitors()` via Hyprland crate's `Monitors::get()`
- [x] 1.4.4 Map Hyprland monitor data to `MonitorInfo` struct
- [x] 1.4.5 Implement `MonitorEnumerator` trait for `LinuxBackend`
- [x] 1.4.6 Handle Hyprland not running (error)

### 1.5 Portal Client (Main App)
- [x] 1.5.1 Add `ashpd` crate dependency
- [x] 1.5.2 Create `portal_client.rs` with basic portal flow
- [x] 1.5.3 Integrate portal client with IPC server (store selection before request)
- [x] 1.5.4 Handle portal response and extract PipeWire node ID

### 1.6 Installation Files
- [x] 1.6.1 Create `resources/linux/screen-recorder.portal`
- [x] 1.6.2 Create `resources/linux/screen-recorder-picker.service` (systemd user service)
- [x] 1.6.3 Create `resources/linux/hyprland-portals.conf`
- [x] 1.6.4 Document installation steps in README

### 1.7 Integration Testing
- [x] 1.7.1 Test picker service starts and registers on D-Bus
- [x] 1.7.2 Test IPC connection between app and picker
- [x] 1.7.3 Test portal request routes to our picker
- [x] 1.7.4 Test auto-approval with display selection
- [x] 1.7.5 Verify PipeWire node ID returned correctly

## Phase 2: PipeWire Capture + Window Support

### 2.1 PipeWire Video Capture
- [ ] 2.1.1 Add `pipewire` crate dependency
- [ ] 2.1.2 Create `src-tauri/src/capture/linux/pipewire.rs`
- [ ] 2.1.3 Implement PipeWire stream connection from node ID
- [ ] 2.1.4 Implement frame buffer handling (SHM path first)
- [ ] 2.1.5 Convert frames to BGRA format
- [ ] 2.1.6 Implement frame channel (FrameReceiver)
- [ ] 2.1.7 Handle stream errors and disconnection

### 2.2 Display Recording Integration
- [ ] 2.2.1 Implement `start_display_capture()` for LinuxBackend
- [ ] 2.2.2 Wire up selection → IPC → portal → PipeWire flow
- [ ] 2.2.3 Test full display recording on Hyprland
- [ ] 2.2.4 Verify output video quality and framerate

### 2.3 Hyprland Window Enumeration
- [x] 2.3.1 Implement `list_windows()` via Hyprland crate's `Clients::get()` (done in 1.4)
- [x] 2.3.2 Map Hyprland client data to `WindowInfo` struct (done in 1.4)
- [x] 2.3.3 Filter out hidden/special windows (done in 1.4)
- [x] 2.3.4 Implement `WindowEnumerator` trait for LinuxBackend (done in 1.4)
- [x] 2.3.5 Map window address to usable ID for portal (done in 1.4)

### 2.4 Window Capture Integration
- [x] 2.4.1 Extend IPC protocol for window selection (done in 1.3)
- [x] 2.4.2 Update picker to handle window source type (done in 1.2)
- [ ] 2.4.3 Implement `start_window_capture()` for LinuxBackend
- [ ] 2.4.4 Handle window resize during capture
- [ ] 2.4.5 Handle window close during capture
- [ ] 2.4.6 Test full window recording on Hyprland

## Phase 3: Region Capture

### 3.1 Region Selection
- [ ] 3.1.1 Adapt selection overlay for Wayland/Tauri
- [ ] 3.1.2 Get region bounds relative to monitor
- [ ] 3.1.3 Store region in app state for IPC

### 3.2 Region Capture Implementation
- [x] 3.2.1 Extend IPC protocol for region selection (monitor + bounds) - done in 1.3
- [ ] 3.2.2 Capture full monitor via portal
- [ ] 3.2.3 Implement region cropping from monitor stream
- [ ] 3.2.4 Handle region boundary validation
- [ ] 3.2.5 Test region recording on Hyprland

## Phase 4: Audio Capture

### 4.1 PipeWire Audio Integration
- [ ] 4.1.1 Extend portal request to include audio sources
- [ ] 4.1.2 Update picker to approve audio
- [ ] 4.1.3 Implement audio stream handling in PipeWire capture
- [ ] 4.1.4 Create audio sample buffer and channel

### 4.2 Audio Encoding
- [ ] 4.2.1 Add audio encoding to FFmpeg pipeline
- [ ] 4.2.2 Implement audio-video timestamp synchronization
- [ ] 4.2.3 Test muxed output file

## Phase 5: Polish & Error Handling

### 5.1 Error Handling
- [ ] 5.1.1 Handle picker service not running
- [ ] 5.1.2 Handle main app not running (picker side)
- [ ] 5.1.3 Handle IPC connection failures
- [ ] 5.1.4 Handle PipeWire disconnection
- [ ] 5.1.5 Handle Hyprland IPC failures
- [ ] 5.1.6 User-friendly error messages

### 5.2 Performance Optimization
- [ ] 5.2.1 Investigate DMA-BUF support for zero-copy capture
- [ ] 5.2.2 Profile frame processing pipeline
- [ ] 5.2.3 Optimize buffer allocations

### 5.3 Documentation & Installation
- [ ] 5.3.1 Document Linux build requirements
- [ ] 5.3.2 Document installation steps
- [ ] 5.3.3 Create install script for picker service
- [ ] 5.3.4 Document Hyprland version requirements

## Validation Checkpoints

- [x] **V0 (POC)**: Portal client can request screencast (with system picker UI)
- [x] **V1**: Picker service registers on D-Bus and receives requests
- [x] **V2**: IPC works between main app and picker
- [ ] **V3**: Display capture works end-to-end (no system picker UI) - requires Phase 2 PipeWire
- [ ] **V4**: Window capture works with Hyprland enumeration
- [ ] **V5**: Region capture works with overlay
- [ ] **V6**: Audio capture works and syncs with video
- [ ] **V7**: All error paths handled gracefully

## Dependencies

```
Phase 1:
  1.2 (Picker) depends on nothing
  1.3 (IPC) depends on nothing
  1.4 (Hyprland) depends on nothing
  1.5 (Portal client) depends on 1.3
  1.6 (Installation) depends on 1.2
  1.7 (Integration) depends on 1.2, 1.3, 1.4, 1.5, 1.6

Phase 2:
  2.1 (PipeWire) depends on Phase 1
  2.2 (Display recording) depends on 2.1
  2.3 (Window enum) depends on 1.4
  2.4 (Window capture) depends on 2.2, 2.3

Phase 3:
  All depend on Phase 2

Phase 4:
  All depend on Phase 2

Phase 5:
  Depends on Phases 1-4
```

## Notes

- The picker service is a **separate binary** that must be installed and running
- Main app uses **Hyprland IPC directly** for enumeration (no portal)
- Main app uses **portal** only for capture authorization
- Picker **auto-approves** based on IPC query to main app
- If main app not running, picker **denies** the request (for now)
