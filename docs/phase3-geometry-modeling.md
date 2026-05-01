# Phase 3 — Primitive Geometry Modeling

## Goal
Provide constructive geometry tools in Rust and a transfer format so models can be sent to the browser for rendering.

---

## Tasks

### 3.1 Mesh Data Structure
- [ ] `Mesh` struct: `Vec<[f32;3]>` positions, `Vec<[f32;3]>` normals, `Vec<u32>` indices
- [ ] `Mesh::validate()` — checks index bounds, degenerate triangles
- [ ] `Mesh::compute_normals()` — flat and smooth variants
- [ ] `Mesh::merge(other: &Mesh)` — combine two meshes into one

### 3.2 Primitive Mesh Generators
- [ ] `mesh::box_(width, height, depth) -> Mesh`
- [ ] `mesh::sphere(radius, segments, rings) -> Mesh`
- [ ] `mesh::cylinder(radius, height, segments) -> Mesh`
- [ ] `mesh::cone(radius, height, segments) -> Mesh`
- [ ] `mesh::plane(width, depth, subdivisions) -> Mesh`

### 3.3 Constructive Operations
- [ ] `extrude(profile: &[Vec2], direction: Vec3) -> Mesh`
- [ ] `revolve(profile: &[Vec2], axis: Vec3, angle: f32, segments: u32) -> Mesh`
- [ ] Boolean union/difference (basic AABB-level; full BSP deferred to later)

### 3.4 Serialization
- [ ] Define `MeshData` as a plain struct with `Vec<f32>` and `Vec<u32>` fields
- [ ] Derive `serde::Serialize` / `Deserialize` for `MeshData`
- [ ] `Mesh::to_mesh_data()` and `Mesh::from_mesh_data()` conversions
- [ ] JSON serialization via `serde_json` (for debugging)
- [ ] Binary serialization via `bincode` (for performance)

### 3.5 WASM Bridge
- [ ] Expose `generate_box(w, h, d) -> JsValue` via `#[wasm_bindgen]` (returns JSON)
- [ ] Expose `generate_sphere(r, seg, rings) -> JsValue`
- [ ] Expose generic `mesh_to_typed_arrays(mesh_json: &str) -> JsValue` returning `{positions, normals, indices}`
- [ ] TypeScript type declarations for all exposed functions

### 3.6 Tests
- [ ] Unit tests for each mesh generator (vertex count, index count, no degenerate triangles)
- [ ] Round-trip test: `Mesh → MeshData → Mesh` preserves geometry
- [ ] WASM integration test: call bridge from Node.js/headless test runner

---

## Acceptance Criteria
- All primitive generators produce valid, renderable meshes
- `MeshData` round-trips through JSON and binary without loss
- Browser can receive mesh arrays from WASM and log vertex counts
