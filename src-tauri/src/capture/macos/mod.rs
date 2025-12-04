//! macOS platform capture implementation (stub).
//!
//! This module provides stub implementations that return NotImplemented errors.
//! Actual macOS capture support (via ScreenCaptureKit) will be added in a future change.

use crate::capture::error::{CaptureError, EnumerationError};
use crate::capture::types::{
    CaptureRegion, FrameReceiver, MonitorInfo, StopHandle, WindowInfo,
};
use crate::capture::{CaptureBackend, HighlightProvider, MonitorEnumerator, WindowEnumerator};

/// macOS platform capture backend (stub).
pub struct MacOSBackend;

impl MacOSBackend {
    /// Create a new macOS backend.
    pub fn new() -> Self {
        Self
    }
}

impl Default for MacOSBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowEnumerator for MacOSBackend {
    fn list_windows(&self) -> Result<Vec<WindowInfo>, EnumerationError> {
        Err(EnumerationError::NotImplemented(
            "macOS window enumeration not yet implemented. ScreenCaptureKit support coming soon.".to_string()
        ))
    }
}

impl MonitorEnumerator for MacOSBackend {
    fn list_monitors(&self) -> Result<Vec<MonitorInfo>, EnumerationError> {
        Err(EnumerationError::NotImplemented(
            "macOS monitor enumeration not yet implemented. ScreenCaptureKit support coming soon.".to_string()
        ))
    }
}

impl CaptureBackend for MacOSBackend {
    fn start_window_capture(
        &self,
        _window_handle: isize,
    ) -> Result<(FrameReceiver, StopHandle), CaptureError> {
        Err(CaptureError::NotImplemented(
            "macOS window capture not yet implemented. ScreenCaptureKit support coming soon.".to_string()
        ))
    }

    fn start_region_capture(
        &self,
        _region: CaptureRegion,
    ) -> Result<(FrameReceiver, StopHandle), CaptureError> {
        Err(CaptureError::NotImplemented(
            "macOS region capture not yet implemented. ScreenCaptureKit support coming soon.".to_string()
        ))
    }

    fn start_display_capture(
        &self,
        _monitor_id: String,
        _width: u32,
        _height: u32,
    ) -> Result<(FrameReceiver, StopHandle), CaptureError> {
        Err(CaptureError::NotImplemented(
            "macOS display capture not yet implemented. ScreenCaptureKit support coming soon.".to_string()
        ))
    }
}

impl HighlightProvider for MacOSBackend {
    fn show_highlight(&self, _x: i32, _y: i32, _width: i32, _height: i32) {
        eprintln!("macOS display highlight not yet implemented");
    }
}
