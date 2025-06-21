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
    size: f64, // Half the side length (distance from center to face).
    material: Arc<dyn Material>,
    // Local coordinate system - three orthonormal vectors.
    u: Direction, // Right vector
    v: Direction, // Up vector
    w: Direction, // Forward vector
}

impl Cube {
    // Create a cube aligned with world coordinates.
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

    fn world_to_local(&self, point: &Point3) -> [f64; 3] {
        let offset = *point - self.center;
        [
            self.u.dot(&offset),
            self.v.dot(&offset),
            self.w.dot(&offset),
        ]
    }

    fn direction_to_local(&self, dir: &Direction) -> [f64; 3] {
        [self.u.dot(dir), self.v.dot(dir), self.w.dot(dir)]
    }

    /* Slick way of writing ths change of basis:
    Direction::new(
            local_dir[0] * self.u.x + local_dir[1] * self.v.x + local_dir[2] * self.w.x,
            local_dir[0] * self.u.y + local_dir[1] * self.v.y + local_dir[2] * self.w.y,
            local_dir[0] * self.u.z + local_dir[1] * self.v.z + local_dir[2] * self.w.z,
        )
     */
    fn direction_to_world(&self, local_dir: &Direction) -> Direction {
        [self.u, self.v, self.w]
            .into_iter()
            .zip(local_dir)
            .map(|(basis, s)| basis * s)
            .reduce(|a, b| a + b)
            .unwrap()
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let local_origin = self.world_to_local(&ray.origin);
        let local_dir = self.direction_to_local(&ray.direction);

        let mut t_min = ray_t.min;
        let mut t_max = ray_t.max;
        let mut hit_axis = None;
        let mut hit_dir_sign = 0.0;

        for (axis, (&origin, &dir)) in local_origin.iter().zip(&local_dir).enumerate() {
            if dir.abs() < f64::EPSILON {
                if origin.abs() > self.size {
                    return None; // Parallel and outside slab.
                }
                continue;
            }

            let inv_dir = 1.0 / dir;
            let t1 = (-self.size - origin) * inv_dir;
            let t2 = (self.size - origin) * inv_dir;
            let (t_near, t_far) = if t1 < t2 { (t1, t2) } else { (t2, t1) };

            if t_near > t_min {
                t_min = t_near;
                hit_axis = Some(axis);
                hit_dir_sign = dir.signum();
            }

            if t_far < t_max {
                t_max = t_far;
            }

            if t_min > t_max {
                return None; // Slabs don't overlap.
            }
        }

        if !ray_t.contains(t_min) {
            return None;
        }

        let t = t_min;
        let point = ray.at(t);

        // Determine normal in local space.
        let mut normal_local = Direction::new(0., 0., 0.);
        if let Some(axis) = hit_axis {
            normal_local[axis] = -hit_dir_sign;
        }

        let world_normal = self.direction_to_world(&normal_local);

        Some(HitRecord::new(
            point,
            world_normal,
            t,
            self.material.clone(),
            ray,
        ))
    }
}
