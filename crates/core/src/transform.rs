use crate::math::{Mat4, Quat, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            translation: Vec3::zeros(),
            rotation: Quat::identity(),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self { translation, rotation, scale }
    }

    /// Apply the transform to a point: T + R * (S ⊙ p)
    pub fn apply(&self, point: Vec3) -> Vec3 {
        todo!()
    }

    /// Apply only the rotation to a direction (ignores translation and scale).
    pub fn apply_direction(&self, dir: Vec3) -> Vec3 {
        todo!()
    }

    /// Invert the transform. Exact for uniform scale; approximate for non-uniform scale.
    pub fn inverse(&self) -> Self {
        todo!()
    }

    /// Compose: `self.compose(other).apply(p) == self.apply(other.apply(p))`
    /// Exact for uniform scale; approximate for non-uniform scale.
    pub fn compose(&self, other: &Self) -> Self {
        todo!()
    }

    /// Build the equivalent 4×4 TRS matrix.
    pub fn to_matrix(&self) -> Mat4 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use proptest::prelude::*;

    fn rotation_90_around_y() -> Quat {
        Quat::from_axis_angle(&nalgebra::Unit::new_normalize(Vec3::new(0.0, 1.0, 0.0)), std::f32::consts::FRAC_PI_2)
    }

    // --- apply ---
    #[test]
    fn identity_apply_is_noop() {
        let t = Transform::identity();
        let p = Vec3::new(1.0, 2.0, 3.0);
        assert_abs_diff_eq!(t.apply(p), p, epsilon = 1e-6);
    }

    #[test]
    fn translation_only_shifts_point() {
        let t = Transform::new(Vec3::new(1.0, 2.0, 3.0), Quat::identity(), Vec3::new(1.0, 1.0, 1.0));
        assert_abs_diff_eq!(t.apply(Vec3::zeros()), Vec3::new(1.0, 2.0, 3.0), epsilon = 1e-6);
    }

    #[test]
    fn rotation_90_around_y_maps_x_to_neg_z() {
        let t = Transform::new(Vec3::zeros(), rotation_90_around_y(), Vec3::new(1.0, 1.0, 1.0));
        let result = t.apply(Vec3::new(1.0, 0.0, 0.0));
        assert_abs_diff_eq!(result, Vec3::new(0.0, 0.0, -1.0), epsilon = 1e-5);
    }

    #[test]
    fn uniform_scale_doubles_distance() {
        let t = Transform::new(Vec3::zeros(), Quat::identity(), Vec3::new(2.0, 2.0, 2.0));
        assert_abs_diff_eq!(t.apply(Vec3::new(1.0, 0.0, 0.0)), Vec3::new(2.0, 0.0, 0.0), epsilon = 1e-6);
    }

    // --- inverse ---
    #[test]
    fn inverse_roundtrip_translation_only() {
        let t = Transform::new(Vec3::new(3.0, -1.0, 2.0), Quat::identity(), Vec3::new(1.0, 1.0, 1.0));
        let p = Vec3::new(5.0, 5.0, 5.0);
        assert_abs_diff_eq!(t.inverse().apply(t.apply(p)), p, epsilon = 1e-5);
    }

    #[test]
    fn inverse_roundtrip_rotation_only() {
        let t = Transform::new(Vec3::zeros(), rotation_90_around_y(), Vec3::new(1.0, 1.0, 1.0));
        let p = Vec3::new(1.0, 2.0, 3.0);
        assert_abs_diff_eq!(t.inverse().apply(t.apply(p)), p, epsilon = 1e-5);
    }

    #[test]
    fn inverse_roundtrip_uniform_scale() {
        let t = Transform::new(Vec3::new(1.0, 0.0, 0.0), rotation_90_around_y(), Vec3::new(2.0, 2.0, 2.0));
        let p = Vec3::new(1.0, 1.0, 1.0);
        assert_abs_diff_eq!(t.inverse().apply(t.apply(p)), p, epsilon = 1e-4);
    }

    // --- compose ---
    #[test]
    fn compose_two_translations() {
        let a = Transform::new(Vec3::new(1.0, 0.0, 0.0), Quat::identity(), Vec3::new(1.0, 1.0, 1.0));
        let b = Transform::new(Vec3::new(0.0, 2.0, 0.0), Quat::identity(), Vec3::new(1.0, 1.0, 1.0));
        let c = a.compose(&b);
        assert_abs_diff_eq!(c.apply(Vec3::zeros()), Vec3::new(1.0, 2.0, 0.0), epsilon = 1e-5);
    }

    #[test]
    fn compose_is_equivalent_to_chained_apply() {
        let a = Transform::new(Vec3::new(1.0, 0.0, 0.0), rotation_90_around_y(), Vec3::new(1.0, 1.0, 1.0));
        let b = Transform::new(Vec3::new(0.0, 0.0, 1.0), Quat::identity(), Vec3::new(1.0, 1.0, 1.0));
        let p = Vec3::new(1.0, 0.0, 0.0);
        assert_abs_diff_eq!(a.compose(&b).apply(p), a.apply(b.apply(p)), epsilon = 1e-5);
    }

    // --- to_matrix ---
    #[test]
    fn to_matrix_identity_is_identity_matrix() {
        let m = Transform::identity().to_matrix();
        assert_abs_diff_eq!(m, Mat4::identity(), epsilon = 1e-6);
    }

    #[test]
    fn to_matrix_apply_matches_struct_apply() {
        let t = Transform::new(Vec3::new(1.0, 2.0, 3.0), rotation_90_around_y(), Vec3::new(1.0, 1.0, 1.0));
        let p = Vec3::new(1.0, 0.0, 0.0);
        let via_struct = t.apply(p);
        let ph = nalgebra::Vector4::new(p.x, p.y, p.z, 1.0);
        let via_matrix = t.to_matrix() * ph;
        assert_abs_diff_eq!(via_matrix.xyz(), via_struct, epsilon = 1e-5);
    }

    // --- proptest: round-trip ---
    proptest! {
        #[test]
        fn inverse_roundtrip_translation_proptest(
            tx in -100.0f32..100.0, ty in -100.0f32..100.0, tz in -100.0f32..100.0,
            px in -10.0f32..10.0,   py in -10.0f32..10.0,   pz in -10.0f32..10.0,
        ) {
            let t = Transform::new(
                Vec3::new(tx, ty, tz),
                Quat::identity(),
                Vec3::new(1.0, 1.0, 1.0),
            );
            let p = Vec3::new(px, py, pz);
            let roundtrip = t.inverse().apply(t.apply(p));
            prop_assert!((roundtrip - p).norm() < 1e-3);
        }
    }
}
