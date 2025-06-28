use std::sync::Arc;

use rand::rngs::SmallRng;

use crate::{
    hittables::{HitRecord, Hittable},
    interval::Interval,
    materials::Material,
    ray::Ray,
    vec3::{Direction, Point3},
};

pub struct Tube {
    pub center_of_base: Point3,
    pub axis: Direction,
    pub radius: f64,
    pub height: f64,
    pub material: Arc<dyn Material>,
}

impl Tube {
    pub fn new(
        center_of_base: Point3,
        mut axis: Direction,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        let height = axis.length();
        axis = axis.normalize();
        Tube {
            center_of_base,
            axis,
            radius,
            height,
            material,
        }
    }
}

impl Hittable for Tube {
    fn hit(&self, ray: &Ray, ray_t: &Interval, _rng: &mut SmallRng) -> Option<HitRecord> {
        let origin_to_center = ray.origin - self.center_of_base;
        let axis = self.axis;

        let direction_dot_axis = ray.direction.dot(&axis);
        let origin_to_center_dot_axis = origin_to_center.dot(&axis);

        let direction_perp = ray.direction - direction_dot_axis * axis;
        let origin_to_center_perp = origin_to_center - origin_to_center_dot_axis * axis;

        let a = direction_perp.dot(&direction_perp);
        let h = direction_perp.dot(&origin_to_center_perp);
        let c = origin_to_center_perp.dot(&origin_to_center_perp) - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let t0 = (-h - sqrt_d) / a;
        let t1 = (-h + sqrt_d) / a;

        for &t in &[t0, t1] {
            if !ray_t.surrounds(t) {
                continue;
            }
            let point = ray.at(t);
            let center_to_point = point - self.center_of_base;
            let height_along_axis = center_to_point.dot(&self.axis);
            if height_along_axis < 0.0 || height_along_axis > self.height {
                continue;
            }
            let projection = self.center_of_base + height_along_axis * self.axis;
            let outward_normal = (point - projection).normalize();
            return Some(HitRecord::new(
                point,
                outward_normal,
                t,
                self.material.clone(),
                ray,
            ));
        }

        None
    }
}
