# Phase 4 — 3D Rendering

## Goal
Display geometry from WASM in an interactive 3D viewport in the browser.

---

## Tasks

### 4.1 Library Selection & Setup
- [x] Add Three.js (+ `@types/three`) to `frontend/package.json`
- [x] Create `Renderer` class wrapping Three.js (`src/renderer.ts`)
- [x] Initialize canvas, WebGL context, scene, and render loop (`requestAnimationFrame`)

### 4.2 Camera
- [x] Perspective camera with configurable FOV, near/far planes
- [x] Orbit controls via `OrbitControls` with damping: left-drag to rotate, right-drag to pan, scroll to zoom
- [x] `renderer.fitToScene()` — auto-frames camera to bounding box of all mesh objects
- [x] Keyboard shortcut `H` to reset camera to home position

### 4.3 Mesh Upload & Display
- [x] `KinematicsScene.addMesh(json, opts)` — parses WASM JSON, creates `THREE.Mesh`, returns `ObjectId`
- [x] `KinematicsScene.removeMesh(id)` disposes geometry+material; `clear()` removes all
- [x] `main.ts` calls WASM bridge (`generate_box`, `generate_sphere`, `generate_cylinder`) to populate demo scene
- [x] Box, sphere, and cylinder render in the browser from WASM geometry

### 4.4 Lighting
- [x] `AmbientLight` (0.4 intensity)
- [x] `DirectionalLight` (sun-style, 1.2 intensity, shadow map enabled)

### 4.5 Materials & Shading
- [x] Default `MeshPhongMaterial` with configurable `color` parameter
- [x] `wireframe` option on `addMesh`
- [x] `transparent` + `opacity` options on `addMesh` for ghost material
- [ ] Face-normal debug visualization (deferred to Phase 7 UI polish)

### 4.6 Selection & Highlighting
- [x] `Raycaster` on canvas click → finds nearest mesh intersection
- [x] Selected object gets emissive highlight (`0x333333`); deselect on empty click
- [x] `onSelect(cb)` callback emits selected `ObjectId` (or `null`) to app

### 4.7 Axes & Grid
- [x] `AxesHelper` (XYZ colored arrows, toggleable via `setAxesVisible`)
- [x] `GridHelper` ground plane (toggleable via `setGridVisible`)
- [ ] Per-object local axis display (deferred)

### 4.8 Performance
- [x] Frustum culling: automatic via Three.js `Frustum` + `Object3D.frustumCulled`
- [x] FPS counter overlay in dev mode (`showFps` option)
- [ ] Static mesh batching (deferred — premature optimisation for current scene size)

---

## Acceptance Criteria
- User can orbit, pan, and zoom freely
- Box and sphere from WASM render with correct normals and lighting
- Clicking an object highlights it and logs its ID to the console
- Runs at ≥ 60 fps with at least 50 simultaneous primitive meshes
