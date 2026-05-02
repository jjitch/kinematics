use crate::mesh::Mesh;

/// Axis-aligned box centred at the origin. 24 vertices (4 per face), 36 indices.
pub fn box_(width: f32, height: f32, depth: f32) -> Mesh {
    let (hw, hh, hd) = (width * 0.5, height * 0.5, depth * 0.5);
    let mut mesh = Mesh::new();

    let faces: [([[f32; 3]; 4], [f32; 3]); 6] = [
        // +X
        (
            [[hw, hh, -hd], [hw, hh, hd], [hw, -hh, hd], [hw, -hh, -hd]],
            [1.0, 0.0, 0.0],
        ),
        // -X
        (
            [
                [-hw, -hh, -hd],
                [-hw, -hh, hd],
                [-hw, hh, hd],
                [-hw, hh, -hd],
            ],
            [-1.0, 0.0, 0.0],
        ),
        // +Y
        (
            [[-hw, hh, -hd], [-hw, hh, hd], [hw, hh, hd], [hw, hh, -hd]],
            [0.0, 1.0, 0.0],
        ),
        // -Y
        (
            [
                [-hw, -hh, hd],
                [-hw, -hh, -hd],
                [hw, -hh, -hd],
                [hw, -hh, hd],
            ],
            [0.0, -1.0, 0.0],
        ),
        // +Z
        (
            [[-hw, -hh, hd], [hw, -hh, hd], [hw, hh, hd], [-hw, hh, hd]],
            [0.0, 0.0, 1.0],
        ),
        // -Z
        (
            [
                [hw, -hh, -hd],
                [-hw, -hh, -hd],
                [-hw, hh, -hd],
                [hw, hh, -hd],
            ],
            [0.0, 0.0, -1.0],
        ),
    ];

    for (positions, normal) in &faces {
        let base = mesh.positions.len() as u32;
        mesh.positions.extend_from_slice(positions);
        mesh.normals.extend_from_slice(&[*normal; 4]);
        mesh.indices
            .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }
    mesh
}

/// UV sphere centred at the origin.
/// Vertex count: `(rings+1) × (segments+1)`.  Index count: `rings × segments × 6`.
pub fn sphere(radius: f32, segments: u32, rings: u32) -> Mesh {
    use std::f32::consts::PI;
    let mut mesh = Mesh::new();

    for ring in 0..=rings {
        // Offset by 0.5 / (rings+1) so no ring lands exactly on a pole,
        // avoiding the degenerate collapsed-point triangles that fail validate().
        let lat = PI * (ring as f32 + 0.5) / (rings as f32 + 1.0);
        let sin_lat = lat.sin();
        let cos_lat = lat.cos();
        for seg in 0..=segments {
            let lon = 2.0 * PI * seg as f32 / segments as f32;
            let nx = sin_lat * lon.cos();
            let ny = cos_lat;
            let nz = sin_lat * lon.sin();
            mesh.positions.push([radius * nx, radius * ny, radius * nz]);
            mesh.normals.push([nx, ny, nz]);
        }
    }

    let row = segments + 1;
    for ring in 0..rings {
        for seg in 0..segments {
            let a = ring * row + seg;
            let b = ring * row + seg + 1;
            let c = (ring + 1) * row + seg;
            let d = (ring + 1) * row + seg + 1;
            mesh.indices.extend_from_slice(&[a, d, b, a, c, d]);
        }
    }
    mesh
}

/// Cylinder centred at the origin, aligned along Y, with flat end-caps.
pub fn cylinder(radius: f32, height: f32, segments: u32) -> Mesh {
    use std::f32::consts::TAU;
    let mut mesh = Mesh::new();
    let hh = height * 0.5;

    // Side
    for seg in 0..=segments {
        let angle = TAU * seg as f32 / segments as f32;
        let (cos_a, sin_a) = (angle.cos(), angle.sin());
        let nx = cos_a;
        let nz = sin_a;
        mesh.positions.push([radius * cos_a, -hh, radius * sin_a]);
        mesh.normals.push([nx, 0.0, nz]);
        mesh.positions.push([radius * cos_a, hh, radius * sin_a]);
        mesh.normals.push([nx, 0.0, nz]);
    }
    for seg in 0..segments {
        let b = seg * 2;
        let t = seg * 2 + 1;
        let nb = b + 2;
        let nt = t + 2;
        mesh.indices.extend_from_slice(&[b, nb, nt, b, nt, t]);
    }

    // Top cap (+Y)
    let cap_start = mesh.positions.len() as u32;
    mesh.positions.push([0.0, hh, 0.0]);
    mesh.normals.push([0.0, 1.0, 0.0]);
    for seg in 0..=segments {
        let angle = TAU * seg as f32 / segments as f32;
        mesh.positions
            .push([radius * angle.cos(), hh, radius * angle.sin()]);
        mesh.normals.push([0.0, 1.0, 0.0]);
    }
    let center = cap_start;
    for seg in 0..segments {
        let a = cap_start + 1 + seg;
        let b = cap_start + 1 + seg + 1;
        mesh.indices.extend_from_slice(&[center, b, a]);
    }

    // Bottom cap (-Y)
    let cap_start = mesh.positions.len() as u32;
    mesh.positions.push([0.0, -hh, 0.0]);
    mesh.normals.push([0.0, -1.0, 0.0]);
    for seg in 0..=segments {
        let angle = TAU * seg as f32 / segments as f32;
        mesh.positions
            .push([radius * angle.cos(), -hh, radius * angle.sin()]);
        mesh.normals.push([0.0, -1.0, 0.0]);
    }
    let center = cap_start;
    for seg in 0..segments {
        let a = cap_start + 1 + seg;
        let b = cap_start + 1 + seg + 1;
        mesh.indices.extend_from_slice(&[center, a, b]);
    }

    mesh
}

/// Cone with apex at `+Y` and base circle at `-Y`, centred at the origin.
pub fn cone(radius: f32, height: f32, segments: u32) -> Mesh {
    use std::f32::consts::TAU;
    let mut mesh = Mesh::new();
    let hh = height * 0.5;

    // Side faces — each is a triangle: apex + two base edge points
    let slant = (radius * radius + height * height).sqrt();
    let ny_side = radius / slant;
    let nr_side = height / slant;

    let apex: [f32; 3] = [0.0, hh, 0.0];
    for seg in 0..segments {
        let a0 = TAU * seg as f32 / segments as f32;
        let a1 = TAU * (seg + 1) as f32 / segments as f32;
        let (c0, s0) = (a0.cos(), a0.sin());
        let (c1, s1) = (a1.cos(), a1.sin());

        let base = mesh.positions.len() as u32;
        mesh.positions.push(apex);
        let anx = nr_side * (c0 + c1) * 0.5;
        let anz = nr_side * (s0 + s1) * 0.5;
        let alen = (anx * anx + ny_side * ny_side + anz * anz).sqrt();
        mesh.normals.push(if alen > f32::EPSILON {
            [anx / alen, ny_side / alen, anz / alen]
        } else {
            [0.0, 1.0, 0.0]
        });
        mesh.positions.push([radius * c0, -hh, radius * s0]);
        mesh.normals.push([nr_side * c0, ny_side, nr_side * s0]);
        mesh.positions.push([radius * c1, -hh, radius * s1]);
        mesh.normals.push([nr_side * c1, ny_side, nr_side * s1]);
        mesh.indices.extend_from_slice(&[base, base + 1, base + 2]);
    }

    // Base cap (-Y)
    let cap_start = mesh.positions.len() as u32;
    mesh.positions.push([0.0, -hh, 0.0]);
    mesh.normals.push([0.0, -1.0, 0.0]);
    for seg in 0..=segments {
        let angle = TAU * seg as f32 / segments as f32;
        mesh.positions
            .push([radius * angle.cos(), -hh, radius * angle.sin()]);
        mesh.normals.push([0.0, -1.0, 0.0]);
    }
    let center = cap_start;
    for seg in 0..segments {
        let a = cap_start + 1 + seg;
        let b = cap_start + 1 + seg + 1;
        mesh.indices.extend_from_slice(&[center, a, b]);
    }

    mesh
}

/// Flat XZ-plane centred at the origin. All normals point up (+Y).
/// `subdivisions` quads per axis: vertex count `(s+1)²`, index count `s²·6`.
pub fn plane(width: f32, depth: f32, subdivisions: u32) -> Mesh {
    let mut mesh = Mesh::new();
    let s = subdivisions as usize;
    let hw = width * 0.5;
    let hd = depth * 0.5;

    for row in 0..=s {
        for col in 0..=s {
            let x = -hw + width * col as f32 / s as f32;
            let z = -hd + depth * row as f32 / s as f32;
            mesh.positions.push([x, 0.0, z]);
            mesh.normals.push([0.0, 1.0, 0.0]);
        }
    }

    let cols = (s + 1) as u32;
    for row in 0..s as u32 {
        for col in 0..s as u32 {
            let a = row * cols + col;
            let b = row * cols + col + 1;
            let c = (row + 1) * cols + col;
            let d = (row + 1) * cols + col + 1;
            mesh.indices.extend_from_slice(&[a, c, d, a, d, b]);
        }
    }
    mesh
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    fn normals_are_unit(mesh: &Mesh) {
        for n in &mesh.normals {
            let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            assert_abs_diff_eq!(len, 1.0, epsilon = 1e-5);
        }
    }

    // --- box_ ---
    #[test]
    fn box_has_24_vertices_and_36_indices() {
        let m = box_(1.0, 1.0, 1.0);
        assert_eq!(m.positions.len(), 24);
        assert_eq!(m.indices.len(), 36);
    }

    #[test]
    fn box_is_valid() {
        assert!(box_(2.0, 3.0, 4.0).validate().is_ok());
    }

    #[test]
    fn box_normals_are_unit() {
        normals_are_unit(&box_(1.0, 1.0, 1.0));
    }

    #[test]
    fn box_normals_are_axis_aligned() {
        let m = box_(1.0, 1.0, 1.0);
        for n in &m.normals {
            let rounded: Vec<i32> = n.iter().map(|v| v.round() as i32).collect();
            let sum: i32 = rounded.iter().map(|v| v.abs()).sum();
            assert_eq!(sum, 1, "normal {n:?} is not axis-aligned");
        }
    }

    // --- sphere ---
    #[test]
    fn sphere_is_valid() {
        assert!(sphere(1.0, 8, 4).validate().is_ok());
    }

    #[test]
    fn sphere_vertex_count() {
        let m = sphere(1.0, 8, 4);
        assert_eq!(m.positions.len(), 5 * 9);
    }

    #[test]
    fn sphere_index_count() {
        let m = sphere(1.0, 8, 4);
        assert_eq!(m.indices.len(), 4 * 8 * 6);
    }

    #[test]
    fn sphere_normals_are_unit() {
        normals_are_unit(&sphere(1.0, 16, 8));
    }

    #[test]
    fn sphere_normals_point_outward() {
        let m = sphere(1.0, 8, 4);
        for (pos, norm) in m.positions.iter().zip(m.normals.iter()) {
            let dot = pos[0] * norm[0] + pos[1] * norm[1] + pos[2] * norm[2];
            assert!(dot > 0.99, "normal not outward: pos={pos:?} norm={norm:?}");
        }
    }

    // --- cylinder ---
    #[test]
    fn cylinder_is_valid() {
        assert!(cylinder(1.0, 2.0, 8).validate().is_ok());
    }

    #[test]
    fn cylinder_indices_multiple_of_three() {
        assert_eq!(cylinder(1.0, 2.0, 8).indices.len() % 3, 0);
    }

    #[test]
    fn cylinder_normals_are_unit() {
        normals_are_unit(&cylinder(1.0, 2.0, 8));
    }

    // --- cone ---
    #[test]
    fn cone_is_valid() {
        assert!(cone(1.0, 2.0, 8).validate().is_ok());
    }

    #[test]
    fn cone_indices_multiple_of_three() {
        assert_eq!(cone(1.0, 2.0, 8).indices.len() % 3, 0);
    }

    #[test]
    fn cone_normals_are_unit() {
        normals_are_unit(&cone(1.0, 2.0, 8));
    }

    // --- plane ---
    #[test]
    fn plane_single_subdivision_has_4_vertices_6_indices() {
        let m = plane(1.0, 1.0, 1);
        assert_eq!(m.positions.len(), 4);
        assert_eq!(m.indices.len(), 6);
    }

    #[test]
    fn plane_is_valid() {
        assert!(plane(2.0, 3.0, 4).validate().is_ok());
    }

    #[test]
    fn plane_normals_point_up() {
        let m = plane(1.0, 1.0, 2);
        for n in &m.normals {
            assert_abs_diff_eq!(n[1], 1.0, epsilon = 1e-5);
        }
    }

    #[test]
    fn plane_vertex_count() {
        let s = 3u32;
        let m = plane(1.0, 1.0, s);
        assert_eq!(m.positions.len() as u32, (s + 1) * (s + 1));
    }
}
