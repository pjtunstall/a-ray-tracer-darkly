use std::sync::Arc;

use crate::{
    color::Color,
    hittables::{HittableList, sphere::Sphere},
    materials::{Dielectric, Lambertian, Metal},
    vec3::Point3,
};

pub fn make() -> HittableList {
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_bubble = Arc::new(Dielectric::new(1.));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.));

    let ground = Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground.clone(),
    ));
    let center = Box::new(Sphere::new(
        Point3::new(0.0, 0., -1.),
        0.5,
        material_center.clone(),
    ));
    let left = Box::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        material_left.clone(),
    ));
    let bubble = Box::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.4,
        material_bubble.clone(),
    ));
    let right = Box::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        material_right.clone(),
    ));

    let mut world = HittableList::new();
    world.add(ground);
    world.add(center);
    world.add(left);
    world.add(bubble);
    world.add(right);

    world
}
