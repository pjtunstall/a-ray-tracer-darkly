use std::rc::Rc;

use rt::{
    color::Color,
    hittable::HittableList,
    material::{Lambertian, Metal},
    sphere::Sphere,
    vec3::Point3,
};

pub fn fuzzy_metal() -> HittableList {
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.));

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
    let right = Box::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        material_right.clone(),
    ));

    let mut world = HittableList::new();
    world.add(ground);
    world.add(center);
    world.add(left);
    world.add(right);

    world
}
