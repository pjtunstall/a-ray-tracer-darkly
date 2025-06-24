use std::sync::Arc;

use crate::{
    hittables::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Direction, Point3},
};

pub struct Plane {
    pub point: Point3,
    pub normal: Direction,
    pub material: Arc<dyn Material>,
    pub offset: f64,
}

impl Plane {
    pub fn new(point: Point3, mut normal: Direction, material: Arc<dyn Material>) -> Self {
        assert!(!normal.near_zero(), "Normal vector too close to zero");
        normal = normal.normalize();
        let offset = normal.dot(&point);
        Self {
            point,
            normal,
            material,
            offset,
        }
    }

    pub fn from_span(
        point: Point3,
        mut u: Direction,
        mut v: Direction,
        material: Arc<dyn Material>,
    ) -> Self {
        u = u.normalize();
        v = v.normalize();
        let normal = u.cross(&v).normalize();
        let offset = normal.dot(&point);
        Self {
            point,
            normal,
            material,
            offset,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let denominator = self.normal.dot(&ray.direction);

        // Ray is parallel to the plane.
        if denominator.abs() < 1e-8 {
            return None;
        }

        // Hit point parameter is outside of the ray interval.
        let t = (self.offset - self.normal.dot(&ray.origin)) / denominator;
        if !ray_t.contains(t) {
            return None;
        }

        let point = ray.at(t);
        let outward_normal = self.normal;

        Some(HitRecord::new(
            point,
            outward_normal,
            t,
            self.material.clone(),
            ray,
        ))
    }
}
