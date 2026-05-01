# Phase 2 — Core Math & Primitive Geometry

## Goal
Build a well-tested Rust library for 3D math and primitive geometry that the rest of the project depends on.

---

## Tasks

### 2.1 Math Primitives
- [x] Decide on math library: `nalgebra` chosen (SVD support needed for phase 6 IK solver)
- [x] Add chosen library to `crates/core/Cargo.toml`
- [x] Re-export or wrap: `Vec3`, `Vec4`, `Mat3`, `Mat4`, `Quat` (type aliases in `math.rs`)
- [x] Implement utility helpers: `lerp`, `clamp`, `deg_to_rad`, `rad_to_deg`

### 2.2 Primitive Geometry Types
- [x] `Point3` — alias for `nalgebra::Point3<f32>`
- [x] `Direction3` — alias for `nalgebra::Unit<Vec3>` (unit enforced by nalgebra)
- [x] `Ray` — origin + direction
- [x] `Segment` — two endpoints
- [x] `Line` — point + direction (infinite)
- [x] `Plane` — normal + distance from origin
- [x] `Sphere` — center + radius
- [x] `Aabb` — axis-aligned bounding box (min/max)
- [x] `Triangle` — three vertices

### 2.3 Geometry Calculations
- [x] Point–plane: signed distance, projection
- [x] Point–sphere: distance, inside test
- [x] Point–AABB: distance, inside test
- [x] Ray–plane: intersection (point + t)
- [x] Ray–sphere: intersection (0, 1, or 2 hits)
- [x] Ray–triangle: Möller–Trumbore intersection
- [x] Ray–AABB: slab method intersection
- [x] Segment–segment: closest point pair + distance
- [x] Line–line: closest point pair + distance
- [x] Plane–plane: intersection line

### 2.4 Transform Utilities
- [x] `Transform` struct: translation + rotation (quaternion) + scale
- [x] `Transform::apply(point)` and `Transform::apply_direction(dir)`
- [x] `Transform::inverse()` (exact for uniform scale)
- [x] `Transform::compose(other)` — chains two transforms

### 2.5 Unit Tests
- [x] One test module per geometry type (`#[cfg(test)]` in same file)
- [x] Test degenerate cases: parallel lines, ray tangent to sphere, zero-length segment
- [x] Property-based tests with `proptest` for distance non-negativity and round-trip transforms

---

## Acceptance Criteria
- `cargo test -p core` passes with zero failures ✓ (65/65)
- No `clippy` warnings at `deny(warnings)` level ✓
- All calculation functions have at least one degenerate-case test ✓
