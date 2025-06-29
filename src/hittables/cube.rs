use std::sync::Arc;

use rand::rngs::SmallRng;

use crate::{
    hittables::{HitRecord, Hittable},
    interval::Interval,
    materials::Material,
    ray::Ray,
    vec3::{Basis, Direction, Point3},
};

pub struct Cube {
    center: Point3,
    size: f64, // Half the side length (distance from center to face).
    material: Arc<dyn Material>,
    u: Direction, // Local coordinate system: three orthonormal vectors.
    v: Direction,
    w: Direction,
}

impl Cube {
    // Create a cube aligned with world coordinates.
    pub fn new(center: Point3, size: f64, material: Arc<dyn Material>) -> Cube {
        assert!(1e-8 < size, "Size is too small");
        Cube {
            center,
            size,
            u: Direction::new(1.0, 0.0, 0.0),
            v: Direction::new(0.0, 1.0, 0.0),
            w: Direction::new(0.0, 0.0, 1.0),
            material,
        }
    }

    // Create a cube with custom orientation.
    pub fn new_oriented(
        center: Point3,
        size: f64,
        orientation: &Basis,
        material: Arc<dyn Material>,
    ) -> Cube {
        assert!(1e-8 < size, "Size is too small");
        Cube {
            center,
            size,
            u: orientation.x,
            v: orientation.y,
            w: orientation.z,
            material,
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

    fn direction_to_local(&self, direction: &Direction) -> [f64; 3] {
        [
            self.u.dot(direction),
            self.v.dot(direction),
            self.w.dot(direction),
        ]
    }

    /* This function contains a slick way of writing the change of basis more plainly expresssed as follows.
    Direction::new(
            local_direction[0] * self.u.x + local_direction[1] * self.v.x + local_direction[2] * self.w.x,
            local_direction[0] * self.u.y + local_direction[1] * self.v.y + local_direction[2] * self.w.y,
            local_direction[0] * self.u.z + local_direction[1] * self.v.z + local_direction[2] * self.w.z,
        )
     */
    fn direction_to_world(&self, local_direction: &Direction) -> Direction {
        [self.u, self.v, self.w]
            .into_iter()
            .zip(local_direction)
            .map(|(basis, s)| basis * s)
            .reduce(|a, b| a + b)
            .unwrap()
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, ray_t: &Interval, _rng: &mut SmallRng) -> Option<HitRecord> {
        let local_origin = self.world_to_local(&ray.origin);
        let local_direction = self.direction_to_local(&ray.direction);

        // Start with infinite bounds: find ALL intersections first.
        let mut t_min = f64::NEG_INFINITY;
        let mut t_max = f64::INFINITY;
        let mut min_axis = None;
        let mut max_axis = None;
        let mut min_direction_sign = 0.0;
        let mut max_direction_sign = 0.0;

        for (axis, (&origin, &direction)) in local_origin.iter().zip(&local_direction).enumerate() {
            if direction.abs() < 1e-8 {
                if origin.abs() > self.size {
                    return None; // Parallel and outside the slab.
                }
                continue;
            }

            let inverse_direction = 1.0 / direction;
            let t1 = (-self.size - origin) * inverse_direction;
            let t2 = (self.size - origin) * inverse_direction;
            let (slab_min, slab_max) = if t1 < t2 { (t1, t2) } else { (t2, t1) };

            if slab_min > t_min {
                t_min = slab_min;
                min_axis = Some(axis);
                min_direction_sign = if t1 < t2 {
                    -direction.signum()
                } else {
                    direction.signum()
                };
            }
            if slab_max < t_max {
                t_max = slab_max;
                max_axis = Some(axis);
                max_direction_sign = if t1 < t2 {
                    direction.signum()
                } else {
                    -direction.signum()
                };
            }

            if t_min > t_max {
                return None; // Slabs do not overlap.
            }
        }

        // Now find which hit is within the requested ray_t interval.
        let (t, axis, direction_sign) = if ray_t.contains(t_min) {
            (t_min, min_axis, min_direction_sign)
        } else if ray_t.contains(t_max) {
            (t_max, max_axis, max_direction_sign)
        } else {
            return None; // No intersection within the requested interval.
        };

        let point = ray.at(t);
        let mut normal_local = Direction::new(0., 0., 0.);
        if let Some(axis) = axis {
            normal_local[axis] = direction_sign;
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
