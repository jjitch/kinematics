# Phase 7 — UI & Polish

## Goal
Deliver a complete, usable tool with a clean interface, simulation playback, and data export.

---

## Tasks

### 7.1 Layout & Shell
- [x] Three-panel layout: left sidebar (scene tree), center 3D viewport, right properties panel
- [ ] Responsive sizing (panels resizable via drag handle)
- [x] Toolbar: New, Save, + Body, + Joint, Undo, Redo, JSON, CSV export buttons
- [x] Status bar: FPS counter, solver status, selection info

### 7.2 Scene Tree (Left Panel)
- [x] Hierarchical list of all bodies and joints
- [x] Click to select
- [ ] Double-click to rename
- [ ] Context menu: Add Body, Add Joint, Delete, Duplicate
- [ ] Visibility toggle (eye icon) per object

### 7.3 Properties Panel (Right Panel)
- [x] Show transform (position) for selected body; inline editing
- [x] Show joint parameters: type, axis limits, current value; inline editing
- [ ] Show constraint parameters: target position/orientation
- [ ] Drag-to-change on number inputs

### 7.4 Constraint Editor
- [ ] "Add Constraint" dialog: choose type, target body, target value
- [ ] List of active constraints with enable/disable toggle
- [ ] Delete constraint button
- [ ] Visual link in 3D view between constrained body and its target gizmo

### 7.5 Simulation Timeline
- [x] Timeline bar at the bottom of the viewport
- [x] Keyframe system: record joint values at specific time points
- [x] Play / Pause / Stop controls
- [x] Scrub handle to manually move through time
- [x] Loop toggle

### 7.6 Export
- [ ] Export current mesh as OBJ or GLTF
- [x] Export joint trajectory as CSV (`time, j0, j1, ..., jN`)
- [x] Export solved pose as JSON (body transforms at current time)
- [ ] Copy current camera view to clipboard as PNG

### 7.7 Keyboard Shortcuts
- [ ] `G` — translate selected gizmo
- [ ] `R` — rotate selected gizmo
- [ ] `S` — scale selected object
- [ ] `Del` — delete selected
- [x] `Space` — play/pause simulation
- [x] `F` — fit/frame all objects in camera
- [x] `Ctrl+Z` / `Ctrl+Y` — undo/redo

### 7.8 Undo / Redo
- [x] Command pattern: every user action is a reversible `Command` object
- [x] `UndoStack` with configurable depth (default 50)
- [x] Property edits, add operations, joint value drags are undoable

### 7.9 Performance & Profiling
- [ ] Profile WASM solver time with `console.time` / `wasm_timer`
- [ ] Profile render time with `GPUQuerySet` or Three.js stats
- [ ] Identify and fix the top-3 bottlenecks before release
- [ ] Target: 60 fps with a 10-body chain and active IK solve

### 7.10 Documentation & Release
- [ ] Update `README.md` with build instructions, screenshots, feature list
- [ ] Add `docs/architecture.md` describing crate layout and data flow
- [ ] Hosted demo (GitHub Pages or similar)
- [ ] `CHANGELOG.md` for version history

---

## Acceptance Criteria
- New user can build a 3-body chain, add a position constraint, and drag the target within 5 minutes without reading docs
- All user actions are undoable
- OBJ export opens correctly in Blender
- CSV trajectory export contains correct joint values for a recorded animation
