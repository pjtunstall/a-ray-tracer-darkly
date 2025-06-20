use std::{f64::consts::PI, rc::Rc};

use rt::{
    color::Color, hittable::HittableList, material::Lambertian, sphere::Sphere, vec3::Point3,
};

pub fn wide_angle() -> HittableList {
    let material_left = Rc::new(Lambertian::new(Color::new(0., 0., 1.)));
    let material_right = Rc::new(Lambertian::new(Color::new(1., 0., 0.)));

    let r = (PI / 4.).cos();

    let left = Box::new(Sphere::new(
        Point3::new(-r, 0., -1.),
        r,
        material_left.clone(),
    ));
    let right = Box::new(Sphere::new(
        Point3::new(r, 0., -1.),
        r,
        material_right.clone(),
    ));

    let mut world = HittableList::new();
    world.add(left);
    world.add(right);

    world
}
