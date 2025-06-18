use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    vec3::Point3,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval, record: &mut HitRecord) -> bool {
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
                return false;
            }
        }

        record.t = root;
        record.point = ray.at(root);
        record.normal = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, record.normal);

        true
    }
}
