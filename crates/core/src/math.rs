pub use nalgebra;

pub type Vec2 = nalgebra::Vector2<f32>;
pub type Vec3 = nalgebra::Vector3<f32>;
pub type Vec4 = nalgebra::Vector4<f32>;
pub type Mat3 = nalgebra::Matrix3<f32>;
pub type Mat4 = nalgebra::Matrix4<f32>;
pub type Quat = nalgebra::UnitQuaternion<f32>;

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn clamp(val: f32, min: f32, max: f32) -> f32 {
    val.clamp(min, max)
}

pub fn deg_to_rad(deg: f32) -> f32 {
    deg.to_radians()
}

pub fn rad_to_deg(rad: f32) -> f32 {
    rad.to_degrees()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn lerp_at_zero_returns_a() {
        assert_abs_diff_eq!(lerp(1.0, 3.0, 0.0), 1.0);
    }

    #[test]
    fn lerp_at_one_returns_b() {
        assert_abs_diff_eq!(lerp(1.0, 3.0, 1.0), 3.0);
    }

    #[test]
    fn lerp_midpoint() {
        assert_abs_diff_eq!(lerp(0.0, 4.0, 0.5), 2.0);
    }

    #[test]
    fn clamp_below_min() {
        assert_abs_diff_eq!(clamp(-1.0, 0.0, 1.0), 0.0);
    }

    #[test]
    fn clamp_above_max() {
        assert_abs_diff_eq!(clamp(2.0, 0.0, 1.0), 1.0);
    }

    #[test]
    fn clamp_in_range() {
        assert_abs_diff_eq!(clamp(0.5, 0.0, 1.0), 0.5);
    }

    #[test]
    fn deg_rad_roundtrip() {
        assert_abs_diff_eq!(rad_to_deg(deg_to_rad(90.0)), 90.0, epsilon = 1e-5);
    }

    #[test]
    fn deg_to_rad_known_value() {
        assert_abs_diff_eq!(deg_to_rad(180.0), std::f32::consts::PI, epsilon = 1e-6);
    }
}
