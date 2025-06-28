use std::sync::Arc;

use rand::{Rng, rngs::SmallRng};

use crate::{
    color::Color,
    hittables::{HitRecord, Hittable},
    interval::Interval,
    materials::Isotropic,
    ray::Ray,
    vec3::Direction,
};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    negative_inverse_density: f64,
    phase_function: Arc<Isotropic>,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, color: Color) -> Self {
        Self {
            boundary,
            negative_inverse_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(color)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rng: &mut SmallRng) -> Option<HitRecord> {
        let rec1 = self.boundary.hit(ray, &Interval::FULL, rng)?;
        let rec2 = self
            .boundary
            .hit(ray, &Interval::new(rec1.t + 0.0001, f64::INFINITY), rng)?;

        let mut t1 = rec1.t;
        let mut t2 = rec2.t;

        if t1 < ray_t.min {
            t1 = ray_t.min;
        }
        if t2 > ray_t.max {
            t2 = ray_t.max;
        }

        if t1 >= t2 {
            return None;
        }

        if t1 < 0.0 {
            t1 = 0.0;
        }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (t2 - t1) * ray_length;

        let u: f64 = rng.random_range(0.0..1.0);
        let hit_distance = self.negative_inverse_density * u.ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = t1 + hit_distance / ray_length;
        let point = ray.at(t);

        Some(HitRecord {
            t,
            point,
            normal: Direction::new(1.0, 0.0, 0.0), // arbitrary
            front_face: true,                      // arbitrary
            material: self.phase_function.clone(),
        })
    }
}
