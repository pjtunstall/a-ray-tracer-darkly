use std::rc::Rc;

use rt::{
    color::Color,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    vec3::Point3,
};

pub fn hollow_glass() -> HittableList {
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_bubble = Rc::new(Dielectric::new(1.));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.));

    let ground = Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground.clone(),
    ));
    let center = Box::new(Sphere::new(
        Point3::new(0.1, 0., -1.),
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
