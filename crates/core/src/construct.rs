use crate::math::Vec3;
use crate::mesh::Mesh;

/// Extrudes a closed 2-D profile (XY plane) along `direction`.
/// Profile points are (x, y); z = 0 at the start, z = direction at the end.
/// Produces side quads + convex fan-triangulated caps.
pub fn extrude(profile: &[[f32; 2]], direction: Vec3) -> Mesh {
    let n = profile.len();
    let mut mesh = Mesh::new();
    if n < 2 {
        return mesh;
    }

    // Side quads
    let closed = n > 2;
    let edge_count = if closed { n } else { n - 1 };
    for i in 0..edge_count {
        let j = (i + 1) % n;
        let p0 = Vec3::new(profile[i][0], profile[i][1], 0.0);
        let p1 = Vec3::new(profile[j][0], profile[j][1], 0.0);
        let p2 = p1 + direction;
        let p3 = p0 + direction;

        let edge = p1 - p0;
        let side_n = edge.cross(&direction);
        let side_n = if side_n.norm_squared() > f32::EPSILON {
            side_n.normalize()
        } else {
            Vec3::new(0.0, 0.0, 1.0)
        };
        let nv: [f32; 3] = side_n.into();

        let base = mesh.positions.len() as u32;
        mesh.positions
            .extend_from_slice(&[p0.into(), p1.into(), p2.into(), p3.into()]);
        mesh.normals.extend_from_slice(&[nv; 4]);
        mesh.indices
            .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    if !closed {
        return mesh;
    }

    // Bottom cap (fan from profile[0], normal = -direction)
    let bot_n: [f32; 3] = if direction.norm_squared() > f32::EPSILON {
        (-direction.normalize()).into()
    } else {
        [0.0, 0.0, -1.0]
    };
    let cap_base = mesh.positions.len() as u32;
    for p in profile {
        mesh.positions.push([p[0], p[1], 0.0]);
        mesh.normals.push(bot_n);
    }
    for i in 1..n as u32 - 1 {
        mesh.indices
            .extend_from_slice(&[cap_base, cap_base + i + 1, cap_base + i]);
    }

    // Top cap (fan from profile[0] + direction, normal = +direction)
    let top_n: [f32; 3] = if direction.norm_squared() > f32::EPSILON {
        direction.normalize().into()
    } else {
        [0.0, 0.0, 1.0]
    };
    let cap_base = mesh.positions.len() as u32;
    for p in profile {
        let pos = Vec3::new(p[0], p[1], 0.0) + direction;
        mesh.positions.push(pos.into());
        mesh.normals.push(top_n);
    }
    for i in 1..n as u32 - 1 {
        mesh.indices
            .extend_from_slice(&[cap_base, cap_base + i, cap_base + i + 1]);
    }

    mesh
}

/// Revolves a 2-D profile around the Y axis through `angle` radians with `segments` steps.
/// Profile: `[radius, height]` pairs. Produces a smooth-shaded revolution surface.
pub fn revolve(profile: &[[f32; 2]], angle: f32, segments: u32) -> Mesh {
    let np = profile.len();
    let mut mesh = Mesh::new();
    if np < 2 || segments == 0 {
        return mesh;
    }

    for seg in 0..=segments {
        let theta = angle * seg as f32 / segments as f32;
        let (cos_t, sin_t) = (theta.cos(), theta.sin());
        for p in profile {
            let r = p[0];
            let y = p[1];
            mesh.positions.push([r * cos_t, y, r * sin_t]);
            mesh.normals.push([cos_t, 0.0, sin_t]); // placeholder; smoothed below
        }
    }

    // Build indices (outward winding: a, d, b + a, c, d)
    for seg in 0..segments {
        for ring in 0..np as u32 - 1 {
            let a = seg * np as u32 + ring;
            let b = seg * np as u32 + ring + 1;
            let c = (seg + 1) * np as u32 + ring;
            let d = (seg + 1) * np as u32 + ring + 1;
            mesh.indices.extend_from_slice(&[a, d, b, a, c, d]);
        }
    }

    mesh.compute_normals_smooth();
    mesh
}

/// Mesh union: merges both meshes into one (AABB-level; no true CSG).
pub fn boolean_union(a: &Mesh, b: &Mesh) -> Mesh {
    let mut result = a.clone();
    result.merge(b);
    result
}

/// Mesh difference: returns a clone of `a` (AABB-level placeholder; no true CSG).
pub fn boolean_difference(a: &Mesh, _b: &Mesh) -> Mesh {
    a.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh_gen;

    fn square_profile() -> Vec<[f32; 2]> {
        vec![[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]]
    }

    fn line_profile() -> Vec<[f32; 2]> {
        vec![[1.0, 0.0], [1.0, 1.0], [1.0, 2.0]]
    }

    // --- extrude ---
    #[test]
    fn extrude_square_profile_is_valid() {
        let m = extrude(&square_profile(), Vec3::new(0.0, 0.0, 2.0));
        assert!(m.validate().is_ok());
    }

    #[test]
    fn extrude_square_profile_has_indices_multiple_of_three() {
        let m = extrude(&square_profile(), Vec3::new(0.0, 0.0, 2.0));
        assert_eq!(m.indices.len() % 3, 0);
    }

    #[test]
    fn extrude_single_edge_produces_two_triangles() {
        let profile = vec![[0.0_f32, 0.0], [1.0, 0.0]];
        let m = extrude(&profile, Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(m.indices.len(), 6);
    }

    // --- revolve ---
    #[test]
    fn revolve_line_profile_is_valid() {
        let m = revolve(&line_profile(), std::f32::consts::TAU, 8);
        assert!(m.validate().is_ok());
    }

    #[test]
    fn revolve_line_profile_indices_multiple_of_three() {
        let m = revolve(&line_profile(), std::f32::consts::TAU, 8);
        assert_eq!(m.indices.len() % 3, 0);
    }

    #[test]
    fn revolve_vertex_count() {
        let profile = line_profile();
        let segments = 8u32;
        let m = revolve(&profile, std::f32::consts::TAU, segments);
        assert_eq!(
            m.positions.len() as u32,
            profile.len() as u32 * (segments + 1)
        );
    }

    // --- boolean_union ---
    #[test]
    fn boolean_union_has_combined_vertex_count() {
        let a = mesh_gen::box_(1.0, 1.0, 1.0);
        let b = mesh_gen::box_(1.0, 1.0, 1.0);
        let va = a.positions.len();
        let vb = b.positions.len();
        let u = boolean_union(&a, &b);
        assert_eq!(u.positions.len(), va + vb);
        assert!(u.validate().is_ok());
    }

    // --- boolean_difference ---
    #[test]
    fn boolean_difference_returns_a_unchanged() {
        let a = mesh_gen::box_(1.0, 1.0, 1.0);
        let b = mesh_gen::box_(0.5, 0.5, 0.5);
        let d = boolean_difference(&a, &b);
        assert_eq!(d.positions, a.positions);
        assert_eq!(d.indices, a.indices);
    }
}
