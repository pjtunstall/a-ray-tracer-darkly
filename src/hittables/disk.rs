use std::sync::Arc;

use crate::{
    hittables::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Direction, Point3},
};

pub struct Disk {
    pub point: Point3,
    pub normal: Direction,
    pub material: Arc<dyn Material>,
    pub offset: f64,
    pub u: Direction,
    pub v: Direction,
    pub radius: f64,
}

impl Disk {
    pub fn new(
        point: Point3,
        radius: f64,
        mut u: Direction,
        mut v: Direction,
        material: Arc<dyn Material>,
    ) -> Self {
        assert!(1e-8 < radius, "Radius is too small");
        assert!(
            !u.near_zero() && !v.near_zero(),
            "Spanning vector(s) too close to zero"
        );

        u = u.normalize();
        v = v.normalize();
        let normal = u.cross(&v).normalize();
        assert!(
            !normal.near_zero(),
            "Normal vector too close to zero: spanning vectors too close to parallel?"
        );

        let offset = normal.dot(&point);
        Self {
            point,
            normal,
            material,
            offset,
            u,
            v,
            radius,
        }
    }
}

impl Hittable for Disk {
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

        if alpha * alpha + beta * beta > self.radius.powf(2.) {
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
