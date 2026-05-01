# Phase 2 — Core Math & Primitive Geometry

## Goal
Build a well-tested Rust library for 3D math and primitive geometry that the rest of the project depends on.

---

## Tasks

### 2.1 Math Primitives
- [ ] Decide on math library: `nalgebra` (feature-rich) or `glam` (lightweight, SIMD)
- [ ] Add chosen library to `crates/core/Cargo.toml`
- [ ] Re-export or wrap: `Vec3`, `Vec4`, `Mat3`, `Mat4`, `Quaternion`
- [ ] Implement utility helpers: `lerp`, `clamp`, `deg_to_rad`, `rad_to_deg`

### 2.2 Primitive Geometry Types
- [ ] `Point3` — a position in 3D space
- [ ] `Direction3` — unit vector (enforced on construction)
- [ ] `Ray` — origin + direction
- [ ] `Segment` — two endpoints
- [ ] `Line` — point + direction (infinite)
- [ ] `Plane` — normal + distance from origin
- [ ] `Sphere` — center + radius
- [ ] `Aabb` — axis-aligned bounding box (min/max)
- [ ] `Triangle` — three vertices

### 2.3 Geometry Calculations
- [ ] Point–plane: signed distance, projection
- [ ] Point–sphere: distance, inside test
- [ ] Point–AABB: distance, inside test
- [ ] Ray–plane: intersection (point + t)
- [ ] Ray–sphere: intersection (0, 1, or 2 hits)
- [ ] Ray–triangle: Möller–Trumbore intersection
- [ ] Ray–AABB: slab method intersection
- [ ] Segment–segment: closest point pair + distance
- [ ] Line–line: closest point pair + distance
- [ ] Plane–plane: intersection line

### 2.4 Transform Utilities
- [ ] `Transform` struct: translation + rotation (quaternion) + scale
- [ ] `Transform::apply(point)` and `Transform::apply_direction(dir)`
- [ ] `Transform::inverse()`
- [ ] Compose two transforms

### 2.5 Unit Tests
- [ ] One test module per geometry type (`#[cfg(test)]` in same file)
- [ ] Test degenerate cases: parallel lines, ray tangent to sphere, zero-length segment
- [ ] Property-based tests with `proptest` for distance symmetry and round-trip transforms

---

## Acceptance Criteria
- `cargo test -p core` passes with zero failures
- No `clippy` warnings at `deny(warnings)` level
- All calculation functions have at least one degenerate-case test
