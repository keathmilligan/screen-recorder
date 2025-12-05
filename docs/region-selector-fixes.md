# Region Selector Fixes for Hyprland

This document summarizes the fixes applied to make the region selector work correctly on Hyprland/Wayland.

## Issues Fixed

### Issue 1: Wrong URL - Showing Main Window Instead of Overlay

**Problem**: The region selector window was loading the main window content instead of the selection overlay.

**Root Cause**: The URL path was incorrect. Vite builds multi-page apps with their directory structure preserved, so `src/selection-overlay.html` is built to `dist/src/selection-overlay.html`, not `dist/selection-overlay.html`.

**Fix**: Updated the URL in `src/main.ts` (line 225-226):

```typescript
// Before (incorrect):
const overlayUrl = isDev
  ? "http://localhost:1420/selection-overlay.html"
  : "selection-overlay.html";

// After (correct):
const overlayUrl = isDev
  ? "http://localhost:1420/src/selection-overlay.html"
  : "src/selection-overlay.html";
```

**File Modified**: `src/main.ts`

---

### Issue 2: Window Being Tiled Instead of Floating

**Problem**: On Hyprland, the region selector window was being tiled into the workspace layout instead of appearing as a floating overlay.

**Root Cause**: 
- Tauri's `WebviewWindow` API doesn't expose window class/app_id configuration on Wayland
- Hyprland tiles all windows by default unless window rules specify otherwise
- Window properties like `transparent`, `alwaysOnTop` are not sufficient on Hyprland

**Fix**: Added a Tauri command to dynamically configure Hyprland window rules via `hyprctl`:

1. **Backend** (`src-tauri/src/lib.rs`):
   - New command: `configure_region_selector_window`
   - Executes `hyprctl keyword windowrulev2 <rule>` for each rule
   - Only runs on Hyprland (checks `HYPRLAND_INSTANCE_SIGNATURE` env var)

2. **Frontend** (`src/main.ts`):
   - Calls `configure_region_selector_window` after window creation
   - Handles errors gracefully

**Window Rules Applied**:
- `float,title:^(Region Selection)$` - Makes window floating
- `noborder,title:^(Region Selection)$` - Removes borders
- `noshadow,title:^(Region Selection)$` - Removes shadows
- `noblur,title:^(Region Selection)$` - Disables blur
- `rounding 0,title:^(Region Selection)$` - Sharp corners

**Files Modified**: 
- `src-tauri/src/lib.rs` (new command)
- `src/main.ts` (call to new command)

---

## Testing Steps

1. **Build the application**:
   ```bash
   pnpm build
   cd src-tauri
   cargo build
   ```

2. **Run the application** on Hyprland:
   ```bash
   cargo run
   ```

3. **Test region selection**:
   - Switch to "Region" capture mode
   - Click "Select Region" button
   - Expected: Transparent overlay window with resize handles appears
   - Expected: Window is floating (not tiled)
   - Expected: Window shows dimension display, not main UI

4. **Verify behavior**:
   - Window should be transparent with visible borders
   - Resize handles (corners and edges) should be visible
   - Dragging center should move the window
   - Dragging edges/corners should resize
   - Pressing Escape should close the overlay
   - Dimensions should update in real-time

5. **Check console logs**:
   ```
   Creating selector window: { overlayUrl: "src/selection-overlay.html", ... }
   [configure_region_selector] Configuring Hyprland rules for window: region-selector
   [configure_region_selector] Applied: float,title:^(Region Selection)$
   ...
   ```

---

## Debugging

If the region selector still doesn't work:

1. **Check the URL is correct**:
   ```typescript
   console.log("Creating selector window:", { overlayUrl, ... });
   ```
   Should show: `overlayUrl: "src/selection-overlay.html"` (production) or `"http://localhost:1420/src/selection-overlay.html"` (dev)

2. **Verify file exists**:
   ```bash
   ls -la dist/src/selection-overlay.html  # Production
   ls -la src/selection-overlay.html       # Dev
   ```

3. **Check Hyprland window rules**:
   ```bash
   hyprctl clients | grep -A 10 "Region Selection"
   ```
   Should show `floating: 1`

4. **Check hyprctl is working**:
   ```bash
   hyprctl keyword windowrulev2 float,title:^(Region Selection)$
   ```
   Should not error

5. **Check browser console** (if using dev mode):
   - Open the region selector window's dev tools
   - Check for JavaScript errors
   - Verify monitor list is loaded
   - Check region-updated events are firing

---

## Known Limitations

1. **Window rules persist**: The Hyprland window rules are added to the current session and persist until Hyprland restarts. This is normal behavior.

2. **Title collision**: If another application uses "Region Selection" as a window title, these rules will apply to it too. This is unlikely but possible.

3. **No dev tools shortcut**: The region selector window doesn't have a menu bar, so you can't open dev tools with F12. Use the Tauri dev tools API if needed.

---

## Related Files

- `src/selection-overlay.html` - Overlay UI markup
- `src/selection-overlay.ts` - Overlay logic (resize, drag, events)
- `src/selection-overlay.css` - Overlay styling
- `src/main.ts` - Region selector window creation
- `src-tauri/src/lib.rs` - Hyprland configuration command
- `vite.config.ts` - Multi-page build configuration
