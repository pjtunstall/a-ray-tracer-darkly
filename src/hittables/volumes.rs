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
    phase_function: Arc<Isotropic>, // a material
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, color: Color, density: f64) -> Self {
        Self {
            boundary,
            negative_inverse_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(color)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rng: &mut SmallRng) -> Option<HitRecord> {
        let record_1 = self.boundary.hit(ray, &Interval::FULL, rng)?; // entry
        let record_2 =
            self.boundary
                .hit(ray, &Interval::new(record_1.t + 0.0001, f64::INFINITY), rng)?; // exit

        let mut t1 = record_1.t;
        let mut t2 = record_2.t;

        t1 = t1.max(ray_t.min);
        t2 = t2.min(ray_t.max);

        if t1 >= t2 {
            return None;
        }

        t1 = t1.max(0.0);

        let distance_inside_boundary = t2 - t1;

        let u: f64 = rng.random_range(0.0..1.0);
        let hit_distance = self.negative_inverse_density * u.ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = t1 + hit_distance;
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
