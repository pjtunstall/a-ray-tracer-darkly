use std::sync::Arc;

use crate::{
    color::Color,
    hittables::{HittableList, sphere::Sphere},
    materials::Lambertian,
    vec3::Point3,
};

pub fn make() -> HittableList {
    let lambertian = Arc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });

    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0., -1.),
        0.5,
        lambertian.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        lambertian.clone(),
    )));

    world
}
