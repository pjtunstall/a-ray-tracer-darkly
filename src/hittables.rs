pub mod cube;
pub mod cylinder;
pub mod disk;
pub mod plane;
pub mod quad;
pub mod sphere;
pub mod tube;

use std::sync::Arc;

use crate::{interval::Interval, material::Material, ray::Ray, vec3::Direction};

pub struct HitRecord {
    pub point: crate::vec3::Point3,
    pub normal: crate::vec3::Direction,
    pub t: f64,
    pub material: Arc<dyn Material>,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        point: crate::vec3::Point3,
        outward_normal: Direction,
        t: f64,
        material: Arc<dyn Material>,
        ray: &Ray,
    ) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.;
        Self {
            point,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            t,
            material,
            front_face,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}

impl<T: Hittable + ?Sized> Hittable for Arc<T> {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        (**self).hit(r, ray_t)
    }
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
