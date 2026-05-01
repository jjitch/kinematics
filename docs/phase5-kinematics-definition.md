# Phase 5 — Kinematics Definition

## Goal
Allow users to build kinematic chains (bodies connected by joints) and attach them to geometry, with the structure visible in the 3D view.

---

## Tasks

### 5.1 Core Data Model (Rust)
- [ ] `Body` struct: id, name, local transform, attached mesh id (optional)
- [ ] `JointType` enum: `Revolute { axis: Vec3 }`, `Prismatic { axis: Vec3 }`, `Fixed`
- [ ] `Joint` struct: id, parent body id, child body id, joint type, rest transform
- [ ] `Chain` struct: ordered list of bodies and joints (tree, not necessarily linear)
- [ ] `Chain::add_body()`, `Chain::add_joint()`, `Chain::validate()`

### 5.2 Joint Parameters
- [ ] `JointValue`: angle (rad) for revolute, displacement for prismatic
- [ ] Joint limits: `min: f32`, `max: f32` per joint
- [ ] `Chain::set_joint_value(joint_id, value)` with clamping to limits
- [ ] `Chain::joint_values() -> Vec<f32>`

### 5.3 Forward Kinematics (prerequisite for Phase 6)
- [ ] `Chain::compute_transforms() -> HashMap<BodyId, Transform>`
  - Traverse the tree root→leaves
  - Compose parent world transform × joint transform × child local transform
- [ ] Expose via WASM: `fk_compute(chain_json: &str) -> JsValue`

### 5.4 WASM Bridge
- [ ] `create_chain() -> JsValue` — returns an empty chain as JSON
- [ ] `add_body(chain_json, body_json) -> JsValue`
- [ ] `add_joint(chain_json, joint_json) -> JsValue`
- [ ] `set_joint_value(chain_json, joint_id, value) -> JsValue`
- [ ] TypeScript types for `Chain`, `Body`, `Joint`, `JointValue`

### 5.5 3D Visualization
- [ ] Render joint axes as colored arrows (X=red, Y=green, Z=blue)
- [ ] Render bones (line segments between parent and child body origins)
- [ ] Highlight the active/selected joint
- [ ] Animate geometry to follow computed body transforms in real time
- [ ] Joint value slider in UI (one per joint, clamped to limits)

### 5.6 Serialization
- [ ] `Chain` serializes to/from JSON via `serde`
- [ ] Save/load chain definition from browser `localStorage`

### 5.7 Tests
- [ ] Unit tests: `Chain::validate()` rejects cycles and orphan joints
- [ ] FK test: single revolute joint at 90° rotates child body correctly
- [ ] FK test: chain of 3 joints, verify leaf world position

---

## Acceptance Criteria
- User can define a chain of ≥ 3 bodies with revolute joints via the UI
- Moving joint sliders animates the geometry in the 3D view in real time
- Chain state persists across page reloads via `localStorage`
