use crate::math::Vec3;
use crate::primitives::Aabb;

#[derive(Debug, Clone, PartialEq)]
pub enum MeshError {
    IndicesNotMultipleOfThree,
    IndexOutOfBounds { index: u32, max: usize },
    NormalCountMismatch,
    DegenerateTriangle { face: usize },
}

#[derive(Debug, Clone, Default)]
pub struct Mesh {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks index bounds, normal count, and degenerate triangles.
    pub fn validate(&self) -> Result<(), MeshError> {
        if self.indices.len() % 3 != 0 {
            return Err(MeshError::IndicesNotMultipleOfThree);
        }
        if self.normals.len() != self.positions.len() {
            return Err(MeshError::NormalCountMismatch);
        }
        let max = self.positions.len();
        for (face, tri) in self.indices.chunks(3).enumerate() {
            for &idx in tri {
                if idx as usize >= max {
                    return Err(MeshError::IndexOutOfBounds { index: idx, max });
                }
            }
            let a = Vec3::from(self.positions[tri[0] as usize]);
            let b = Vec3::from(self.positions[tri[1] as usize]);
            let c = Vec3::from(self.positions[tri[2] as usize]);
            if (b - a).cross(&(c - a)).norm_squared() < f32::EPSILON {
                return Err(MeshError::DegenerateTriangle { face });
            }
        }
        Ok(())
    }

    /// Recomputes flat (per-face) normals from triangle geometry.
    pub fn compute_normals_flat(&mut self) {
        self.normals.resize(self.positions.len(), [0.0; 3]);
        for tri in self.indices.chunks(3) {
            let a = Vec3::from(self.positions[tri[0] as usize]);
            let b = Vec3::from(self.positions[tri[1] as usize]);
            let c = Vec3::from(self.positions[tri[2] as usize]);
            let n = (b - a).cross(&(c - a));
            let n = if n.norm_squared() > f32::EPSILON {
                n.normalize()
            } else {
                Vec3::new(0.0, 1.0, 0.0)
            };
            let nv: [f32; 3] = n.into();
            self.normals[tri[0] as usize] = nv;
            self.normals[tri[1] as usize] = nv;
            self.normals[tri[2] as usize] = nv;
        }
    }

    /// Recomputes smooth normals by accumulating and normalising per-vertex face contributions.
    pub fn compute_normals_smooth(&mut self) {
        let mut accum = vec![Vec3::zeros(); self.positions.len()];
        for tri in self.indices.chunks(3) {
            let a = Vec3::from(self.positions[tri[0] as usize]);
            let b = Vec3::from(self.positions[tri[1] as usize]);
            let c = Vec3::from(self.positions[tri[2] as usize]);
            let face_n = (b - a).cross(&(c - a));
            accum[tri[0] as usize] += face_n;
            accum[tri[1] as usize] += face_n;
            accum[tri[2] as usize] += face_n;
        }
        self.normals = accum
            .into_iter()
            .map(|n| {
                if n.norm_squared() > f32::EPSILON {
                    n.normalize().into()
                } else {
                    [0.0, 1.0, 0.0]
                }
            })
            .collect();
    }

    /// Appends `other` into `self`, offsetting `other`'s indices by the current vertex count.
    pub fn merge(&mut self, other: &Mesh) {
        let offset = self.positions.len() as u32;
        self.positions.extend_from_slice(&other.positions);
        self.normals.extend_from_slice(&other.normals);
        self.indices
            .extend(other.indices.iter().map(|i| i + offset));
    }

    /// Returns the axis-aligned bounding box, or `None` if the mesh is empty.
    pub fn compute_aabb(&self) -> Option<Aabb> {
        if self.positions.is_empty() {
            return None;
        }
        let mut min = [f32::MAX; 3];
        let mut max = [f32::MIN; 3];
        for pos in &self.positions {
            for i in 0..3 {
                min[i] = min[i].min(pos[i]);
                max[i] = max[i].max(pos[i]);
            }
        }
        Some(Aabb::new(
            Vec3::new(min[0], min[1], min[2]),
            Vec3::new(max[0], max[1], max[2]),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh_gen;
    use approx::assert_abs_diff_eq;

    fn unit_triangle() -> Mesh {
        Mesh {
            positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            normals: vec![[0.0, 0.0, 1.0]; 3],
            indices: vec![0, 1, 2],
        }
    }

    // --- validate ---
    #[test]
    fn validate_empty_mesh_is_ok() {
        assert!(Mesh::new().validate().is_ok());
    }

    #[test]
    fn validate_valid_triangle_is_ok() {
        assert!(unit_triangle().validate().is_ok());
    }

    #[test]
    fn validate_detects_index_out_of_bounds() {
        let mut m = unit_triangle();
        m.indices[2] = 99;
        assert_eq!(
            m.validate(),
            Err(MeshError::IndexOutOfBounds { index: 99, max: 3 })
        );
    }

    #[test]
    fn validate_detects_non_multiple_of_three_indices() {
        let mut m = unit_triangle();
        m.indices.push(0);
        assert_eq!(m.validate(), Err(MeshError::IndicesNotMultipleOfThree));
    }

    #[test]
    fn validate_detects_normal_count_mismatch() {
        let mut m = unit_triangle();
        m.normals.pop();
        assert_eq!(m.validate(), Err(MeshError::NormalCountMismatch));
    }

    #[test]
    fn validate_detects_degenerate_triangle() {
        let m = Mesh {
            positions: vec![[0.0, 0.0, 0.0]; 3],
            normals: vec![[0.0, 0.0, 1.0]; 3],
            indices: vec![0, 1, 2],
        };
        assert_eq!(m.validate(), Err(MeshError::DegenerateTriangle { face: 0 }));
    }

    // --- compute_normals_flat ---
    #[test]
    fn flat_normals_are_unit_length() {
        let mut m = mesh_gen::box_(1.0, 1.0, 1.0);
        m.compute_normals_flat();
        for n in &m.normals {
            let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            assert_abs_diff_eq!(len, 1.0, epsilon = 1e-5);
        }
    }

    #[test]
    fn flat_normals_xy_triangle_point_along_z() {
        let mut m = unit_triangle();
        m.compute_normals_flat();
        for n in &m.normals {
            assert_abs_diff_eq!(n[2], 1.0, epsilon = 1e-5);
        }
    }

    // --- merge ---
    #[test]
    fn merge_combines_vertex_and_index_counts() {
        let a = mesh_gen::box_(1.0, 1.0, 1.0);
        let b = mesh_gen::box_(2.0, 2.0, 2.0);
        let a_verts = a.positions.len();
        let b_verts = b.positions.len();
        let a_idxs = a.indices.len();
        let b_idxs = b.indices.len();
        let mut combined = a;
        combined.merge(&b);
        assert_eq!(combined.positions.len(), a_verts + b_verts);
        assert_eq!(combined.indices.len(), a_idxs + b_idxs);
    }

    #[test]
    fn merge_offsets_indices_correctly() {
        let mut a = unit_triangle();
        let b = unit_triangle();
        a.merge(&b);
        assert_eq!(&a.indices[3..], &[3, 4, 5]);
        assert!(a.validate().is_ok());
    }

    // --- compute_aabb ---
    #[test]
    fn aabb_of_unit_box_is_half_extents() {
        let m = mesh_gen::box_(2.0, 4.0, 6.0);
        let aabb = m.compute_aabb().unwrap();
        assert_abs_diff_eq!(aabb.min, Vec3::new(-1.0, -2.0, -3.0), epsilon = 1e-5);
        assert_abs_diff_eq!(aabb.max, Vec3::new(1.0, 2.0, 3.0), epsilon = 1e-5);
    }

    #[test]
    fn aabb_of_empty_mesh_is_none() {
        assert!(Mesh::new().compute_aabb().is_none());
    }
}
