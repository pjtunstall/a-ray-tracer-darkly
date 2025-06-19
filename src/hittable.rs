use std::rc::Rc;

use crate::{interval::Interval, material::Material, ray::Ray, vec3::Direction};

pub struct HitRecord {
    pub point: crate::vec3::Point3,
    pub normal: crate::vec3::Direction,
    pub t: f64,
    pub material: Rc<dyn Material>,
    pub front_face: Direction,
}

impl HitRecord {
    pub fn new(
        point: crate::vec3::Point3,
        normal: crate::vec3::Direction,
        t: f64,
        material: Rc<dyn Material>,
    ) -> Self {
        Self {
            point,
            normal,
            t,
            material,
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
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut option = None;

        for object in &self.objects {
            if let Some(record) = object.hit(ray, &Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = record.t;
                option = Some(record);
            }
        }

        option
    }
}
