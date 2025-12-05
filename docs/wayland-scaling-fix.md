# Wayland HiDPI Scaling Fix

## Problem

On Wayland/Hyprland with HiDPI displays (scale > 1.0), the recorded region was larger than the selected area. For example, on a 2x scaled display, selecting a 400x300 region would record an 800x600 area.

## Root Cause

**Coordinate System Mismatch**:

1. **Tauri Window API**: Returns positions and sizes in **logical pixels**
   - Logical pixels = Physical pixels / Scale factor
   - Example: On a 2x display, a 400px logical width = 800px physical width

2. **Hyprland IPC**: Reports monitor dimensions in **physical pixels**
   - Example: A 3840x2160 monitor at 2x scale reports 3840x2160 (not 1920x1080)

3. **PipeWire Capture**: Captures frames in **physical pixels**
   - The actual video stream is in physical pixel dimensions

The region selector was mixing these coordinate systems:
- Window position from Tauri (logical) - Monitor position from Hyprland (physical) = **Wrong coordinates**
- Window size from Tauri (logical) directly to capture = **Wrong size**

## Solution

Convert all coordinates to physical pixels before sending to the capture backend.

### Changes Made

**File**: `src/selection-overlay.ts`

#### 1. Get Scale Factor

```typescript
const scaleFactor = await currentWindow.scaleFactor();
```

#### 2. Convert Window Coordinates to Physical Pixels

```typescript
// Tauri returns logical pixels
const logicalRecordX = pos.x + BORDER_WIDTH;
const logicalRecordY = pos.y + BORDER_WIDTH;
const logicalRecordWidth = size.width - (BORDER_WIDTH * 2);
const logicalRecordHeight = size.height - (BORDER_WIDTH * 2);

// Convert to physical pixels
const physicalRecordX = Math.round(logicalRecordX * scaleFactor);
const physicalRecordY = Math.round(logicalRecordY * scaleFactor);
const physicalRecordWidth = Math.round(logicalRecordWidth * scaleFactor);
const physicalRecordHeight = Math.round(logicalRecordHeight * scaleFactor);
```

#### 3. Use Physical Coordinates for Monitor Matching

```typescript
// Find monitor using physical coordinates (matches Hyprland's coordinate system)
const centerX = physicalRecordX + physicalRecordWidth / 2;
const centerY = physicalRecordY + physicalRecordHeight / 2;
let monitor = findMonitorAt(centerX, centerY);
```

#### 4. Calculate Monitor-Relative Coordinates

```typescript
// Both values now in physical pixels
const region: CaptureRegion = {
  monitor_id: monitor.id,
  monitor_name: monitor.name,
  x: physicalRecordX - monitor.x,  // Physical - Physical = Correct
  y: physicalRecordY - monitor.y,
  width: physicalRecordWidth,
  height: physicalRecordHeight,
};
```

#### 5. Display Physical Dimensions

Updated the dimension display to show physical pixels (what will actually be recorded):

```typescript
async function updateDisplay(): Promise<void> {
  const size = await currentWindow.innerSize();
  const scaleFactor = await currentWindow.scaleFactor();
  
  const logicalRecordWidth = size.width - (BORDER_WIDTH * 2);
  const logicalRecordHeight = size.height - (BORDER_WIDTH * 2);
  
  // Show physical pixels (what will actually be recorded)
  const physicalRecordWidth = Math.round(logicalRecordWidth * scaleFactor);
  const physicalRecordHeight = Math.round(logicalRecordHeight * scaleFactor);
  
  dimensionsEl.textContent = `${physicalRecordWidth} × ${physicalRecordHeight}`;
}
```

## Example: 2x Scaled Display

**Before Fix**:
- User selects: 400x300 region (logical)
- Sent to capture: x=100, y=50, width=400, height=300
- Actually recorded: 800x600 region (2x larger)

**After Fix**:
- User selects: 400x300 region (logical)
- Scale factor: 2.0
- Converted: x=200, y=100, width=800, height=600 (physical)
- Sent to capture: x=200, y=100, width=800, height=600
- Actually recorded: 800x600 region (correct!)
- Display shows: "800 × 600" (physical pixels)

## Verification

To verify the fix is working:

1. **Check scale factor in console**:
   ```
   Region (scale=2, logical=400x300, physical=800x600): {...}
   ```

2. **Check dimension display**:
   - The overlay should show physical pixel dimensions
   - On 2x display: 400px logical width displays as "800"

3. **Record and verify**:
   - Select a specific area (e.g., a window corner to corner)
   - Record and check the output video dimensions
   - Should match the physical dimensions shown in the overlay

4. **Check backend logs**:
   ```
   [Linux] Monitor DP-1: 3840x2160 scale=2
   [Linux] Starting region capture for DP-1 (800x600 at 200,100)
   [PipeWire] Starting capture thread for node 45 (3840x2160) with crop region (800x600 at 200,100)
   ```

## Related Changes

Also added debug logging in `src-tauri/src/capture/linux/mod.rs`:

```rust
eprintln!("[Linux] Monitor {}: {}x{} scale={}", 
    monitor.name, monitor.width, monitor.height, monitor.scale);
```

This helps verify that Hyprland is reporting the correct scale factor.

## Testing on Different Scale Factors

| Scale | Logical | Physical | Notes |
|-------|---------|----------|-------|
| 1.0 | 400x300 | 400x300 | No scaling |
| 1.25 | 400x300 | 500x375 | Fractional scaling |
| 1.5 | 400x300 | 600x450 | Common laptop scale |
| 2.0 | 400x300 | 800x600 | HiDPI/Retina |

The fix works for all scale factors, including fractional scaling.
