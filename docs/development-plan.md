# Development Plan

## Overview

This project builds a 3D kinematics analysis tool.
The backend is implemented in Rust compiled to WebAssembly, and the frontend provides a 3D view using web technologies.

---

## Feature Areas

The following feature areas are derived from the README:

1. Primitive geometry calculation
2. 3D rendering
3. Primitive geometry modeling
4. Defining kinematics on geometry
5. Solving kinematics under given constraints

---

## Phases

### Phase 1 — Project Setup

- Initialize Rust workspace with `wasm-pack`
- Set up web frontend (bundler, TypeScript)
- Establish WASM ↔ JS bridge
- CI pipeline (build + test)

Deliverable: "Hello from WASM" rendered in the browser.

---

### Phase 2 — Core Math & Primitive Geometry

- 3D vector and matrix types (Vec3, Mat4, Quaternion)
- Primitive geometry types: Point, Line, Ray, Plane, Sphere, Box
- Geometry calculations:
  - Distance between primitives
  - Intersection tests
  - Projection and closest-point queries
- Unit tests for all calculations

Deliverable: Rust library passing geometry calculation tests.

---

### Phase 3 — Primitive Geometry Modeling

- Mesh representation (vertices, edges, faces)
- Constructive operations: extrude, revolve, boolean (union/difference)
- Serialization format for geometry (JSON or binary)
- Export mesh data to the WASM/JS bridge

Deliverable: Geometry models constructable from Rust and transferable to the browser.

---

### Phase 4 — 3D Rendering

- Integrate a WebGL library (e.g., Three.js or raw WebGL)
- Render meshes received from WASM
- Camera controls (orbit, pan, zoom)
- Basic lighting and material shading
- Highlight/selection of geometry objects

Deliverable: Interactive 3D viewport displaying geometry created in Phase 3.

---

### Phase 5 — Kinematics Definition

- Kinematic chain data model (bodies, joints)
- Joint types:
  - Revolute (rotation about an axis)
  - Prismatic (translation along an axis)
  - Fixed
- Attach kinematic structure to geometry objects
- Visualize joints and axes in the 3D view

Deliverable: User can define a kinematic chain and see it rendered.

---

### Phase 6 — Kinematics Solver

- Forward kinematics (FK): compute end-effector pose from joint values
- Inverse kinematics (IK): solve joint values from target pose
- Constraint types:
  - Position constraint
  - Orientation constraint
  - Joint limit constraint
- Iterative solver (e.g., Jacobian transpose / pseudo-inverse)
- Real-time update of 3D view as constraints are solved

Deliverable: User can drag an end-effector and the chain solves to follow it.

---

### Phase 7 — UI & Polish

- Sidebar panel for scene/object list
- Constraint editor (add, remove, edit constraints)
- Playback / simulation timeline
- Export solved poses
- Performance profiling and optimization

Deliverable: Complete interactive kinematics analysis tool.

---

## Technology Stack

| Layer      | Technology                    |
|------------|-------------------------------|
| Backend    | Rust + WebAssembly (wasm-pack) |
| Math/Solver| Rust (nalgebra or glam)       |
| Frontend   | TypeScript + Vite (or similar) |
| 3D Render  | Three.js (or raw WebGL)       |
| Bridge     | wasm-bindgen                  |

---

## Milestones Summary

| Phase | Milestone                        |
|-------|----------------------------------|
| 1     | WASM running in browser          |
| 2     | Geometry calculations tested     |
| 3     | Geometry models transferable     |
| 4     | 3D viewport interactive          |
| 5     | Kinematic chains visualized      |
| 6     | IK solver working in real-time   |
| 7     | Polished, exportable tool        |
