use std::sync::Arc;

use crate::{
    hittables::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Direction, Point3},
};

pub struct Quad {
    pub point: Point3,
    pub normal: Direction,
    pub material: Arc<dyn Material>,
    pub offset: f64,
    pub u: Direction,
    pub v: Direction,
}

impl Quad {
    pub fn new(point: Point3, u: Direction, v: Direction, material: Arc<dyn Material>) -> Self {
        let normal = u.cross(&v).normalize();
        let offset = normal.dot(&point);
        Self {
            point,
            normal,
            material,
            offset,
            u,
            v,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let denominator = self.normal.dot(&ray.direction);

        // Return no hit if ray is parallel to the plane.
        if denominator.abs() < 1e-8 {
            return None;
        }

        // Return no hit if point parameter is outside of the ray interval.
        let t = (self.offset - self.normal.dot(&ray.origin)) / denominator;
        if !ray_t.contains(t) {
            return None;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = ray.at(t);
        let p = intersection - self.point;
        let alpha = self.normal.dot(&p.cross(&self.v));
        let beta = self.normal.dot(&self.u.cross(&p));

        if !Interval::UNIT.contains(alpha) || !Interval::UNIT.contains(beta) {
            return None;
        }

        Some(HitRecord::new(
            intersection,
            self.normal,
            t,
            self.material.clone(),
            ray,
        ))
    }
}
