use std::sync::Arc;

use rand::rngs::SmallRng;

use crate::{
    hittables::{HitRecord, Hittable},
    interval::Interval,
    materials::Material,
    ray::Ray,
    vec3::{Direction, Point3},
};

pub struct Quad {
    pub point: Point3,
    pub u: Direction,
    pub v: Direction,
    pub material: Arc<dyn Material>,
    pub normal: Direction,
    pub offset: f64,
    w: Direction,
    inv_area: f64,
}

impl Quad {
    pub fn new(point: Point3, u: Direction, v: Direction, material: Arc<dyn Material>) -> Self {
        assert!(
            !u.near_zero() && !v.near_zero(),
            "Spanning vector(s) too close to zero"
        );

        let w = u.cross(&v);
        let area_squared = w.length_squared();
        assert!(
            area_squared > 1e-16,
            "Normal vector too close to zero: spanning vectors too close to parallel?"
        );

        let normal = w.normalize();
        let offset = normal.dot(&point);
        let inv_area = 1.0 / area_squared;

        Self {
            point,
            u,
            v,
            material,
            normal,
            offset,
            w,
            inv_area,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: &Interval, _rng: &mut SmallRng) -> Option<HitRecord> {
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
        let alpha = self.w.dot(&p.cross(&self.v)) * self.inv_area;
        let beta = self.w.dot(&self.u.cross(&p)) * self.inv_area;
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
