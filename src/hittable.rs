use crate::{interval::Interval, ray::Ray, vec3::Direction};

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
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool;
}

use std::rc::Rc;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: &Interval, record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(ray, &Interval::new(ray_t.min, closest_so_far), record) {
                hit_anything = true;
                closest_so_far = record.t;
            }
        }

        hit_anything
    }
}
