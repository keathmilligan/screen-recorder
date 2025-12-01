# Change: Update UI Theme and Layout

## Why

The current UI uses a light theme and a centered button-group style for mode selection. This change updates the visual design to provide a more modern, polished look with a dark cool gray gradient theme, a left-aligned tab-style mode selector, and a fixed window size that properly contains all UI elements without overflow.

## What Changes

- Apply a dark cool gray gradient background theme to the main window
- Convert mode selection from centered toggle buttons to left-aligned tab-style interface
- Set fixed window dimensions that accommodate all UI elements without overflow
- Update color palette to use cool gray tones throughout
- Adjust element styling to complement the dark theme

## Impact

- Affected specs: New `ui-theme` capability
- Affected code:
  - `src/styles.css` - Complete theme overhaul
  - `index.html` - Minor structural adjustments for tab layout
  - `src-tauri/tauri.conf.json` - Window size configuration
