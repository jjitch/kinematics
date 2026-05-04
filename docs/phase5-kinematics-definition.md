# Phase 5 — Kinematics Definition

## Goal
Allow users to build kinematic chains (bodies connected by joints) and attach them to geometry, with the structure visible in the 3D view.

---

## Tasks

### 5.1 Core Data Model (Rust)
- [x] `Body` struct: id, name, local transform, attached mesh id (optional)
- [x] `JointType` enum: `Revolute { axis: Vec3 }`, `Prismatic { axis: Vec3 }`, `Fixed`
- [x] `Joint` struct: id, parent body id, child body id, joint type, rest transform
- [x] `Chain` struct: ordered list of bodies and joints (tree, not necessarily linear)
- [x] `Chain::add_body()`, `Chain::add_joint()`, `Chain::validate()`

### 5.2 Joint Parameters
- [x] `JointValue`: angle (rad) for revolute, displacement for prismatic (stored as `value: f32` on `Joint`)
- [x] Joint limits: `min: f32`, `max: f32` per joint
- [x] `Chain::set_joint_value(joint_id, value)` with clamping to limits
- [x] `Chain::joint_values() -> Vec<(JointId, f32)>`

### 5.3 Forward Kinematics (prerequisite for Phase 6)
- [x] `Chain::compute_transforms() -> HashMap<BodyId, Pose>` — BFS root→leaves, compose parent world × rest × active × child local
- [x] Expose via WASM: `chain_compute_fk(chain_json: &str) -> String`

### 5.4 WASM Bridge
- [x] `chain_new() -> String` — returns an empty chain as JSON
- [x] `chain_add_body(chain_json, name) -> String` — returns `{"ok":true,"chain":...,"id":N}`
- [x] `chain_add_joint(chain_json, parent_id, child_id, kind, ax, ay, az, min, max) -> String`
- [x] `chain_set_joint_value(chain_json, joint_id, value) -> String`
- [x] TypeScript types for `Chain`, `Body`, `Joint`, `Pose`, `FkResult` in `kinematics.ts`

### 5.5 3D Visualization
- [ ] Render joint axes as colored arrows (X=red, Y=green, Z=blue) — deferred to Phase 7
- [x] Render bones (line segments between parent and child body origins) via `ChainViz`
- [ ] Highlight the active/selected joint — deferred to Phase 7
- [x] Animate geometry to follow computed body transforms in real time
- [x] Joint value slider in UI (one per joint, clamped to limits)

### 5.6 Serialization
- [x] `Chain` serializes to/from JSON via `serde`
- [x] Save/load chain joint values from browser `localStorage`

### 5.7 Tests
- [x] Unit tests: `Chain::validate()` rejects cycles and orphan joints (`add_joint` enforces all invariants eagerly)
- [x] FK test: single revolute joint at 90° rotates child body correctly
- [x] FK test: chain of 3 joints, verify leaf world position

---

## Acceptance Criteria
- User can define a chain of ≥ 3 bodies with revolute joints via the UI ✓ (demo chain in main.ts)
- Moving joint sliders animates the geometry in the 3D view in real time ✓
- Chain state persists across page reloads via `localStorage` ✓
