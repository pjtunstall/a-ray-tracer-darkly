use std::sync::Arc;

use rand::rngs::SmallRng;

use crate::{
    hittables::{HitRecord, Hittable, disk::Disk, tube::Tube},
    interval::Interval,
    materials::Material,
    ray::Ray,
    vec3::{Direction, Point3},
};

pub struct Cylinder {
    tube: Arc<Tube>,
    top: Arc<Disk>,
    bottom: Arc<Disk>,
}

impl Cylinder {
    pub fn new(
        center_of_base: Point3,
        axis: Direction,
        radius: f64,
        material_tube: Arc<dyn Material>,
        material_top: Arc<dyn Material>,
        material_bottom: Arc<dyn Material>,
    ) -> Self {
        assert!(1e-8 < axis.length(), "Axis vector is too small");
        let [u, v] = orthonormal_basis_2d(&axis.normalize());

        let top = Arc::new(Disk::new(
            center_of_base + axis,
            radius,
            u.clone(),
            v.clone(),
            material_top,
        ));
        let bottom = Arc::new(Disk::new(center_of_base, radius, u, v, material_bottom));
        let tube = Arc::new(Tube::new(center_of_base, axis, radius, material_tube));

        Self {
            tube: tube.clone(),
            top: top.clone(),
            bottom: bottom.clone(),
        }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rng: &mut SmallRng) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;

        for part in [
            &self.tube as &dyn Hittable,
            &self.top as &dyn Hittable,
            &self.bottom as &dyn Hittable,
        ] {
            if let Some(current_hit) = part.hit(ray, ray_t, rng) {
                match &closest_hit {
                    None => {
                        closest_hit = Some(current_hit);
                    }
                    Some(previous_hit) if current_hit.t < previous_hit.t => {
                        closest_hit = Some(current_hit);
                    }
                    _ => {}
                }
            }
        }

        closest_hit
    }
}

fn orthonormal_basis_2d(axis: &Direction) -> [Direction; 2] {
    let w = axis.normalize();
    let a = if w.x.abs() > 0.9 {
        Direction::new(0.0, 1.0, 0.0)
    } else {
        Direction::new(1.0, 0.0, 0.0)
    };

    let v = w.cross(&a).normalize();
    let u = w.cross(&v);
    [u, v]
}
