pub mod cube;
pub mod cylinder;
pub mod disk;
pub mod plane;
pub mod quad;
pub mod sphere;
pub mod tube;
pub mod volumes;

use std::sync::Arc;

use rand::rngs::SmallRng;

use crate::{interval::Interval, materials::Material, ray::Ray, vec3::Direction};

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
    fn hit(&self, ray: &Ray, ray_t: &Interval, rng: &mut SmallRng) -> Option<HitRecord>;
}

impl<T: Hittable + ?Sized> Hittable for Arc<T> {
    fn hit(&self, r: &Ray, ray_t: &Interval, rng: &mut SmallRng) -> Option<HitRecord> {
        (**self).hit(r, ray_t, rng)
    }
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rng: &mut SmallRng) -> Option<HitRecord> {
        let mut all_hits = Vec::new();

        // Collect ALL hits from all objects. This is necessary to allow a HittableList to be the boundary of a `ConstantMedium`, which creates its own interval to check entry and exit points in order to calculate the probability of hits within the volume.
        for object in &self.objects {
            if let Some(hit) = object.hit(ray, &Interval::FULL, rng) {
                all_hits.push(hit);
            }
        }

        // Sort by t value.
        all_hits.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        // Return first hit within the requested interval.
        for hit in all_hits {
            if ray_t.contains(hit.t) {
                return Some(hit);
            }
        }

        None
    }
}
