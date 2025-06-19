use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::Point3,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let origin_to_center = self.center - ray.origin;
        let a = ray.direction.dot(&ray.direction);
        let h = ray.direction.dot(&origin_to_center);
        let c = origin_to_center.dot(&origin_to_center) - self.radius * self.radius;
        let discriminant = h * h - a * c;
        let sqrt_d = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let root = (h - sqrt_d) / a;
        if !ray_t.surrounds(root) {
            let root = (h + sqrt_d) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;

        let mut record = HitRecord::new(point, normal, t, self.material.clone());
        record.set_face_normal(ray, record.normal);

        Some(record)
    }
}
