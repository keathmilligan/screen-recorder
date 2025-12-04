//! Linux platform capture implementation (stub).
//!
//! This module provides stub implementations that return NotImplemented errors.
//! Actual Linux capture support (via PipeWire/Wayland) will be added in a future change.

use crate::capture::error::{CaptureError, EnumerationError};
use crate::capture::types::{
    CaptureRegion, FrameReceiver, MonitorInfo, StopHandle, WindowInfo,
};
use crate::capture::{CaptureBackend, HighlightProvider, MonitorEnumerator, WindowEnumerator};

/// Linux platform capture backend (stub).
pub struct LinuxBackend;

impl LinuxBackend {
    /// Create a new Linux backend.
    pub fn new() -> Self {
        Self
    }
}

impl Default for LinuxBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowEnumerator for LinuxBackend {
    fn list_windows(&self) -> Result<Vec<WindowInfo>, EnumerationError> {
        Err(EnumerationError::NotImplemented(
            "Linux window enumeration not yet implemented. Wayland/PipeWire support coming soon.".to_string()
        ))
    }
}

impl MonitorEnumerator for LinuxBackend {
    fn list_monitors(&self) -> Result<Vec<MonitorInfo>, EnumerationError> {
        Err(EnumerationError::NotImplemented(
            "Linux monitor enumeration not yet implemented. Wayland/PipeWire support coming soon.".to_string()
        ))
    }
}

impl CaptureBackend for LinuxBackend {
    fn start_window_capture(
        &self,
        _window_handle: isize,
    ) -> Result<(FrameReceiver, StopHandle), CaptureError> {
        Err(CaptureError::NotImplemented(
            "Linux window capture not yet implemented. Wayland/PipeWire support coming soon.".to_string()
        ))
    }

    fn start_region_capture(
        &self,
        _region: CaptureRegion,
    ) -> Result<(FrameReceiver, StopHandle), CaptureError> {
        Err(CaptureError::NotImplemented(
            "Linux region capture not yet implemented. Wayland/PipeWire support coming soon.".to_string()
        ))
    }

    fn start_display_capture(
        &self,
        _monitor_id: String,
        _width: u32,
        _height: u32,
    ) -> Result<(FrameReceiver, StopHandle), CaptureError> {
        Err(CaptureError::NotImplemented(
            "Linux display capture not yet implemented. Wayland/PipeWire support coming soon.".to_string()
        ))
    }
}

impl HighlightProvider for LinuxBackend {
    fn show_highlight(&self, _x: i32, _y: i32, _width: i32, _height: i32) {
        eprintln!("Linux display highlight not yet implemented");
    }
}
