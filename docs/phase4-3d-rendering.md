# Phase 4 — 3D Rendering

## Goal
Display geometry from WASM in an interactive 3D viewport in the browser.

---

## Tasks

### 4.1 Library Selection & Setup
- [ ] Add Three.js (or raw WebGL) to `frontend/package.json`
- [ ] Create `Renderer` class wrapping the chosen library
- [ ] Initialize canvas, WebGL context, scene, and render loop (`requestAnimationFrame`)

### 4.2 Camera
- [ ] Perspective camera with configurable FOV, near/far planes
- [ ] Orbit controls: left-drag to rotate, right-drag to pan, scroll to zoom
- [ ] `camera.fitToScene()` — auto-frame all visible objects
- [ ] Keyboard shortcut to reset camera to home position

### 4.3 Mesh Upload & Display
- [ ] `Scene.addMesh(positions, normals, indices, material)` — creates a renderable object
- [ ] `Scene.removeMesh(id)` and `Scene.clear()`
- [ ] Call WASM bridge to get mesh arrays and feed them to `Scene.addMesh()`
- [ ] Verify a box and a sphere render correctly

### 4.4 Lighting
- [ ] Ambient light
- [ ] Directional light (sun-style) with configurable direction
- [ ] Optional: point lights attached to scene objects

### 4.5 Materials & Shading
- [ ] Default solid material (Phong or PBR) with color parameter
- [ ] Wireframe overlay toggle
- [ ] Transparent/ghost material for visualizing occluded geometry
- [ ] Face-normal debug visualization (color-coded normals)

### 4.6 Selection & Highlighting
- [ ] Ray-cast from mouse cursor into scene to pick objects
- [ ] Highlight selected object (outline or color tint)
- [ ] Emit `select` event to the rest of the app with selected object ID

### 4.7 Axes & Grid
- [ ] World-space axis gizmo (XYZ arrows in corner)
- [ ] Ground grid plane (toggleable)
- [ ] Per-object local axis display (toggleable)

### 4.8 Performance
- [ ] Frustum culling for large scenes
- [ ] Merge static meshes into a single draw call where possible
- [ ] FPS counter overlay in dev mode

---

## Acceptance Criteria
- User can orbit, pan, and zoom freely
- Box and sphere from WASM render with correct normals and lighting
- Clicking an object highlights it and logs its ID to the console
- Runs at ≥ 60 fps with at least 50 simultaneous primitive meshes
