# Hyprland Region Selector Configuration

## Problem

On Hyprland (Wayland compositor), the region selector window was being tiled as a regular window instead of appearing as a floating overlay. This is because:

1. Tauri's `WebviewWindow` API doesn't provide a way to set custom window classes/app_ids on Wayland
2. Hyprland treats all windows as tiled by default unless window rules specify otherwise
3. Window properties like `transparent`, `alwaysOnTop`, and `decorations: false` are not sufficient on Hyprland

## Solution

We use `hyprctl` commands to dynamically add window rules after the region selector window is created. These rules match the window by its title ("Region Selection") and configure it to:

- Float instead of tiling
- Have no borders, shadows, or blur
- Have no rounding (sharp selection rectangle)

## Implementation

### Backend (Rust)

Added a new Tauri command `configure_region_selector_window` in `src-tauri/src/lib.rs`:

```rust
#[cfg(target_os = "linux")]
#[tauri::command]
async fn configure_region_selector_window(window_label: String) -> Result<(), String> {
    // Check if we're on Hyprland
    if std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_err() {
        return Ok(()); // Skip on non-Hyprland systems
    }
    
    // Apply window rules via hyprctl
    let rules = vec![
        "float,title:^(Region Selection)$",
        "noborder,title:^(Region Selection)$",
        "noshadow,title:^(Region Selection)$",
        "noblur,title:^(Region Selection)$",
        "rounding 0,title:^(Region Selection)$",
    ];
    
    for rule in rules {
        std::process::Command::new("hyprctl")
            .args(&["keyword", "windowrulev2", rule])
            .output();
    }
    
    Ok(())
}
```

### Frontend (TypeScript)

Called the command immediately after creating the region selector window in `src/main.ts`:

```typescript
const selector = new WebviewWindow("region-selector", {
  url: overlayUrl,
  title: "Region Selection",
  decorations: false,
  transparent: true,
  alwaysOnTop: true,
  skipTaskbar: true,
  // ... position and size
});

await selector.once("tauri://created", async () => {
  // Configure Hyprland window rules
  await invoke("configure_region_selector_window", { 
    windowLabel: "region-selector" 
  });
});
```

## Window Rules Explained

- **`float`**: Makes the window floating instead of tiled into the workspace layout
- **`noborder`**: Removes window borders for a clean overlay appearance
- **`noshadow`**: Removes drop shadows
- **`noblur`**: Disables background blur (important for transparent windows)
- **`rounding 0`**: Disables corner rounding for sharp selection rectangle edges

## Window Matching

The rules use regex matching on the window title:
- Pattern: `^(Region Selection)$`
- Matches exactly: "Region Selection"
- This is necessary because Tauri doesn't expose window class/app_id configuration

## Limitations

- Window rules are applied dynamically at runtime and persist in the Hyprland session
- Rules won't affect other applications with "Region Selection" titles (unlikely collision)
- If the user has conflicting window rules in their `hyprland.conf`, those take precedence

## Testing

To verify the fix works:

1. Start the application on Hyprland
2. Switch to Region capture mode
3. Click "Select Region"
4. The region selector should appear as a floating, transparent overlay
5. The window should be resizable by dragging edges/corners
6. The window should not be tiled into the workspace

## Future Improvements

- Use a more unique window title or investigate setting custom properties via Tauri plugins
- Automatically remove window rules when the application exits
- Consider using Hyprland layer-shell for true overlay windows (requires additional dependencies)
