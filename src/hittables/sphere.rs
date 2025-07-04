use std::sync::Arc;

use rand::rngs::SmallRng;

use crate::{
    hittables::{HitRecord, Hittable},
    interval::Interval,
    materials::Material,
    ray::Ray,
    vec3::Point3,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Sphere {
        assert!(1e-8 < radius, "Radius is too small");

        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval, _rng: &mut SmallRng) -> Option<HitRecord> {
        let origin_to_center = self.center - ray.origin;
        let a = ray.direction.dot(&ray.direction);
        let h = ray.direction.dot(&origin_to_center);
        let c = origin_to_center.dot(&origin_to_center) - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut t = (h - sqrt_d) / a;
        if !ray_t.surrounds(t) {
            t = (h + sqrt_d) / a;
            if !ray_t.surrounds(t) {
                return None;
            }
        }

        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        let record = HitRecord::new(point, outward_normal, t, self.material.clone(), ray);

        Some(record)
    }
}
