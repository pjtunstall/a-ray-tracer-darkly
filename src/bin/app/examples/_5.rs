use std::rc::Rc;

use super::_2;
use rt::{
    color::Color,
    hittable::{HitRecord, Hittable, HittableList},
    interval::Interval,
    ray::Ray,
    sphere::Sphere,
    vec3::Point3,
};

pub fn ground() {
    let mut world = HittableList::new();

    world.add(Rc::new(Sphere::new(Point3::new(0.1, 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));
}

fn ray_color<T: Hittable>(ray: &Ray, world: &T) -> Color {
    let mut record = HitRecord::default();
    if world.hit(ray, &Interval::new(0., f64::INFINITY), &mut record) {
        record.normal.to_color()
    } else {
        _2::sky(ray)
    }
}
