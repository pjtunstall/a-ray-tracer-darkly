use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Direction, Point3},
};

pub struct Cube {
    center: Point3,
    size: f64, // Half the side length (distance from center to face)
    material: Arc<dyn Material>,
    // Local coordinate system - three orthonormal vectors
    u: Direction, // Right vector
    v: Direction, // Up vector
    w: Direction, // Forward vector
}

impl Cube {
    // Create an axis-aligned cube
    pub fn new(center: Point3, size: f64, material: Arc<dyn Material>) -> Cube {
        Cube {
            center,
            size: size.max(f64::EPSILON),
            material,
            u: Direction::new(1.0, 0.0, 0.0),
            v: Direction::new(0.0, 1.0, 0.0),
            w: Direction::new(0.0, 0.0, 1.0),
        }
    }

    // Create a cube with custom orientation.
    pub fn new_oriented(
        center: Point3,
        size: f64,
        material: Arc<dyn Material>,
        u: Direction, // Right vector (should be normalized)
        v: Direction, // Up vector (should be normalized)
        w: Direction, // Forward vector (should be normalized)
    ) -> Cube {
        Cube {
            center,
            size: size.max(f64::EPSILON),
            material,
            u,
            v,
            w,
        }
    }

    // Helper function to transform a point from world space to cube local space.
    fn world_to_local(&self, point: &Point3) -> [f64; 3] {
        let offset = Point3::new(
            point.x - self.center.x,
            point.y - self.center.y,
            point.z - self.center.z,
        );

        [
            offset.x * self.u.x + offset.y * self.u.y + offset.z * self.u.z,
            offset.x * self.v.x + offset.y * self.v.y + offset.z * self.v.z,
            offset.x * self.w.x + offset.y * self.w.y + offset.z * self.w.z,
        ]
    }

    // Helper function to transform a direction from world space to cube local space.
    fn direction_to_local(&self, dir: &Direction) -> [f64; 3] {
        [
            dir.x * self.u.x + dir.y * self.u.y + dir.z * self.u.z,
            dir.x * self.v.x + dir.y * self.v.y + dir.z * self.v.z,
            dir.x * self.w.x + dir.y * self.w.y + dir.z * self.w.z,
        ]
    }

    // Helper function to transform a direction from cube local space to world space.
    fn direction_to_world(&self, local_dir: &[f64; 3]) -> Direction {
        Direction::new(
            local_dir[0] * self.u.x + local_dir[1] * self.v.x + local_dir[2] * self.w.x,
            local_dir[0] * self.u.y + local_dir[1] * self.v.y + local_dir[2] * self.w.y,
            local_dir[0] * self.u.z + local_dir[1] * self.v.z + local_dir[2] * self.w.z,
        )
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // Transform ray to cube's local coordinate system
        let local_origin = self.world_to_local(&ray.origin);
        let local_direction = self.direction_to_local(&ray.direction);

        let mut t_min = ray_t.min;
        let mut t_max = ray_t.max;
        let mut hit_normal_local = [0., 0., 0.];

        // Check intersection with each pair of parallel planes (x, y, z) in local space.
        for axis in 0..3 {
            let ray_dir_component = local_direction[axis];
            let ray_origin_component = local_origin[axis];

            if ray_dir_component.abs() < f64::EPSILON {
                // Ray is parallel to the planes
                if ray_origin_component.abs() > self.size {
                    return None; // Ray misses the cube entirely.
                }
            } else {
                // Calculate intersection distances with both planes.
                let inv_dir = 1.0 / ray_dir_component;
                let t1 = (-self.size - ray_origin_component) * inv_dir;
                let t2 = (self.size - ray_origin_component) * inv_dir;

                let (t_near, t_far) = if t1 < t2 { (t1, t2) } else { (t2, t1) };

                // Update the intersection interval.
                if t_near > t_min {
                    t_min = t_near;
                    // Determine which face we're hitting in local space.
                    hit_normal_local = [0.0, 0.0, 0.0];
                    hit_normal_local[axis] = if ray_dir_component > 0.0 { -1.0 } else { 1.0 };
                }

                if t_far < t_max {
                    t_max = t_far;
                }

                // No intersection if the intervals don't overlap.
                if t_min > t_max {
                    return None;
                }
            }
        }

        // Check if the intersection point is within our ray interval.
        if !ray_t.contains(t_min) {
            return None;
        }

        let t = t_min;
        let point = ray.at(t);

        // Transform the normal from local space back to world space.
        let world_normal = self.direction_to_world(&hit_normal_local);

        let record = HitRecord::new(point, world_normal, t, self.material.clone(), ray);

        Some(record)
    }
}
