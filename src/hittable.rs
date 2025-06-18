use crate::{ray::Ray, vec3::Direction};

pub struct HitRecord {
    pub point: crate::vec3::Point3,
    pub normal: crate::vec3::Direction,
    pub t: f64,
    pub front_face: Direction,
}

impl HitRecord {
    pub fn new(point: crate::vec3::Point3, normal: crate::vec3::Direction, t: f64) -> Self {
        Self {
            point,
            normal,
            t,
            front_face: Direction::new(0.0, 0.0, 0.0),
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: crate::vec3::Direction) {
        self.front_face = if ray.direction.dot(&outward_normal) < 0. {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord)
    -> bool;
}
