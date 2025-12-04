# Tasks: Refactor Platform Abstraction

## 1. Create Foundation

- [x] 1.1 Create directory structure (`capture/windows/`, `capture/linux/`, `capture/macos/`)
- [x] 1.2 Create `capture/types.rs` with shared types (`WindowInfo`, `MonitorInfo`, `CapturedFrame`, `CaptureRegion`)
- [x] 1.3 Create `capture/error.rs` with `CaptureError` and `EnumerationError` enums
- [x] 1.4 Define traits in `capture/mod.rs` (`CaptureBackend`, `WindowEnumerator`, `MonitorEnumerator`, `HighlightProvider`)

## 2. Relocate Windows Code

- [x] 2.1 Move `windows_list.rs` to `capture/windows/window_list.rs`
- [x] 2.2 Move `monitor_list.rs` to `capture/windows/monitor_list.rs`
- [x] 2.3 Move `recorder.rs` to `capture/windows/recorder.rs`
- [x] 2.4 Move `region_recorder.rs` to `capture/windows/region.rs`
- [x] 2.5 Move `highlight.rs` to `capture/windows/highlight.rs`
- [x] 2.6 Create `capture/windows/mod.rs` to re-export Windows module

## 3. Implement Windows Traits

- [x] 3.1 Implement `WindowEnumerator` for Windows backend
- [x] 3.2 Implement `MonitorEnumerator` for Windows backend
- [x] 3.3 Implement `CaptureBackend` for Windows backend (window, region, display capture)
- [x] 3.4 Implement `HighlightProvider` for Windows backend
- [x] 3.5 Create `WindowsBackend` struct combining all traits

## 4. Create Platform Stubs

- [x] 4.1 Create `capture/linux/mod.rs` with `LinuxBackend` stub returning `NotImplemented`
- [x] 4.2 Create `capture/macos/mod.rs` with `MacOSBackend` stub returning `NotImplemented`
- [x] 4.3 Add platform dispatch in `capture/mod.rs` using `#[cfg(target_os)]`

## 5. Update Consumers

- [x] 5.1 Update `lib.rs` to use platform abstraction for `get_windows` command
- [x] 5.2 Update `lib.rs` to use platform abstraction for `get_monitors` command
- [x] 5.3 Update `lib.rs` to use platform abstraction for `show_display_highlight` command
- [x] 5.4 Update `state.rs` to use `CaptureBackend` trait for recording operations
- [x] 5.5 Remove direct imports of Windows-specific modules from shared code

## 6. Update Build Configuration

- [x] 6.1 Reorganize `Cargo.toml` with platform-conditional dependencies
- [x] 6.2 Move `windows-capture` to `[target.'cfg(windows)'.dependencies]`
- [x] 6.3 Move `windows` crate to `[target.'cfg(windows)'.dependencies]`
- [x] 6.4 Add placeholder sections for Linux and macOS dependencies (commented)

## 7. Validation

- [ ] 7.1 Verify Windows build succeeds with `cargo build --target x86_64-pc-windows-msvc`
- [x] 7.2 Verify Linux build succeeds with `cargo build --target x86_64-unknown-linux-gnu` (stub only)
- [ ] 7.3 Verify macOS build succeeds with `cargo build --target x86_64-apple-darwin` (stub only)
- [x] 7.4 Run existing tests on Linux (`cargo test`)
- [ ] 7.5 Test window enumeration functionality on Windows
- [ ] 7.6 Test monitor enumeration functionality on Windows
- [ ] 7.7 Test window recording functionality on Windows
- [ ] 7.8 Test region recording functionality on Windows
- [ ] 7.9 Test display recording functionality on Windows
- [ ] 7.10 Test highlight functionality on Windows

## Dependencies

- Tasks in section 2 depend on section 1
- Tasks in section 3 depend on section 2
- Tasks in section 4 can run in parallel with section 3
- Tasks in section 5 depend on sections 3 and 4
- Tasks in section 6 can run in parallel with sections 2-5
- Tasks in section 7 depend on all previous sections

## Notes

- Linux and macOS validation (7.1, 7.3) require cross-compilation or testing on native systems
- Windows-specific tests (7.5-7.10) require testing on a Windows machine
- Linux build successfully compiles and all Rust tests pass on Linux
- The platform abstraction is fully implemented with stub backends for Linux and macOS
