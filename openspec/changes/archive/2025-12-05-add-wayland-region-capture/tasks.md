# Implementation Tasks

## 1. Implement Region Capture Backend

- [x] 1.1 Remove stub implementation from `LinuxBackend::start_region_capture()` in `src-tauri/src/capture/linux/mod.rs`
- [x] 1.2 Call `portal_client.request_region_capture()` with monitor_id and region geometry (x, y, width, height)
- [x] 1.3 Await the PipeWire stream setup and extract node_id
- [x] 1.4 Set up PipeWire capture for the full monitor stream
- [x] 1.5 Implement frame cropping logic to extract region from full monitor frames
- [x] 1.6 Return frame receiver and stop handle

## 2. Frame Cropping Implementation

- [x] 2.1 Add region coordinates to PipeWire capture state
- [x] 2.2 In frame processing callback, crop each frame to region dimensions
- [x] 2.3 Ensure cropped frame has correct dimensions (region width x height)
- [x] 2.4 Handle edge cases: region extends beyond monitor, monitor resolution changes

## 3. Testing and Validation

- [ ] 3.1 Test region selection and recording on Hyprland (manual testing required)
- [ ] 3.2 Verify picker outputs correct region format to XDPH (manual testing required)
- [ ] 3.3 Verify PipeWire stream is cropped correctly (manual testing required)
- [ ] 3.4 Test edge cases: small regions, regions near monitor edges, multi-monitor setups
- [ ] 3.5 Verify recording stops cleanly and file is saved with correct dimensions

## 4. Documentation

- [x] 4.1 Update Linux-specific documentation if needed
- [x] 4.2 Remove any "Phase 3" or "not implemented" comments related to region capture
