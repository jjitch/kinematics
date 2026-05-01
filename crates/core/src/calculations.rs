use crate::math::Vec3;
use crate::primitives::{Aabb, Direction3, Line, Plane, Ray, Segment, Sphere, Triangle};

/// Signed distance from `point` to `plane` (positive = normal side).
pub fn point_plane_signed_distance(point: &Vec3, plane: &Plane) -> f32 {
    plane.signed_distance_to_point(point)
}

/// Projection of `point` onto `plane`.
pub fn point_plane_projection(point: &Vec3, plane: &Plane) -> Vec3 {
    let d = plane.signed_distance_to_point(point);
    point - plane.normal.as_ref() * d
}

/// Distance from `point` to the sphere surface (0 if inside).
pub fn point_sphere_distance(point: &Vec3, sphere: &Sphere) -> f32 {
    ((point - sphere.center).norm() - sphere.radius).max(0.0)
}

/// Distance from `point` to the AABB surface (0 if inside).
pub fn point_aabb_distance(point: &Vec3, aabb: &Aabb) -> f32 {
    let dx = (aabb.min.x - point.x).max(0.0).max(point.x - aabb.max.x);
    let dy = (aabb.min.y - point.y).max(0.0).max(point.y - aabb.max.y);
    let dz = (aabb.min.z - point.z).max(0.0).max(point.z - aabb.max.z);
    Vec3::new(dx, dy, dz).norm()
}

/// Ray–plane intersection. Returns `t` along the ray, or `None` if parallel or behind origin.
pub fn ray_plane_intersect(ray: &Ray, plane: &Plane) -> Option<f32> {
    let denom = plane.normal.dot(ray.direction.as_ref());
    if denom.abs() < f32::EPSILON {
        return None;
    }
    let t = (plane.distance - plane.normal.dot(&ray.origin)) / denom;
    if t >= 0.0 {
        Some(t)
    } else {
        None
    }
}

/// Ray–sphere intersection. Returns `(t_near, t_far)` for positive hits, or `None`.
pub fn ray_sphere_intersect(ray: &Ray, sphere: &Sphere) -> Option<(f32, f32)> {
    let oc = ray.origin - sphere.center;
    let b = oc.dot(ray.direction.as_ref());
    let c = oc.norm_squared() - sphere.radius * sphere.radius;
    let discriminant = b * b - c;
    if discriminant < 0.0 {
        return None;
    }
    let sqrt_d = discriminant.sqrt();
    let t0 = -b - sqrt_d;
    let t1 = -b + sqrt_d;
    if t1 < 0.0 {
        None
    } else {
        Some((t0.max(0.0), t1))
    }
}

/// Ray–triangle intersection via Möller–Trumbore. Returns `t`, or `None`.
pub fn ray_triangle_intersect(ray: &Ray, tri: &Triangle) -> Option<f32> {
    let edge1 = tri.b - tri.a;
    let edge2 = tri.c - tri.a;
    let h = ray.direction.cross(&edge2);
    let det = edge1.dot(&h);
    if det.abs() < f32::EPSILON {
        return None;
    }
    let inv_det = 1.0 / det;
    let s = ray.origin - tri.a;
    let u = inv_det * s.dot(&h);
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = s.cross(&edge1);
    let v = inv_det * ray.direction.dot(&q);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let t = inv_det * edge2.dot(&q);
    if t > f32::EPSILON {
        Some(t)
    } else {
        None
    }
}

/// Ray–AABB intersection via the slab method. Returns `(t_enter, t_exit)`, or `None`.
pub fn ray_aabb_intersect(ray: &Ray, aabb: &Aabb) -> Option<(f32, f32)> {
    let dir = ray.direction.as_ref();
    let mut t_min = f32::NEG_INFINITY;
    let mut t_max = f32::INFINITY;
    for i in 0..3 {
        let inv_d = 1.0 / dir[i];
        let (t0, t1) = {
            let a = (aabb.min[i] - ray.origin[i]) * inv_d;
            let b = (aabb.max[i] - ray.origin[i]) * inv_d;
            if inv_d < 0.0 {
                (b, a)
            } else {
                (a, b)
            }
        };
        t_min = t_min.max(t0);
        t_max = t_max.min(t1);
    }
    if t_max < 0.0 || t_min > t_max {
        None
    } else {
        Some((t_min.max(0.0), t_max))
    }
}

/// Closest point pair and distance between two line segments.
/// Returns `(point_on_a, point_on_b, distance)`.
pub fn segment_segment_closest(a: &Segment, b: &Segment) -> (Vec3, Vec3, f32) {
    let d1 = a.end - a.start;
    let d2 = b.end - b.start;
    let r = a.start - b.start;
    let len1_sq = d1.norm_squared();
    let len2_sq = d2.norm_squared();
    let f = d2.dot(&r);

    let (s, t) = if len1_sq < f32::EPSILON && len2_sq < f32::EPSILON {
        (0.0_f32, 0.0_f32)
    } else if len1_sq < f32::EPSILON {
        (0.0, (f / len2_sq).clamp(0.0, 1.0))
    } else {
        let e = d1.dot(&r);
        if len2_sq < f32::EPSILON {
            ((-e / len1_sq).clamp(0.0, 1.0), 0.0)
        } else {
            let c = d1.dot(&d2);
            let denom = len1_sq * len2_sq - c * c;
            let s = if denom > f32::EPSILON {
                ((c * f - e * len2_sq) / denom).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let t_unclamped = (c * s + f) / len2_sq;
            if t_unclamped < 0.0 {
                ((-e / len1_sq).clamp(0.0, 1.0), 0.0)
            } else if t_unclamped > 1.0 {
                (((c - e) / len1_sq).clamp(0.0, 1.0), 1.0)
            } else {
                (s, t_unclamped)
            }
        }
    };

    let p1 = a.start + d1 * s;
    let p2 = b.start + d2 * t;
    (p1, p2, (p1 - p2).norm())
}

/// Closest point pair and distance between two infinite lines.
/// Returns `(point_on_a, point_on_b, distance)`.
pub fn line_line_closest(a: &Line, b: &Line) -> (Vec3, Vec3, f32) {
    let a_dir = a.direction.as_ref();
    let b_dir = b.direction.as_ref();
    let w = a.point - b.point;
    let dot = a_dir.dot(b_dir);
    let denom = 1.0 - dot * dot;
    let d = a_dir.dot(&w);
    let e = b_dir.dot(&w);

    let (s, t) = if denom < f32::EPSILON {
        // Parallel: fix s=0, project a.point onto b
        (0.0_f32, e)
    } else {
        let t = (e - dot * d) / denom;
        let s = dot * t - d;
        (s, t)
    };

    let pa = a.point + a_dir * s;
    let pb = b.point + b_dir * t;
    (pa, pb, (pa - pb).norm())
}

/// Intersection of two planes. Returns the intersection `Line`, or `None` if parallel.
pub fn plane_plane_intersect(a: &Plane, b: &Plane) -> Option<Line> {
    let na = a.normal.as_ref();
    let nb = b.normal.as_ref();
    let dir_vec = na.cross(nb);
    let dir = Direction3::try_new(dir_vec, f32::EPSILON)?;

    // Find a point on both planes: set the coordinate along the largest component of dir to 0
    // and solve the resulting 2×2 system.
    let abs = dir.abs();
    let point = if abs.x >= abs.y && abs.x >= abs.z {
        let mat = nalgebra::Matrix2::new(na.y, na.z, nb.y, nb.z);
        let rhs = nalgebra::Vector2::new(a.distance, b.distance);
        let sol = mat.try_inverse()? * rhs;
        Vec3::new(0.0, sol.x, sol.y)
    } else if abs.y >= abs.x && abs.y >= abs.z {
        let mat = nalgebra::Matrix2::new(na.x, na.z, nb.x, nb.z);
        let rhs = nalgebra::Vector2::new(a.distance, b.distance);
        let sol = mat.try_inverse()? * rhs;
        Vec3::new(sol.x, 0.0, sol.y)
    } else {
        let mat = nalgebra::Matrix2::new(na.x, na.y, nb.x, nb.y);
        let rhs = nalgebra::Vector2::new(a.distance, b.distance);
        let sol = mat.try_inverse()? * rhs;
        Vec3::new(sol.x, sol.y, 0.0)
    };

    Some(Line::new(point, dir))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use proptest::prelude::*;

    fn dir(x: f32, y: f32, z: f32) -> Direction3 {
        Direction3::new_normalize(Vec3::new(x, y, z))
    }

    fn y_plane(height: f32) -> Plane {
        Plane::from_point_normal(Vec3::new(0.0, height, 0.0), dir(0.0, 1.0, 0.0))
    }

    // --- point_plane ---
    #[test]
    fn point_plane_signed_distance_on_plane_is_zero() {
        let plane = y_plane(2.0);
        assert_abs_diff_eq!(
            point_plane_signed_distance(&Vec3::new(3.0, 2.0, -1.0), &plane),
            0.0,
            epsilon = 1e-6
        );
    }

    #[test]
    fn point_plane_signed_distance_above_positive() {
        let plane = y_plane(0.0);
        assert_abs_diff_eq!(
            point_plane_signed_distance(&Vec3::new(0.0, 3.0, 0.0), &plane),
            3.0,
            epsilon = 1e-6
        );
    }

    #[test]
    fn point_plane_projection_lies_on_plane() {
        let plane = y_plane(1.0);
        let proj = point_plane_projection(&Vec3::new(2.0, 5.0, -3.0), &plane);
        assert_abs_diff_eq!(plane.signed_distance_to_point(&proj), 0.0, epsilon = 1e-6);
    }

    // --- point_sphere ---
    #[test]
    fn point_sphere_distance_inside_is_zero() {
        let s = Sphere::new(Vec3::zeros(), 5.0);
        assert_abs_diff_eq!(point_sphere_distance(&Vec3::new(1.0, 0.0, 0.0), &s), 0.0);
    }

    #[test]
    fn point_sphere_distance_outside() {
        let s = Sphere::new(Vec3::zeros(), 1.0);
        assert_abs_diff_eq!(
            point_sphere_distance(&Vec3::new(4.0, 0.0, 0.0), &s),
            3.0,
            epsilon = 1e-6
        );
    }

    // --- point_aabb ---
    #[test]
    fn point_aabb_distance_inside_is_zero() {
        let b = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        assert_abs_diff_eq!(point_aabb_distance(&Vec3::zeros(), &b), 0.0);
    }

    #[test]
    fn point_aabb_distance_outside_one_axis() {
        let b = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        assert_abs_diff_eq!(
            point_aabb_distance(&Vec3::new(3.0, 0.0, 0.0), &b),
            2.0,
            epsilon = 1e-6
        );
    }

    // --- ray_plane ---
    #[test]
    fn ray_plane_hits_from_above() {
        let ray = Ray::new(Vec3::new(0.0, 5.0, 0.0), dir(0.0, -1.0, 0.0));
        let plane = y_plane(0.0);
        let t = ray_plane_intersect(&ray, &plane).unwrap();
        assert_abs_diff_eq!(t, 5.0, epsilon = 1e-6);
    }

    #[test]
    fn ray_plane_parallel_returns_none() {
        let ray = Ray::new(Vec3::new(0.0, 1.0, 0.0), dir(1.0, 0.0, 0.0));
        let plane = y_plane(0.0);
        assert!(ray_plane_intersect(&ray, &plane).is_none());
    }

    #[test]
    fn ray_plane_pointing_away_returns_none() {
        let ray = Ray::new(Vec3::new(0.0, 5.0, 0.0), dir(0.0, 1.0, 0.0));
        let plane = y_plane(0.0);
        assert!(ray_plane_intersect(&ray, &plane).is_none());
    }

    // --- ray_sphere ---
    #[test]
    fn ray_sphere_hits_unit_sphere() {
        let ray = Ray::new(Vec3::new(-5.0, 0.0, 0.0), dir(1.0, 0.0, 0.0));
        let sphere = Sphere::new(Vec3::zeros(), 1.0);
        let (t0, t1) = ray_sphere_intersect(&ray, &sphere).unwrap();
        assert_abs_diff_eq!(t0, 4.0, epsilon = 1e-5);
        assert_abs_diff_eq!(t1, 6.0, epsilon = 1e-5);
    }

    #[test]
    fn ray_sphere_miss_returns_none() {
        let ray = Ray::new(Vec3::new(-5.0, 5.0, 0.0), dir(1.0, 0.0, 0.0));
        let sphere = Sphere::new(Vec3::zeros(), 1.0);
        assert!(ray_sphere_intersect(&ray, &sphere).is_none());
    }

    #[test]
    fn ray_sphere_tangent_returns_some() {
        // Ray grazes the sphere exactly at the equator
        let ray = Ray::new(Vec3::new(-5.0, 1.0, 0.0), dir(1.0, 0.0, 0.0));
        let sphere = Sphere::new(Vec3::zeros(), 1.0);
        assert!(ray_sphere_intersect(&ray, &sphere).is_some());
    }

    // --- ray_triangle ---
    #[test]
    fn ray_triangle_hits_xy_plane_triangle() {
        let tri = Triangle::new(
            Vec3::new(-1.0, -1.0, 0.0),
            Vec3::new(1.0, -1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let ray = Ray::new(Vec3::new(0.0, 0.0, 2.0), dir(0.0, 0.0, -1.0));
        let t = ray_triangle_intersect(&ray, &tri).unwrap();
        assert_abs_diff_eq!(t, 2.0, epsilon = 1e-5);
    }

    #[test]
    fn ray_triangle_miss_returns_none() {
        let tri = Triangle::new(
            Vec3::new(-1.0, -1.0, 0.0),
            Vec3::new(1.0, -1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let ray = Ray::new(Vec3::new(10.0, 10.0, 2.0), dir(0.0, 0.0, -1.0));
        assert!(ray_triangle_intersect(&ray, &tri).is_none());
    }

    #[test]
    fn ray_triangle_parallel_returns_none() {
        let tri = Triangle::new(
            Vec3::new(-1.0, -1.0, 0.0),
            Vec3::new(1.0, -1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let ray = Ray::new(Vec3::zeros(), dir(1.0, 0.0, 0.0));
        assert!(ray_triangle_intersect(&ray, &tri).is_none());
    }

    // --- ray_aabb ---
    #[test]
    fn ray_aabb_hits_unit_box() {
        let aabb = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        let ray = Ray::new(Vec3::new(-5.0, 0.0, 0.0), dir(1.0, 0.0, 0.0));
        let (t0, t1) = ray_aabb_intersect(&ray, &aabb).unwrap();
        assert_abs_diff_eq!(t0, 4.0, epsilon = 1e-5);
        assert_abs_diff_eq!(t1, 6.0, epsilon = 1e-5);
    }

    #[test]
    fn ray_aabb_miss_returns_none() {
        let aabb = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        let ray = Ray::new(Vec3::new(-5.0, 5.0, 0.0), dir(1.0, 0.0, 0.0));
        assert!(ray_aabb_intersect(&ray, &aabb).is_none());
    }

    // --- segment_segment ---
    #[test]
    fn segment_segment_perpendicular_crossing() {
        // X-axis and Y-axis cross at origin
        let a = Segment::new(Vec3::new(-1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let b = Segment::new(Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let (pa, pb, dist) = segment_segment_closest(&a, &b);
        assert_abs_diff_eq!(dist, 0.0, epsilon = 1e-5);
        assert_abs_diff_eq!(pa, Vec3::zeros(), epsilon = 1e-5);
        assert_abs_diff_eq!(pb, Vec3::zeros(), epsilon = 1e-5);
    }

    #[test]
    fn segment_segment_parallel() {
        let a = Segment::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let b = Segment::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 0.0));
        let (_, _, dist) = segment_segment_closest(&a, &b);
        assert_abs_diff_eq!(dist, 1.0, epsilon = 1e-5);
    }

    #[test]
    fn segment_segment_degenerate_both_points() {
        let a = Segment::new(Vec3::zeros(), Vec3::zeros());
        let b = Segment::new(Vec3::new(3.0, 4.0, 0.0), Vec3::new(3.0, 4.0, 0.0));
        let (_, _, dist) = segment_segment_closest(&a, &b);
        assert_abs_diff_eq!(dist, 5.0, epsilon = 1e-5);
    }

    // --- line_line ---
    #[test]
    fn line_line_skew_lines() {
        // X-axis and Y-axis offset by 1 in Z
        let a = Line::new(
            Vec3::zeros(),
            Direction3::new_normalize(Vec3::new(1.0, 0.0, 0.0)),
        );
        let b = Line::new(
            Vec3::new(0.0, 0.0, 1.0),
            Direction3::new_normalize(Vec3::new(0.0, 1.0, 0.0)),
        );
        let (pa, pb, dist) = line_line_closest(&a, &b);
        assert_abs_diff_eq!(dist, 1.0, epsilon = 1e-5);
        assert_abs_diff_eq!(pa, Vec3::zeros(), epsilon = 1e-5);
        assert_abs_diff_eq!(pb, Vec3::new(0.0, 0.0, 1.0), epsilon = 1e-5);
    }

    #[test]
    fn line_line_parallel() {
        let a = Line::new(
            Vec3::zeros(),
            Direction3::new_normalize(Vec3::new(1.0, 0.0, 0.0)),
        );
        let b = Line::new(
            Vec3::new(0.0, 1.0, 0.0),
            Direction3::new_normalize(Vec3::new(1.0, 0.0, 0.0)),
        );
        let (_, _, dist) = line_line_closest(&a, &b);
        assert_abs_diff_eq!(dist, 1.0, epsilon = 1e-5);
    }

    // --- plane_plane ---
    #[test]
    fn plane_plane_intersect_xz_yz_gives_z_axis() {
        // XZ-plane (normal Y) and YZ-plane (normal X) intersect along Z-axis
        let xz = Plane::from_point_normal(
            Vec3::zeros(),
            Direction3::new_normalize(Vec3::new(0.0, 1.0, 0.0)),
        );
        let yz = Plane::from_point_normal(
            Vec3::zeros(),
            Direction3::new_normalize(Vec3::new(1.0, 0.0, 0.0)),
        );
        let line = plane_plane_intersect(&xz, &yz).unwrap();
        // The intersection line direction should be parallel to Z
        let dot = line.direction.dot(&Vec3::new(0.0, 0.0, 1.0)).abs();
        assert_abs_diff_eq!(dot, 1.0, epsilon = 1e-5);
        // The intersection point should lie on both planes
        assert_abs_diff_eq!(
            xz.signed_distance_to_point(&line.point),
            0.0,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            yz.signed_distance_to_point(&line.point),
            0.0,
            epsilon = 1e-5
        );
    }

    #[test]
    fn plane_plane_parallel_returns_none() {
        let a = y_plane(0.0);
        let b = y_plane(1.0);
        assert!(plane_plane_intersect(&a, &b).is_none());
    }

    // --- proptest: distance symmetry ---
    proptest! {
        #[test]
        fn point_sphere_distance_non_negative(
            x in -100.0f32..100.0, y in -100.0f32..100.0, z in -100.0f32..100.0,
            r in 0.1f32..50.0,
        ) {
            let sphere = Sphere::new(Vec3::zeros(), r);
            let d = point_sphere_distance(&Vec3::new(x, y, z), &sphere);
            prop_assert!(d >= 0.0);
        }

        #[test]
        fn point_aabb_distance_non_negative(
            x in -100.0f32..100.0, y in -100.0f32..100.0, z in -100.0f32..100.0,
        ) {
            let aabb = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
            let d = point_aabb_distance(&Vec3::new(x, y, z), &aabb);
            prop_assert!(d >= 0.0);
        }
    }
}
