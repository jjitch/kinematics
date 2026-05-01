use crate::math::Vec3;

pub type Point3 = nalgebra::Point3<f32>;
pub type Direction3 = nalgebra::Unit<Vec3>;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Direction3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Direction3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction.as_ref() * t
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Segment {
    pub start: Vec3,
    pub end: Vec3,
}

impl Segment {
    pub fn new(start: Vec3, end: Vec3) -> Self {
        Self { start, end }
    }

    pub fn length(&self) -> f32 {
        (self.end - self.start).norm()
    }

    pub fn direction(&self) -> Option<Direction3> {
        Direction3::try_new(self.end - self.start, f32::EPSILON)
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.start + (self.end - self.start) * t
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub point: Vec3,
    pub direction: Direction3,
}

impl Line {
    pub fn new(point: Vec3, direction: Direction3) -> Self {
        Self { point, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.point + self.direction.as_ref() * t
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub normal: Direction3,
    /// Signed distance from the origin along the normal: n · p = distance for points on the plane.
    pub distance: f32,
}

impl Plane {
    pub fn new(normal: Direction3, distance: f32) -> Self {
        Self { normal, distance }
    }

    pub fn from_point_normal(point: Vec3, normal: Direction3) -> Self {
        let distance = normal.dot(&point);
        Self { normal, distance }
    }

    pub fn signed_distance_to_point(&self, point: &Vec3) -> f32 {
        self.normal.dot(point) - self.distance
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn contains(&self, point: &Vec3) -> bool {
        (point - self.center).norm_squared() <= self.radius * self.radius
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, point: &Vec3) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z
    }

    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    pub fn half_extents(&self) -> Vec3 {
        (self.max - self.min) * 0.5
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self { a, b, c }
    }

    pub fn normal(&self) -> Option<Direction3> {
        Direction3::try_new((self.b - self.a).cross(&(self.c - self.a)), f32::EPSILON)
    }

    pub fn area(&self) -> f32 {
        (self.b - self.a).cross(&(self.c - self.a)).norm() * 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    fn dir(x: f32, y: f32, z: f32) -> Direction3 {
        Direction3::new_normalize(Vec3::new(x, y, z))
    }

    // --- Ray ---
    #[test]
    fn ray_at_zero_is_origin() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), dir(0.0, 0.0, 1.0));
        assert_abs_diff_eq!(r.at(0.0), Vec3::new(1.0, 2.0, 3.0), epsilon = 1e-6);
    }

    #[test]
    fn ray_at_advances_along_direction() {
        let r = Ray::new(Vec3::zeros(), dir(1.0, 0.0, 0.0));
        assert_abs_diff_eq!(r.at(5.0), Vec3::new(5.0, 0.0, 0.0), epsilon = 1e-6);
    }

    // --- Segment ---
    #[test]
    fn segment_length_345_triangle() {
        let s = Segment::new(Vec3::zeros(), Vec3::new(3.0, 4.0, 0.0));
        assert_abs_diff_eq!(s.length(), 5.0, epsilon = 1e-6);
    }

    #[test]
    fn segment_zero_length_has_no_direction() {
        let s = Segment::new(Vec3::zeros(), Vec3::zeros());
        assert!(s.direction().is_none());
    }

    #[test]
    fn segment_at_midpoint() {
        let s = Segment::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 0.0, 0.0));
        assert_abs_diff_eq!(s.at(0.5), Vec3::new(1.0, 0.0, 0.0), epsilon = 1e-6);
    }

    // --- Line ---
    #[test]
    fn line_at_advances() {
        let l = Line::new(Vec3::zeros(), dir(0.0, 1.0, 0.0));
        assert_abs_diff_eq!(l.at(3.0), Vec3::new(0.0, 3.0, 0.0), epsilon = 1e-6);
    }

    // --- Plane ---
    #[test]
    fn plane_from_point_normal_point_is_on_plane() {
        let p = Vec3::new(0.0, 2.0, 0.0);
        let plane = Plane::from_point_normal(p, dir(0.0, 1.0, 0.0));
        assert_abs_diff_eq!(plane.signed_distance_to_point(&p), 0.0, epsilon = 1e-6);
    }

    #[test]
    fn plane_signed_distance_above_is_positive() {
        let plane = Plane::from_point_normal(Vec3::zeros(), dir(0.0, 1.0, 0.0));
        assert!(plane.signed_distance_to_point(&Vec3::new(0.0, 1.0, 0.0)) > 0.0);
    }

    #[test]
    fn plane_signed_distance_below_is_negative() {
        let plane = Plane::from_point_normal(Vec3::zeros(), dir(0.0, 1.0, 0.0));
        assert!(plane.signed_distance_to_point(&Vec3::new(0.0, -1.0, 0.0)) < 0.0);
    }

    // --- Sphere ---
    #[test]
    fn sphere_contains_center() {
        let s = Sphere::new(Vec3::new(1.0, 2.0, 3.0), 5.0);
        assert!(s.contains(&Vec3::new(1.0, 2.0, 3.0)));
    }

    #[test]
    fn sphere_does_not_contain_outside() {
        let s = Sphere::new(Vec3::zeros(), 1.0);
        assert!(!s.contains(&Vec3::new(2.0, 0.0, 0.0)));
    }

    #[test]
    fn sphere_contains_surface_point() {
        let s = Sphere::new(Vec3::zeros(), 1.0);
        assert!(s.contains(&Vec3::new(1.0, 0.0, 0.0)));
    }

    // --- Aabb ---
    #[test]
    fn aabb_contains_center() {
        let b = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        assert!(b.contains(&Vec3::zeros()));
    }

    #[test]
    fn aabb_does_not_contain_outside() {
        let b = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        assert!(!b.contains(&Vec3::new(2.0, 0.0, 0.0)));
    }

    #[test]
    fn aabb_center() {
        let b = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 4.0, 6.0));
        assert_abs_diff_eq!(b.center(), Vec3::new(1.0, 2.0, 3.0), epsilon = 1e-6);
    }

    // --- Triangle ---
    #[test]
    fn triangle_area_right_triangle() {
        let t = Triangle::new(
            Vec3::zeros(),
            Vec3::new(3.0, 0.0, 0.0),
            Vec3::new(0.0, 4.0, 0.0),
        );
        assert_abs_diff_eq!(t.area(), 6.0, epsilon = 1e-6);
    }

    #[test]
    fn degenerate_triangle_has_no_normal() {
        let t = Triangle::new(Vec3::zeros(), Vec3::zeros(), Vec3::zeros());
        assert!(t.normal().is_none());
    }

    #[test]
    fn triangle_normal_points_up_for_xy_triangle() {
        let t = Triangle::new(
            Vec3::zeros(),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let n = t.normal().unwrap();
        assert_abs_diff_eq!(n.into_inner(), Vec3::new(0.0, 0.0, 1.0), epsilon = 1e-6);
    }
}
